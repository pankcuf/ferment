use std::fmt::{Debug, Display, Formatter};
use quote::ToTokens;
use syn::__private::TokenStream2;
use syn::{Attribute, Generics, Item, ItemConst, ItemEnum, ItemFn, ItemImpl, ItemMod, ItemStruct, ItemTrait, ItemType, ParenthesizedGenericArguments, Path, PathSegment, QSelf, Signature, Type};
use syn::punctuated::Punctuated;
use syn::token::PathSep;
use crate::ast::{CommaPunctuated, PathHolder};
use crate::composable::{NestedArgument, TraitDecompositionPart1, TraitModel, TypeModel, TypeModeled};
use crate::composer::CommaPunctuatedNestedArguments;
use crate::context::ScopeContext;
use crate::kind::{DictFermentableModelKind, DictTypeModelKind, GenericBoundsModel, GenericTypeKind, GroupModelKind, ScopeItemKind, SmartPointerModelKind, TypeKind, TypeModelKind};
use crate::ext::{AsType, collect_bounds, MaybeLambdaArgs, ResolveAttrs, ToType, ValueReplaceScenario, handle_type_path_model};
use crate::lang::{NameComposable, Specification};

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum ObjectKind {
    Type(TypeModelKind),
    Item(TypeModelKind, ScopeItemKind),
    Empty
}

impl ObjectKind {
    pub fn model_type<T: Fn(TypeModel) -> TypeModelKind>(model_kind_composer: T, model: TypeModel) -> Self {
        Self::Type(model_kind_composer(model))
    }
    pub fn model_item<T: Fn(TypeModel) -> TypeModelKind>(model_kind_composer: T, model: TypeModel, item: ScopeItemKind) -> Self {
        Self::Item(model_kind_composer(model), item)
    }

    pub fn object_model_type(model: TypeModel) -> Self {
        Self::model_type(TypeModelKind::Object, model)
    }
    pub fn array_model_type(model: TypeModel) -> Self {
        Self::model_type(TypeModelKind::Array, model)
    }
    pub fn slice_model_type(model: TypeModel) -> Self {
        Self::model_type(TypeModelKind::Slice, model)
    }
    pub fn tuple_model_type(model: TypeModel) -> Self {
        Self::model_type(TypeModelKind::Tuple, model)
    }
    pub fn optional_model_type(model: TypeModel) -> Self {
        Self::model_type(TypeModelKind::Optional, model)
    }
    pub fn trait_model_type(model: TypeModel) -> Self {
        Self::model_type(TypeModelKind::TraitType, model)
    }
    pub fn fn_model_type(model: TypeModel) -> Self {
        Self::model_type(TypeModelKind::Fn, model)
    }
    pub fn fn_pointer_model_type(model: TypeModel) -> Self {
        Self::model_type(TypeModelKind::FnPointer, model)
    }
    pub fn unknown_model_type(model: TypeModel) -> Self {
        Self::model_type(TypeModelKind::Unknown, model)
    }
    pub fn unknown_model_type_path(qself: Option<QSelf>, sep: Option<PathSep>, segments: Punctuated<PathSegment, PathSep>, nested_arguments: CommaPunctuatedNestedArguments) -> Self {
        Self::model_type(TypeModelKind::Unknown, handle_type_path_model(qself, sep, segments, nested_arguments))
    }
    pub fn unknown_type(ty: Type) -> Self {
        Self::Type(TypeModelKind::unknown_type(ty))
    }
    pub fn unknown_type_with_nested_arguments(ty: Type, nested_arguments: CommaPunctuatedNestedArguments) -> Self {
        Self::Type(TypeModelKind::unknown_type_with_nested_arguments(ty, nested_arguments))
    }
    pub fn bounds_type(model: GenericBoundsModel) -> Self {
        Self::Type(TypeModelKind::Bounds(model))
    }
    pub fn dict_type(kind: DictTypeModelKind) -> Self {
        Self::Type(TypeModelKind::Dictionary(kind))
    }
    pub fn imported_model_type(model: TypeModel, path: Path) -> Self {
        Self::Type(TypeModelKind::Imported(model, path))
    }
    pub fn primitive_type(ty: Type) -> Self {
        Self::dict_type(DictTypeModelKind::Primitive(TypeModel::new_non_nested(ty, None)))
    }
    pub fn non_primitive_fermentable_type(kind: DictFermentableModelKind) -> Self {
        Self::dict_type(DictTypeModelKind::NonPrimitiveFermentable(kind))
    }
    pub fn group_type(kind: GroupModelKind) -> Self {
        Self::non_primitive_fermentable_type(DictFermentableModelKind::Group(kind))
    }
    pub fn lambda_fn_model_type(model: TypeModel) -> Self {
        Self::dict_type(DictTypeModelKind::LambdaFn(model))
    }
    pub fn smart_ptr_type(kind: SmartPointerModelKind) -> Self {
        Self::non_primitive_fermentable_type(DictFermentableModelKind::SmartPointer(kind))
    }

    pub fn str_type(ty: Type) -> Self {
        Self::non_primitive_fermentable_type(DictFermentableModelKind::Str(TypeModel::new_non_nested(ty, None)))
    }
    pub fn string_type(ty: Type) -> Self {
        Self::non_primitive_fermentable_type(DictFermentableModelKind::String(TypeModel::new_non_nested(ty, None)))
    }
    pub fn i128_type(ty: Type) -> Self {
        Self::non_primitive_fermentable_type(DictFermentableModelKind::I128(TypeModel::new_non_nested(ty, None)))
    }
    pub fn u128_type(ty: Type) -> Self {
        Self::non_primitive_fermentable_type(DictFermentableModelKind::U128(TypeModel::new_non_nested(ty, None)))
    }

    pub fn new_item(ty: TypeModelKind, item: ScopeItemKind) -> Self {
        Self::Item(ty, item)
    }
    fn new_obj_item(ty: TypeModel, item: ScopeItemKind) -> Self {
        Self::model_item(TypeModelKind::Object, ty, item)
    }
    pub fn new_generic_obj_item(ty: Type, generics: Generics, nested_arguments: CommaPunctuatedNestedArguments, item: ScopeItemKind) -> Self {
        Self::new_obj_item(TypeModel::new_generic(ty, generics, nested_arguments), item)
    }
    pub fn new_generic_non_nested_obj_item(ty: Type, generics: Generics, item: ScopeItemKind) -> Self {
        Self::new_obj_item(TypeModel::new_generic_non_nested(ty, generics), item)
    }
    pub fn new_trait_item(model: TraitModel, item: ScopeItemKind) -> Self {
        Self::new_item(TypeModelKind::Trait(model), item)
    }
    pub fn new_fn_item(ty: TypeModel, item: ScopeItemKind) -> Self {
        Self::model_item(TypeModelKind::Fn, ty, item)
    }
    pub fn new_fn_pointer_item(ty: TypeModel, item: ScopeItemKind) -> Self {
        Self::model_item(TypeModelKind::FnPointer, ty, item)
    }

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
        let item_kind = ScopeItemKind::item_ref(value, scope);
        match value {
            Item::Trait(ItemTrait { ident, generics, items, supertraits, .. }) =>
                Ok(ObjectKind::new_trait_item(TraitModel::new(TypeModel::new_generic_non_nested(ident.to_type(), generics.clone()), TraitDecompositionPart1::from_trait_items(ident, items), collect_bounds(supertraits)), item_kind)),
            Item::Const(ItemConst { ident, generics, .. }) |
            Item::Struct(ItemStruct { ident, generics, .. }) |
            Item::Enum(ItemEnum { ident, generics, .. }) |
            Item::Fn(ItemFn { sig: Signature { ident, generics, .. }, .. }) =>
                Ok(ObjectKind::new_generic_non_nested_obj_item(ident.to_type(), generics.clone(), item_kind)),
            Item::Type(ItemType { ident, generics, ty, .. }) =>
                Ok(match &**ty {
                    Type::BareFn(..) =>
                        ObjectKind::new_fn_pointer_item(TypeModel::new_generic(ident.to_type(), generics.clone(), CommaPunctuatedNestedArguments::from_iter([NestedArgument::Object(ObjectKind::fn_model_type(TypeModel::new_generic_non_nested(*ty.clone(), generics.clone())))])), item_kind),
                    _ =>
                        ObjectKind::new_generic_non_nested_obj_item(ident.to_type(), generics.clone(), item_kind)
                }),
            Item::Impl(ItemImpl { self_ty, generics, .. }) =>
                Ok(ObjectKind::new_generic_non_nested_obj_item(*self_ty.clone(), generics.clone(), item_kind)),
            Item::Mod(ItemMod { ident, .. }) =>
                Ok(ObjectKind::new_item(TypeModelKind::unknown_type(ident.to_type()), item_kind)),
            _ =>
                Err(()),
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