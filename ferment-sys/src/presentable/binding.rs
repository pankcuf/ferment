use proc_macro2::Ident;
use syn::{BareFnArg, Expr, Path, ReturnType, Type};
use syn::__private::TokenStream2;
use crate::ast::{CommaPunctuated, CommaPunctuatedTokens};
use crate::composable::FieldComposer;
use crate::composer::{BindingAccessorContext, CommaPunctuatedArgKinds, AspectArgComposers, NameKind, ArgKindPair, OwnerAspectSequence, SemiPunctuatedArgKinds, VarComposer, CommaPunctuatedArgs, SignatureAspect};
use crate::kind::SmartPointerKind;
use crate::lang::Specification;
use crate::presentable::{Aspect, SeqKind};
use crate::presentation::DictionaryExpr;

pub enum SmartPointerPresentableContext<SPEC> where SPEC: Specification {
    Ctor(FieldComposer<SPEC>, SPEC::Expr),
    Dtor(NameKind),
    Read(FieldComposer<SPEC>, Type, SPEC::Expr, SPEC::Expr),
    Write(FieldComposer<SPEC>, FieldComposer<SPEC>, SPEC::Expr, SPEC::Expr),
}
pub enum BindingPresentableContext<SPEC>
    where SPEC: Specification {
    Constructor(Aspect<SPEC::TYC>, SPEC::Attr, SPEC::Lt, SPEC::Gen, NameKind, CommaPunctuatedArgKinds<SPEC>, CommaPunctuatedArgKinds<SPEC>),
    VariantConstructor(Aspect<SPEC::TYC>, SPEC::Attr, SPEC::Lt, SPEC::Gen, NameKind, CommaPunctuatedArgKinds<SPEC>, CommaPunctuatedArgKinds<SPEC>),
    Destructor(Aspect<SPEC::TYC>, SPEC::Attr, SPEC::Lt, SPEC::Gen, NameKind),
    Callback(Aspect<SPEC::TYC>, SPEC::Attr, SPEC::Lt, SPEC::Gen, Ident, CommaPunctuatedArgs, ReturnType, CommaPunctuated<SPEC::Expr>, DictionaryExpr, ReturnType, CommaPunctuated<BareFnArg>),
    Getter(Aspect<SPEC::TYC>, SPEC::Attr, SPEC::Lt, SPEC::Gen, VarComposer<SPEC>, TokenStream2),
    Setter(Aspect<SPEC::TYC>, SPEC::Attr, SPEC::Lt, SPEC::Gen, VarComposer<SPEC>, TokenStream2),
    RegFn(Path, SPEC::Attr, SPEC::Lt, SPEC::Gen, bool, CommaPunctuatedArgKinds<SPEC>, ReturnType, SeqKind<SPEC>, SPEC::Expr),
    #[allow(unused)]
    RegFn2(Path, SPEC::Attr, SPEC::Lt, SPEC::Gen, bool, CommaPunctuatedTokens, CommaPunctuatedArgKinds<SPEC>, ReturnType, Type, SemiPunctuatedArgKinds<SPEC>, SPEC::Expr),

    SmartPointer(Aspect<SPEC::TYC>, SPEC::Attr, SPEC::Lt, SPEC::Gen, SmartPointerKind, SmartPointerPresentableContext<SPEC>),

    TraitVTableInnerFn(SPEC::Attr, Ident, CommaPunctuatedArgKinds<SPEC>, ReturnType),

    ArrayGetAtIndex(Aspect<SPEC::TYC>, SPEC::Attr, SPEC::Lt, SPEC::Gen, Type, Type, Expr),
    ArraySetAtIndex(Aspect<SPEC::TYC>, SPEC::Attr, SPEC::Lt, SPEC::Gen, Type, Type, Expr),

    ValueByKey(Aspect<SPEC::TYC>, SPEC::Attr, SPEC::Lt, SPEC::Gen, Type, Type, Type, Expr),
    SetValueForKey(Aspect<SPEC::TYC>, SPEC::Attr, SPEC::Lt, SPEC::Gen, Type, Type, Type, Expr),
    KeyByValue(Aspect<SPEC::TYC>, SPEC::Attr, SPEC::Lt, SPEC::Gen, Type, Type, Type, Expr),
    SetKeyForValue(Aspect<SPEC::TYC>, SPEC::Attr, SPEC::Lt, SPEC::Gen, Type, Type, Type, Expr),
    ResultOk(SPEC::Attr, SPEC::Lt, SPEC::Gen, Type, SPEC::Var),
    ResultError(SPEC::Attr, SPEC::Lt, SPEC::Gen, Type, SPEC::Var),
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
        let (obj_type, (attrs, lifetimes, generics), field_type, field_name) = context;
        Self::Getter(obj_type, attrs, lifetimes, generics, field_type, field_name)
    }
    pub fn set(context: BindingAccessorContext<SPEC>) -> Self {
        let (obj_type, (attrs, lifetimes, generics), field_type, field_name) = context;
        Self::Setter(obj_type, attrs, lifetimes, generics, field_type, field_name)
    }
    pub fn smart_pointer(kind: &SmartPointerKind, aspect: &Aspect<SPEC::TYC>, attrs: &SPEC::Attr, lifetimes: &SPEC::Lt, generics: &SPEC::Gen, lock_context: SmartPointerPresentableContext<SPEC>) -> Self {
        Self::SmartPointer(aspect.clone(), attrs.clone(), lifetimes.clone(), generics.clone(), kind.clone(), lock_context)
    }

    pub fn get_at_index(context: (
        Aspect<<SPEC as Specification>::TYC>,
        SignatureAspect<SPEC>,
        Type,
        Type,
    ), to_conversion_expr_value: Expr) -> Self {
        let (obj_type, (attrs, lifetimes, generics), arr_type, nested_type) = context;
        Self::ArrayGetAtIndex(obj_type, attrs, lifetimes, generics, arr_type, nested_type, to_conversion_expr_value)
    }
    pub fn set_at_index(context: (
        Aspect<<SPEC as Specification>::TYC>,
        SignatureAspect<SPEC>,
        Type,
        Type,
    ), from_conversion_expr_value: Expr) -> Self {
        let (obj_type, (attrs, lifetimes, generics), arr_type, nested_type) = context;
        Self::ArraySetAtIndex(obj_type, attrs, lifetimes, generics, arr_type, nested_type, from_conversion_expr_value)
    }
    pub fn key_by_value(context: (
        Aspect<<SPEC as Specification>::TYC>,
        SignatureAspect<SPEC>,
        Type,
        Type,
        Type,
    ), to_conversion_expr_value: Expr) -> Self {
        let (obj_type, (attrs, lifetimes, generics), map_type, key_type, value_type) = context;
        Self::KeyByValue(obj_type, attrs, lifetimes, generics, map_type, key_type, value_type, to_conversion_expr_value)
    }
    pub fn value_by_key(context: (
        Aspect<<SPEC as Specification>::TYC>,
        SignatureAspect<SPEC>,
        Type,
        Type,
        Type,
    ), to_conversion_expr_value: Expr) -> Self {
        let (obj_type, (attrs, lifetimes, generics), map_type, key_type, value_type) = context;
        Self::ValueByKey(obj_type, attrs, lifetimes, generics, map_type, key_type, value_type, to_conversion_expr_value)
    }
    pub fn set_key_for_value(context: (
        Aspect<<SPEC as Specification>::TYC>,
        SignatureAspect<SPEC>,
        Type,
        Type,
        Type,
    ), to_conversion_expr_value: Expr) -> Self {
        let (obj_type, (attrs, lifetimes, generics), map_type, key_type, value_type) = context;
        Self::SetKeyForValue(obj_type, attrs, lifetimes, generics, map_type, key_type, value_type, to_conversion_expr_value)
    }
    pub fn set_value_for_key(context: (
        Aspect<<SPEC as Specification>::TYC>,
        SignatureAspect<SPEC>,
        Type,
        Type,
        Type,
    ), to_conversion_expr_value: Expr) -> Self {
        let (obj_type, (attrs, lifetimes, generics), map_type, key_type, value_type) = context;
        Self::SetValueForKey(obj_type, attrs, lifetimes, generics, map_type, key_type, value_type, to_conversion_expr_value)
    }

    pub fn ctor_result_ok(context: (
        SignatureAspect<SPEC>,
        Type,
        SPEC::Var,
    )) -> Self {
        let ((attrs, lifetimes, generics), result_type, ok_type) = context;
        Self::ResultOk(attrs, lifetimes, generics, result_type, ok_type)
    }

    pub fn ctor_result_error(context: (
        SignatureAspect<SPEC>,
        Type,
        SPEC::Var,
    )) -> Self {
        let ((attrs, lifetimes, generics), result_type, error_type) = context;
        Self::ResultError(attrs, lifetimes, generics, result_type, error_type)
    }

}

