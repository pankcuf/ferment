use quote::ToTokens;
use syn::TypePath;
use crate::holder::Holder;
use crate::impl_holder;

impl_holder!(TypePathHolder, TypePath);
