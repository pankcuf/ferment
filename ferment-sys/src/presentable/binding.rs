use syn::{Path, ReturnType, Type};
use syn::__private::TokenStream2;
use crate::ast::CommaPunctuatedTokens;
use crate::composable::FieldComposer;
use crate::composer::{BindingAccessorContext, CommaPunctuatedArgKinds, AspectArgComposers, NameKind, ArgKindPair, OwnerAspectSequence, SemiPunctuatedArgKinds, VariableComposer};
use crate::kind::SmartPointerKind;
use crate::lang::Specification;
use crate::presentable::{Aspect, SeqKind};

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
    Getter(Aspect<SPEC::TYC>, SPEC::Attr, SPEC::Lt, SPEC::Gen, VariableComposer<SPEC>, TokenStream2),
    Setter(Aspect<SPEC::TYC>, SPEC::Attr, SPEC::Lt, SPEC::Gen, VariableComposer<SPEC>, TokenStream2),
    RegFn(Path, bool, CommaPunctuatedArgKinds<SPEC>, ReturnType, SeqKind<SPEC>, SPEC::Expr, SPEC::Attr, SPEC::Lt, SPEC::Gen),
    #[allow(unused)]
    RegFn2(Path, bool, CommaPunctuatedTokens, CommaPunctuatedArgKinds<SPEC>, ReturnType, Type, SemiPunctuatedArgKinds<SPEC>, SPEC::Expr, SPEC::Attr, SPEC::Lt, SPEC::Gen),

    SmartPointer(SmartPointerKind, Aspect<SPEC::TYC>, SPEC::Attr, SPEC::Lt, SmartPointerPresentableContext<SPEC>),
}

impl<SPEC> BindingPresentableContext<SPEC>
    where SPEC: Specification {
    pub fn ctor<Iter: IntoIterator<Item=ArgKindPair<SPEC>>>(context: OwnerAspectSequence<SPEC, Iter>) -> Self {
        let ((ffi_type, (attrs, lifetimes, generics), name_kind, .. ), field_pairs) = context;
        let (args, names): (CommaPunctuatedArgKinds<SPEC>, CommaPunctuatedArgKinds<SPEC>) = field_pairs.into_iter().unzip();
        BindingPresentableContext::Constructor(ffi_type, attrs, lifetimes, generics, name_kind, args, names)
    }
    pub fn variant_ctor<Iter: IntoIterator<Item=ArgKindPair<SPEC>>>(context: OwnerAspectSequence<SPEC, Iter>) -> Self {
        let ((aspect, (attrs, lifetimes, generics), name_kind, .. ), field_pairs) = context;
        let (args, names): (CommaPunctuatedArgKinds<SPEC>, CommaPunctuatedArgKinds<SPEC>) = field_pairs.into_iter().unzip();
        BindingPresentableContext::VariantConstructor(aspect, attrs, lifetimes, generics, name_kind, args, names)
    }
    pub fn dtor(context: AspectArgComposers<SPEC>) -> Self {
        let ((ffi_type, (attrs, lifetimes, generics), name_kind), ..) = context;
        BindingPresentableContext::Destructor(ffi_type, attrs, lifetimes, generics, name_kind)
    }
    pub fn get(context: BindingAccessorContext<SPEC>) -> Self {
        let (obj_type, (attrs, lifetimes, generics), field_type, field_name, ..) = context;
        BindingPresentableContext::Getter(obj_type, attrs, lifetimes, generics, field_type, field_name)
    }
    pub fn set(context: BindingAccessorContext<SPEC>) -> Self {
        let (obj_type, (attrs, lifetimes, generics), field_type, field_name, ..) = context;
        BindingPresentableContext::Setter(obj_type, attrs, lifetimes, generics, field_type, field_name)
    }
    pub fn smart_pointer(kind: &SmartPointerKind, aspect: &Aspect<SPEC::TYC>, attrs: &SPEC::Attr, lifetimes: &SPEC::Lt, lock_context: SmartPointerPresentableContext<SPEC>) -> Self {
        BindingPresentableContext::SmartPointer(kind.clone(), aspect.clone(), attrs.clone(), lifetimes.clone(), lock_context)
    }
    // pub fn reg_fn(context: FunctionContext<SPEC>) -> Self {
    //     let (((ffi_type, attrs, generics, .. ), is_round), field_pairs) = context;
    //     let (args, names): (CommaPunctuatedOwnedItems<SPEC>, CommaPunctuatedOwnedItems<SPEC>) = field_pairs.into_iter().unzip();
    //     BindingPresentableContext::RegFn(
    //         path,
    //         asyncness.is_some(),
    //         arguments,
    //         return_type_presentation,
    //         input_conversions,
    //         return_type_conversion,
    //         attrs,
    //         generics
    //     )
    // }
}

