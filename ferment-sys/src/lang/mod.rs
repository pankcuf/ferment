pub(crate) mod rust;

#[cfg(feature = "objc")]
pub(crate) mod objc;

#[cfg(feature = "java")]
pub(crate) mod java;


use std::fmt::{Debug, Display};
use quote::ToTokens;
use syn::{Attribute, Generics, Type};
use crate::composer::VarComposable;
use crate::error;
use crate::ext::{Mangle, MangleDefault, ToType};
use crate::lang::objc::composers::AttrWrapper;
use crate::presentable::{Aspect, NameTreeContext, ArgKind, ScopeContextPresentable, SeqKind, TypeContext, Expression, ExpressionComposable};
use crate::presentation::{FFIVariable, InterfacePresentation, Name, RustFermentate};
use crate::tree::CrateTree;


pub trait CrateTreeConsumer {
    fn generate(&self, crate_tree: &CrateTree) -> Result<(), error::Error>;
}

pub trait LangFermentable: Clone + Debug {}
pub trait Specification<LANG>: Clone + Debug
    where LANG: LangFermentable,
          Aspect<Self::TYC>: ScopeContextPresentable {
    type Attr: Clone + LangAttrSpecification<LANG> + Debug;
    type Gen: LangGenSpecification<LANG>;
    type TYC: NameTreeContext;
    type Interface: ToTokens;
    type Expr: ExpressionComposable<LANG, Self>;
    type Var: VarComposable<LANG, Self> + ToType;
    type Name: Clone + Display + ToTokens + Mangle<MangleDefault>;
    // type Alloc: AspectAllocator<Self::TYC>;
}

// pub trait CommonSpecificationBounds<LANG, SPEC>: Specification<LANG>
//     where
//         LANG: LangFermentable,
//         SPEC: Specification<LANG>,
//         Aspect<SPEC::TYC>: ScopeContextPresentable,  // Explicitly require this bound for TYC
//         SPEC::Expr: ScopeContextPresentable,         // Explicitly require this bound for Expr
//         SeqKind<LANG, SPEC>: ScopeContextPresentable,
//         ArgKind<LANG, SPEC>: ScopeContextPresentable,
//         <SPEC::Expr as ScopeContextPresentable>::Presentation: ToTokens,
// {
//     // You can add additional bounds here if needed.
// }

// impl<LANG, SPEC> CommonSpecificationBounds<LANG, SPEC> for SPEC
//     where
//         LANG: LangFermentable,
//         SPEC: PresentableSpecification<LANG>,
//         Aspect<SPEC::TYC>: ScopeContextPresentable,  // Ensure the trait bound is satisfied
//         SPEC::Expr: ScopeContextPresentable,         // Ensure the trait bound is satisfied
//         SeqKind<LANG, SPEC>: ScopeContextPresentable,
//         ArgKind<LANG, SPEC>: ScopeContextPresentable,
//         <SPEC::Expr as ScopeContextPresentable>::Presentation: ToTokens,
// {}

pub trait PresentableSpecification<LANG>
: Specification<
    LANG,
    Attr: Debug,
    Expr=Expression<LANG, Self>,
    Var: ToType
>
    where LANG: LangFermentable,
          Aspect<Self::TYC>: ScopeContextPresentable,
          Expression<LANG, Self>: ScopeContextPresentable,
          SeqKind<LANG, Self>: ScopeContextPresentable,
          ArgKind<LANG, Self>: ScopeContextPresentable,
          <Self::Expr as ScopeContextPresentable>::Presentation: ToTokens {}

impl<LANG, SPEC> PresentableSpecification<LANG> for SPEC
    where LANG: LangFermentable,
          SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>>,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          Expression<LANG, SPEC>: ScopeContextPresentable,
          ArgKind<LANG, SPEC>: ScopeContextPresentable,
          SeqKind<LANG, SPEC>: ScopeContextPresentable {}


pub trait RustSpecification:
    PresentableSpecification<RustFermentate,
        Attr=Vec<Attribute>,
        Gen=Option<Generics>,
        Interface=InterfacePresentation,
        TYC=TypeContext,
        Expr=Expression<RustFermentate, Self>,
        Var=FFIVariable<Type, RustFermentate, Self>,
        Name=Name<RustFermentate, Self>
    > {}
// pub struct RustAspectAllocator {
//
// }
// impl<WR> AspectAllocator<WR, Comma, Aspect<>> for RustAspectAllocator
// where WR: DelimiterTrait + ?Sized {
//     fn allocate<LANG, SPEC>(&self, aspect: &Wrapped<AspectPresentableArguments<Comma, LANG, SPEC>, Comma, WR>) -> Self {
//
//     }
//
// }
impl<SPEC> Specification<RustFermentate> for SPEC where SPEC: RustSpecification {
    type Attr = Vec<Attribute>;
    type Gen = Option<Generics>;
    type TYC = TypeContext;
    type Interface = InterfacePresentation;
    type Expr = Expression<RustFermentate, SPEC>;
    type Var = FFIVariable<Type, RustFermentate, SPEC>;
    type Name = Name<RustFermentate, SPEC>;
    // type Alloc = RustAspectAllocator;
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
