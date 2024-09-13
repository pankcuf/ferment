// use std::fmt::{Debug, Display, Formatter};
// use std::marker::PhantomData;
// use quote::ToTokens;
// use syn::{Generics, Type};
// use crate::composer::DestructorContext;
// use crate::lang::LangAttrSpecification;
//
// #[derive(Clone, Debug)]
// pub enum ConstructorPresentableContext<LANG, SPEC>
//     where LANG: Clone,
//           SPEC: LangAttrSpecification<LANG> {
//     EnumVariant(DestructorContext<LANG, SPEC>),
//     Default(DestructorContext<LANG, SPEC>),
// }
//
// impl<LANG, SPEC> ConstructorPresentableContext<LANG, SPEC>
//     where LANG: Clone,
//           SPEC: LangAttrSpecification<LANG> {
//     pub fn default(ty: Type, attrs: SPEC, generics: Option<Generics>) -> Self {
//         Self::Default((ty, attrs, generics, PhantomData::default()))
//     }
// }
// impl<LANG, SPEC> Display for ConstructorPresentableContext<LANG, SPEC>
//     where LANG: Clone
//             + Debug,
//           SPEC: LangAttrSpecification<LANG>
//             + Display
//             + Debug {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         match self {
//             Self::EnumVariant((ty, attrs, generics, ..)) =>
//                 f.write_str(format!("EnumVariant({}, {}, {})", ty.to_token_stream(), attrs.to_string(), generics.to_token_stream()).as_str()),
//             Self::Default((ty, attrs, generics, ..)) =>
//                 f.write_str(format!("Default({}, {}, {})", ty.to_token_stream(), attrs.to_string(), generics.to_token_stream()).as_str()),
//         }
//     }
// }
