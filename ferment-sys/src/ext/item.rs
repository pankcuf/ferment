use syn::{AngleBracketedGenericArguments, Attribute, GenericArgument, GenericParam, Generics, Item, ItemConst, ItemEnum, ItemExternCrate, ItemFn, ItemImpl, ItemMacro, ItemMacro2, ItemMod, ItemStatic, ItemStruct, ItemTrait, ItemTraitAlias, ItemType, ItemUnion, ParenthesizedGenericArguments, Path, PathArguments, PathSegment, ReturnType, Signature, TraitBound, Type, TypeArray, TypeParam, TypeParamBound, TypePath, TypePtr, TypeReference, TypeTraitObject};
use proc_macro2::{Ident, Span, TokenStream as TokenStream2};
use syn::punctuated::Punctuated;
use quote::{quote, ToTokens};
use crate::ast::{AddPunctuated, CommaPunctuated};
use crate::composable::NestedArgument;
use crate::composer::CommaPunctuatedNestedArguments;
use crate::conversion::TypeKind;
use crate::ext::VisitScopeType;
use crate::tree::ScopeTreeExportID;

pub trait ItemExtension {
    fn scope_tree_export_id(&self) -> ScopeTreeExportID;
    fn maybe_attrs(&self) -> Option<&Vec<Attribute>>;
    fn maybe_ident(&self) -> Option<&Ident>;
    fn ident_string(&self) -> String {
        self.maybe_ident().map_or("(None)".to_string(), Ident::to_string)
    }
    fn maybe_generics(&self) -> Option<&Generics>;

    fn maybe_generic_bound_for_path(&self, path: &Path) -> Option<(Generics, TypeParam)> {
        self.maybe_generics()
            .and_then(|generics|
                maybe_generic_type_bound(path, generics)
                    .map(|bound| (generics.clone(), bound.clone())))
    }
}


impl ItemExtension for Item {
    fn scope_tree_export_id(&self) -> ScopeTreeExportID {
        match self {
            Item::Mod(ItemMod { ident, .. }, ..) |
            Item::Struct(ItemStruct { ident, .. }, ..) |
            Item::Enum(ItemEnum { ident, .. }, ..) |
            Item::Type(ItemType { ident, .. }, ..) |
            Item::Fn(ItemFn { sig: Signature { ident, .. }, .. }, ..) |
            Item::Trait(ItemTrait { ident, .. }, ..) |
            Item::Const(ItemConst { ident, .. }, ..) =>
                ScopeTreeExportID::Ident(ident.clone()),
            Item::Impl(ItemImpl { self_ty, trait_, generics, .. }, ..) =>
                ScopeTreeExportID::Impl(*self_ty.clone(), trait_.clone().map(|(_, path, _)| path), generics.clone()),
            item => panic!("ScopeTreeExportID Not supported for {}", quote!(#item)),
        }

    }

    fn maybe_attrs(&self) -> Option<&Vec<Attribute>> {
        match self {
            Item::Const(item) => Some(&item.attrs),
            Item::Enum(item) => Some(&item.attrs),
            Item::ExternCrate(item) => Some(&item.attrs),
            Item::Fn(item) => Some(&item.attrs),
            Item::ForeignMod(item) => Some(&item.attrs),
            Item::Impl(item) => Some(&item.attrs),
            Item::Macro(item) => Some(&item.attrs),
            Item::Macro2(item) => Some(&item.attrs),
            Item::Mod(item) => Some(&item.attrs),
            Item::Static(item) => Some(&item.attrs),
            Item::Struct(item) => Some(&item.attrs),
            Item::Trait(item) => Some(&item.attrs),
            Item::TraitAlias(item) => Some(&item.attrs),
            Item::Type(item) => Some(&item.attrs),
            Item::Union(item) => Some(&item.attrs),
            Item::Use(item) => Some(&item.attrs),
            _ => None,
        }
    }

    fn maybe_ident(&self) -> Option<&Ident> {
        match self {
            Item::Const(ItemConst { ident, .. }) |
            Item::Enum(ItemEnum { ident, .. }) |
            Item::ExternCrate(ItemExternCrate { ident, .. }) |
            Item::Fn(ItemFn { sig: Signature { ident, .. }, .. }) |
            Item::Macro2(ItemMacro2 { ident, .. }) |
            Item::Mod(ItemMod { ident, .. }) |
            Item::Struct(ItemStruct { ident, ..  }) |
            Item::Static(ItemStatic { ident, ..  }) |
            Item::Trait(ItemTrait { ident, ..  }) |
            Item::TraitAlias(ItemTraitAlias { ident, ..  }) |
            Item::Type(ItemType { ident, .. }) |
            Item::Union(ItemUnion { ident, .. }) => Some(ident),
            Item::Macro(ItemMacro { ident, .. }) => ident.as_ref(),
            Item::Impl(ItemImpl { self_ty, .. }) => type_ident_ref(self_ty),
            _ => None
        }
    }
    fn maybe_generics(&self) -> Option<&Generics> {
        match self {
            Item::Enum(ItemEnum { generics, .. }) |
            Item::Fn(ItemFn { sig: Signature { generics, .. }, .. }) |
            Item::Impl(ItemImpl { generics, .. }) |
            Item::Struct(ItemStruct { generics, .. }) |
            Item::Trait(ItemTrait { generics, .. }) |
            Item::TraitAlias(ItemTraitAlias { generics, .. }) |
            Item::Type(ItemType { generics, .. }) |
            Item::Union(ItemUnion { generics, .. }) =>
                Some(generics),
            _ => None
        }
    }
}


fn maybe_generic_type_bound<'a>(path: &'a Path, generics: &'a Generics) -> Option<&'a TypeParam> {
    // println!("maybe_generic_type_bound.1: {} in: [{}] where: [{}]", path.to_token_stream(), generics.params.to_token_stream(), generics.where_clause.to_token_stream());
    path.segments.last()
        .and_then(|last_segment|
            generics.params.iter()
                .find_map(|param| match param {
                    GenericParam::Type(type_param) =>
                        last_segment.ident.eq(&type_param.ident)
                            .then(|| type_param),
                    _ => None
                }))
    // TODO: where
}

pub fn segment_arguments_to_types(segment: &PathSegment) -> Vec<&Type> {
    match &segment.arguments {
        PathArguments::AngleBracketed(AngleBracketedGenericArguments { args, .. }) =>
            args.iter().filter_map(|arg| match arg {
                GenericArgument::Type(ty) => Some(ty),
                _ => None
            }).collect(),
        // PathArguments::Parenthesized(ParenthesizedGenericArguments { inputs, output, .. }) =>

        _ => Vec::new(),
    }
}

pub fn path_arguments_to_types(arguments: &PathArguments) -> Vec<&Type> {
    // println!("path_arguments_to_types: {}", arguments.to_token_stream());
    match arguments {
        PathArguments::AngleBracketed(AngleBracketedGenericArguments { args, .. }) =>
            args.iter().filter_map(|arg| match arg {
                GenericArgument::Type(ty) => Some(ty),
                _ => None
            }).collect(),
        _ => Vec::new(),
    }
}

#[allow(unused)]
pub fn path_arguments_to_nested_objects(arguments: &PathArguments, source: &<Type as VisitScopeType>::Source) -> CommaPunctuatedNestedArguments {
    match arguments {
        PathArguments::None => Punctuated::new(),
        PathArguments::Parenthesized(ParenthesizedGenericArguments { inputs, output, .. }) => {
            let mut nested = CommaPunctuated::new();
            inputs.iter().for_each(|arg| {
                nested.push(NestedArgument::Object(arg.visit_scope_type(source)));
            });
            match output {
                ReturnType::Default => {}
                ReturnType::Type(_, ty) => {
                    nested.push(NestedArgument::Object(ty.visit_scope_type(source)));
                }
            }
            nested
        },
        PathArguments::AngleBracketed(AngleBracketedGenericArguments { args, .. }) => {
            args.iter().filter_map(|arg| match arg {
                GenericArgument::Type(inner_type) =>
                    Some(NestedArgument::Object(inner_type.visit_scope_type(source))),
                _ => None
            }
            ).collect()
        }
    }
}

pub fn path_arguments_to_type_conversions(arguments: &PathArguments) -> Vec<TypeKind> {
    path_arguments_to_types(arguments)
        .into_iter()
        .map(TypeKind::from)
        .collect()
}

pub fn usize_to_tokenstream(value: usize) -> TokenStream2 {
    let lit = syn::LitInt::new(&value.to_string(), Span::call_site());
    lit.to_token_stream()
}

pub fn collect_bounds(bounds: &AddPunctuated<TypeParamBound>) -> Vec<Path> {
    bounds.iter().filter_map(|bound| match bound {
        TypeParamBound::Trait(TraitBound { path, .. }) => Some(path.clone()),
        TypeParamBound::Lifetime(_lifetime) => None
    }).collect()
}
fn path_ident_ref(path: &Path) -> Option<&Ident> {
    path.segments.last().map(|last_segment| &last_segment.ident)
}

fn type_ident_ref(ty: &Type) -> Option<&Ident> {
    match ty {
        Type::Path(TypePath { path, .. }) =>
            path_ident_ref(path),
        Type::Reference(TypeReference { elem, .. }) |
        Type::Ptr(TypePtr { elem, .. }) =>
            type_ident_ref(elem),
        Type::TraitObject(TypeTraitObject { bounds, .. }) => {
            bounds.iter().find_map(|b| match b {
                TypeParamBound::Trait(TraitBound { path, ..}) => path_ident_ref(path),
                _ => None
            })
        },
        Type::Array(TypeArray { elem, .. }) => type_ident_ref(elem),
        _ => None
    }
}

impl ItemExtension for Signature {
    fn scope_tree_export_id(&self) -> ScopeTreeExportID {
        ScopeTreeExportID::Ident(self.ident.clone())
    }
    fn maybe_attrs(&self) -> Option<&Vec<Attribute>> {
        None
    }

    fn maybe_ident(&self) -> Option<&Ident> {
       Some(&self.ident)
    }

    fn maybe_generics(&self) -> Option<&Generics> {
        Some(&self.generics)
    }
}
