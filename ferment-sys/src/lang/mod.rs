pub(crate) mod rust;

#[cfg(feature = "objc")]
pub(crate) mod objc;

#[cfg(feature = "java")]
pub(crate) mod java;


use std::fmt::{Debug, Display};
use proc_macro2::Ident;
use quote::ToTokens;
use syn::{Attribute, Generics, Lifetime, Type};
use crate::composer::VarComposable;
use crate::error;
use crate::ext::{Mangle, MangleDefault, ToType};
use crate::lang::objc::composers::AttrWrapper;
use crate::presentable::{NameTreeContext, ScopeContextPresentable, TypeContext, Expression, ExpressionComposable};
use crate::presentation::{DictionaryName, FFIVariable, InterfacePresentation, Name, RustFermentate};
use crate::tree::CrateTree;


pub trait CrateTreeConsumer {
    fn generate(&self, crate_tree: &CrateTree) -> Result<(), error::Error>;
}

pub trait FromDictionary {
    fn dictionary_name(dictionary: DictionaryName) -> Self;
}

pub trait NameComposable<LANG, SPEC>
    where LANG: LangFermentable,
          SPEC: Specification<LANG> {
    fn ident(ident: Ident) -> Self;
    fn index(ident: usize) -> Self;
    fn unnamed_arg(index: usize) -> Self;
}

pub trait LangFermentable: Clone + Debug {}
pub trait Specification<LANG>: Clone + Debug
    where LANG: LangFermentable {
    type Attr: Clone + LangAttrSpecification<LANG> + Debug;
    type Gen: LangGenSpecification<LANG>;
    type Lt: LangLifetimeSpecification<LANG>;
    type TYC: NameTreeContext;
    type Interface: ToTokens;
    type Expr: ExpressionComposable<LANG, Self>;
    type Var: VarComposable<LANG, Self> + ToType;
    type Name: Clone + Default + Display + ToTokens + Mangle<MangleDefault> + FromDictionary + NameComposable<LANG, Self>;
}

pub trait PresentableSpecification<LANG>:
    Specification<LANG, Expr=Expression<LANG, Self>>
    where LANG: LangFermentable,
          Expression<LANG, Self>: ScopeContextPresentable,
          <Self::Expr as ScopeContextPresentable>::Presentation: ToTokens {}

impl<LANG, SPEC> PresentableSpecification<LANG> for SPEC
    where LANG: LangFermentable,
          SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>>,
          Expression<LANG, SPEC>: ScopeContextPresentable {}


pub trait RustSpecification:
    PresentableSpecification<RustFermentate,
        Attr=Vec<Attribute>,
        Gen=Option<Generics>,
        Lt=Vec<Lifetime>,
        Interface=InterfacePresentation,
        TYC=TypeContext,
        Expr=Expression<RustFermentate, Self>,
        Var=FFIVariable<RustFermentate, Self, Type>,
        Name=Name<RustFermentate, Self>
    > {}

impl<SPEC> Specification<RustFermentate> for SPEC where SPEC: RustSpecification {
    type Attr = Vec<Attribute>;
    type Gen = Option<Generics>;
    type Lt = Vec<Lifetime>;
    type TYC = TypeContext;
    type Interface = InterfacePresentation;
    type Expr = Expression<RustFermentate, SPEC>;
    type Var = FFIVariable<RustFermentate, SPEC, Type>;
    type Name = Name<RustFermentate, SPEC>;
}


pub trait LangAttrSpecification<T: Clone>: Clone + Default {
    fn from_attrs(attrs: Vec<Attribute>) -> Self;
}
pub trait LangGenSpecification<T: Clone>: Clone + Default + Debug {
    fn from_generics(generics: Option<Generics>) -> Self;
}
pub trait LangLifetimeSpecification<T: Clone>: Clone + Default + Debug {
    #[allow(unused)]
    fn from_lifetimes(lifetimes: Vec<Lifetime>) -> Self;
}

impl<T> LangAttrSpecification<T> for Vec<Attribute> where T: Clone {
    fn from_attrs(attrs: Vec<Attribute>) -> Self {
        attrs
    }
}
impl<T> LangGenSpecification<T> for Option<Generics> where T: Clone {
    fn from_generics(generics: Option<Generics>) -> Self {
        generics
    }
}
impl<T> LangLifetimeSpecification<T> for Vec<Lifetime> where T: Clone {
    fn from_lifetimes(lifetimes: Vec<Lifetime>) -> Self {
        lifetimes
    }
}
impl<T> LangAttrSpecification<T> for AttrWrapper where T: Clone {
    fn from_attrs(attrs: Vec<Attribute>) -> Self {
        AttrWrapper::from(attrs)
    }
}

#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum Lang {
    #[cfg(feature = "objc")]
    ObjC(objc::Config),
    #[cfg(feature = "java")]
    Java(java::Config)
}

impl CrateTreeConsumer for Lang {
    fn generate(&self, crate_tree: &CrateTree) -> Result<(), error::Error> {
        match self {
            #[cfg(feature = "objc")]
            Lang::ObjC(config) =>
                config.generate(crate_tree),
            #[cfg(feature = "java")]
            Lang::Java(config) =>
                config.generate(crate_tree),
            #[cfg(all(not(feature = "objc"), not(feature = "java")))]
            _ => Ok(())
        }
    }
}
