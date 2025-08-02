use proc_macro2::Ident;
use syn::{BareFnArg, Path, ReturnType, Type};
use syn::__private::TokenStream2;
use crate::ast::{CommaPunctuated, CommaPunctuatedTokens};
use crate::composable::FieldComposer;
use crate::composer::{BindingAccessorContext, CommaPunctuatedArgKinds, AspectArgComposers, NameKind, ArgKindPair, OwnerAspectSequence, SemiPunctuatedArgKinds, VarComposer, CommaPunctuatedArgs};
use crate::kind::SmartPointerKind;
use crate::lang::Specification;
use crate::presentable::{Aspect, SeqKind};
use crate::presentation::DictionaryExpr;

pub enum SmartPointerPresentableContext<SPEC> where SPEC: Specification {
    Ctor(FieldComposer<SPEC>, SPEC::Expr),
    Dtor(SPEC::Gen, NameKind),
    Read(FieldComposer<SPEC>, Type, SPEC::Expr, SPEC::Expr),
    Write(FieldComposer<SPEC>, FieldComposer<SPEC>, SPEC::Expr, SPEC::Expr),
}
pub enum BindingPresentableContext<SPEC>
    where SPEC: Specification {
    Constructor(Aspect<SPEC::TYC>, SPEC::Attr, SPEC::Lt, SPEC::Gen, NameKind, CommaPunctuatedArgKinds<SPEC>, CommaPunctuatedArgKinds<SPEC>),
    VariantConstructor(Aspect<SPEC::TYC>, SPEC::Attr, SPEC::Lt, SPEC::Gen, NameKind, CommaPunctuatedArgKinds<SPEC>, CommaPunctuatedArgKinds<SPEC>),
    Destructor(Aspect<SPEC::TYC>, SPEC::Attr, SPEC::Lt, SPEC::Gen, NameKind),
    Getter(Aspect<SPEC::TYC>, SPEC::Attr, SPEC::Lt, SPEC::Gen, VarComposer<SPEC>, TokenStream2),
    Setter(Aspect<SPEC::TYC>, SPEC::Attr, SPEC::Lt, SPEC::Gen, VarComposer<SPEC>, TokenStream2),
    RegFn(Path, bool, CommaPunctuatedArgKinds<SPEC>, ReturnType, SeqKind<SPEC>, SPEC::Expr, SPEC::Attr, SPEC::Lt, SPEC::Gen),
    #[allow(unused)]
    RegFn2(Path, bool, CommaPunctuatedTokens, CommaPunctuatedArgKinds<SPEC>, ReturnType, Type, SemiPunctuatedArgKinds<SPEC>, SPEC::Expr, SPEC::Attr, SPEC::Lt, SPEC::Gen),

    SmartPointer(SmartPointerKind, Aspect<SPEC::TYC>, SPEC::Attr, SPEC::Lt, SmartPointerPresentableContext<SPEC>),

    TraitVTableInnerFn(SPEC::Attr, Ident, CommaPunctuatedArgKinds<SPEC>, ReturnType),
    Callback(Aspect<SPEC::TYC>, SPEC::Attr, SPEC::Lt, SPEC::Gen, Ident, CommaPunctuatedArgs, ReturnType, CommaPunctuated<SPEC::Expr>, DictionaryExpr, ReturnType, CommaPunctuated<BareFnArg>),
}

impl<SPEC> BindingPresentableContext<SPEC>
    where SPEC: Specification {
    pub fn ctor<Iter: IntoIterator<Item=ArgKindPair<SPEC>>>(context: OwnerAspectSequence<SPEC, Iter>) -> Self {
        let ((ffi_type, (attrs, lifetimes, generics), name_kind, .. ), field_pairs) = context;
        let (args, names): (CommaPunctuatedArgKinds<SPEC>, CommaPunctuatedArgKinds<SPEC>) = field_pairs.into_iter().unzip();
        Self::Constructor(ffi_type, attrs, lifetimes, generics, name_kind, args, names)
    }
    pub fn variant_ctor<Iter: IntoIterator<Item=ArgKindPair<SPEC>>>(context: OwnerAspectSequence<SPEC, Iter>) -> Self {
        let ((aspect, (attrs, lifetimes, generics), name_kind, .. ), field_pairs) = context;
        let (args, names): (CommaPunctuatedArgKinds<SPEC>, CommaPunctuatedArgKinds<SPEC>) = field_pairs.into_iter().unzip();
        Self::VariantConstructor(aspect, attrs, lifetimes, generics, name_kind, args, names)
    }
    pub fn dtor(context: AspectArgComposers<SPEC>) -> Self {
        let ((ffi_type, (attrs, lifetimes, generics), name_kind), ..) = context;
        Self::Destructor(ffi_type, attrs, lifetimes, generics, name_kind)
    }
    pub fn get(context: BindingAccessorContext<SPEC>) -> Self {
        let (obj_type, (attrs, lifetimes, generics), field_type, field_name, ..) = context;
        Self::Getter(obj_type, attrs, lifetimes, generics, field_type, field_name)
    }
    pub fn set(context: BindingAccessorContext<SPEC>) -> Self {
        let (obj_type, (attrs, lifetimes, generics), field_type, field_name, ..) = context;
        Self::Setter(obj_type, attrs, lifetimes, generics, field_type, field_name)
    }
    pub fn smart_pointer(kind: &SmartPointerKind, aspect: &Aspect<SPEC::TYC>, attrs: &SPEC::Attr, lifetimes: &SPEC::Lt, lock_context: SmartPointerPresentableContext<SPEC>) -> Self {
        Self::SmartPointer(kind.clone(), aspect.clone(), attrs.clone(), lifetimes.clone(), lock_context)
    }
}

