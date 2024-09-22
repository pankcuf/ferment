// use quote::{format_ident, quote, ToTokens};
// use syn::{Type, TypeTuple};
// use crate::ast::{CommaPunctuated, Depunctuated, SemiPunctuated};
// use crate::composer::{AnyOtherComposer, BoundsComposer, CallbackComposer, Composer, GenericComposer};
// use crate::context::ScopeContext;
// use crate::conversion::{GenericBoundsModel, GenericTypeKind, MixinKind};
// use crate::ext::Mangle;
// use crate::lang::objc::composers::AttrWrapper;
// use crate::lang::objc::ObjCFermentate;
// use crate::lang::objc::presentation::{ImplementationPresentation, InterfacePresentation, Property};
// use crate::presentable::ScopeContextPresentable;
// use crate::presentation::Name;
//
// impl ScopeContextPresentable for GenericComposer<ObjCFermentate, AttrWrapper> {
//     type Presentation = ObjCFermentate;
//
//     fn present(&self, source: &ScopeContext) -> Self::Presentation {
//         let Self { kind, attrs, .. } = self;
//         println!("GenericComposer::ferment-sys: {} ---- {}", kind, attrs);
//         match kind {
//             MixinKind::Bounds(model) =>
//                 BoundsComposer::new(model, attrs).compose(source),
//             MixinKind::Generic(GenericTypeKind::AnyOther(ty)) =>
//                 AnyOtherComposer::new(ty, attrs).compose(source),
//             MixinKind::Generic(GenericTypeKind::Callback(ty)) =>
//                 CallbackComposer::new(ty, attrs).compose(source),
//             MixinKind::Generic(GenericTypeKind::Result(ty)) =>
//                 compose_result(ty, attrs, source),
//             MixinKind::Generic(GenericTypeKind::Slice(ty)) =>
//                 compose_slice(ty, attrs, source),
//             MixinKind::Generic(GenericTypeKind::Tuple(Type::Tuple(type_tuple))) =>
//                 compose_tuple(type_tuple, attrs, source),
//             MixinKind::Generic(GenericTypeKind::Map(ty) | GenericTypeKind::IndexMap(ty) | GenericTypeKind::SerdeJsonMap(ty)) =>
//                 compose_map(ty, attrs, source),
//
//             MixinKind::Generic(GenericTypeKind::BTreeSet(ty) | GenericTypeKind::HashSet(ty) | GenericTypeKind::Vec(ty)) => {
//                 ObjCFermentate::Empty
//             },
//             MixinKind::Generic(GenericTypeKind::Array(ty)) => {
//                 ObjCFermentate::Empty
//             },
//             _ => ObjCFermentate::Empty
//         }
//     }
// }
//
// fn compose_bounds(model: &GenericBoundsModel, attrs: &AttrWrapper, source: &ScopeContext) -> ObjCFermentate {
//     ObjCFermentate::Empty
// }
//
// pub fn compose_any_other(ty: &Type, attrs: &AttrWrapper, source: &ScopeContext) -> ObjCFermentate {
//     ObjCFermentate::Empty
// }
//
// pub fn compose_map(ty: &Type, attrs: &AttrWrapper, source: &ScopeContext) -> ObjCFermentate {
//     ObjCFermentate::Empty
// }
// pub fn compose_callback(ty: &Type, attrs: &AttrWrapper, source: &ScopeContext) -> ObjCFermentate {
//     ObjCFermentate::Empty
// }
// pub fn compose_result(ty: &Type, attrs: &AttrWrapper, source: &ScopeContext) -> ObjCFermentate {
//     // struct Result_ok_String_err_Option_u32 {
//     //     char *ok;
//     //     uint32_t *error;
//     // };
//     // struct Result_ok_String_err_Option_u32 *Result_ok_String_err_Option_u32_ctor(char *ok, uint32_t *error);
//     // void Result_ok_String_err_Option_u32_destroy(struct Result_ok_String_err_Option_u32 *ffi);
//     let c_name = ty.mangle_ident_default().to_token_stream();
//     let global = source.context.read().unwrap();
//     let config = global.config.maybe_objc_config().unwrap();
//     let prefix = config.class_prefix();
//     let objc_name = Name::Ident(format_ident!("{}{}", prefix, c_name.to_string()));
//
//     ObjCFermentate::Item {
//         header_name: config.xcode.framework_name.clone(),
//         imports: Depunctuated::new(),
//         interfaces: Depunctuated::from_iter([
//             InterfacePresentation::Default {
//                 name: objc_name.clone(),
//                 c_type: c_name.clone(),
//                 properties: Default::default(),
//             },
//             InterfacePresentation::C {
//                 name: objc_name.clone(),
//                 c_type: c_name.clone()
//             },
//             InterfacePresentation::Rust {
//                 name: objc_name.clone(),
//                 c_type: c_name.clone()
//             },
//             InterfacePresentation::Args {
//                 name: objc_name.clone(),
//                 c_type: c_name.clone(),
//                 args: Default::default(),
//             }
//         ]),
//         implementations: Depunctuated::from_iter([
//             ImplementationPresentation::Default {
//                 objc_name: objc_name.clone(),
//                 c_type: c_name.clone(),
//                 properties_inits: SemiPunctuated::new(),
//             },
//             ImplementationPresentation::C {
//                 objc_name: objc_name.clone(),
//                 c_type: c_name.clone(),
//                 property_ctors: Default::default(),
//                 property_dtors: Default::default(),
//             },
//             ImplementationPresentation::Rust {
//                 objc_name: objc_name.clone(),
//                 c_type: c_name.clone(),
//                 c_var: quote!(struct #c_name *),
//                 property_names: CommaPunctuated::new(),
//                 property_ctors: Default::default(),
//             },
//             ImplementationPresentation::Args {
//                 objc_name: objc_name.clone(),
//                 prop_implementations: Default::default(),
//             }
//         ])
//     }
// }
// pub fn compose_slice(ty: &Type, attrs: &AttrWrapper, source: &ScopeContext) -> ObjCFermentate {
//     ObjCFermentate::Empty
// }
// pub fn compose_tuple(type_tuple: &TypeTuple, attrs: &AttrWrapper, source: &ScopeContext) -> ObjCFermentate {
//     ObjCFermentate::Empty
// }
//
// fn compose_generic_item() -> ObjCFermentate {
//     ObjCFermentate::Empty
// }