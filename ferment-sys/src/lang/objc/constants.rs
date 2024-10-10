use std::fmt::Debug;
use quote::ToTokens;
use syn::Field;
use syn::punctuated::Punctuated;
use crate::composable::FieldComposer;
use crate::composer::{field_composers_iterator, FieldsComposerRef};
use crate::ext::ToType;
use crate::lang::{LangFermentable, Specification};
use crate::presentable::{Aspect, Expression, ScopeContextPresentable};
use crate::presentation::Name;


pub const fn objc_empty_fields_composer<LANG, SPEC>()
    -> FieldsComposerRef<LANG, SPEC>
    where LANG: LangFermentable,
          SPEC: Specification<LANG, Attr: Debug, Expr=Expression<LANG, SPEC>, Var: ToType>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable {
    |_| Punctuated::new()
}
pub const fn objc_struct_unnamed_fields_composer<LANG, SPEC>()
    -> FieldsComposerRef<LANG, SPEC>
    where LANG: LangFermentable,
          SPEC: Specification<LANG, Attr: Debug, Expr=Expression<LANG, SPEC>, Name=Name<LANG, SPEC>, Var: ToType>,
          SPEC::Expr: ScopeContextPresentable,
          Name<LANG, SPEC>: ToTokens,
          Aspect<SPEC::TYC>: ScopeContextPresentable {
    |fields|
        field_composers_iterator(
            fields,
            |index, Field { ty, attrs, .. }|
                FieldComposer::typed(Name::UnnamedStructFieldsComp(ty.clone(), index), ty, false, attrs))
}
pub const fn objc_struct_named_fields_composer<LANG, SPEC>()
    -> FieldsComposerRef<LANG, SPEC>
    where LANG: LangFermentable,
          SPEC: Specification<LANG, Attr: Debug, Expr=Expression<LANG, SPEC>, Name=Name<LANG, SPEC>, Var: ToType>,
          SPEC::Expr: ScopeContextPresentable,
          Name<LANG, SPEC>: ToTokens,
          Aspect<SPEC::TYC>: ScopeContextPresentable {
    |fields|
        field_composers_iterator(fields, |_index, Field { ident, ty, attrs, .. }|
            FieldComposer::typed(Name::Optional(ident.clone()), ty, true, attrs))
}
pub const fn objc_enum_variant_unnamed_fields_composer<LANG, SPEC>()
    -> FieldsComposerRef<LANG, SPEC>
    where LANG: LangFermentable,
          SPEC: Specification<LANG, Attr: Debug, Expr=Expression<LANG, SPEC>, Name=Name<LANG, SPEC>, Var: ToType>,
          SPEC::Expr: ScopeContextPresentable,
          Name<LANG, SPEC>: ToTokens,
          Aspect<SPEC::TYC>: ScopeContextPresentable {
    |fields|
        field_composers_iterator(fields, |index, Field { ty, attrs, .. }|
            FieldComposer::typed(Name::UnnamedArg(index), ty, false, attrs))
}

// pub const OBJC_EMPTY_FIELDS_COMPOSER: FieldsComposerRef<ObjCFermentate, AttrWrapper> = |_| Punctuated::new();

// pub const OBJC_STRUCT_UNNAMED_FIELDS_COMPOSER: FieldsComposerRef<ObjCFermentate, AttrWrapper> = |fields|
//     field_composers_iterator(
//         fields,
//         |index, Field { ty, attrs, .. }|
//             FieldComposer::new(
//                 Name::UnnamedStructFieldsComp(ty.clone(), index),
//                 FieldTypeKind::r#type(ty),
//                 false,
//                 AttrWrapper::from(attrs.cfg_attributes())
//             ));

// pub const OBJC_STRUCT_NAMED_FIELDS_COMPOSER: ComposerPresenterByRef<
//         CommaPunctuatedFields,
//         FieldComposers<ObjCFermentate, AttrWrapper>> = |fields|
//     field_composers_iterator(fields, |_index, Field { ident, ty, attrs, .. }|
//         FieldComposer::new(
//                 Name::Optional(ident.clone()),
//                 FieldTypeKind::r#type(ty),
//                 true,
//                 AttrWrapper::from(attrs.cfg_attributes()),
//         ));
//
// pub const OBJC_ENUM_VARIANT_UNNAMED_FIELDS_COMPOSER: FieldsComposerRef<ObjCFermentate, AttrWrapper> = |fields|
//     field_composers_iterator(fields, |index, Field { ty, attrs, .. }| FieldComposer::new(
//             Name::UnnamedArg(index),
//             FieldTypeKind::r#type(ty),
//             false,
//             AttrWrapper::from(attrs.cfg_attributes())));
