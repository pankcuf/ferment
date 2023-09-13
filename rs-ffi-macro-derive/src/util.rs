use std::vec;
use syn::__private::Span;
use syn::{Field, Ident, Path, Type, TypePath, TypePtr};
use syn::token::Mut;
use crate::interface::FFI_TYPE_PATH_CONVERTER;
use crate::mangle_path;
use crate::path_conversion::PathConversion;

#[allow(unused)]
fn create_field(name: &str, ty: Type) -> Field {
    // println!("create_field: name: {:?} type: {:?}", name, &ty);
    Field {
        attrs: vec![],
        vis: syn::Visibility::Inherited,
        ident: Some(Ident::new(name, Span::call_site())),
        colon_token: Some(Default::default()),
        ty
    }
}

// #[allow(unused)]
// fn create_struct(ident: Ident, fields: Fields) -> ItemStruct {
//     ItemStruct {
//         ident,
//         fields,
//         attrs: vec![],
//         generics: Default::default(),
//         semi_token: None,
//         struct_token: Default::default(),
//         vis: syn::Visibility::Inherited,
//     }
// }
//
// #[allow(unused)]
// fn create_named_struct(ident_suffix: Ident, fields: Vec<Field>) -> ItemStruct {
//     // println!("create_named_struct: ident: {:?}", &ident_suffix);
//     // println!("create_named_struct: fields: {:?}", &fields);
//     create_struct(
//         ident_suffix,
//         Fields::Named(FieldsNamed {
//             named: Punctuated::from_iter(fields),
//             brace_token: Default::default()
//         }))
// }
//
// #[allow(unused)]
// pub fn create_vec_struct(ident: Ident, value_type: Type) -> ItemStruct {
//     // println!("create_vec_struct: ident: {:?}", &ident);
//     // println!("create_vec_struct: value_type: {:?}", &value_type);
//     create_named_struct(
//         ident,
//         vec![
//             create_field("count", Type::Path(syn::parse_quote!(usize))),
//             create_field("values", value_type)])
// }
//
//
// #[allow(unused)]
// pub fn create_map_struct(ident: Ident, key_type: Type, value_type: Type) -> ItemStruct {
//     // println!("create_map_struct: ident: {:?}", &ident);
//     // println!("create_map_struct: key_type: {:?}", &key_type);
//     // println!("create_map_struct: value_type: {:?}", &value_type);
//     create_named_struct(
//         ident,
//         vec![
//             create_field("count", Type::Path(syn::parse_quote!(usize))),
//             create_field("keys", key_type),
//             create_field("values", value_type)])
// }

#[allow(unused)]
pub fn mangle(path: &Path) -> Type {
    match PathConversion::from(path) {
        PathConversion::Simple(path) => Type::Ptr(TypePtr {
            star_token: Default::default(),
            const_token: None,
            mutability: Some(Mut::default()),
            elem: Box::new(Type::Path(TypePath { qself: None, path })),
        }),
        PathConversion::Complex(path) => Type::Ptr(TypePtr {
            star_token: Default::default(),
            const_token: None,
            mutability: Some(Mut::default()),
            elem: Box::new(Type::Ptr(TypePtr {
                star_token: Default::default(),
                const_token: None,
                mutability: Some(Mut::default()),
                elem: Box::new(Type::Path(TypePath { qself: None, path: FFI_TYPE_PATH_CONVERTER(&path) })),
            })),
        }),
        PathConversion::Vec(path) |
        PathConversion::Map(path) => Type::Ptr(TypePtr {
            star_token: Default::default(),
            const_token: None,
            mutability: Some(Mut::default()),
            elem: Box::new(Type::Ptr(TypePtr {
                star_token: Default::default(),
                const_token: None,
                mutability: Some(Mut::default()),
                elem: Box::new(Type::Path(TypePath { qself: None, path: mangle_path(&path) })),
            })),
        })
    }
}

// fn convert_to_ffi_type(field_type: &Type) -> Type {
//     match field_type {
//         Type::P
//         Type::Path(TypePath { path, .. }) => Type::Path(TypePath { qself: None, path: mangle_simple_generic_type(path.clone()) }),
//         _ => unimplemented!("convert_to_ffi_type: unexpected args: {:?}", field_type)
//     }
// }
//
// #[allow(unused)]
// pub fn transform_vec(path: Path) -> ItemStruct {
//     let value_type = match path_arguments_to_types(&path.segments.last().unwrap().arguments)[..] {
//         [Type::Path(TypePath { path, .. })] => mangle(&path),
//         _ => unimplemented!("transform_vec: unexpected args: {}", quote!(#path))
//     };
//     let struct_name_args = VEC_PATH_PRESENTER(&path);
//     // println!("transform_vec: struct_name_args: {:?}", &struct_name_args);
//     // println!("transform_vec: value_type: {:?}", &value_type);
//     // create_vec_struct(parse_quote!(#struct_name_args), value_type)
//     create_vec_struct(format_ident!("{}FFI", struct_name_args.to_string()), value_type)
// }
//
// #[allow(unused)]
// pub fn transform_map(path: Path) -> ItemStruct {
//     let (key_type, value_type) = match path_arguments_to_types(&path.segments.last().unwrap().arguments)[..] {
//         [Type::Path(TypePath { path: key_path, .. }), Type::Path(TypePath { path: value_path, .. })] =>
//             {
//                 // println!("transform_map: key_path {:?}", key_path);
//                 // println!("transform_map: value_path {:?}", value_path);
//                 let key_type = mangle(key_path);
//                 let value_type = mangle(value_path);
//                 // println!("transform_map: key_type {:?}", &key_type);
//                 // println!("transform_map: value_type {:?}", &value_type);
//                 (key_type, value_type)
//             },
//         _ => unimplemented!("transform_vec: unexpected args: {}", quote!(#path))
//     };
//     let struct_name_args = MAP_PATH_PRESENTER(&path);
//     // println!("transform_map: struct_name_args: {:?}", &struct_name_args);
//     // println!("transform_map: key_type: {:?}", &key_type);
//     // println!("transform_map: value_type: {:?}", &value_type);
//     create_map_struct(
//         parse_quote!(#struct_name_args),
//         key_type,
//         value_type)
// }


// /// Drop Interface Cases for Generics
//
// fn from_conversion_type(conversion_type: PathConversion, vec: TokenStream2) -> TokenStream2 {
//     match conversion_type {
//         PathConversion::Simple(path) => quote!(*values.add(i)),
//         _ => ffi_from_conversion(quote!(*values.add(i)))
//         // _ => rs_ffi_interfaces::FFIConversion::ffi_from(*values.add(i))
//     }
// }
//
// fn drop_conversion_type(conversion_type: PathConversion, vec: TokenStream2) -> TokenStream2 {
//     match conversion_type {
//         PathConversion::Simple(path) => package_unbox_vec_ptr(vec, quote!(self.count)),
//         _ => package_unbox_any_vec_ptr(vec, quote!(self.count))
//     }
// }
//
// fn drop_conversions_map(conversion_types: [PathConversion; 2]) -> Vec<TokenStream2> {
//     match conversion_types {
//         [key_conversion_type, value_conversion_type] =>
//             vec![
//                 drop_conversion_type(key_conversion_type, quote!(self.keys)),
//                 drop_conversion_type(value_conversion_type, quote!(self.values))
//             ]
//     }
// }
//
// fn drop_conversions_vec(value_conversion_type: PathConversion) -> Vec<TokenStream2> {
//     vec![drop_conversion_type(value_conversion_type, quote!(self.values))]
// }
//
// fn drop_conversion_simple_vec(field_path: TokenStream2) -> TokenStream2 {
//     package_unbox_vec_ptr(field_path, quote!(self.count))
// }
//
// fn drop_conversion_complex_vec(field_path: TokenStream2) -> TokenStream2 {
//     package_unbox_any_vec_ptr(field_path, quote!(self.count))
// }
//
// fn drop_conversion_simple_map(field_path: TokenStream2) -> TokenStream2 {
//     package_unbox_vec_ptr(field_path, quote!(self.count))
// }
//
// /// from Interface Cases for Generics
//
// fn from_conversions_map(conversion_types: [PathConversion; 2]) -> Vec<TokenStream2> {
//     match conversion_types {
//         [key_conversion_type, value_conversion_type] =>
//             vec![
//                 from_conversion_type(key_conversion_type, quote!(self.keys)),
//                 from_conversion_type(value_conversion_type, quote!(self.values))
//             ]
//     }
// }
