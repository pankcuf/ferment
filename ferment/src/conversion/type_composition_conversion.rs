use std::fmt::{Debug, Display, Formatter};
use std::hash::{Hash, Hasher};
use syn::{ParenthesizedGenericArguments, parse_quote, Path, PathArguments, PathSegment, Type, TypePath, TypeReference};
use quote::ToTokens;
use proc_macro2::{TokenStream as TokenStream2};
use crate::ast::CommaPunctuated;
pub use crate::composable::{GenericBoundsModel, TypeModel, TraitDecompositionPart1};
use crate::composable::TypeModeled;
use crate::context::ScopeContext;
use crate::ext::{AsType, DictionaryType, Pop, ToType};
use crate::presentation::Name;

#[derive(Clone)]
pub enum GroupModelKind {
    Result(TypeModel),
    Vec(TypeModel),
    Map(TypeModel),
    BTreeSet(TypeModel),
    HashSet(TypeModel),
    IndexMap(TypeModel)
}


impl<'a> AsType<'a> for GroupModelKind {
    fn as_type(&'a self) -> &'a Type {
        self.type_model_ref().as_type()
    }
}

impl TypeModeled for GroupModelKind {
    fn type_model_mut(&mut self) -> &mut TypeModel {
        match self {
            GroupModelKind::Result(model) |
            GroupModelKind::Vec(model) |
            GroupModelKind::Map(model) |
            GroupModelKind::BTreeSet(model) |
            GroupModelKind::HashSet(model) |
            GroupModelKind::IndexMap(model) => model,
        }
    }

    fn type_model_ref(&self) -> &TypeModel {
        match self {
            GroupModelKind::Result(model) |
            GroupModelKind::Vec(model) |
            GroupModelKind::Map(model) |
            GroupModelKind::BTreeSet(model) |
            GroupModelKind::HashSet(model) |
            GroupModelKind::IndexMap(model) => model
        }
    }
}

impl Debug for GroupModelKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            GroupModelKind::Result(model) =>
                format!("Result({})", model),
            GroupModelKind::Vec(model) =>
                format!("Vec({})", model),
            GroupModelKind::Map(model) =>
                format!("Map({})", model),
            GroupModelKind::BTreeSet(model) =>
                format!("BTreeSet({})", model),
            GroupModelKind::HashSet(model) =>
                format!("HashSet({})", model),
            GroupModelKind::IndexMap(model) =>
                format!("IndexMap({})", model),
        }.as_str())
    }
}

impl Display for GroupModelKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}

#[derive(Clone)]
pub enum SmartPointerModelKind {
    Box(TypeModel),
    Arc(TypeModel),
    Rc(TypeModel),
    Mutex(TypeModel),
    RwLock(TypeModel),
    RefCell(TypeModel),
    Pin(TypeModel)
}

impl<'a> AsType<'a> for SmartPointerModelKind {
    fn as_type(&'a self) -> &'a Type {
        self.type_model_ref().as_type()
    }
}

impl TypeModeled for SmartPointerModelKind {
    fn type_model_mut(&mut self) -> &mut TypeModel {
        match self {
            SmartPointerModelKind::Box(model) |
            SmartPointerModelKind::Arc(model) |
            SmartPointerModelKind::Rc(model) |
            SmartPointerModelKind::Mutex(model) |
            SmartPointerModelKind::RwLock(model) |
            SmartPointerModelKind::RefCell(model) |
            SmartPointerModelKind::Pin(model) => model
        }
    }
    fn type_model_ref(&self) -> &TypeModel {
        match self {
            SmartPointerModelKind::Box(model) |
            SmartPointerModelKind::Arc(model) |
            SmartPointerModelKind::Rc(model) |
            SmartPointerModelKind::Mutex(model) |
            SmartPointerModelKind::RwLock(model) |
            SmartPointerModelKind::RefCell(model) |
            SmartPointerModelKind::Pin(model) => model
        }
    }
}


impl Debug for SmartPointerModelKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            SmartPointerModelKind::Arc(model) =>
                format!("Arc({})", model),
            SmartPointerModelKind::Rc(model) =>
                format!("Rc({})", model),
            SmartPointerModelKind::Mutex(model) =>
                format!("Mutex({})", model),
            SmartPointerModelKind::RwLock(model) =>
                format!("RwLock({})", model),
            SmartPointerModelKind::RefCell(model) =>
                format!("RefCell({})", model),
            SmartPointerModelKind::Pin(model) =>
                format!("Pin({})", model),
            SmartPointerModelKind::Box(model) =>
                format!("Box({})", model),
        }.as_str())
    }
}

impl Display for SmartPointerModelKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}

#[derive(Clone)]
pub enum DictFermentableModelKind {
    SmartPointer(SmartPointerModelKind),
    Group(GroupModelKind),
    String(TypeModel),
    Str(TypeModel),
    Other(TypeModel),
    Digit128(TypeModel),
}

impl<'a> AsType<'a> for DictFermentableModelKind {
    fn as_type(&'a self) -> &'a Type {
        match self {
            DictFermentableModelKind::SmartPointer(kind) => kind.as_type(),
            DictFermentableModelKind::Group(kind) => kind.as_type(),
            DictFermentableModelKind::Str(model) |
            DictFermentableModelKind::String(model) |
            DictFermentableModelKind::Other(model) |
            DictFermentableModelKind::Digit128(model) => model.as_type()
        }
    }
}

impl TypeModeled for DictFermentableModelKind {
    fn type_model_mut(&mut self) -> &mut TypeModel {
        match self {
            DictFermentableModelKind::SmartPointer(kind) => kind.type_model_mut(),
            DictFermentableModelKind::Group(kind) => kind.type_model_mut(),
            DictFermentableModelKind::Str(model) |
            DictFermentableModelKind::String(model) |
            DictFermentableModelKind::Digit128(model) |
            DictFermentableModelKind::Other(model) => model
        }
    }
    fn type_model_ref(&self) -> &TypeModel {
        match self {
            DictFermentableModelKind::SmartPointer(kind) => kind.type_model_ref(),
            DictFermentableModelKind::Group(kind) => kind.type_model_ref(),
            DictFermentableModelKind::Str(model) |
            DictFermentableModelKind::String(model) |
            DictFermentableModelKind::Digit128(model) |
            DictFermentableModelKind::Other(model) => model
        }
    }
}
impl Debug for DictFermentableModelKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            DictFermentableModelKind::SmartPointer(model) =>
                format!("SmartPointer({})", model),
            DictFermentableModelKind::Group(model) =>
                format!("Group({})", model),
            DictFermentableModelKind::Str(model) =>
                format!("Str({})", model),
            DictFermentableModelKind::String(model) =>
                format!("String({})", model),
            DictFermentableModelKind::Other(model) =>
                format!("Other({})", model),
            DictFermentableModelKind::Digit128(model) =>
                format!("Digit128({})", model),
        }.as_str())
    }
}

impl Display for DictFermentableModelKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}

#[derive(Clone)]
pub enum DictTypeModelKind {
    Primitive(TypeModel),
    LambdaFn(TypeModel),
    NonPrimitiveFermentable(DictFermentableModelKind),
    NonPrimitiveOpaque(TypeModel),
}

impl<'a> AsType<'a> for DictTypeModelKind {
    fn as_type(&'a self) -> &'a Type {
        match self {
            DictTypeModelKind::Primitive(model) |
            DictTypeModelKind::LambdaFn(model) |
            DictTypeModelKind::NonPrimitiveOpaque(model) => model.as_type(),
            DictTypeModelKind::NonPrimitiveFermentable(kind) => kind.as_type(),
        }
    }
}

impl TypeModeled for DictTypeModelKind {
    fn type_model_mut(&mut self) -> &mut TypeModel {
        match self {
            DictTypeModelKind::Primitive(model) |
            DictTypeModelKind::NonPrimitiveOpaque(model) |
            DictTypeModelKind::LambdaFn(model) => model,
            DictTypeModelKind::NonPrimitiveFermentable(kind) => kind.type_model_mut()
        }
    }
    fn type_model_ref(&self) -> &TypeModel {
        match self {
            DictTypeModelKind::Primitive(model) |
            DictTypeModelKind::NonPrimitiveOpaque(model) |
            DictTypeModelKind::LambdaFn(model) => model,
            DictTypeModelKind::NonPrimitiveFermentable(kind) => kind.type_model_ref()
        }
    }
}
impl Debug for DictTypeModelKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            DictTypeModelKind::Primitive(ty) =>
                format!("Primitive({})", ty),
            DictTypeModelKind::NonPrimitiveFermentable(ty) =>
                format!("NonPrimitiveFermentable({})", ty),
            DictTypeModelKind::NonPrimitiveOpaque(ty) =>
                format!("NonPrimitiveOpaque({})", ty),
            DictTypeModelKind::LambdaFn(ty) =>
                format!("LambdaFn({})", ty),
        }.as_str())
    }
}

impl Display for DictTypeModelKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}

#[derive(Clone)]
pub enum TypeModelKind {
    Dictionary(DictTypeModelKind),
    Trait(TypeModel, TraitDecompositionPart1, Vec<Path>),
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
            TypeModelKind::Trait(model, ..) |
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
            TypeModelKind::Bounds(model) => model.type_model_mut(),
            TypeModelKind::Dictionary(kind) => kind.type_model_mut()
        }
    }

    fn type_model_ref(&self) -> &TypeModel {
        match self {
            TypeModelKind::Trait(model, ..) |
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

impl TypeModelKind {

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
    pub fn maybe_lambda_args(&self) -> Option<CommaPunctuated<Name>> {
        match self.maybe_callback() {
            Some(ParenthesizedGenericArguments { inputs, ..}) =>
                Some(CommaPunctuated::from_iter(inputs.iter().enumerate().map(|(index, _ty)| Name::UnnamedArg(index)))),
            _ => None
        }
    }

    pub fn maybe_trait_model_kind_or_same(&self, source: &ScopeContext) -> Option<TypeModelKind> {
        match self {
            TypeModelKind::Trait(ty, ..) => {
                println!("TypeModelKind:: (Trait Conversion): {}", ty);
                ty.maybe_trait_object_maybe_model_kind(source)
            },
            _ => {
                None
            },
        }.unwrap_or_else(|| {
            println!("TypeModelKind (Non-Trait Conversion): {}", self.to_token_stream());
            Some(self.clone())
        })
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
            if let Type::Path(TypePath { path, .. }) = ty.as_type() {
                if let Some(PathSegment { arguments, ident: last_ident, ..}) = &path.segments.last() {
                    if last_ident.is_lambda_fn() {
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
            TypeModelKind::Trait(model, _, _) |
            TypeModelKind::TraitType(model) |
            TypeModelKind::Object(model) |
            TypeModelKind::Optional(model) |
            TypeModelKind::FnPointer(model) |
            TypeModelKind::Fn(model) |
            TypeModelKind::Array(model) |
            TypeModelKind::Slice(model) |
            TypeModelKind::Tuple(model) |
            TypeModelKind::Unknown(model) => model.as_type(),
            TypeModelKind::Imported(model, _) => model.as_type(),
            // TODO: Should we use import chunk here as well?
        }
    }
}

impl ToType for TypeModelKind {
    fn to_type(&self) -> Type {
        match self {
            TypeModelKind::Imported(ty, import_path) => {
                let ty = ty.as_type();
                let path = import_path.popped();
                match ty {
                    Type::Reference(TypeReference { elem, mutability, .. }) => {
                        parse_quote!(&#mutability #path::#elem)
                    },
                    _ => parse_quote!(#path::#ty)
                }
            },
            _ => self.as_type().clone()
        }
    }
}

impl Debug for TypeModelKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            TypeModelKind::Trait(ty, _decomposition, _super_bounds) =>
                format!("Trait({})", ty),
            TypeModelKind::Object(ty) =>
                format!("Object({})", ty),
            TypeModelKind::Optional(ty) =>
                format!("Optional({})", ty),
            TypeModelKind::Unknown(ty) =>
               format!("Unknown({})", ty),
            TypeModelKind::TraitType(ty) =>
                format!("TraitType({})", ty),
            TypeModelKind::Bounds(gbc) =>
                format!("Bounds({})", gbc),
            TypeModelKind::Fn(ty) =>
                format!("Fn({})", ty),
            TypeModelKind::Imported(ty, import_path) =>
                format!("Imported({}, {})", ty, import_path.to_token_stream()),
            TypeModelKind::Array(ty) =>
                format!("Array({})", ty),
            TypeModelKind::Slice(ty) =>
                format!("Slice({})", ty),
            TypeModelKind::Tuple(ty) =>
                format!("Tuple({})", ty),
            TypeModelKind::FnPointer(ty) =>
                format!("FnPointer({})", ty),
            TypeModelKind::Dictionary(ty) =>
                format!("Dictionary({})", ty),
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
