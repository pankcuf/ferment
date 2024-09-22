use quote::{quote, ToTokens};
use syn::{Expr, ExprPath, Pat, PatWild, Type, Visibility, VisPublic};
use crate::composable::{FieldComposer, FieldTypeKind};
use crate::composer::{Composer, FromConversionFullComposer, VariableComposer};
use crate::context::{ScopeContext, ScopeSearch, ScopeSearchKey};
use crate::ext::{Mangle, Resolve, ToPath, ToType};
use crate::lang::{RustSpecification, Specification};
use crate::presentable::{ScopeContextPresentable, PresentableSequence, Aspect, Expression};
use crate::presentation::{ArgPresentation, FFIVariable, RustFermentate};


#[derive(Clone)]
pub enum PresentableArgument<LANG, SPEC>
    where LANG: Clone,
          SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>>,
          Aspect<SPEC::TYC>: ScopeContextPresentable {
    BindingArg(FieldComposer<LANG, SPEC>),
    BindingFieldName(FieldComposer<LANG, SPEC>),
    Named(FieldComposer<LANG, SPEC>, Visibility),
    CallbackArg(FieldComposer<LANG, SPEC>),
    DefaultFieldConversion(FieldComposer<LANG, SPEC>),
    DefaultFieldType(FieldComposer<LANG, SPEC>),

    AttrExhaustive(SPEC::Attr),
    AttrSequence(PresentableSequence<LANG, SPEC>, SPEC::Attr),
    AttrExpression(SPEC::Expr, SPEC::Attr),
}
// impl<LANG, SPEC> std::fmt::Display for ArgumentPresentableContext<LANG, SPEC>
//     where LANG: Clone + Debug,
//           SPEC: Specification<LANG> + Debug,
//           <SPEC as Specification<LANG>>::Attr: Debug {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         Debug::fmt(self, f)
//     }
// }

impl<LANG, SPEC> PresentableArgument<LANG, SPEC>
    where LANG: Clone,
          SPEC: Specification<LANG, Expr=Expression<LANG, SPEC>>,
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
        Self::DefaultFieldType(composer.clone())
    }
    pub fn public_named(composer: &FieldComposer<LANG, SPEC>) -> Self {
        Self::Named(composer.clone(), Visibility::Public(VisPublic { pub_token: Default::default() }))
    }
    pub fn attr_expr(composer: &FieldComposer<LANG, SPEC>) -> Self {
        Self::AttrExpression(SPEC::Expr::expr(Expr::Path(ExprPath { attrs: vec![], qself: None, path: composer.tokenized_name().to_path() })), composer.attrs.clone())
    }
    pub fn callback_ctor_pair(composer: &FieldComposer<LANG, SPEC>) -> (Self, Self) {
        (Self::CallbackArg(composer.clone()),
         Self::binding_field_name(composer))
    }
    pub fn unnamed_struct_ctor_pair(composer: &FieldComposer<LANG, SPEC>) -> (Self, Self) {
        (Self::BindingArg(composer.clone()),
         Self::binding_field_name(composer))
    }
    pub fn named_struct_ctor_pair(composer: &FieldComposer<LANG, SPEC>) -> (Self, Self) {
        (Self::Named(composer.clone(), Visibility::Inherited),
         Self::attr_expr(composer))
    }
    pub fn opaque_named_struct_ctor_pair(composer: &FieldComposer<LANG, SPEC>) -> (Self, Self) {
        (Self::Named(composer.clone(), Visibility::Inherited),
         Self::DefaultFieldConversion(composer.clone()))
    }
}

impl<SPEC> ScopeContextPresentable for PresentableArgument<RustFermentate, SPEC>
    where SPEC: RustSpecification {
    type Presentation = ArgPresentation;

    fn present(&self, source: &ScopeContext) -> Self::Presentation {
        match self {
            PresentableArgument::AttrExpression(field_type_context, attrs) =>
                ArgPresentation::expr(attrs, field_type_context.present(source).to_token_stream()),
            PresentableArgument::AttrSequence(seq, attrs) =>
                ArgPresentation::expr(attrs, seq.present(source)),
            PresentableArgument::DefaultFieldType(composer) =>
                ArgPresentation::expr(&composer.attrs, <Type as Resolve<FFIVariable>>::resolve(composer.ty(), source).to_token_stream()),
            PresentableArgument::BindingFieldName(FieldComposer { name, named, attrs, .. }) =>
                ArgPresentation::expr(attrs, named.then(|| name.to_token_stream()).unwrap_or(name.anonymous().to_token_stream())),
            PresentableArgument::DefaultFieldConversion(FieldComposer { name, kind, attrs, .. }) =>
                ArgPresentation::field(attrs, Visibility::Inherited, Some(name.mangle_ident_default()), Type::Verbatim(FromConversionFullComposer::<RustFermentate, SPEC>::new(name.clone(), ScopeSearch::KeyInScope(ScopeSearchKey::maybe_from_ref(kind.ty()).unwrap(), &source.scope), None).compose(source).present(source).to_token_stream())),
            PresentableArgument::BindingArg(FieldComposer { name, kind, named, attrs, .. }) => {
                let (ident, ty) = match kind {
                    FieldTypeKind::Type(field_type) => (
                        Some((*named).then(|| name.mangle_ident_default()).unwrap_or(name.anonymous())),
                        <Type as Resolve<FFIVariable>>::resolve(field_type, source).to_type()
                    ),
                    FieldTypeKind::Conversion(conversion) => (
                        Some(name.mangle_ident_default()), Type::Verbatim(conversion.clone()))
                };
                ArgPresentation::field(attrs, Visibility::Inherited, ident, ty)
            },
            PresentableArgument::Named(FieldComposer { attrs, name, kind, ..}, visibility) =>
                ArgPresentation::field(attrs, visibility.clone(), Some(name.mangle_ident_default()), VariableComposer::from(kind.ty()).compose(source).to_type()),
            PresentableArgument::CallbackArg(FieldComposer { attrs, name, kind, .. }) =>
                ArgPresentation::field(attrs, Visibility::Inherited, Some(name.mangle_ident_default()), kind.ty().clone()),

            // PresentableArgument::Lambda(name, value, attrs) =>
            //     ArgPresentation::arm(attrs, Pat::Verbatim(name.clone()), value.clone()),
            PresentableArgument::AttrExhaustive(attrs) =>
                ArgPresentation::arm(attrs, Pat::Wild(PatWild { attrs: vec![], underscore_token: Default::default() }), quote!(unreachable!("This is unreachable"))),
        }
    }
}
