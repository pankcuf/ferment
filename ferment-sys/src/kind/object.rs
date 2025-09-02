use std::fmt::{Debug, Display, Formatter};
use quote::ToTokens;
use syn::__private::TokenStream2;
use syn::{Attribute, Item, ItemConst, ItemEnum, ItemFn, ItemImpl, ItemMod, ItemStruct, ItemTrait, ItemType, ParenthesizedGenericArguments, Signature, Type};
use syn::punctuated::Punctuated;
use crate::ast::{CommaPunctuated, PathHolder};
use crate::composable::{NestedArgument, TraitDecompositionPart1, TraitModel, TypeModel, TypeModeled};
use crate::composer::CommaPunctuatedNestedArguments;
use crate::context::ScopeContext;
use crate::kind::{GenericTypeKind, ScopeItemKind, TypeKind, TypeModelKind};
use crate::ext::{AsType, collect_bounds, MaybeLambdaArgs, ResolveAttrs, ToType, ValueReplaceScenario};
use crate::lang::{NameComposable, Specification};

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum ObjectKind {
    Type(TypeModelKind),
    Item(TypeModelKind, ScopeItemKind),
    Empty
}

impl ObjectKind {
    pub fn is_type(&self, ty: &Type) -> bool {
        match self {
            ObjectKind::Type(conversion) |
            ObjectKind::Item(conversion, _) =>
                ty.eq(conversion.as_type()),
            ObjectKind::Empty => false
        }
    }
    pub fn is_refined(&self) -> bool {
        match self {
            ObjectKind::Type(conversion) => conversion.is_refined(),
            _ => true
        }
    }
    pub fn maybe_callback<'a>(&'a self) -> Option<&'a ParenthesizedGenericArguments> {
        match self {
            ObjectKind::Type(tyc) |
            ObjectKind::Item(tyc, _) => tyc.maybe_callback(),
            ObjectKind::Empty => None
        }
    }

    pub fn maybe_trait_or_same_kind(&self, source: &ScopeContext) -> Option<TypeModelKind> {
        match self {
            ObjectKind::Item(.., ScopeItemKind::Fn(..)) =>
                source.maybe_parent_trait_or_regular_model_kind(),
            ObjectKind::Type(ref type_model_kind) |
            ObjectKind::Item(ref type_model_kind, ..) =>
                type_model_kind.maybe_trait_model_kind_or_same(source),
            ObjectKind::Empty => None
        }
    }
    pub fn maybe_fn_or_trait_or_same_kind(&self, source: &ScopeContext) -> Option<TypeModelKind> {
        match self {
            ObjectKind::Item(.., ScopeItemKind::Fn(..)) =>
                source.maybe_parent_trait_or_regular_model_kind(),
            ObjectKind::Type(ref type_model_kind) |
            ObjectKind::Item(ref type_model_kind, ..) =>
                type_model_kind.maybe_trait_object_maybe_model_kind(source)
                    .unwrap_or_else(|| Some(type_model_kind.clone())),
            ObjectKind::Empty => None
        }
    }

    pub fn maybe_fn_or_trait_or_same_kind2(&self, source: &ScopeContext) -> Option<TypeModelKind> {
        match self {
            ObjectKind::Item(.., ScopeItemKind::Fn(..)) =>
                source.maybe_parent_trait_or_regular_model_kind(),
            ObjectKind::Type(ref type_model_kind) |
                 ObjectKind::Item(ref type_model_kind, ..) =>
                type_model_kind.maybe_trait_model_kind_or_same(source),
            _ => None,
        }
    }

    pub fn maybe_scope_item(&self) -> Option<&ScopeItemKind> {
        match self {
            ObjectKind::Item(_, scope_item) => Some(scope_item),
            _ => None
        }
    }
}

impl<SPEC> MaybeLambdaArgs<SPEC> for ObjectKind
    where SPEC: Specification {
    fn maybe_lambda_arg_names(&self) -> Option<CommaPunctuated<SPEC::Name>> {
        match self.maybe_callback() {
            Some(ParenthesizedGenericArguments { inputs, ..}) =>
                Some(CommaPunctuated::from_iter(inputs.iter().enumerate().map(|(index, _ty)| SPEC::Name::unnamed_arg(index)))),
            _ => None
        }
    }
}

impl ValueReplaceScenario for ObjectKind {
    fn should_replace_with(&self, other: &Self) -> bool {
        // println!("ObjectKind ::: should_replace_with:::: {}: {}", self, other);
        match (self, other) {
            (_, ObjectKind::Item(..)) => true,
            (ObjectKind::Type(self_ty), ObjectKind::Type(candidate_ty)) => {
                // let should = !self_ty.is_refined() && candidate_ty.is_refined();
                let should = !self_ty.is_refined() || candidate_ty.is_bounds();
                // let should = !self_ty.is_refined() && candidate_ty.is_refined() || self_ty.is_tuple();
                // println!("MERGE? {} [{}]:\n\t {} [{}]: {}", should, self_ty.is_refined(), self_ty, candidate_ty.is_refined(), candidate_ty);
                // MERGE? false [true]:
                //     Bounds(GenericBoundsModel(ty: $Ty(DC, []), bounds: Fn (dash_spv_platform :: FFIContext , platform_value :: Identifier) -> Result < Option < std :: sync :: Arc < dpp :: data_contract :: DataContract > > , drive_proof_verifier :: error :: ContextProviderError >,Send,Sync, predicates: , nested_args: )) [true]: Bounds(GenericBoundComposition(ty: $Ty(DC, []), bounds: Fn (dash_spv_platform :: FFIContext , platform_value :: types :: identifier :: Identifier) -> Result < Option < std :: sync :: Arc < dpp :: data_contract :: DataContract > > , drive_proof_verifier :: error :: ContextProviderError >,Send,Sync, predicates: , nested_args: ))
                should
            }
            _ => false
        }
    }

}



impl ToTokens for ObjectKind {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        self.maybe_type().to_tokens(tokens)
    }
}
impl Debug for ObjectKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ObjectKind::Type(tc) =>
                f.write_str(format!("Type({})", tc).as_str()),
            ObjectKind::Item(tc, item) =>
                f.write_str(format!("Item({}, {})", tc, item).as_str()),
            ObjectKind::Empty =>
                f.write_str("Empty"),
        }
    }
}

impl Display for ObjectKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

impl ObjectKind {
    pub fn replace_composition_type(&mut self, with_ty: Type) {
        match self {
            ObjectKind::Type(ty) => ty.replace_model_type(with_ty),
            // actually it has no sense since items can never be imported where they are defined
            ObjectKind::Item(ty, _) => ty.replace_model_type(with_ty),
            ObjectKind::Empty => {}
        }
    }

    pub fn new_item(ty: TypeModelKind, item: ScopeItemKind) -> ObjectKind {
        ObjectKind::Item(ty, item)
    }
    pub fn new_obj_item(ty: TypeModel, item: ScopeItemKind) -> ObjectKind {
        ObjectKind::Item(TypeModelKind::Object(ty), item)
    }
    pub fn maybe_type_model_kind_ref(&self) -> Option<&TypeModelKind> {
        match self {
            ObjectKind::Type(tyc) |
            ObjectKind::Item(tyc, ..) => Some(tyc),
            ObjectKind::Empty => None
        }
    }
    pub fn maybe_type_model_kind(&self) -> Option<TypeModelKind> {
        self.maybe_type_model_kind_ref().cloned()
    }
    pub fn maybe_generic_type_kind(&self) -> Option<GenericTypeKind> {
        match self.maybe_type_model_kind_ref() {
            Some(kind) => match TypeKind::from(kind.to_type()) {
                TypeKind::Generic(kind) => Some(kind),
                _ => None
            }
            _ => None
        }
    }
    pub fn maybe_type_ref(&self) -> Option<&Type> {
        self.maybe_type_model_ref()
            .map(TypeModel::as_type)
    }
    pub fn maybe_type_model_ref(&self) -> Option<&TypeModel> {
        self.maybe_type_model_kind_ref()
            .map(TypeModelKind::type_model_ref)
    }
    pub fn maybe_type(&self) -> Option<Type> {
        self.maybe_type_model_kind_ref()
            .map(TypeModelKind::to_type)
    }
}

impl TryFrom<(&Item, &PathHolder)> for ObjectKind {
    type Error = ();

    fn try_from((value, scope): (&Item, &PathHolder)) -> Result<Self, Self::Error> {
        match value {
            Item::Trait(ItemTrait { ident, generics, items, supertraits, .. }) => {
                Ok(ObjectKind::new_item(
                    TypeModelKind::Trait(TraitModel::new(TypeModel::new(ident.to_type(), Some(generics.clone()), Punctuated::new()), TraitDecompositionPart1::from_trait_items(ident, items), collect_bounds(supertraits))),
                    ScopeItemKind::Item(value.clone(), scope.clone())))
            },
            Item::Struct(ItemStruct { ident, generics, .. }) => {
                Ok(ObjectKind::new_obj_item(
                    TypeModel::new(ident.to_type(), Some(generics.clone()), Punctuated::new()),
                    ScopeItemKind::Item(value.clone(), scope.clone())))
            },
            Item::Enum(ItemEnum { ident, generics, .. }) => {
                Ok(ObjectKind::new_obj_item(
                    TypeModel::new(ident.to_type(), Some(generics.clone()), Punctuated::new()),
                    ScopeItemKind::Item(value.clone(), scope.clone())))
            },
            Item::Type(ItemType { ident, generics, ty, .. }) => {
                let conversion = ScopeItemKind::Item(value.clone(), scope.clone());
                let obj = match &**ty {
                    Type::BareFn(..) => {
                        let mut nested_arguments = CommaPunctuatedNestedArguments::new();

                        nested_arguments.push(NestedArgument::Object(ObjectKind::Type(TypeModelKind::Fn(TypeModel::new(*ty.clone(), Some(generics.clone()), CommaPunctuated::new())))));

                        ObjectKind::Item(TypeModelKind::FnPointer(TypeModel::new(ident.to_type(), Some(generics.clone()), nested_arguments)/*,
                            TypeComposition::new(*ty.clone(), Some(generics.clone()), Punctuated::new())*/), conversion)
                    },
                    _ => ObjectKind::new_obj_item(TypeModel::new(ident.to_type(), Some(generics.clone()), Punctuated::new()), conversion)
                };
                Ok(obj)
            },
            Item::Const(ItemConst { ident, .. }) => {
                Ok(ObjectKind::new_obj_item(
                    TypeModel::new(ident.to_type(), None, Punctuated::new()),
                    ScopeItemKind::Item(value.clone(), scope.clone())))
            },
            Item::Impl(ItemImpl { self_ty, generics, .. }) => {
                Ok(ObjectKind::new_obj_item(
                    TypeModel::new(*self_ty.clone(), Some(generics.clone()), Punctuated::new()),
                    ScopeItemKind::Item(value.clone(), scope.clone())))
            },
            Item::Fn(ItemFn { sig: Signature { ident, generics, .. }, .. }) => {
                Ok(ObjectKind::new_obj_item(
                    TypeModel::new(ident.to_type(), Some(generics.clone()), Punctuated::new()),
                    ScopeItemKind::Item(value.clone(), scope.clone())))
                    // ScopeItemKind::Fn(value.clone())))
            },
            Item::Mod(ItemMod { ident, .. }) => {
                Ok(ObjectKind::new_item(
                    TypeModelKind::unknown_type(ident.to_type()),
                    ScopeItemKind::Item(value.clone(), scope.clone())))

            }
            _ => Err(()),
        }
    }
}

impl ResolveAttrs for ObjectKind {
    fn resolve_attrs(&self) -> Vec<Option<Attribute>> {
        match self {
            ObjectKind::Item(_, item) =>
                item.resolve_attrs(),
            _ => vec![],
        }
    }
}