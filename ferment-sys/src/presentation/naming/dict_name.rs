use std::fmt::Formatter;
use quote::{quote, ToTokens};
use syn::__private::TokenStream2;
use syn::{parse_quote, Path};
use crate::ext::ToPath;

#[derive(Clone, Debug)]
pub enum DictionaryName {
    Ok,
    Error,
    Key,
    Keys,
    Value,
    NewValue,
    OldValue,
    Values,
    Count,
    Obj,
    Object,
    Vtable,
    Self_,
    I,
    O,
    Package,
    InterfaceFrom,
    InterfaceTo,
    InterfaceDestroy,
    Ffi,
    FfiRef,
    FFiResult,
    Caller,
    Destructor,
    Tag,
    Index,
}

impl std::fmt::Display for DictionaryName {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}
impl ToTokens for DictionaryName {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        match self {
            DictionaryName::Ok => quote!(ok),
            DictionaryName::Error => quote!(error),
            DictionaryName::Key => quote!(key),
            DictionaryName::Keys => quote!(keys),
            DictionaryName::Value => quote!(value),
            DictionaryName::NewValue => quote!(new_value),
            DictionaryName::OldValue => quote!(old_value),
            DictionaryName::Values => quote!(values),
            DictionaryName::Count => quote!(count),
            DictionaryName::Obj => quote!(obj),
            DictionaryName::Object => quote!(object),
            DictionaryName::Package => quote!(ferment),
            DictionaryName::InterfaceFrom => quote!(FFIConversionFrom),
            DictionaryName::InterfaceTo => quote!(FFIConversionTo),
            DictionaryName::InterfaceDestroy => quote!(FFIConversionDestroy),
            DictionaryName::Self_ => quote!(self_),
            DictionaryName::I => quote!(i),
            DictionaryName::O => quote!(o),
            DictionaryName::Ffi => quote!(ffi),
            DictionaryName::FfiRef => quote!(ffi_ref),
            DictionaryName::Vtable => quote!(vtable),
            DictionaryName::FFiResult => quote!(ffi_result),
            DictionaryName::Caller => quote!(caller),
            DictionaryName::Destructor => quote!(destructor),
            DictionaryName::Tag => quote!(tag),
            DictionaryName::Index => quote!(index),
        }
            .to_tokens(tokens)
    }
}

impl ToPath for DictionaryName {
    fn to_path(&self) -> Path {
        let tokens = self.to_token_stream();
        parse_quote!(#tokens)
    }
}