use std::fmt::{Debug, Display, Formatter};
use std::marker::PhantomData;
use syn::{Attribute, Type};
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};
use ferment_macro::Display;
use crate::composable::CfgAttributes;
use crate::composer::SourceFermentable;
use crate::context::ScopeContext;
use crate::ext::ToType;
use crate::lang::{LangAttrSpecification, LangFermentable, PresentableSpecification, RustSpecification, Specification};
use crate::presentable::{Aspect, Expression, ArgKind, SeqKind, ScopeContextPresentable};
use crate::presentation::{DictionaryName, Name, RustFermentate};

#[derive(Clone, Debug, Display)]
pub enum FieldTypeKind<LANG, SPEC>
    where LANG: LangFermentable,
          SPEC: Specification<LANG, Var: ToType>,
          Aspect<SPEC::TYC>: ScopeContextPresentable {
    Type(Type),
    Conversion(TokenStream2),
    Var(SPEC::Var)
}

// impl<LANG, SPEC> ToType for FieldTypeKind<LANG, SPEC>
//     where LANG: LangFermentable,
//           SPEC: Specification<LANG>,
//           Aspect<SPEC::TYC>: ScopeContextPresentable {
//     fn to_type(&self) -> Type {
//         match self {
//             FieldTypeKind::Type(ty) => ty.clone(),
//             FieldTypeKind::Var(var) => {
//                 var.to_type()
//             },
//             _ => panic!("improper use of conversion as type")
//         }
//     }
// }
impl<LANG, SPEC> ToTokens for FieldTypeKind<LANG, SPEC>
    where LANG: LangFermentable,
          SPEC: Specification<LANG, Var: ToType>,
          Aspect<SPEC::TYC>: ScopeContextPresentable {
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
          SPEC: Specification<LANG, Var: ToType>,
          Aspect<SPEC::TYC>: ScopeContextPresentable {
    fn to_type(&self) -> Type {
        match self {
            FieldTypeKind::Type(ty) => ty.clone(),
            FieldTypeKind::Var(var) => var.to_type(),
            // FieldTypeKind::Var(var) =>{
            //     let ty = var.to_type();
            //     &ty
            // },// var.to_type()
            _ => panic!("improper use of conversion as type")
        }
    }
}
impl<LANG, SPEC> FieldTypeKind<LANG, SPEC>
    where LANG: LangFermentable,
          SPEC: Specification<LANG, Var: ToType>,
          Aspect<SPEC::TYC>: ScopeContextPresentable {
    pub fn ty(&self) -> &Type {
        match self {
            FieldTypeKind::Type(ty) => ty,
            // FieldTypeKind::Var(var) =>{
            //     let ty = var.to_type();
            //     &ty
            // },// var.to_type()
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
          SPEC: Specification<LANG, Var: ToType>,
          Aspect<SPEC::TYC>: ScopeContextPresentable {
    pub attrs: SPEC::Attr,
    pub name: SPEC::Name,
    pub kind: FieldTypeKind<LANG, SPEC>,
    pub named: bool,
    _marker: PhantomData<LANG>,
}

impl<LANG, SPEC> Display for FieldComposer<LANG, SPEC>
    where LANG: LangFermentable,
          SPEC: Specification<LANG, Attr: std::fmt::Display, Name: ToTokens, Var: ToType> + Display,
          Aspect<SPEC::TYC>: ScopeContextPresentable {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("FieldComposer({}({}), {}({}), {}, {}",
                            self.name, self.name.to_token_stream(),
                            self.kind, self.kind.to_token_stream(),
                            self.named, self.attrs).as_str())
    }
}

impl<SPEC> FieldComposer<RustFermentate, SPEC> where SPEC: RustSpecification {
    pub fn self_typed(ty: Type, attrs: &Vec<Attribute>) -> Self {
        Self::new(Name::Dictionary(DictionaryName::Self_), FieldTypeKind::Type(ty), true, attrs.cfg_attributes())
    }


}

impl<LANG, SPEC> FieldComposer<LANG, SPEC>
    where LANG: LangFermentable,
          SPEC: Specification<LANG, Var: ToType>,
          SPEC::Name: ToTokens,
          Aspect<SPEC::TYC>: ScopeContextPresentable {
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

impl<LANG, SPEC> ToTokens for FieldComposer<LANG, SPEC>
    where LANG: LangFermentable + ToTokens,
          SPEC: PresentableSpecification<LANG, Var: ToType>,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          Expression<LANG, SPEC>: ScopeContextPresentable,
          SeqKind<LANG, SPEC>: ScopeContextPresentable,
          ArgKind<LANG, SPEC>: ScopeContextPresentable,
          Self: SourceFermentable<LANG> {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        self.ferment().to_tokens(tokens);
        // let Self { name, kind, attrs, .. } = self;
        // quote!(#(#attrs)* #name: #kind).to_tokens(tokens)
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
          SPEC: Specification<LANG, Var: ToType>,
          Aspect<SPEC::TYC>: ScopeContextPresentable {
    type Presentation = TokenStream2;

    fn present(&self, _source: &ScopeContext) -> Self::Presentation {
        self.to_token_stream()
    }
}
