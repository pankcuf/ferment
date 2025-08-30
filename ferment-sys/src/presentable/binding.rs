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
    Constructor(Aspect<SPEC::TYC>, SignatureAspect<SPEC>, NameKind, CommaPunctuatedArgKinds<SPEC>, CommaPunctuatedArgKinds<SPEC>),
    VariantConstructor(Aspect<SPEC::TYC>, SignatureAspect<SPEC>, NameKind, CommaPunctuatedArgKinds<SPEC>, CommaPunctuatedArgKinds<SPEC>),
    Destructor(Aspect<SPEC::TYC>, SignatureAspect<SPEC>, NameKind),
    Callback(Aspect<SPEC::TYC>, SignatureAspect<SPEC>, Ident, CommaPunctuatedArgs, ReturnType, CommaPunctuated<SPEC::Expr>, DictionaryExpr, ReturnType, CommaPunctuated<BareFnArg>),
    Getter(Aspect<SPEC::TYC>, SignatureAspect<SPEC>, VarComposer<SPEC>, TokenStream2),
    Setter(Aspect<SPEC::TYC>, SignatureAspect<SPEC>, VarComposer<SPEC>, TokenStream2),
    RegFn(Path, SignatureAspect<SPEC>, bool, CommaPunctuatedArgKinds<SPEC>, ReturnType, SeqKind<SPEC>, SPEC::Expr),
    #[allow(unused)]
    RegFn2(Path, SignatureAspect<SPEC>, bool, CommaPunctuatedTokens, CommaPunctuatedArgKinds<SPEC>, ReturnType, Type, SemiPunctuatedArgKinds<SPEC>, SPEC::Expr),

    SmartPointer(Aspect<SPEC::TYC>, SignatureAspect<SPEC>, SmartPointerKind, SmartPointerPresentableContext<SPEC>),

    TraitVTableInnerFn(SignatureAspect<SPEC>, Ident, CommaPunctuatedArgKinds<SPEC>, ReturnType),

    ArrayGetAtIndex(Aspect<SPEC::TYC>, SignatureAspect<SPEC>, Type, Type, Expr),
    ArraySetAtIndex(Aspect<SPEC::TYC>, SignatureAspect<SPEC>, Type, Type, Expr),

    ValueByKey(Aspect<SPEC::TYC>, SignatureAspect<SPEC>, Type, Type, Type, Expr),
    SetValueForKey(Aspect<SPEC::TYC>, SignatureAspect<SPEC>, Type, Type, Type, Expr),
    KeyByValue(Aspect<SPEC::TYC>, SignatureAspect<SPEC>, Type, Type, Type, Expr),
    SetKeyForValue(Aspect<SPEC::TYC>, SignatureAspect<SPEC>, Type, Type, Type, Expr),
    ResultOk(SignatureAspect<SPEC>, Type, SPEC::Var),
    ResultError(SignatureAspect<SPEC>, Type, SPEC::Var),
}

impl<SPEC> BindingPresentableContext<SPEC>
    where SPEC: Specification {
    pub fn ctor<Iter: IntoIterator<Item=ArgKindPair<SPEC>>>(context: OwnerAspectSequence<SPEC, Iter>) -> Self {
        let ((ffi_type, signature_context, name_kind, .. ), field_pairs) = context;
        let (args, names): (CommaPunctuatedArgKinds<SPEC>, CommaPunctuatedArgKinds<SPEC>) = field_pairs.into_iter().unzip();
        Self::Constructor(ffi_type, signature_context, name_kind, args, names)
    }
    pub fn variant_ctor<Iter: IntoIterator<Item=ArgKindPair<SPEC>>>(context: OwnerAspectSequence<SPEC, Iter>) -> Self {
        let ((aspect, signature_context, name_kind, .. ), field_pairs) = context;
        let (args, names): (CommaPunctuatedArgKinds<SPEC>, CommaPunctuatedArgKinds<SPEC>) = field_pairs.into_iter().unzip();
        Self::VariantConstructor(aspect, signature_context, name_kind, args, names)
    }
    pub fn dtor(context: AspectArgComposers<SPEC>) -> Self {
        let ((ffi_type, signature_context, name_kind), ..) = context;
        Self::Destructor(ffi_type, signature_context, name_kind)
    }
    pub fn get(context: BindingAccessorContext<SPEC>) -> Self {
        let (obj_type, signature_context, field_type, field_name) = context;
        Self::Getter(obj_type, signature_context, field_type, field_name)
    }
    pub fn set(context: BindingAccessorContext<SPEC>) -> Self {
        let (obj_type, signature_context, field_type, field_name) = context;
        Self::Setter(obj_type, signature_context, field_type, field_name)
    }
    pub fn smart_pointer(kind: &SmartPointerKind, aspect: &Aspect<SPEC::TYC>, signature_aspect: &SignatureAspect<SPEC>, lock_context: SmartPointerPresentableContext<SPEC>) -> Self {
        Self::SmartPointer(aspect.clone(), signature_aspect.clone(), kind.clone(), lock_context)
    }

    pub fn get_at_index(context: (
        Aspect<<SPEC as Specification>::TYC>,
        SignatureAspect<SPEC>,
        Type,
        Type,
    ), to_conversion_expr_value: Expr) -> Self {
        let (obj_type, signature_context, arr_type, nested_type) = context;
        Self::ArrayGetAtIndex(obj_type, signature_context, arr_type, nested_type, to_conversion_expr_value)
    }
    pub fn set_at_index(context: (
        Aspect<<SPEC as Specification>::TYC>,
        SignatureAspect<SPEC>,
        Type,
        Type,
    ), from_conversion_expr_value: Expr) -> Self {
        let (obj_type, signature_context, arr_type, nested_type) = context;
        Self::ArraySetAtIndex(obj_type, signature_context, arr_type, nested_type, from_conversion_expr_value)
    }
    pub fn key_by_value(context: (
        Aspect<<SPEC as Specification>::TYC>,
        SignatureAspect<SPEC>,
        Type,
        Type,
        Type,
    ), to_conversion_expr_value: Expr) -> Self {
        let (obj_type, signature_context, map_type, key_type, value_type) = context;
        Self::KeyByValue(obj_type, signature_context, map_type, key_type, value_type, to_conversion_expr_value)
    }
    pub fn value_by_key(context: (
        Aspect<<SPEC as Specification>::TYC>,
        SignatureAspect<SPEC>,
        Type,
        Type,
        Type,
    ), to_conversion_expr_value: Expr) -> Self {
        let (obj_type, signature_context, map_type, key_type, value_type) = context;
        Self::ValueByKey(obj_type, signature_context, map_type, key_type, value_type, to_conversion_expr_value)
    }
    pub fn set_key_for_value(context: (
        Aspect<<SPEC as Specification>::TYC>,
        SignatureAspect<SPEC>,
        Type,
        Type,
        Type,
    ), to_conversion_expr_value: Expr) -> Self {
        let (obj_type, signature_context, map_type, key_type, value_type) = context;
        Self::SetKeyForValue(obj_type, signature_context, map_type, key_type, value_type, to_conversion_expr_value)
    }
    pub fn set_value_for_key(context: (
        Aspect<<SPEC as Specification>::TYC>,
        SignatureAspect<SPEC>,
        Type,
        Type,
        Type,
    ), to_conversion_expr_value: Expr) -> Self {
        let (obj_type, signature_context, map_type, key_type, value_type) = context;
        Self::SetValueForKey(obj_type, signature_context, map_type, key_type, value_type, to_conversion_expr_value)
    }

    pub fn ctor_result_ok(context: (
        SignatureAspect<SPEC>,
        Type,
        SPEC::Var,
    )) -> Self {
        let (signature_context, result_type, ok_type) = context;
        Self::ResultOk(signature_context, result_type, ok_type)
    }

    pub fn ctor_result_error(context: (
        SignatureAspect<SPEC>,
        Type,
        SPEC::Var,
    )) -> Self {
        let (signature_context, result_type, error_type) = context;
        Self::ResultError(signature_context, result_type, error_type)
    }

}

