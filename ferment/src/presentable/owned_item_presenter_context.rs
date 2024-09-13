use std::fmt::Debug;
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};
use syn::{Attribute, Pat, PatWild, Type, Visibility};
use crate::composable::{FieldComposer, FieldTypeKind};
use crate::composer::{Composer, FromConversionFullComposer, VariableComposer};
use crate::context::{ScopeContext, ScopeSearch, ScopeSearchKey};
use crate::ext::{Mangle, Resolve, ToType};
use crate::lang::LangAttrSpecification;
use crate::presentable::{Expression, ScopeContextPresentable, SequenceOutput};
use crate::presentation::{ArgPresentation, FFIVariable, RustFermentate};


#[derive(Clone, Debug)]
pub enum  OwnedItemPresentableContext<LANG, SPEC>
    where LANG: Clone,
          SPEC: LangAttrSpecification<LANG> {
    BindingArg(FieldComposer<LANG, SPEC>),
    BindingFieldName(FieldComposer<LANG, SPEC>),
    Named(FieldComposer<LANG, SPEC>, Visibility),
    CallbackArg(FieldComposer<LANG, SPEC>),
    DefaultFieldConversion(FieldComposer<LANG, SPEC>),

    DefaultFieldType(Type, SPEC),
    Lambda(TokenStream2, TokenStream2, SPEC),
    Exhaustive(SPEC),
    SequenceOutput(SequenceOutput<LANG, SPEC>, SPEC),
    Expression(Expression<LANG, SPEC>, SPEC),
}
impl<LANG, SPEC> std::fmt::Display for OwnedItemPresentableContext<LANG, SPEC>
    where LANG: Clone + Debug,
          SPEC: LangAttrSpecification<LANG> + Debug {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}


impl ScopeContextPresentable for OwnedItemPresentableContext<RustFermentate, Vec<Attribute>> {
    type Presentation = ArgPresentation;

    fn present(&self, source: &ScopeContext) -> Self::Presentation {
        match self {
            OwnedItemPresentableContext::Expression(field_type_context, attrs) =>
                ArgPresentation::expr(attrs, field_type_context.present(source)),
            OwnedItemPresentableContext::SequenceOutput(seq, attrs) =>
                ArgPresentation::expr(attrs, seq.present(source)),
            OwnedItemPresentableContext::DefaultFieldType(field_type, attrs) =>
                ArgPresentation::expr(attrs, <Type as Resolve<FFIVariable>>::resolve(field_type, source).to_token_stream()),
            OwnedItemPresentableContext::BindingFieldName(FieldComposer { name, named, attrs, .. }) =>
                ArgPresentation::expr(attrs, named.then(|| name.to_token_stream()).unwrap_or(name.anonymous().to_token_stream())),

            OwnedItemPresentableContext::DefaultFieldConversion(FieldComposer { name, kind, attrs, .. }) =>
                ArgPresentation::field(attrs, Visibility::Inherited, Some(name.mangle_ident_default()), Type::Verbatim(FromConversionFullComposer::<RustFermentate, Vec<Attribute>>::new(name.clone(), ScopeSearch::KeyInScope(ScopeSearchKey::maybe_from_ref(kind.ty()).unwrap(), &source.scope), None).compose(source).present(source))),
            OwnedItemPresentableContext::BindingArg(FieldComposer { name, kind, named, attrs, .. }) => {
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
            OwnedItemPresentableContext::Named(FieldComposer { attrs, name, kind, ..}, visibility) =>
                ArgPresentation::field(attrs, visibility.clone(), Some(name.mangle_ident_default()), VariableComposer::from(kind.ty()).compose(source).to_type()),
            OwnedItemPresentableContext::CallbackArg(FieldComposer { attrs, name, kind, .. }) =>
                ArgPresentation::field(attrs, Visibility::Inherited, Some(name.mangle_ident_default()), kind.ty().clone()),

            OwnedItemPresentableContext::Lambda(name, value, attrs) =>
                ArgPresentation::arm(attrs, Pat::Verbatim(name.clone()), value.clone()),
            OwnedItemPresentableContext::Exhaustive(attrs) =>
                ArgPresentation::arm(attrs, Pat::Wild(PatWild { attrs: vec![], underscore_token: Default::default() }), quote!(unreachable!("This is unreachable"))),
        }
    }
}
