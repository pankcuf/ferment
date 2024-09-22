use quote::ToTokens;
use syn::TypePath;
use crate::ast::Holder;
use crate::impl_holder;

impl_holder!(TypePathHolder, TypePath);
