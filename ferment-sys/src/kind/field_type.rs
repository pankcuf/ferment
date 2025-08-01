use quote::ToTokens;
use syn::{parse_quote, Type};
use syn::__private::TokenStream2;
use ferment_macro::Display;
use crate::ext::ToType;
use crate::lang::Specification;

#[derive(Clone, Debug, Display)]
pub enum FieldTypeKind<SPEC>
where SPEC: Specification {
    Type(Type),
    Conversion(TokenStream2),
    Var(SPEC::Var)
}
impl<SPEC> ToTokens for FieldTypeKind<SPEC>
where SPEC: Specification {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        match self {
            FieldTypeKind::Type(ty) => ty.to_tokens(tokens),
            FieldTypeKind::Conversion(conversion) => conversion.to_tokens(tokens),
            FieldTypeKind::Var(var) => var.to_tokens(tokens)
        }
    }
}
impl<SPEC> ToType for FieldTypeKind<SPEC>
where SPEC: Specification {
    fn to_type(&self) -> Type {
        match self {
            FieldTypeKind::Type(ty) => ty.clone(),
            FieldTypeKind::Var(var) => var.to_type(),
            _ => panic!("improper use of kind as type")
        }
    }
}
impl<SPEC> FieldTypeKind<SPEC>
where SPEC: Specification {

    pub fn conversion<T: ToTokens>(conversion: T) -> Self {
        Self::Conversion(conversion.to_token_stream())
    }
    pub fn r#type(ty: &Type) -> Self {
        Self::Type(ty.clone())
    }
    pub fn type_count() -> Self {
        Self::Type(parse_quote!(usize))
    }
}