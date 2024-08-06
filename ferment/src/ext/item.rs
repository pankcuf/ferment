use syn::{AngleBracketedGenericArguments, Attribute, GenericArgument, GenericParam, Generics, Item, ItemConst, ItemEnum, ItemExternCrate, ItemFn, ItemImpl, ItemMacro, ItemMacro2, ItemMod, ItemStatic, ItemStruct, ItemTrait, ItemTraitAlias, ItemType, ItemUnion, ItemUse, ParenthesizedGenericArguments, Path, PathArguments, PathSegment, ReturnType, Signature, TraitBound, Type, TypeParam, TypeParamBound};
use proc_macro2::{Ident, Span, TokenStream as TokenStream2};
use syn::punctuated::Punctuated;
use quote::{quote, ToTokens};
use crate::ast::{AddPunctuated, CommaPunctuated};
use crate::composable::NestedArgument;
use crate::composer::CommaPunctuatedNestedArguments;
use crate::conversion::{type_ident_ref, TypeConversion};
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
            Item::Const(ItemConst { ident, .. }, ..) => ScopeTreeExportID::Ident(ident.clone()),
            Item::Use(ItemUse { .. }, ..) =>
                panic!("Not  supported"),
            Item::Impl(ItemImpl { self_ty, trait_, .. }, ..) => ScopeTreeExportID::Impl(*self_ty.clone(), trait_.clone().map(|(_, path, _)| path)),
            item => panic!("ScopeTreeExportID Not supported for {}", quote!(#item)),
            // type_ident(self_ty).unwrap(),
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
                nested.push(NestedArgument::Object(arg.update_nested_generics(source)));
            });
            match output {
                ReturnType::Default => {}
                ReturnType::Type(_, ty) => {
                    nested.push(NestedArgument::Object(ty.update_nested_generics(source)));
                }
            }
            nested
        },
        PathArguments::AngleBracketed(AngleBracketedGenericArguments { args, .. }) => {
            args.iter().filter_map(|arg| match arg {
                GenericArgument::Type(inner_type) =>
                    Some(NestedArgument::Object(inner_type.update_nested_generics(source))),
                _ => None
            }
            ).collect()
        }
    }
}

pub fn path_arguments_to_type_conversions(arguments: &PathArguments) -> Vec<TypeConversion> {
    path_arguments_to_types(arguments)
        .into_iter()
        .map(TypeConversion::from)
        .collect()
}

pub fn usize_to_tokenstream(value: usize) -> TokenStream2 {
    let lit = syn::LitInt::new(&value.to_string(), Span::call_site());
    lit.to_token_stream()
}

// fn cache_fields_in(container: &mut HashMap<ImportConversion, HashSet<ImportComposition>>, fields: &Fields, imports: &HashMap<PathHolder, Path>) {
//     match fields {
//         Fields::Unnamed(FieldsUnnamed { unnamed: fields, .. }) |
//         Fields::Named(FieldsNamed { named: fields, .. }) =>
//             fields.iter()
//                 .for_each(|field| cache_type_in(container, &field.ty, imports)),
//         Fields::Unit => {}
//     }
// }
//
// fn cache_type_in(_container: &mut HashMap<ImportConversion, HashSet<ImportComposition>>, _ty: &Type, _imports: &HashMap<PathHolder, Path>) {
    // Types which are used as a part of types (for generics and composite types)
    // let involved: HashSet<Type> = ty.nested_items();
    // involved.iter()
    //     .for_each(|ty| {
            // println!("involved: {}", ty.to_token_stream());
            // match ty {
            //     Type::Array(type_array) => {
            //         let path = type_array.elem.to_path();
            //         cache_path_in(container, &path, imports);
            //     },
            //     Type::Slice(type_slice) => {
            //         let path = type_slice.elem.to_path();
            //         cache_path_in(container, &path, imports);
            //     },
            //     Type::Path(type_path) => {
            //         cache_path_in(container, &type_path.path, imports);
            //     }
            //     Type::Reference(type_reference) => {
            //         let path = type_reference.elem.to_path();
            //         cache_path_in(container, &path, imports);
            //     },
            //     // Type::Ptr(_) => {}
            //     // Type::TraitObject(_) => {}
            //     // Type::Tuple(TypeTuple { elems }) => {
            //     //
            //     // }
            //     _ => {
            //         let path = ty.to_path();
            //         cache_path_in(container, &path, imports);
            //     }
            // }
        // });
// }


pub fn collect_bounds(bounds: &AddPunctuated<TypeParamBound>) -> Vec<Path> {
    bounds.iter().filter_map(|bound| match bound {
        TypeParamBound::Trait(TraitBound { path, .. }) => Some(path.clone()),
        TypeParamBound::Lifetime(_lifetime) => None
    }).collect()
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
