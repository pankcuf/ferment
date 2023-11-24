// use std::collections::HashMap;
// use quote::quote;
// use syn::__private::TokenStream2;
// use syn::{ReturnType, Type, TypePath};
// use crate::helper::to_path;
// use crate::interface::FFI_DICTIONARY_TYPE_PRESENTER;
// use crate::scope::Scope;
// use crate::type_conversion::TypeConversion;
//
// pub struct TreeContext {
//     pub scope: Scope,
//     pub tree: HashMap<TypeConversion, Type>,
// }
//
// pub struct SignatureConversionComposer {
//
// }
//
//
// fn add_full_qualified_signature<'ast>(visitor: &'ast mut Visitor, sig: &Signature, scope: &Scope) {
//     if let ReturnType::Type(_, ty) = &sig.output {
//         visitor.add_full_qualified_type_match(scope.clone(), ty)
//     }
//     sig.inputs.iter().for_each(|arg| match arg {
//         FnArg::Typed(PatType { ty, .. }) => {
//             visitor.add_full_qualified_type_match(scope.clone(), ty);
//         },
//         _ => {}
//     });
// }
//
// fn fn_expansion(item_fn: &ItemFn, _scope: &Scope, tree: HashMap<TypeConversion, Type>) -> Expansion {
//     // println!("fn_expansion: [{}]: {}", scope.to_token_stream(), item_fn.sig.ident.to_token_stream());
//     // println!("fn_expansion: [{:?}]:", tree);
//     let Signature {
//         output,
//         ident: fn_name,
//         inputs,
//         ..
//     } = &item_fn.sig;
//     let (output_expression, output_conversions) = match output {
//         ReturnType::Default => (quote!(()), quote!(;)),
//         ReturnType::Type(_, field_type) => (
//             FFI_DICTIONARY_TYPE_PRESENTER(&field_type, &tree),
//             match &**field_type {
//                 Type::Path(TypePath { path, .. }) => to_path(quote!(obj), &path, None),
//                 _ => panic!("error: output conversion: {}", quote!(#field_type)),
//             },
//         ),
//     };
//
//     // TODO: replace Fn arguments with crate::fermented::generics::#ident or #import
//     let (fn_args, conversions) = inputs
//         .iter()
//         .map(|arg| match arg {
//             FnArg::Typed(PatType { ty, pat, .. }) => (
//                 NAMED_CONVERSION_PRESENTER(pat.to_token_stream(), FFI_DICTIONARY_TYPE_PRESENTER(&ty, &tree)),
//                 handle_arg_type(&**ty, &**pat)
//             ),
//             _ => panic!("Arg type not supported: {:?}", quote!(#arg)),
//         })
//         .unzip();
//
//     Expansion::Function {
//         input: quote!(#item_fn),
//         comment: DocPresentation::Safety(quote!(#fn_name)),
//         ffi_presentation: FFIObjectPresentation::Function {
//             name_and_arguments: ROUND_BRACES_FIELDS_PRESENTER((format_ident!("ffi_{}", fn_name).to_token_stream(), fn_args)),
//             input_conversions: ROUND_BRACES_FIELDS_PRESENTER((quote!(#fn_name), conversions)),
//             output_expression,
//             output_conversions,
//         },
//     }
// }
// fn trait_expansion(item_trait: &ItemTrait, _scope: &Scope, tree: HashMap<TypeConversion, Type>) -> Expansion {
//     let fields = item_trait.items.iter().filter_map(|trait_item| match trait_item {
//         TraitItem::Method(TraitItemMethod { sig: Signature { output, ident: fn_name, inputs, .. }, .. }) => {
//             let (output_expression, _output_conversions) = match output {
//                 ReturnType::Default => (quote!(()), quote!(;)),
//                 ReturnType::Type(_, field_type) => (
//                     FFI_DICTIONARY_TYPE_PRESENTER(&field_type, &tree),
//                     match &**field_type {
//                         Type::Path(TypePath { path, .. }) => to_path(quote!(obj), &path, None),
//                         _ => panic!("error: output conversion: {}", quote!(#field_type)),
//                     },
//                 ),
//             };
//
//             // TODO: replace Fn arguments with crate::fermented::generics::#ident or #import
//             let (fn_args, _conversions): (Vec<TokenStream2>, Vec<TokenStream2>) = inputs
//                 .iter()
//                 .map(|arg| match arg {
//                     FnArg::Receiver(Receiver { mutability, .. }) => (
//                         match mutability {
//                             Some(..) => quote!(*mut ()),
//                             _ => quote!(*const ())
//                         },
//                         quote!()
//                     ),
//                     FnArg::Typed(PatType { ty, pat, .. }) => (
//                         NAMED_CONVERSION_PRESENTER(pat.to_token_stream(), FFI_DICTIONARY_TYPE_PRESENTER(&ty, &tree)),
//                         handle_arg_type(&**ty, &**pat)
//                     ),
//                 })
//                 .unzip();
//             let name_and_args = ROUND_BRACES_FIELDS_PRESENTER((quote!(unsafe extern "C" fn), fn_args));
//             Some(quote!(pub #fn_name: #name_and_args -> #output_expression))
//         },
//         _ => None
//     }).collect();
//     Expansion::Trait {
//         input: quote!(#item_trait),
//         comment: DocPresentation::Empty,
//         vtable: create_vtable_struct(&item_trait.ident, CURLY_ITER_PRESENTER(fields)),
//         trait_object: create_trait_obj_struct(&item_trait.ident)
//     }
// }