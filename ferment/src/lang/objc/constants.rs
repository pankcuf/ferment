use syn::Field;
use syn::punctuated::Punctuated;
use crate::composable::{CfgAttributes, FieldComposer, FieldTypeKind};
use crate::composer::{CommaPunctuatedFields, field_composers_iterator, ComposerPresenterByRef, FieldComposers, FieldsComposerRef};
use crate::lang::objc::composers::AttrWrapper;
use crate::lang::objc::ObjCFermentate;
use crate::presentation::Name;

pub const OBJC_EMPTY_FIELDS_COMPOSER: FieldsComposerRef<ObjCFermentate, AttrWrapper> = |_| Punctuated::new();

pub const OBJC_STRUCT_UNNAMED_FIELDS_COMPOSER: FieldsComposerRef<ObjCFermentate, AttrWrapper> = |fields|
    field_composers_iterator(
        fields,
        |index, Field { ty, attrs, .. }|
            FieldComposer::new(
                Name::UnnamedStructFieldsComp(ty.clone(), index),
                FieldTypeKind::r#type(ty),
                false,
                AttrWrapper::from(attrs.cfg_attributes())
            ));

pub const OBJC_STRUCT_NAMED_FIELDS_COMPOSER: ComposerPresenterByRef<
        CommaPunctuatedFields,
        FieldComposers<ObjCFermentate, AttrWrapper>> = |fields|
    field_composers_iterator(fields, |_index, Field { ident, ty, attrs, .. }|
        FieldComposer::new(
                Name::Optional(ident.clone()),
                FieldTypeKind::r#type(ty),
                true,
                AttrWrapper::from(attrs.cfg_attributes()),
        ));

pub const OBJC_ENUM_VARIANT_UNNAMED_FIELDS_COMPOSER: FieldsComposerRef<ObjCFermentate, AttrWrapper> = |fields|
    field_composers_iterator(fields, |index, Field { ty, attrs, .. }| FieldComposer::new(
            Name::UnnamedArg(index),
            FieldTypeKind::r#type(ty),
            false,
            AttrWrapper::from(attrs.cfg_attributes())));
