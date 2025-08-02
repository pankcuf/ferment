use quote::quote;
use syn::{Pat, PatWild, Type};
use crate::composable::FieldComposer;
use crate::composer::{ConversionFromComposer, SourceComposable, VarComposer};
use crate::context::ScopeContext;
use crate::ext::{Mangle, Resolve, ToType};
use crate::kind::FieldTypeKind;
use crate::lang::{RustSpecification, Specification};
use crate::presentable::{ArgKind, ScopeContextPresentable};
use crate::presentation::ArgPresentation;

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
                ArgPresentation::inherited_field(attrs, name.mangle_ident_default(), Type::Verbatim(ConversionFromComposer::<RustSpecification>::key_in_composer_scope(name.clone(), &kind.to_type()).compose(source).present(source))),
            ArgKind::DefaultFieldByValueConversion(FieldComposer { name, attrs, .. }, expr) =>
                ArgPresentation::inherited_field(attrs, name.mangle_ident_default(), Type::Verbatim(expr.present(source))),
            ArgKind::Unnamed(FieldComposer { attrs, kind: FieldTypeKind::Type(ty), .. }) =>
                ArgPresentation::attr_tokens(attrs, Resolve::<<RustSpecification as Specification>::Var>::resolve(ty, source)),
            ArgKind::Unnamed(FieldComposer { attrs, kind: FieldTypeKind::Var(var), .. }) =>
                ArgPresentation::attr_tokens(attrs, var),
            ArgKind::Unnamed(FieldComposer { attrs, kind: FieldTypeKind::Conversion(conversion), .. }) =>
                ArgPresentation::attr_tokens(attrs, conversion),
            ArgKind::Named(FieldComposer { attrs, name, kind, ..}, visibility) =>
                ArgPresentation::field(attrs, visibility.clone(), Some(name.mangle_ident_default()), VarComposer::<RustSpecification>::key_ref_in_composer_scope(&kind.to_type()).compose(source).to_type()),
            ArgKind::NamedReady(FieldComposer { attrs, name, kind, ..}, visibility) =>
                ArgPresentation::field(attrs, visibility.clone(), Some(name.mangle_ident_default()), kind.to_type()),
        }
    }
}
