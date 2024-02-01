use quote::{format_ident, quote, ToTokens};
use syn::{AngleBracketedGenericArguments, Attribute, GenericArgument, GenericParam, Generics, Ident, Item, ItemConst, ItemEnum, ItemExternCrate, ItemFn, ItemImpl, ItemMacro, ItemMacro2, ItemMod, ItemStatic, ItemStruct, ItemTrait, ItemTraitAlias, ItemType, ItemUnion, parse_quote, Path, PathArguments, Signature, TraitBound, Type, TypeArray, TypeParamBound, TypePath, TypePtr, TypeReference, TypeSlice, WherePredicate};
use syn::__private::{Span, TokenStream2};
use syn::punctuated::Punctuated;
use syn::token::Add;
use crate::composition::{GenericBoundComposition, TypeComposition};
use crate::context::ScopeContext;
use crate::conversion::{ItemConversion, PathConversion, type_ident};
use crate::formatter::{format_path_vec, format_token_stream};
use crate::interface::{DEREF_FIELD_PATH, destroy_conversion, ffi_from_conversion, ffi_from_opt_conversion, ffi_to_conversion, ffi_to_opt_conversion, FROM_OFFSET_MAP_PRESENTER, iter_map_collect, OBJ_FIELD_NAME, package_boxed_expression, package_boxed_vec_expression, package_unbox_any_expression_terminated};
use crate::presentation::context::{OwnedItemPresenterContext, OwnerIteratorPresentationContext};
use crate::presentation::ScopeContextPresentable;

pub trait ItemExtension {
    fn maybe_attrs(&self) -> Option<&Vec<Attribute>>;
    fn maybe_ident(&self) -> Option<&Ident>;
    fn maybe_generics(&self) -> Option<&Generics>;
    fn maybe_generic_bound_for_path(&self, path: &Path) -> Option<GenericBoundComposition> {
        self.maybe_generics()
            .and_then(|generics| maybe_generic_type_bound(path, generics))
    }

}

impl ItemExtension for Item {
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


fn generic_trait_bounds(ty: &Path, ident_path: &Path, bounds: &Punctuated<TypeParamBound, Add>) -> Vec<Path> {
    // println!("generic_trait_bounds.1: {} :: {} :: {}", format_token_stream(ty), format_token_stream(ident_path), format_token_stream(bounds));
    let mut has_bound = false;
    let involved = bounds.iter().filter_map(|b| {
        // println!("generic_trait_bounds.2: {}", quote!(#b));
        match b {
            TypeParamBound::Trait(TraitBound { path, .. }) => {
                //println!("generic_trait_bounds: [{}] {} == {} {}", ident_path.eq(ty), format_token_stream(ty), format_token_stream(path), format_token_stream(bounds));
                let has = ident_path.eq(ty);
                if !has_bound && has {
                    has_bound = true;
                }
                has
                    .then_some(path.clone())
            },
            TypeParamBound::Lifetime(_) => None
        }
    }).collect::<Vec<_>>();
    // if !involved.is_empty() {
        // println!("generic_trait_bounds.3: (result) {}", format_path_vec(&involved));
    // }
    involved
}

fn maybe_generic_type_bound<'a>(path: &Path, generics: &Generics) -> Option<GenericBoundComposition> {
    // println!("maybe_generic_type_bound.1: {} in [{} .... {}]", format_token_stream(path), format_token_stream(&generics.params), format_token_stream(&generics.where_clause));
    let result = generics.params.iter().find_map(|param| if let GenericParam::Type(type_param) = param {
        let ty: Type = parse_quote!(#path);
        let ident = &type_param.ident;
        let ident_path: Path = parse_quote!(#ident);
        let has_bounds = ident_path.eq(path);
        let bounds = generic_trait_bounds(path, &ident_path, &type_param.bounds);
        // println!("maybe_generic_type_bound.2: [{}: {}] --> [{}]", has_bounds, quote!(#type_param), format_path_vec(&bounds));
        // println!("maybe_generic_type_bound: (bounds) {} ", format_path_vec(&bounds));
        has_bounds
            .then_some(GenericBoundComposition {
                bounds,
                predicates: generics.where_clause
                    .as_ref()
                    .map(|where_clause|
                        where_clause.predicates
                            .iter()
                            .filter_map(|predicate| match predicate {
                                WherePredicate::Type(predicate_type) => {
                                    // println!("maybe_generic_type_bound:::predicate: [{}] {} ::: {}", ty.eq(&predicate_type.bounded_ty), format_token_stream(predicate_type), format_token_stream(path));
                                    let bounded_ty = &predicate_type.bounded_ty;
                                    let ident_path: Path = parse_quote!(#bounded_ty);
                                    ty.eq(&predicate_type.bounded_ty)
                                        .then_some((
                                            predicate_type.bounded_ty.clone(),
                                            {
                                                let bounds = generic_trait_bounds(&path, &ident_path, &predicate_type.bounds);
                                                // println!("maybe_generic_type_bound.3.... {}: {}: [{}]", format_token_stream(&ident_path), format_token_stream(&predicate_type.bounded_ty), format_path_vec(&bounds));
                                                bounds
                                            }))
                                },
                                _ => None })
                            .collect())
                    .unwrap_or_default(),
                type_composition: TypeComposition::new(ty, Some(generics.clone())),
            })
    } else { None });
    // println!("maybe_generic_type_bound (result): {}", result.as_ref().map_or(format!("None"), |r| r.to_string()));
    result
}

pub fn path_arguments_to_types(arguments: &PathArguments) -> Vec<&Type> {
    match arguments {
        PathArguments::AngleBracketed(AngleBracketedGenericArguments { args, .. }) =>
            args.iter().filter_map(|arg| match arg {
                GenericArgument::Type(ty) => Some(ty),
                _ => None
            }).collect(),
        _ => Vec::new(),
    }
}

fn path_from_type(ty: &Type) -> Option<&Path> {
    match ty {
        Type::Array(TypeArray { elem, len: _, .. }) => path_from_type(elem),
        Type::Path(TypePath { path, .. }) => Some(path),
        _ => None,
    }
}

pub fn path_arguments_to_paths(arguments: &PathArguments) -> Vec<&Path> {
    match arguments {
        PathArguments::AngleBracketed(AngleBracketedGenericArguments { args, .. }) =>
            args.iter().filter_map(|arg| match arg {
                GenericArgument::Type(ty) => path_from_type(ty),
                // GenericArgument::Type(Type::Reference(TypeReference { mutability, elem })) => Some(path),
                _ => None
            }).collect(),
        _ => Vec::new(),
    }
}

pub fn path_arguments_to_path_conversions(arguments: &PathArguments) -> Vec<PathConversion> {
    path_arguments_to_paths(arguments)
        .into_iter()
        .map(PathConversion::from)
        .collect()
}

pub(crate) fn from_array(field_path: TokenStream2, type_array: &TypeArray) -> TokenStream2 {
    match &*type_array.elem {
        Type::Path(TypePath { path: Path { segments, .. }, .. }) =>
            match segments.last().unwrap().ident.to_string().as_str() {
                "u8" => DEREF_FIELD_PATH(field_path),
                _ => panic!("from_array: unsupported segments {} {}", field_path, quote!(#segments))
            },
        _ => panic!("from_array: unsupported {} {}", field_path, quote!(#type_array)),
    }
}
pub(crate) fn from_slice(field_path: TokenStream2, type_slice: &TypeSlice) -> TokenStream2 {
    match &*type_slice.elem {
        Type::Path(TypePath { path: Path { segments, .. }, .. }) =>
            match segments.last().unwrap().ident.to_string().as_str() {
                "u8" => DEREF_FIELD_PATH(field_path),
                _ => panic!("from_slice: unsupported segments {} {}", field_path, quote!(#segments))
            },
        _ => panic!("from_slice: unsupported {} {}", field_path, quote!(#type_slice)),
    }
}

pub(crate) fn destroy_path(field_path: TokenStream2, path: &Path) -> TokenStream2 {
    let last_segment = path.segments.last().unwrap();
    match last_segment.ident.to_string().as_str() {
        "i8" | "u8" | "i16" | "u16" | "i32" | "u32" | "i64" | "u64" | "i128" | "u128" | "isize"
        | "usize" | "bool" => quote!(),
        "VarInt" => quote!(),
        "Option" => match path_arguments_to_paths(&path.segments.last().unwrap().arguments).first().unwrap().segments.last().unwrap().ident.to_string().as_str() {
            "i8" | "u8" | "i16" | "u16" | "i32" | "u32" | "i64" | "u64" | "i128" | "u128" | "isize" | "usize" | "bool" => quote!(),
            _ => {
                let conversion = package_unbox_any_expression_terminated(field_path.clone());
                quote!(if !#field_path.is_null() { #conversion })
            }
        },
        "String" =>
            destroy_conversion(field_path, parse_quote!(std::os::raw::c_char), quote!(#path)),
        "str" =>
            destroy_conversion(field_path, parse_quote!(std::os::raw::c_char), quote!(&#path)),
        _ =>
            package_unbox_any_expression_terminated(field_path),
    }
}

pub(crate) fn from_path(field_path: TokenStream2, path: &Path) -> TokenStream2 {
    let last_segment = path.segments.last().unwrap();
    match last_segment.ident.to_string().as_str() {
        "i8" | "u8" | "i16" | "u16" | "i32" | "u32" | "i64" | "u64" | "i128" | "u128" | "isize" | "usize" | "bool" => field_path,
        "VarInt" => quote!(#path(#field_path)),
        "Option" => match path_arguments_to_paths(&last_segment.arguments).first().unwrap().segments.last().unwrap().ident.to_string().as_str() {
            // std convertible
            // TODO: what to use? 0 or ::MAX
            "i8" | "u8" | "i16" | "u16" | "i32" | "u32" | "i64" | "u64" | "i128" | "u128"
            | "isize" | "usize" => quote!((#field_path > 0).then_some(#field_path)),
            // TODO: mmm shit that's incorrect
            "bool" =>
                quote!((#field_path).then_some(#field_path)),
            _ =>
                ffi_from_opt_conversion(field_path),
        },
        _ => ffi_from_conversion(field_path),
    }
}

pub(crate) fn destroy_ptr(field_path: TokenStream2, type_ptr: &TypePtr) -> TokenStream2 {
    match &*type_ptr.elem {
        Type::Ptr(type_ptr) => destroy_ptr(field_path, type_ptr),
        Type::Path(type_path) => destroy_path(field_path, &type_path.path),
        // _ => destroy_conversion(field_path)
        _ => panic!("Can't destroy_ptr: of type: {}", quote!(#type_ptr)),
    }
}

pub(crate) fn from_ptr(field_path: TokenStream2, type_ptr: &TypePtr) -> TokenStream2 {
    match &*type_ptr.elem {
        Type::Ptr(type_ptr) => match &*type_ptr.elem {
            Type::Path(_type_path) => {
                let ffi_from_conversion =
                    ffi_from_conversion(FROM_OFFSET_MAP_PRESENTER(quote!(*values)));
                quote!((0..count).map(|i| #ffi_from_conversion).collect())
            }
            _ => ffi_from_conversion(field_path),
        },
        Type::Path(type_path) => {
            let field_type = type_path
                .path
                .segments
                .last()
                .unwrap()
                .ident
                .to_token_stream();
            quote!(std::slice::from_raw_parts(values as *const #field_type, count).to_vec())
        }
        _ => ffi_from_conversion(field_path),
    }
}

pub(crate) fn destroy_reference(field_path: TokenStream2, type_reference: &TypeReference) -> TokenStream2 {
    match &*type_reference.elem {
        Type::Path(type_path) => destroy_path(field_path, &type_path.path),
        _ => panic!("from_reference: unsupported type: {} {}", field_path, quote!(#type_reference)),
    }
}

pub(crate) fn from_reference(field_path: TokenStream2, type_reference: &TypeReference) -> TokenStream2 {
    match &*type_reference.elem {
        Type::Path(type_path) => from_path(field_path, &type_path.path),
        _ => panic!("from_reference: unsupported type: {} {}", field_path, quote!(#type_reference)),
    }
}

// pub fn map_args(args: &Punctuated<GenericArgument, Comma>) -> Vec<&GenericArgument> {
//     args.iter().collect::<Vec<_>>()
// }

pub(crate) fn to_path(field_path: TokenStream2, path: &Path, context: &ScopeContext) -> TokenStream2 {
    let last_segment = path.segments.last().unwrap();
    match last_segment.ident.to_string().as_str() {
        "i8" | "u8" | "i16" | "u16" | "i32" | "u32" | "i64" | "u64" | "i128" | "u128" | "isize"
        | "usize" | "bool" => field_path,
        "VarInt" => quote!(#field_path.0),
        "Option" => match path_arguments_to_paths(&last_segment.arguments).first().unwrap().segments.last().unwrap().ident.to_string().as_str() {
            // TODO: MAX/MIN? use optional primitive?
            "i8" | "u8" | "i16" | "u16" | "i32" | "u32" | "i64" | "u64" | "i128" | "u128" | "isize" | "usize" =>
                quote!(#field_path.unwrap_or(0)),
            "bool" =>
                quote!(#field_path.unwrap_or(false)),
            "Vec" =>
                OwnerIteratorPresentationContext::MatchFields(field_path, vec![
                    OwnedItemPresenterContext::Lambda(quote!(Some(vec)), ffi_to_conversion(quote!(vec))),
                    OwnedItemPresenterContext::Lambda(quote!(None), quote!(std::ptr::null_mut())),
                ]).present(context),
            _ => ffi_to_opt_conversion(field_path),
        },
        _ => ffi_to_conversion(field_path),
    }
}

fn to_vec_ptr(ident: TokenStream2, _type_ptr: &TypePtr, _type_arr: &TypeArray) -> TokenStream2 {
    let expr = package_boxed_expression(quote!(o));
    package_boxed_vec_expression(iter_map_collect(OBJ_FIELD_NAME(ident), quote!(|o| #expr)))
}

pub(crate) fn to_ptr(field_path: TokenStream2, type_ptr: &TypePtr, context: &ScopeContext) -> TokenStream2 {
    match &*type_ptr.elem {
        Type::Array(TypeArray { elem, .. }) => match &**elem {
            Type::Path(type_path) => to_path(field_path, &type_path.path, context),
            _ => panic!("to_pointer: Unknown type inside Type::Array {}", quote!(#type_ptr)),
        },
        Type::Ptr(TypePtr { elem, .. }) => match &**elem {
            Type::Path(type_path) => to_path(quote!(*#field_path.add(i)), &type_path.path, context),
            Type::Array(type_arr) => to_vec_ptr(field_path, type_ptr, type_arr),
            _ => panic!("to_pointer: Unknown type inside Type::Ptr {}", quote!(#type_ptr)),
        },
        Type::Path(type_path) => to_path(field_path, &type_path.path, context),
        _ => panic!("to_pointer: Unknown type {}", quote!(#type_ptr)),
    }
}

pub(crate) fn to_reference(field_path: TokenStream2, type_reference: &TypeReference, context: &ScopeContext) -> TokenStream2 {
    match &*type_reference.elem {
        Type::Path(type_path) => to_path(field_path, &type_path.path, context),
        _ => panic!("to_reference: Unknown type {}", quote!(#type_reference)),
    }
}

pub(crate) fn to_array(field_path: TokenStream2, type_array: &TypeArray, context: &ScopeContext) -> TokenStream2 {
    match &*type_array.elem {
        Type::Path(type_path) => to_path(package_boxed_expression(field_path), &type_path.path, context),
        _ => panic!("to_array: Unknown type {}", quote!(#type_array)),
    }
}

pub fn ffi_vtable_name(trait_name: &Ident) -> Ident {
    format_ident!("{}_VTable", trait_name)
}

pub fn ffi_trait_obj_name(trait_name: &Ident) -> Ident {
    // format_ident!("{}_TraitObject", trait_name)
    format_ident!("{}", trait_name)
}

pub fn ffi_fn_name(fn_name: &Ident) -> Ident {
    format_ident!("ffi_{}", fn_name)
}

pub fn ffi_unnamed_arg_name(index: usize) -> Ident {
    format_ident!("o_{}", index)
}

pub fn ffi_constructor_name(item_name: &Ident) -> Ident {
    format_ident!("{}_ctor", item_name)
}
pub fn ffi_destructor_name(item_name: &Ident) -> Ident {
    format_ident!("{}_destroy", item_name)
}

pub fn ffi_mangled_ident(ty: &Type) -> Ident {
    match ty {
        // Here we expect BTreeMap<K, V> | HashMap<K, V> | Vec<V> for now
        Type::Path(TypePath { path, .. }) =>
            PathConversion::from(path)
                .into_mangled_generic_ident(),
        ty => {
            let p: Path = parse_quote!(#ty);
            p.get_ident().unwrap().clone()
        }
    }
}
pub fn usize_to_tokenstream(value: usize) -> TokenStream2 {
    let lit = syn::LitInt::new(&value.to_string(), Span::call_site());
    lit.to_token_stream()
}

pub fn ident_from_item(item: &Item) -> Option<Ident> {
    match item {
        Item::Mod(item_mod) => Some(item_mod.ident.clone()),
        Item::Struct(item_struct) => Some(item_struct.ident.clone()),
        Item::Enum(item_enum) => Some(item_enum.ident.clone()),
        Item::Type(item_type) => Some(item_type.ident.clone()),
        Item::Fn(item_fn) => Some(item_fn.sig.ident.clone()),
        Item::Trait(item_trait) => Some(item_trait.ident.clone()),
        Item::Impl(item_impl) => type_ident(&item_impl.self_ty),
        Item::Use(item_use) => ItemConversion::fold_use(&item_use.tree).first().cloned().cloned(),
        _ => None,
    }
}