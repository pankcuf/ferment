use std::fmt::{Debug, Display, Formatter};
use syn::{parse_quote, Attribute, Field, Type};
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};
use ferment_macro::Display;
use crate::composable::CfgAttributes;
use crate::composer::{FieldPathResolver, SourceFermentable};
use crate::context::ScopeContext;
use crate::ext::{Conversion, ToType};
use crate::lang::{FromDictionary, LangAttrSpecification, RustSpecification, Specification};
use crate::presentable::{Expression, ScopeContextPresentable};
use crate::presentation::{DictionaryName, Name, RustFermentate};

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
            _ => panic!("improper use of conversion as type")
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

#[derive(Clone, Debug)]
pub struct FieldComposer<SPEC>
    where SPEC: Specification {
    pub attrs: SPEC::Attr,
    pub name: SPEC::Name,
    pub kind: FieldTypeKind<SPEC>,
    pub named: bool,
}

impl<SPEC> Display for FieldComposer<SPEC>
    where SPEC: Specification<Attr: Display> + Display {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let Self { name, kind, named, attrs, .. } = self;
        f.write_str(format!("FieldComposer({name}({}), {kind}({}), {named}, {attrs}", name.to_token_stream(), kind.to_token_stream()).as_str())
    }
}

impl FieldComposer<RustSpecification> {
    pub fn self_typed(ty: Type, attrs: &Vec<Attribute>) -> Self {
        Self::new(Name::dictionary_name(DictionaryName::Self_), FieldTypeKind::Type(ty), true, attrs.cfg_attributes())
    }


}

impl<SPEC> FieldComposer<SPEC>
    where SPEC: Specification {
    pub fn typed(name: SPEC::Name, ty: &Type, named: bool, attrs: &Vec<Attribute>) -> Self {
        Self { name, kind: FieldTypeKind::r#type(ty), named, attrs: SPEC::Attr::from_attrs(attrs.cfg_attributes()) }
    }
    pub fn new(name: SPEC::Name, kind: FieldTypeKind<SPEC>, named: bool, attrs: SPEC::Attr) -> Self {
        Self { name, kind, named, attrs }
    }
    pub fn named(name: SPEC::Name, kind: FieldTypeKind<SPEC>) -> Self {
        Self::no_attrs(name, kind, true)
    }
    pub fn named_ref(name: &SPEC::Name, kind: FieldTypeKind<SPEC>) -> Self {
        Self::no_attrs(name.clone(), kind, true)
    }
    pub fn unnamed(name: SPEC::Name, kind: FieldTypeKind<SPEC>) -> Self {
        Self { name, kind, named: false, attrs: SPEC::Attr::default() }
    }
    pub fn no_attrs(name: SPEC::Name, kind: FieldTypeKind<SPEC>, named: bool) -> Self {
        Self { name, kind, named, attrs: SPEC::Attr::default() }
    }
    pub fn tokenized_name(&self) -> TokenStream2 {
        self.name.to_token_stream()
    }
    pub fn ty(&self) -> &Type {
        if let FieldTypeKind::Type(ty) = &self.kind {
            ty
        } else {
            panic!("improper use of conversion as type")
        }
    }

}
impl<SPEC> FieldComposer<SPEC>
where SPEC: Specification<Expr=Expression<SPEC>>,
      SPEC::Expr: ScopeContextPresentable {
    pub const VARIANT_FROM: FieldPathResolver<SPEC> =
        |c| (c.name.clone(), Conversion::expr_from(c, Some(Expression::deref_tokens(&c.name))));
    pub const VARIANT_TO: FieldPathResolver<SPEC> =
        |c| (c.name.clone(), Conversion::expr_to(c, Some(Expression::name(&c.name))));
    pub const VARIANT_DROP: FieldPathResolver<SPEC> =
        |c| (c.name.clone(), Conversion::expr_destroy(c, Some(Expression::deref_tokens(&c.name))));
    pub const STRUCT_FROM: FieldPathResolver<SPEC> =
        |c| (c.name.clone(), Conversion::expr_from(c, Some(Expression::ffi_ref_with_name(&c.name))));
    pub const STRUCT_TO: FieldPathResolver<SPEC> =
        |c| (c.name.clone(), Conversion::expr_to(c, Some(Expression::obj_name(&c.name))));
    pub const STRUCT_DROP: FieldPathResolver<SPEC> =
        |c| (SPEC::Name::default(), Conversion::expr_destroy(c, Some(Expression::ffi_ref_with_name(&c.name))));
    pub const TYPE_TO: FieldPathResolver<SPEC> =
        |c| (SPEC::Name::default(), Conversion::expr_to(c, Some(Expression::Name(SPEC::Name::dictionary_name(DictionaryName::Obj)))));
}

impl<SPEC> FieldComposer<SPEC>
    where SPEC: Specification<Name=Name<SPEC>> {
    pub fn unnamed_variant_producer(field: &Field, index: usize) -> Self {
        let Field { ty, attrs, .. } = field;
        Self::typed(Name::UnnamedArg(index), ty, false, attrs)
    }
    pub fn unnamed_struct_producer(field: &Field, index: usize) -> Self {
        let Field { ty, attrs, .. } = field;
        Self::typed(Name::UnnamedStructFieldsComp(ty.clone(), index), ty, false, attrs)
    }
    pub fn unit_variant_producer(field: &Field, _index: usize) -> Self {
        // Actually just a stab
        let Field { ty, attrs, .. } = field;
        Self::typed(Name::Empty, ty, false, attrs)
    }
    pub fn named_producer(field: &Field, _index: usize) -> Self {
        let Field { ident, ty, attrs, .. } = field;
        Self::typed(Name::Optional(ident.clone()), ty, true, attrs)
    }
}


impl<SPEC> ToTokens for FieldComposer<SPEC>
    where SPEC: Specification,
          Self: SourceFermentable<SPEC::Fermentate> {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        self.ferment().to_tokens(tokens);
    }
}

impl SourceFermentable<RustFermentate> for FieldComposer<RustSpecification> {
    fn ferment(&self) -> RustFermentate {
        let Self { name, kind, attrs, .. } = self;
        RustFermentate::TokenStream(quote!(#(#attrs)* #name: #kind))
    }
}

impl<SPEC> ScopeContextPresentable for FieldComposer<SPEC>
    where Self: ToTokens,
          SPEC: Specification {
    type Presentation = TokenStream2;

    fn present(&self, _source: &ScopeContext) -> Self::Presentation {
        self.to_token_stream()
    }
}
