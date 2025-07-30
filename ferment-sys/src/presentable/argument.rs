use std::fmt::Debug;
use quote::quote;
use syn::{Pat, PatWild, Type, Visibility};
use syn::__private::TokenStream2;
use ferment_macro::Display;
use crate::composable::{FieldComposer, FieldTypeKind};
use crate::composer::{SourceComposable, ConversionFromComposer, VariableComposer, FieldPathResolver, PresentableExprComposerRef};
use crate::context::ScopeContext;
use crate::ext::{Mangle, Resolve, ToType};
use crate::lang::{RustSpecification, Specification};
use crate::presentable::{ScopeContextPresentable, SeqKind};
use crate::presentation::ArgPresentation;


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
    pub fn inherited_named(composer: &FieldComposer<SPEC>) -> Self {
        Self::Named(composer.clone(), Visibility::Inherited)
    }

    pub fn attr_name(composer: &FieldComposer<SPEC>) -> Self {
        Self::AttrName(composer.tokenized_name(), composer.attrs.clone())
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
        (Self::inherited_named(composer), Self::attr_name(composer))
    }
    pub fn opaque_named_struct_ctor_pair(composer: &FieldComposer<SPEC>) -> (Self, Self) {
        (Self::inherited_named(composer), Self::default_field_conversion(composer))
    }
}

impl ScopeContextPresentable for ArgKind<RustSpecification> {
    type Presentation = ArgPresentation;

    fn present(&self, source: &ScopeContext) -> Self::Presentation {
        match self {
            ArgKind::AttrExhaustive(attrs) =>
                ArgPresentation::arm(attrs, Pat::Wild(PatWild { attrs: vec![], underscore_token: Default::default() }), quote!(unreachable!("This is unreachable"))),
            ArgKind::AttrExpression(expr, attrs) =>
                ArgPresentation::attr_tokens(attrs, expr.present(source)),
            ArgKind::AttrExpressionComposer(field_composer, field_path_resolver, expr_composer) => {
                let template = field_path_resolver(field_composer);
                ArgPresentation::attr_tokens(&field_composer.attrs, expr_composer(&template).present(source))
            },
            ArgKind::AttrName(name, attrs) =>
                ArgPresentation::attr_tokens(attrs, name),
            ArgKind::AttrSequence(seq, attrs) =>
                ArgPresentation::attr_tokens(attrs, seq.present(source)),
            ArgKind::BindingArg(FieldComposer { name, kind: FieldTypeKind::Type(ty), named: true, attrs, .. }) =>
                ArgPresentation::inherited_field(attrs, name.mangle_ident_default(), Resolve::<<RustSpecification as Specification>::Var>::resolve(ty, source).to_type()),
            ArgKind::BindingArg(FieldComposer { name, kind: FieldTypeKind::Type(ty), named: false, attrs, .. }) =>
                ArgPresentation::inherited_field(attrs, name.anonymous(), Resolve::<<RustSpecification as Specification>::Var>::resolve(ty, source).to_type()),
            ArgKind::BindingArg(FieldComposer { name, kind: FieldTypeKind::Var(var), named: true, attrs, .. }) =>
                ArgPresentation::inherited_field(attrs, name.mangle_ident_default(), var.to_type()),
            ArgKind::BindingArg(FieldComposer { name, kind: FieldTypeKind::Var(var), named: false, attrs, .. }) =>
                ArgPresentation::inherited_field(attrs, name.anonymous(), var.to_type()),
            ArgKind::BindingArg(FieldComposer { name, kind: FieldTypeKind::Conversion(conversion), attrs, .. }) =>
                ArgPresentation::inherited_field(attrs, name.mangle_ident_default(), Type::Verbatim(conversion.clone())),
            ArgKind::BindingFieldName(FieldComposer { name, named: true, attrs, .. }) =>
                ArgPresentation::attr_tokens(attrs, name),
            ArgKind::BindingFieldName(FieldComposer { name, named: false, attrs, .. }) =>
                ArgPresentation::attr_tokens(attrs, name.anonymous()),
            ArgKind::CallbackArg(FieldComposer { attrs, name, kind, .. }) =>
                ArgPresentation::inherited_field(attrs, name.mangle_ident_default(), kind.to_type()),
            ArgKind::DefaultFieldConversion(FieldComposer { name, kind, attrs, .. }) =>
                ArgPresentation::inherited_field(attrs, name.mangle_ident_default(), Type::Verbatim(ConversionFromComposer::<RustSpecification>::key_in_scope(name.clone(), &kind.to_type(), &source.scope).compose(source).present(source))),
            ArgKind::DefaultFieldByValueConversion(FieldComposer { name, attrs, .. }, expr) =>
                ArgPresentation::inherited_field(attrs, name.mangle_ident_default(), Type::Verbatim(expr.present(source))),
            ArgKind::Unnamed(FieldComposer { attrs, kind: FieldTypeKind::Type(ty), .. }) =>
                ArgPresentation::attr_tokens(attrs, Resolve::<<RustSpecification as Specification>::Var>::resolve(ty, source)),
            ArgKind::Unnamed(FieldComposer { attrs, kind: FieldTypeKind::Var(var), .. }) =>
                ArgPresentation::attr_tokens(attrs, var),
            ArgKind::Unnamed(FieldComposer { attrs, kind: FieldTypeKind::Conversion(conversion), .. }) =>
                ArgPresentation::attr_tokens(attrs, conversion),
            ArgKind::Named(FieldComposer { attrs, name, kind, ..}, visibility) =>
                ArgPresentation::field(attrs, visibility.clone(), Some(name.mangle_ident_default()), VariableComposer::<RustSpecification>::from(&kind.to_type()).compose(source).to_type()),
        }
    }
}
