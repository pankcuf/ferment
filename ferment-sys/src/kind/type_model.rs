use std::fmt::{Debug, Display, Formatter};
use std::hash::{Hash, Hasher};
use syn::{parse_quote, ParenthesizedGenericArguments, Path, PathArguments, PathSegment, Type, TypePath, TypePtr, TypeReference};
use quote::ToTokens;
use proc_macro2::TokenStream as TokenStream2;
use crate::ast::CommaPunctuated;
pub use crate::composable::{GenericBoundsModel, TypeModel};
use crate::composable::{TraitModel, TypeModeled};
use crate::composer::CommaPunctuatedNestedArguments;
use crate::context::ScopeContext;
use crate::kind::dict_type_model::DictTypeModelKind;
use crate::ext::{AsType, DictionaryType, MaybeLambdaArgs, Pop, ResolveTrait, ToType};
use crate::lang::{NameComposable, Specification};


#[derive(Clone)]
pub enum TypeModelKind {
    Dictionary(DictTypeModelKind),
    Trait(TraitModel),
    TraitType(TypeModel),
    Object(TypeModel),
    Optional(TypeModel),
    FnPointer(TypeModel),
    Bounds(GenericBoundsModel),
    Fn(TypeModel),

    Array(TypeModel),
    Slice(TypeModel),
    Tuple(TypeModel),

    Unknown(TypeModel),

    Imported(TypeModel, Path),
}

impl TypeModeled for TypeModelKind {
    fn type_model_mut(&mut self) -> &mut TypeModel {
        match self {
            TypeModelKind::TraitType(model) |
            TypeModelKind::Object(model, ..) |
            TypeModelKind::Optional(model, ..) |
            TypeModelKind::FnPointer(model) |
            TypeModelKind::Array(model) |
            TypeModelKind::Slice(model) |
            TypeModelKind::Tuple(model) |
            TypeModelKind::Imported(model, ..) |
            TypeModelKind::Unknown(model, ..) |
            TypeModelKind::Fn(model, ..) => model,
            TypeModelKind::Trait(model) => model.type_model_mut(),
            TypeModelKind::Bounds(model) => model.type_model_mut(),
            TypeModelKind::Dictionary(kind) => kind.type_model_mut()
        }
    }

    fn type_model_ref(&self) -> &TypeModel {
        match self {
            TypeModelKind::TraitType(model) |
            TypeModelKind::Object(model, ..) |
            TypeModelKind::Optional(model, ..) |
            TypeModelKind::FnPointer(model) |
            TypeModelKind::Unknown(model, ..) |
            TypeModelKind::Array(model) |
            TypeModelKind::Slice(model) |
            TypeModelKind::Tuple(model) |
            TypeModelKind::Imported(model, ..) |
            TypeModelKind::Fn(model, ..) => model,
            TypeModelKind::Trait(model) => model.type_model_ref(),
            TypeModelKind::Bounds(model) => model.type_model_ref(),
            TypeModelKind::Dictionary(kind) => kind.type_model_ref()
        }
    }

}

impl ToTokens for TypeModelKind {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        self.to_type().to_tokens(tokens)
    }
}

impl<SPEC> MaybeLambdaArgs<SPEC> for TypeModelKind
    where SPEC: Specification {
    fn maybe_lambda_arg_names(&self) -> Option<CommaPunctuated<SPEC::Name>> {
        match self.maybe_callback() {
            Some(ParenthesizedGenericArguments { inputs, ..}) =>
                Some(CommaPunctuated::from_iter(inputs.iter().enumerate().map(|(index, _ty)| SPEC::Name::unnamed_arg(index)))),
            _ => None
        }
    }
}

impl TypeModelKind {

    pub fn unknown_type(ty: Type) -> Self {
        Self::Unknown(TypeModel::new_default(ty))
    }
    pub fn unknown_type_with_nested_arguments(ty: Type, nested_arguments: CommaPunctuatedNestedArguments) -> Self {
        Self::Unknown(TypeModel::new_nested(ty, nested_arguments))
    }
    pub fn unknown_type_ref(ty: &Type) -> Self {
        Self::Unknown(TypeModel::from(ty))
    }


    pub fn is_unknown(&self) -> bool {
        match self {
            TypeModelKind::Unknown(..) => true,
            _ => false
        }
    }
    pub fn is_dictionary_opaque(&self) -> bool {
        match self {
            TypeModelKind::Dictionary(DictTypeModelKind::NonPrimitiveOpaque(..)) => true,
            _ => false
        }
    }
    pub fn is_imported(&self) -> bool {
        match self {
            TypeModelKind::Imported(..) => true,
            _ => false
        }
    }
    pub fn is_bounds(&self) -> bool {
        match self {
            TypeModelKind::Bounds(..) => true,
            _ => false
        }
    }
    pub fn is_lambda(&self) -> bool {
        match self {
            TypeModelKind::Dictionary(DictTypeModelKind::LambdaFn(..)) => true,
            _ => false
        }
    }
    pub fn is_optional(&self) -> bool {
        match self {
            TypeModelKind::Optional(..) => true,
            _ => false
        }
    }



    pub(crate) fn maybe_trait_object_maybe_model_kind_or_same(&self, source: &ScopeContext) -> TypeModelKind {
        match self {
            TypeModelKind::Trait(model) =>
                model.as_type().maybe_trait_object_model_kind(source),
            _ =>
                Some(self.clone()),
        }.unwrap_or_else(|| self.clone())
    }
    pub(crate) fn maybe_trait_object_maybe_model_kind(&self, source: &ScopeContext) -> Option<Option<TypeModelKind>> {
        match self {
            TypeModelKind::Trait(model) => model.as_type().maybe_trait_object_maybe_model_kind(source),
            _ => None
        }
    }



    pub fn maybe_trait_model_kind_or_same(&self, source: &ScopeContext) -> Option<TypeModelKind> {
        self.maybe_trait_object_maybe_model_kind(source)
            .unwrap_or_else(|| Some(self.clone()))
    }


    pub fn is_refined(&self) -> bool {
        match self {
            TypeModelKind::Imported(..) |
            TypeModelKind::Unknown(..) => false,
            other => {
                !other.nested_arguments_ref()
                    .iter()
                    .find(|arg| arg.is_refined())
                    .is_some()
            },
        }
    }
    pub fn maybe_callback<'a>(&'a self) -> Option<&'a ParenthesizedGenericArguments> {
        if let TypeModelKind::FnPointer(ty, ..) | TypeModelKind::Dictionary(DictTypeModelKind::LambdaFn(ty)) = self {
            if let Type::Path(TypePath { path: Path { segments, .. }, .. }) = ty.as_type() {
                if let Some(PathSegment { arguments, ident, ..}) = segments.last() {
                    if ident.is_lambda_fn() {
                        if let PathArguments::Parenthesized(args) = arguments {
                            return Some(args)
                        }
                    }
                }
            }
        }
        None
    }

}

impl<'a> AsType<'a> for TypeModelKind {
    fn as_type(&'a self) -> &'a Type {
        match self {
            TypeModelKind::Bounds(model) => model.as_type(),
            TypeModelKind::Dictionary(kind) => kind.as_type(),
            TypeModelKind::TraitType(model) |
            TypeModelKind::Object(model) |
            TypeModelKind::Optional(model) |
            TypeModelKind::FnPointer(model) |
            TypeModelKind::Fn(model) |
            TypeModelKind::Array(model) |
            TypeModelKind::Slice(model) |
            TypeModelKind::Tuple(model) |
            TypeModelKind::Unknown(model) => model.as_type(),
            TypeModelKind::Trait(model) => model.as_type(),
            TypeModelKind::Imported(model, _) => model.as_type(),
            // TODO: Should we use import chunk here as well?
        }
    }
}

impl ToType for TypeModelKind {
    fn to_type(&self) -> Type {
        // TODO: check others like slices
        match self {
            TypeModelKind::Imported(ty, import_path) => {
                let ty = ty.as_type();
                let path = import_path.popped();
                match ty {
                    Type::Reference(TypeReference { elem, mutability, .. }) =>
                        parse_quote!(&#mutability #path::#elem),
                    Type::Ptr(TypePtr { elem, mutability: Some(..), .. }) =>
                        parse_quote!(*mut #path::#elem),
                    Type::Ptr(TypePtr { elem, const_token: Some(..), .. }) =>
                        parse_quote!(*const #path::#elem),
                    _ =>
                        parse_quote!(#path::#ty)
                }
            },
            _ => self.as_type().clone()
        }
    }
}

impl Debug for TypeModelKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            TypeModelKind::Trait(model) =>
                format!("Trait({model})"),
            TypeModelKind::Object(ty) =>
                format!("Object({ty})"),
            TypeModelKind::Optional(ty) =>
                format!("Optional({ty})"),
            TypeModelKind::Unknown(ty) =>
               format!("Unknown({ty})"),
            TypeModelKind::TraitType(ty) =>
                format!("TraitType({ty})"),
            TypeModelKind::Bounds(gbc) =>
                format!("Bounds({gbc})"),
            TypeModelKind::Fn(ty) =>
                format!("Fn({ty})"),
            TypeModelKind::Imported(ty, import_path) =>
                format!("Imported({ty}, {})", import_path.to_token_stream()),
            TypeModelKind::Array(ty) =>
                format!("Array({ty})"),
            TypeModelKind::Slice(ty) =>
                format!("Slice({ty})"),
            TypeModelKind::Tuple(ty) =>
                format!("Tuple({ty})"),
            TypeModelKind::FnPointer(ty) =>
                format!("FnPointer({ty})"),
            TypeModelKind::Dictionary(ty) =>
                format!("Dictionary({ty})"),
        }.as_str())
    }
}

impl Display for TypeModelKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}

impl PartialEq for TypeModelKind {
    fn eq(&self, other: &Self) -> bool {
        let self_tokens = [self.to_type().to_token_stream()];
        let other_tokens = [other.to_type().to_token_stream()];
        self_tokens.iter()
            .map(|t| t.to_string())
            .zip(other_tokens.iter().map(|t| t.to_string()))
            .all(|(a, b)| a == b)
    }
}

impl Eq for TypeModelKind {}

impl Hash for TypeModelKind {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.to_type().to_token_stream().to_string().hash(state);
    }
}
