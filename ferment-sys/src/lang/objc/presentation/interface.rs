// use std::fmt::{Display, Formatter};
// use quote::{format_ident, quote, ToTokens};
// use syn::__private::TokenStream2;
// use crate::ast::{Depunctuated, SemiPunctuated};
// use crate::lang::objc::presentable::ArgPresentation;
// use super::super::CategoryKind;
//
//
// #[derive(Clone, Debug)]
// pub enum InterfacePresentation {
//     // @interface DSArr_u8_32 : NSObject
//     // @property (nonatomic, readwrite) NSArray<NSNumber *> *values;
//     // + (instancetype)initWith:(struct Arr_u8_32 *)self_;
//     // @end
//     Default { name: TokenStream2, c_type: TokenStream2, properties: SemiPunctuated<super::Property> },
//     // @interface DSArr_u8_32 (C)
//     // - (struct Arr_u8_32 *)c_ctor;
//     // + (void)c_dtor:(struct Arr_u8_32 *)self_;
//     // @end
//     C { name: TokenStream2, c_type: TokenStream2 },
//     // @interface DSArr_u8_32 (Rust)
//     // - (struct Arr_u8_32 *)rust_ctor;
//     // + (void)rust_dtor:(struct Arr_u8_32 *)self_;
//     // @end
//     Rust { name: TokenStream2, c_type: TokenStream2 },
//     // @interface DSArr_u8_32 (Args)
//     // + (NSArray<NSNumber *> *)to_values:(struct Arr_u8_32 *)self_;
//     // - (uint8_t *)from_values;
//     // @end
//     Args { name: TokenStream2, c_type: TokenStream2, args: Depunctuated<ArgPresentation> },
//
//     // @interface DSdash_spv_masternode_processor_crypto_byte_util_UInt768 : NSObject
//     // @property (nonatomic, readwrite) DSArr_u8_96 *o_0;
//     // @end
//
//     // Properties {
//     //     // @interface DSdash_spv_masternode_processor_crypto_byte_util_UInt768 : NSObject
//     //     // @property (nonatomic, readwrite) DSArr_u8_96 *o_0;
//     //     // @end
//     //
//     // },
//     //
//     // Bindings {
//     //     objc_name: TokenStream2,
//     //     c_name: TokenStream2,
//     //     arg_conversions: CommaPunctuatedTokens
//     //
//     //     // @implementation DSdash_spv_masternode_processor_crypto_byte_util_UInt768 (Bindings)
//     //     // + (struct dash_spv_masternode_processor_crypto_byte_util_UInt768 *)ffi_ctor:(instancetype)obj {
//     //     //     return dash_spv_masternode_processor_crypto_byte_util_UInt768_ctor([DSArr_u8_96 ffi_to:obj.o_0]);
//     //     // }
//     //     // + (void)ffi_dtor:(struct dash_spv_masternode_processor_crypto_byte_util_UInt768 *)ffi_ref {
//     //     //     dash_spv_masternode_processor_crypto_byte_util_UInt768_destroy(ffi_ref);
//     //     // }
//     //     // @end
//     //
//     // },
//     // Conversions {
//     //     objc_name: TokenStream2,
//     //     c_name: TokenStream2,
//     //     from: Depunctuated<TokenStream2>,
//     //     to: Depunctuated<TokenStream2>,
//     //     destroy: Depunctuated<TokenStream2>,
//     //
//     //     // @interface DSdash_spv_masternode_processor_crypto_byte_util_UInt768 (Conversions)
//     //     // + (instancetype)ffi_from:(struct dash_spv_masternode_processor_crypto_byte_util_UInt768 *)ffi_ref;
//     //     // + (struct dash_spv_masternode_processor_crypto_byte_util_UInt768 *)ffi_to:(instancetype)obj;
//     //     // + (void)ffi_destroy:(struct dash_spv_masternode_processor_crypto_byte_util_UInt768 *)ffi_ref;
//     //     // @end
//     //     // @implementation DSdash_spv_masternode_processor_crypto_byte_util_UInt768 (Conversions)
//     //     //
//     //     // + (instancetype)ffi_from:(struct dash_spv_masternode_processor_crypto_byte_util_UInt768 *)ffi_ref {
//     //     //     id *obj = [[self alloc] init];
//     //     //     if (obj) {
//     //     //         obj.o_0 = [DSArr_u8_96 ffi_from:ffi_ref->o_0];
//     //     //     }
//     //     //     return obj;
//     //     // }
//     //     // + (struct dash_spv_masternode_processor_crypto_byte_util_UInt768 *)ffi_to:(instancetype)obj {
//     //     //     dash_spv_masternode_processor_crypto_byte_util_UInt768 *self_ = malloc(sizeof(dash_spv_masternode_processor_crypto_byte_util_UInt768));
//     //     //     self_->o_0 = [DSArr_u8_96 ffi_to:obj.o_0];
//     //     //     return self_;
//     //     // }
//     //     // + (void)ffi_destroy:(dash_spv_masternode_processor_crypto_byte_util_UInt768 *)ffi_ref {
//     //     //     if (!ffi_ref) return;
//     //     //     [DSArr_u8_96 ffi_destroy:ffi_ref->o_0];
//     //     //     free(ffi_ref);
//     //     // }
//     //     // @end
//     //
//     // },
//
// }
//
// impl ToTokens for InterfacePresentation {
//     fn to_tokens(&self, tokens: &mut TokenStream2) {
//         match self {
//             InterfacePresentation::Default { name, c_type, properties } => {
//                 let mut body = SemiPunctuated::new();
//                 body.push(quote!(+ (instancetype)initWith:(struct #c_type *)self_;));
//                 body.extend(properties.iter().map(ToTokens::to_token_stream));
//                 Interface::def(name.to_token_stream(), Some(quote!(NSObject)), body).to_tokens(tokens)
//             }
//             InterfacePresentation::C { name, c_type } => {
//                 let body = SemiPunctuated::from_iter([
//                     quote!(- (struct #c_type *)c_ctor),
//                     quote!(+ (void)c_dtor:(struct #c_type *)self_),
//                 ]);
//                 Interface::c_ext(name.to_token_stream(), body).to_tokens(tokens)
//             }
//             InterfacePresentation::Rust { name, c_type } => {
//                 let body = SemiPunctuated::from_iter([
//                     quote!(- (struct #c_type *)rust_ctor),
//                     quote!(+ (void)rust_dtor:(struct #c_type *)self_),
//                 ]);
//                 Interface::rust_ext(name.to_token_stream(), body)
//                     .to_tokens(tokens)
//             }
//             InterfacePresentation::Args { name: objc_name, c_type, args } => {
//                 let mut body = SemiPunctuated::new();
//                 args.iter().for_each(|ArgPresentation { objc_ty, c_ty, name, .. }| {
//                     let to_ident = format_ident!("to_{}", name.to_string());
//                     let from_ident = format_ident!("from_{}", name.to_string());
//                     body.push(quote!(+ (#objc_ty)#to_ident:(#c_type)self_));
//                     body.push(quote!(- (#c_ty)#from_ident));
//                 });
//                 Interface::args_ext(objc_name.to_token_stream(), body).to_tokens(tokens)
//             }
//             // InterfacePresentation::Conversions {
//             //     objc_name,
//             //     c_name,
//             //     from,
//             //     to,
//             //     destroy } => {
//             //
//             // }
//             // InterfacePresentation::Bindings { objc_name, c_name, arg_conversions } => {
//             //
//             // }
//         }
//     }
// }
//
// // impl Display for InterfacePresentation {
// //     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
// //         let interface = match self {
// //             InterfacePresentation::Default { name, c_type, properties } => {
// //
// //                 f.write_str(quote!(+ (instancetype)initWith:(struct #c_type *)self_;).to_string().as_str())?;
// //                 f.write_str("\n")?;
// //                 for property in properties {
// //                     f.write_str(property.to_token_stream().to_string().as_str())?;
// //                     f.write_str(";\n")?;
// //                 }
// //
// //                 Interface::def(name.to_token_stream(), Some(quote!(NSObject)), body)
// //             }
// //             InterfacePresentation::C { name, c_type } => {
// //                 let body = SemiPunctuated::from_iter([
// //                     quote!(- (struct #c_type *)c_ctor),
// //                     quote!(+ (void)c_dtor:(struct #c_type *)self_),
// //                 ]);
// //                 Interface::c_ext(name.to_token_stream(), body)
// //             }
// //             InterfacePresentation::Rust { name, c_type } => {
// //                 let body = SemiPunctuated::from_iter([
// //                     quote!(- (struct #c_type *)rust_ctor),
// //                     quote!(+ (void)rust_dtor:(struct #c_type *)self_),
// //                 ]);
// //                 Interface::rust_ext(name.to_token_stream(), body)
// //             }
// //             InterfacePresentation::Args { name: objc_name, c_type, args } => {
// //                 let mut body = SemiPunctuated::new();
// //                 args.iter().for_each(|super::ArgPresentation { objc_ty, c_ty, name, .. }| {
// //                     let to_ident = format_ident!("to_{}", name.to_string());
// //                     let from_ident = format_ident!("from_{}", name.to_string());
// //                     body.push(quote!(+ (#objc_ty)#to_ident:(#c_type)self_));
// //                     body.push(quote!(- (#c_ty)#from_ident));
// //                 });
// //                 Interface::args_ext(objc_name.to_token_stream(), body)
// //             }
// //         };
// //         f.write_str(interface.to_string().as_str())
// //     }
// // }
//
//
//
//
// #[derive(Clone, Debug)]
// pub struct Interface {
//     pub name: TokenStream2,
//     pub super_protocol: Option<TokenStream2>,
//     pub category: Option<CategoryKind>,
//     pub body: SemiPunctuated<TokenStream2>
// }
//
// impl Interface {
//     pub fn def(name: TokenStream2, super_protocol: Option<TokenStream2>, body: SemiPunctuated<TokenStream2>) -> Self {
//         Self { name, super_protocol, category: None, body }
//     }
//     pub fn ext(name: TokenStream2, category: CategoryKind, body: SemiPunctuated<TokenStream2>) -> Self {
//         Self { name, super_protocol: None, category: Some(category), body }
//     }
//
//     pub fn rust_ext(name: TokenStream2, body: SemiPunctuated<TokenStream2>) -> Self {
//         Interface::ext(name, CategoryKind::Rust, body)
//     }
//     pub fn c_ext(name: TokenStream2, body: SemiPunctuated<TokenStream2>) -> Self {
//         Interface::ext(name, CategoryKind::C, body)
//     }
//     pub fn args_ext(name: TokenStream2, body: SemiPunctuated<TokenStream2>) -> Self {
//         Interface::ext(name, CategoryKind::Args, body)
//     }
// }
//
//
// impl ToTokens for Interface {
//     fn to_tokens(&self, tokens: &mut TokenStream2) {
//         let Self { name, super_protocol, category, body } = self;
//         let super_proto = super_protocol.as_ref().map(|sp| quote!(: #sp)).unwrap_or_default();
//         let category = category.as_ref().map(|c| quote!((#c))).unwrap_or_default();
//         let stream = quote! {
//             @interface #name #super_proto #category
//             #body
//             @end
//         };
//         stream.to_tokens(tokens)
//     }
// }
//
// impl Display for Interface {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         let Self { name, super_protocol, category, body } = self;
//         let super_proto = super_protocol.as_ref().map(|sp| quote!(: #sp)).unwrap_or_default();
//         let category = category.as_ref().map(|c| quote!((#c))).unwrap_or_default();
//         f.write_str(quote!(@interface #name #super_proto #category).to_token_stream().to_string().as_str())?;
//         f.write_str("\n")?;
//         f.write_str(body.to_token_stream().to_string().as_str())?;
//         f.write_str("\n")?;
//         f.write_str(quote!(@end).to_token_stream().to_string().as_str())
//     }
// }
//
//
