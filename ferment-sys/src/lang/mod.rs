pub(crate) mod rust;

#[cfg(feature = "objc")]
pub(crate) mod objc;

#[cfg(feature = "java")]
pub(crate) mod java;


use std::fmt::Debug;
use quote::ToTokens;
use syn::{Attribute, Generics, Type};
use crate::composer::VarComposable;
use crate::error;
use crate::ext::ToType;
use crate::lang::objc::composers::AttrWrapper;
use crate::presentable::{Aspect, NameTreeContext, PresentableArgument, ScopeContextPresentable, PresentableSequence, TypeContext, Expression, ExpressionComposable};
use crate::presentation::{FFIVariable, InterfacePresentation, RustFermentate};
use crate::tree::CrateTree;


pub trait CrateTreeConsumer {
    fn generate(&self, crate_tree: &CrateTree) -> Result<(), error::Error>;
}

pub trait LangFermentable: Clone + Debug {}


pub trait Specification<LANG>: Clone + Debug
    where LANG: LangFermentable,
          Aspect<Self::TYC>: ScopeContextPresentable,
          Self::Expr: Clone + ScopeContextPresentable {
    type Attr: LangAttrSpecification<LANG>;
    type Gen: LangGenSpecification<LANG>;
    type TYC: NameTreeContext;
    type Interface: ToTokens;
    // type Expr: Clone + ScopeContextPresentable;
    type Expr: ExpressionComposable<LANG, Self>;
    type Var: VarComposable<LANG, Self>;
    // type Var: ToTokens + Clone + Debug;
}

// pub trait ObjectSpecification<'a, LANG>
//     where LANG: LangFermentable,
//           Aspect<<Self::SPEC as Specification<LANG>>::TYC>: ScopeContextPresentable,
//           Aspect<Self::SPEC>: ScopeContextPresentable {
//     type SPEC: Specification<LANG>;
//     type Type: Composer<'a>;
//     type Variable: Composer<'a>;
//     type Create: Composer<'a>;
//     type Destroy: Composer<'a>;
//     type From: Composer<'a>;
//     type To: Composer<'a>;
// }

// pub trait Specification

pub trait PresentableSpecification<LANG>: Specification<LANG, Expr=Expression<LANG, Self>, Var: ToType>
    where LANG: LangFermentable,
          Aspect<Self::TYC>: ScopeContextPresentable,
          PresentableSequence<LANG, Self>: ScopeContextPresentable,
          PresentableArgument<LANG, Self>: ScopeContextPresentable {}

pub trait RustSpecification:
    PresentableSpecification<RustFermentate,
        Attr=Vec<Attribute>,
        Gen=Option<Generics>,
        Interface=InterfacePresentation,
        TYC=TypeContext,
        Expr=Expression<RustFermentate, Self>,
        Var=FFIVariable<Type, RustFermentate, Self>
    > where <Self::Expr as ScopeContextPresentable>::Presentation: ToTokens {}

impl<T> PresentableSpecification<RustFermentate> for T where T: RustSpecification {}
impl<T> Specification<RustFermentate> for T where T: RustSpecification {
    type Attr = Vec<Attribute>;
    type Gen = Option<Generics>;
    type TYC = TypeContext;
    type Interface = InterfacePresentation;
    type Expr = Expression<RustFermentate, T>;
    type Var = FFIVariable<Type, RustFermentate, T>;
}

pub trait LangAttrSpecification<T: Clone>: Clone + Default {

    fn from_attrs(attrs: Vec<Attribute>) -> Self;
}
pub trait LangGenSpecification<T: Clone>: Clone + Default + Debug {

    fn from_generics(generics: Option<Generics>) -> Self;
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

// pub struct ScopeTreeFermentate {
//     pub lang: Lang,
//     pub tree: ScopeTree
// }