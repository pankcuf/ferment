use std::fmt::Debug;
use syn::{Type, Visibility};
use syn::__private::TokenStream2;
use ferment_macro::Display;
use crate::composable::FieldComposer;
use crate::composer::{FieldPathResolver, PresentableExprComposerRef};
use crate::lang::Specification;
use crate::presentable::SeqKind;

#[derive(Clone, Debug, Display)]
pub enum ArgKind<SPEC>
    where SPEC: Specification {
    AttrExhaustive(SPEC::Attr),
    AttrSequence(SeqKind<SPEC>, SPEC::Attr),
    AttrName(TokenStream2, SPEC::Attr),
    AttrExpression(SPEC::Expr, SPEC::Attr),
    AttrExpressionComposer(FieldComposer<SPEC>, FieldPathResolver<SPEC>, PresentableExprComposerRef<SPEC>),

    BindingArg(FieldComposer<SPEC>),
    BindingFieldName(FieldComposer<SPEC>),
    CallbackArg(FieldComposer<SPEC>),
    DefaultFieldConversion(FieldComposer<SPEC>),
    DefaultFieldByValueConversion(FieldComposer<SPEC>, SPEC::Expr),
    Unnamed(FieldComposer<SPEC>),
    Named(FieldComposer<SPEC>, Visibility),
    NamedReady(FieldComposer<SPEC>, Visibility),
}

impl<SPEC> ArgKind<SPEC>
    where SPEC: Specification {
    pub fn binding_arg(composer: &FieldComposer<SPEC>) -> Self {
        Self::BindingArg(composer.clone())
    }
    pub fn binding_field_name(composer: &FieldComposer<SPEC>) -> Self {
        Self::BindingFieldName(composer.clone())
    }
    pub fn callback_arg(composer: &FieldComposer<SPEC>) -> Self {
        Self::CallbackArg(composer.clone())
    }
    pub fn default_field_conversion(composer: &FieldComposer<SPEC>) -> Self {
        Self::DefaultFieldConversion(composer.clone())
    }
    pub fn default_field_type(composer: &FieldComposer<SPEC>) -> Self {
        Self::Unnamed(composer.clone())
    }
    pub fn public_named(composer: &FieldComposer<SPEC>) -> Self {
        Self::Named(composer.clone(), Visibility::Public(Default::default()))
    }
    pub fn public_named_ready(composer: &FieldComposer<SPEC>) -> Self {
        Self::NamedReady(composer.clone(), Visibility::Public(Default::default()))
    }
    pub fn inherited_named(composer: FieldComposer<SPEC>) -> Self {
        Self::Named(composer, Visibility::Inherited)
    }
    pub fn inherited_named_var(name: SPEC::Name, var: SPEC::Var, attrs: SPEC::Attr) -> Self {
        Self::inherited_named(FieldComposer::named_var(name, var, attrs))
    }
    pub fn inherited_named_type(name: SPEC::Name, ty: &Type, attrs: SPEC::Attr) -> Self {
        Self::inherited_named(FieldComposer::named_type(name, ty, attrs))
    }
    pub fn inherited_named_by_ref(composer: &FieldComposer<SPEC>) -> Self {
        Self::Named(composer.clone(), Visibility::Inherited)
    }
    pub fn inherited_named_ready(composer: &FieldComposer<SPEC>) -> Self {
        Self::NamedReady(composer.clone(), Visibility::Inherited)
    }

    pub fn attr_name(composer: &FieldComposer<SPEC>) -> Self {
        Self::AttrName(composer.tokenized_name(), composer.attrs.clone())
    }
    pub fn expr(expr: SPEC::Expr) -> Self {
        Self::AttrExpression(expr, SPEC::Attr::default())
    }
    pub fn attr_expr_composer(composer: &FieldComposer<SPEC>, field_path_resolver: FieldPathResolver<SPEC>, expr_composer: PresentableExprComposerRef<SPEC>) -> Self {
        Self::AttrExpressionComposer(composer.clone(), field_path_resolver, expr_composer)
}
    pub fn callback_ctor_pair(composer: &FieldComposer<SPEC>) -> (Self, Self) {
        (Self::CallbackArg(composer.clone()), Self::binding_field_name(composer))
    }
    pub fn unnamed_struct_ctor_pair(composer: &FieldComposer<SPEC>) -> (Self, Self) {
        (Self::binding_arg(composer), Self::binding_field_name(composer))
    }
    pub fn named_struct_ctor_pair(composer: &FieldComposer<SPEC>) -> (Self, Self) {
        (Self::inherited_named_by_ref(composer), Self::attr_name(composer))
    }
    pub fn named_ready_struct_ctor_pair(composer: &FieldComposer<SPEC>) -> (Self, Self) {
        (Self::inherited_named_ready(composer), Self::attr_name(composer))
    }
    pub fn opaque_named_struct_ctor_pair(composer: &FieldComposer<SPEC>) -> (Self, Self) {
        (Self::inherited_named_by_ref(composer), Self::default_field_conversion(composer))
    }
}

