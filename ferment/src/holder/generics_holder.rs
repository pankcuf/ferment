use quote::ToTokens;
use syn::Generics;
use crate::holder::Holder;
use crate::impl_holder;

impl_holder!(GenericsHolder, Generics);