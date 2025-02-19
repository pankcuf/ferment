use std::fmt::{Debug, Display, Formatter};
use std::marker::PhantomData;
use syn::{Attribute, Field, Type};
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};
use ferment_macro::Display;
use crate::composable::CfgAttributes;
use crate::composer::{FieldPathResolver, SourceFermentable};
use crate::context::ScopeContext;
use crate::ext::{ConversionType, ToType};
use crate::lang::{FromDictionary, LangAttrSpecification, LangFermentable, RustSpecification, Specification};
use crate::presentable::{Expression, ScopeContextPresentable};
use crate::presentation::{DictionaryName, Name, RustFermentate};

#[derive(Clone, Debug, Display)]
pub enum FieldTypeKind<LANG, SPEC>
    where LANG: LangFermentable,
          SPEC: Specification<LANG> {
    Type(Type),
    Conversion(TokenStream2),
    Var(SPEC::Var)
}
impl<LANG, SPEC> ToTokens for FieldTypeKind<LANG, SPEC>
    where LANG: LangFermentable,
          SPEC: Specification<LANG> {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        match self {
            FieldTypeKind::Type(ty) => ty.to_tokens(tokens),
            FieldTypeKind::Conversion(conversion) => conversion.to_tokens(tokens),
            FieldTypeKind::Var(var) => var.to_tokens(tokens)
        }
    }
}
impl<LANG, SPEC> ToType for FieldTypeKind<LANG, SPEC>
    where LANG: LangFermentable,
          SPEC: Specification<LANG> {
    fn to_type(&self) -> Type {
        match self {
            FieldTypeKind::Type(ty) => ty.clone(),
            FieldTypeKind::Var(var) => var.to_type(),
            _ => panic!("improper use of conversion as type")
        }
    }
}
impl<LANG, SPEC> FieldTypeKind<LANG, SPEC>
    where LANG: LangFermentable,
          SPEC: Specification<LANG> {
    pub fn ty(&self) -> &Type {
        match self {
            FieldTypeKind::Type(ty) => ty,
            _ => panic!("improper use of conversion as type")
        }
    }
    pub fn r#type(ty: &Type) -> Self {
        Self::Type(ty.clone())
    }
}

#[derive(Clone, Debug)]
pub struct FieldComposer<LANG, SPEC>
    where LANG: LangFermentable,
          SPEC: Specification<LANG> {
    pub attrs: SPEC::Attr,
    pub name: SPEC::Name,
    pub kind: FieldTypeKind<LANG, SPEC>,
    pub named: bool,
    _marker: PhantomData<LANG>,
}

impl<LANG, SPEC> Display for FieldComposer<LANG, SPEC>
    where LANG: LangFermentable,
          SPEC: Specification<LANG, Attr: Display> + Display {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let Self { name, kind, named, attrs, .. } = self;
        f.write_str(format!("FieldComposer({name}({}), {kind}({}), {named}, {attrs}", name.to_token_stream(), kind.to_token_stream()).as_str())
    }
}

impl<SPEC> FieldComposer<RustFermentate, SPEC>
    where SPEC: RustSpecification {
    pub fn self_typed(ty: Type, attrs: &Vec<Attribute>) -> Self {
        Self::new(Name::dictionary_name(DictionaryName::Self_), FieldTypeKind::Type(ty), true, attrs.cfg_attributes())
    }


}

impl<LANG, SPEC> FieldComposer<LANG, SPEC>
    where LANG: LangFermentable,
          SPEC: Specification<LANG> {
    pub fn typed(name: SPEC::Name, ty: &Type, named: bool, attrs: &Vec<Attribute>) -> Self {
        Self { name, kind: FieldTypeKind::r#type(ty), named, attrs: SPEC::Attr::from_attrs(attrs.cfg_attributes()), _marker: PhantomData }
    }
    pub fn new(name: SPEC::Name, kind: FieldTypeKind<LANG, SPEC>, named: bool, attrs: SPEC::Attr) -> Self {
        Self { name, kind, named, attrs, _marker: PhantomData }
    }
    pub fn named(name: SPEC::Name, kind: FieldTypeKind<LANG, SPEC>) -> Self {
        Self::no_attrs(name, kind, true)
    }
    pub fn unnamed(name: SPEC::Name, kind: FieldTypeKind<LANG, SPEC>) -> Self {
        Self { name, kind, named: false, attrs: SPEC::Attr::default(), _marker: PhantomData }
    }
    pub fn no_attrs(name: SPEC::Name, kind: FieldTypeKind<LANG, SPEC>, named: bool) -> Self {
        Self { name, kind, named, attrs: SPEC::Attr::default(), _marker: PhantomData }
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
impl<LANG, SPEC> FieldComposer<LANG, SPEC>
where LANG: LangFermentable,
      SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>>,
      SPEC::Expr: ScopeContextPresentable {
    pub const VARIANT_FROM: FieldPathResolver<LANG, SPEC> =
        |c| (c.name.clone(), ConversionType::expr_from(c, Some(Expression::deref_tokens(&c.name))));
    pub const VARIANT_TO: FieldPathResolver<LANG, SPEC> =
        |c| (c.name.clone(), ConversionType::expr_to(c, Some(Expression::name(&c.name))));
    pub const VARIANT_DROP: FieldPathResolver<LANG, SPEC> =
        |c| (c.name.clone(), ConversionType::expr_destroy(c, Some(Expression::deref_tokens(&c.name))));
    pub const STRUCT_FROM: FieldPathResolver<LANG, SPEC> =
        |c| (c.name.clone(), ConversionType::expr_from(c, Some(Expression::ffi_ref_with_name(&c.name))));
    pub const STRUCT_TO: FieldPathResolver<LANG, SPEC> =
        |c| (c.name.clone(), ConversionType::expr_to(c, Some(Expression::obj_name(&c.name))));
    pub const STRUCT_DROP: FieldPathResolver<LANG, SPEC> =
        |c| (SPEC::Name::default(), ConversionType::expr_destroy(c, Some(Expression::ffi_ref_with_name(&c.name))));
    pub const TYPE_TO: FieldPathResolver<LANG, SPEC> =
        |c| (SPEC::Name::default(), ConversionType::expr_to(c, Some(Expression::name(&SPEC::Name::dictionary_name(DictionaryName::Obj)))));
}

impl<LANG, SPEC> FieldComposer<LANG, SPEC>
    where LANG: LangFermentable,
          SPEC: Specification<LANG, Name=Name<LANG, SPEC>> {
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


impl<LANG, SPEC> ToTokens for FieldComposer<LANG, SPEC>
    where LANG: LangFermentable + ToTokens,
          SPEC: Specification<LANG>,
          Self: SourceFermentable<LANG> {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        self.ferment().to_tokens(tokens);
    }
}

impl<SPEC> SourceFermentable<RustFermentate> for FieldComposer<RustFermentate, SPEC>
    where SPEC: RustSpecification {
    fn ferment(&self) -> RustFermentate {
        let Self { name, kind, attrs, .. } = self;
        RustFermentate::TokenStream(quote!(#(#attrs)* #name: #kind))
    }
}

impl<LANG, SPEC> ScopeContextPresentable for FieldComposer<LANG, SPEC>
    where Self: ToTokens,
          LANG: LangFermentable,
          SPEC: Specification<LANG> {
    type Presentation = TokenStream2;

    fn present(&self, _source: &ScopeContext) -> Self::Presentation {
        self.to_token_stream()
    }
}
