use std::fmt::Debug;
use quote::{quote, ToTokens};
use syn::{Pat, PatWild, Type, Visibility, VisPublic};
use syn::__private::TokenStream2;
use ferment_macro::Display;
use crate::composable::{FieldComposer, FieldTypeKind};
use crate::composer::{SourceComposable, FromConversionFullComposer, VariableComposer, FieldPathResolver, PresentableExprComposerRef};
use crate::context::ScopeContext;
use crate::ext::{Mangle, Resolve, ToType};
use crate::lang::{LangFermentable, RustSpecification, Specification};
use crate::presentable::{ScopeContextPresentable, SeqKind, Aspect, Expression};
use crate::presentation::{ArgPresentation, RustFermentate};


#[derive(Clone, Debug, Display)]
pub enum ArgKind<LANG, SPEC>
    where LANG: LangFermentable,
          SPEC: Specification<LANG, Attr: Debug, Expr=Expression<LANG, SPEC>, Var: ToType>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable {
    AttrExhaustive(SPEC::Attr),
    AttrSequence(SeqKind<LANG, SPEC>, SPEC::Attr),
    AttrName(TokenStream2, SPEC::Attr),
    AttrExpression(SPEC::Expr, SPEC::Attr),
    AttrExpressionComposer(FieldComposer<LANG, SPEC>, FieldPathResolver<LANG, SPEC>, PresentableExprComposerRef<LANG, SPEC>),

    BindingArg(FieldComposer<LANG, SPEC>),
    BindingFieldName(FieldComposer<LANG, SPEC>),
    CallbackArg(FieldComposer<LANG, SPEC>),
    DefaultFieldConversion(FieldComposer<LANG, SPEC>),
    Unnamed(FieldComposer<LANG, SPEC>),
    Named(FieldComposer<LANG, SPEC>, Visibility),

}
// impl<LANG, SPEC> std::fmt::Display for ArgumentPresentableContext<LANG, SPEC>
//     where LANG: LangFermentable + Debug,
//           SPEC: Specification<LANG> + Debug,
//           <SPEC as Specification<LANG>>::Attr: Debug {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         Debug::fmt(self, f)
//     }
// }

impl<LANG, SPEC> ArgKind<LANG, SPEC>
    where LANG: LangFermentable,
          SPEC: Specification<LANG, Attr: Debug, Expr=Expression<LANG, SPEC>, Var: ToType>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable {
    pub fn binding_arg(composer: &FieldComposer<LANG, SPEC>) -> Self {
        Self::BindingArg(composer.clone())
    }
    pub fn binding_field_name(composer: &FieldComposer<LANG, SPEC>) -> Self {
        Self::BindingFieldName(composer.clone())
    }
    pub fn callback_arg(composer: &FieldComposer<LANG, SPEC>) -> Self {
        Self::CallbackArg(composer.clone())
    }
    pub fn default_field_conversion(composer: &FieldComposer<LANG, SPEC>) -> Self {
        Self::DefaultFieldConversion(composer.clone())
    }
    pub fn default_field_type(composer: &FieldComposer<LANG, SPEC>) -> Self {
        Self::Unnamed(composer.clone())
    }
    pub fn public_named(composer: &FieldComposer<LANG, SPEC>) -> Self {
        Self::Named(composer.clone(), Visibility::Public(VisPublic { pub_token: Default::default() }))
    }
    // pub fn attr_expr(composer: &FieldComposer<LANG, SPEC>) -> Self {
    //     Self::AttrExpression(SPEC::Expr::expr(Expr::Path(ExprPath { attrs: vec![], qself: None, path: composer.tokenized_name().to_path() })), composer.attrs.clone())
    // }
    pub fn attr_name(composer: &FieldComposer<LANG, SPEC>) -> Self {
        Self::AttrName(composer.tokenized_name(), composer.attrs.clone())
    }
    pub fn attr_expr_composer(composer: &FieldComposer<LANG, SPEC>, field_path_resolver: FieldPathResolver<LANG, SPEC>, expr_composer: PresentableExprComposerRef<LANG, SPEC>) -> Self {
        Self::AttrExpressionComposer(composer.clone(), field_path_resolver, expr_composer)
}
    pub fn callback_ctor_pair(composer: &FieldComposer<LANG, SPEC>) -> (Self, Self) {
        (Self::CallbackArg(composer.clone()), Self::binding_field_name(composer))
    }
    pub fn unnamed_struct_ctor_pair(composer: &FieldComposer<LANG, SPEC>) -> (Self, Self) {
        (Self::binding_arg(composer), Self::binding_field_name(composer))
    }
    pub fn named_struct_ctor_pair(composer: &FieldComposer<LANG, SPEC>) -> (Self, Self) {
        (Self::Named(composer.clone(), Visibility::Inherited), Self::attr_name(composer))
    }
    pub fn opaque_named_struct_ctor_pair(composer: &FieldComposer<LANG, SPEC>) -> (Self, Self) {
        (Self::Named(composer.clone(), Visibility::Inherited), Self::default_field_conversion(composer))
    }
}

impl<SPEC> ScopeContextPresentable for ArgKind<RustFermentate, SPEC>
    where SPEC: RustSpecification {
    type Presentation = ArgPresentation;

    fn present(&self, source: &ScopeContext) -> Self::Presentation {
        match self {
            ArgKind::AttrExhaustive(attrs) =>
                ArgPresentation::arm(attrs, Pat::Wild(PatWild { attrs: vec![], underscore_token: Default::default() }), quote!(unreachable!("This is unreachable"))),
            ArgKind::AttrExpression(expr, attrs) =>
                ArgPresentation::expr(attrs, expr.present(source)),
            ArgKind::AttrExpressionComposer(field_composer, field_path_resolver, expr_composer) => {
                let template = field_path_resolver(field_composer);
                let expr = expr_composer(&template);
                ArgPresentation::expr(&field_composer.attrs.clone(), expr.present(source))
            },
            ArgKind::AttrName(name, attrs) =>
                ArgPresentation::expr(attrs, name.to_token_stream()),
            ArgKind::AttrSequence(seq, attrs) =>
                ArgPresentation::expr(attrs, seq.present(source)),
            ArgKind::BindingArg(FieldComposer { name, kind, named, attrs, .. }) => {
                let (ident, ty) = match kind {
                    FieldTypeKind::Type(field_type) => (
                        Some((*named).then(|| name.mangle_ident_default()).unwrap_or(name.anonymous())),
                        <Type as Resolve<SPEC::Var>>::resolve(field_type, source).to_type()
                    ),
                    FieldTypeKind::Var(field_type) => (
                        Some((*named).then(|| name.mangle_ident_default()).unwrap_or(name.anonymous())),
                        field_type.to_type()
                        // <Type as Resolve<FFIVariable<Type>>>::resolve(field_type, source).to_type()
                    ),
                    FieldTypeKind::Conversion(conversion) => (
                        Some(name.mangle_ident_default()), Type::Verbatim(conversion.clone()))
                };
                ArgPresentation::field(attrs, Visibility::Inherited, ident, ty)
            },
            ArgKind::BindingFieldName(FieldComposer { name, named, attrs, .. }) =>
                ArgPresentation::expr(
                    attrs,
                    named.then(|| name.to_token_stream())
                        .unwrap_or(name.anonymous().to_token_stream())),
            ArgKind::CallbackArg(FieldComposer { attrs, name, kind, .. }) =>
                ArgPresentation::field(
                    attrs,
                    Visibility::Inherited,
                    Some(name.mangle_ident_default()),
                    kind.ty().clone()),
            ArgKind::DefaultFieldConversion(FieldComposer { name, kind, attrs, .. }) =>
                ArgPresentation::field(
                    attrs,
                    Visibility::Inherited,
                    Some(name.mangle_ident_default()),
                    Type::Verbatim(
                        FromConversionFullComposer::<RustFermentate, SPEC>::key_in_scope(name.clone(), kind.ty(), &source.scope)
                            .compose(source)
                            .present(source))),
            ArgKind::Unnamed(composer) =>
                ArgPresentation::expr(
                    &composer.attrs,
                    Resolve::<SPEC::Var>::resolve(composer.ty(), source)
                        .to_token_stream()),
            ArgKind::Named(FieldComposer { attrs, name, kind, ..}, visibility) =>
                ArgPresentation::field(
                    attrs,
                    visibility.clone(),
                    Some(name.mangle_ident_default()),
                    VariableComposer::<RustFermentate, SPEC>::from(kind.ty())
                        .compose(source)
                        .to_type()),
        }
    }
}
