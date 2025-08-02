use quote::ToTokens;
use syn::parse_quote;
use syn::token::Comma;
use crate::ast::{BraceWrapped, CommaPunctuated};
use crate::composable::FieldComposer;
use crate::composer::{AspectPresentable, AttrComposable, SourceAccessible, SourceFermentable, TraitComposer};
use crate::ext::{Mangle, ToPath};
use crate::kind::FieldTypeKind;
use crate::lang::{FromDictionary, RustSpecification};
use crate::presentable::ScopeContextPresentable;
use crate::presentation::{DictionaryName, DocPresentation, FFIObjectPresentation, Name, RustFermentate};

impl SourceFermentable<RustFermentate> for TraitComposer<RustSpecification> {
    fn ferment(&self) -> RustFermentate {
        // TODO: source.scope or local_scope?
        let attrs = self.compose_attributes();
        let ffi_type = self.present_ffi_aspect();
        let mangled_ty = ffi_type.mangle_ident_default();
        let vtable_name = Name::<RustSpecification>::Vtable(mangled_ty.clone());
        RustFermentate::Trait {
            comment: DocPresentation::Empty,
            vtable: FFIObjectPresentation::TraitVTable {
                attrs: attrs.clone(),
                name: vtable_name.to_path(),
                fields: BraceWrapped::<_, Comma>::new(
                    CommaPunctuated::from_iter(
                        self.methods.iter()
                            .map(|composer| composer.borrow().ferment())))
                    .to_token_stream()
            },
            trait_object: FFIObjectPresentation::TraitObject {
                attrs,
                name: Name::<RustSpecification>::TraitObj(mangled_ty).to_path(),
                fields: BraceWrapped::new(
                    CommaPunctuated::from_iter([
                        FieldComposer::<RustSpecification>::named_no_attrs(
                            Name::dictionary_name(DictionaryName::Object),
                            FieldTypeKind::Type(parse_quote!(*const ()))),
                        FieldComposer::<RustSpecification>::named_no_attrs(
                            Name::dictionary_name(DictionaryName::Vtable),
                            FieldTypeKind::Type(parse_quote!(*const #vtable_name))),
                    ])).present(&self.context().borrow())
                }
            }
    }
}

