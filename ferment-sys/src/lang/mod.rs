pub(crate) mod rust;

#[cfg(feature = "objc")]
pub(crate) mod objc;

#[cfg(feature = "java")]
pub(crate) mod java;


use std::fmt::{Debug, Display};
use proc_macro2::Ident;
use quote::ToTokens;
use syn::{Attribute, Generics, Lifetime, Type};
use crate::composer::{ConversionFromComposer, ConversionToComposer, VarComposable, VarComposer};
#[cfg(any(feature = "objc", feature = "java"))]
use crate::error;
#[cfg(feature = "objc")]
use crate::Config;
#[cfg(feature = "objc")]
use crate::kind::GenericTypeKind;
#[cfg(feature = "objc")]
use crate::ext::FFIVarResolve;
use crate::ext::{Mangle, MangleDefault, ToType};
#[cfg(feature = "objc")]
use crate::lang::objc::composers::AttrWrapper;
use crate::presentable::{NameTreeContext, TypeContext, Expression, ExpressionComposable};
use crate::presentation::{DictionaryName, FFIVariable, InterfacePresentation, Name, RustFermentate};
#[cfg(any(feature = "objc", feature = "java"))]
use crate::tree::CrateTree;


#[cfg(any(feature = "objc", feature = "java"))]
pub trait CrateTreeConsumer {
    fn generate(&self, crate_tree: &CrateTree) -> Result<(), error::Error>;
}

pub trait FromDictionary {
    fn dictionary_name(dictionary: DictionaryName) -> Self;
}

pub trait NameComposable<SPEC>
    where SPEC: Specification {
    fn ident(ident: Ident) -> Self;
    fn index(ident: usize) -> Self;
    fn unnamed_arg(index: usize) -> Self;
}

pub trait LangFermentable: Clone + Debug {

}
pub trait Specification: Clone + Debug {
    type Attr: Clone + LangAttrSpecification<Self::Fermentate> + Debug;
    type Gen: LangGenSpecification<Self::Fermentate>;
    type Lt: LangLifetimeSpecification<Self::Fermentate>;
    type TYC: NameTreeContext;
    type Interface: ToTokens;
    type Expr: ExpressionComposable<Self>;
    type Var: VarComposable<Self> + ToType;
    type Name: Clone + Default + Display + ToTokens + Mangle<MangleDefault> + FromDictionary + NameComposable<Self>;
    type Fermentate: LangFermentable + ToTokens;

    fn value_var(ty: &Type) -> VarComposer<Self> {
        VarComposer::<Self>::value(ty)
    }

    fn value_expr_from(name: Self::Name, ty: &Type, expr: Self::Expr) -> ConversionFromComposer<Self> {
        ConversionFromComposer::<Self>::value_expr(name, ty, expr)
    }
    fn value_expr_to(name: Self::Name, ty: &Type, expr: Self::Expr) -> ConversionToComposer<Self> {
        ConversionToComposer::<Self>::value_expr(name, ty, expr)
    }
}

#[derive(Clone, Debug)]
pub struct RustSpecification;

impl Specification for RustSpecification {
    type Attr = Vec<Attribute>;
    type Gen = Option<Generics>;
    type Lt = Vec<Lifetime>;
    type TYC = TypeContext;
    type Interface = InterfacePresentation;
    type Expr = Expression<Self>;
    type Var = FFIVariable<Self, Type>;
    type Name = Name<Self>;
    type Fermentate = RustFermentate;
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
#[cfg(feature = "objc")]
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

#[cfg(any(feature = "objc", feature = "java"))]
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

#[cfg(feature = "objc")]
impl Config {
    pub fn maybe_objc_config(&self) -> Option<&objc::Config> {
        self.languages.iter().find_map(|lang| match lang {
            Lang::ObjC(config) => Some(config),
            #[cfg(feature = "java")]
            _ => None
        })
    }
}

#[cfg(feature = "objc")]
impl FFIVarResolve<objc::ObjCSpecification> for GenericTypeKind {}
