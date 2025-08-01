use syn::{Type, TypeInfer};
use crate::composer::{SourceComposable, TargetVarComposer};
use crate::context::ScopeContext;
use crate::kind::{ObjectKind, TypeModelKind};
use crate::ext::ToType;
use crate::lang::{RustSpecification, Specification};
use crate::presentation::FFIVariable;

impl<'a> SourceComposable for TargetVarComposer<'a, RustSpecification> {
    type Source = ScopeContext;
    type Output = <RustSpecification as Specification>::Var;

    fn compose(&self, source: &Self::Source) -> Self::Output {
        let search_key = self.search.search_key();
        let maybe_obj = source.maybe_object_by_predicate_ref(&self.search);
        //println!("TargetVar: {}", search_key.to_token_stream());
        let accessor_composer = if search_key.maybe_originally_is_const_ptr() {
            FFIVariable::const_ptr
        } else if search_key.maybe_originally_is_mut_ptr() {
            FFIVariable::mut_ptr
        } else if search_key.maybe_originally_is_mut_ref() {
            FFIVariable::mut_ref
        } else if search_key.maybe_originally_is_dyn() {
            FFIVariable::r#dyn
        } else if search_key.maybe_originally_is_ref() {
            FFIVariable::r#ref
        } else {
            FFIVariable::direct
        };
        let full_ty = maybe_obj
            .as_ref()
            .and_then(ObjectKind::maybe_type)
            .unwrap_or_else(|| search_key.to_type());
        accessor_composer(match maybe_obj {
            Some(ObjectKind::Type(ref ty_model_kind)) |
            Some(ObjectKind::Item(ref ty_model_kind, ..)) => match ty_model_kind.maybe_trait_object_maybe_model_kind_or_same(source) {
                TypeModelKind::Bounds(..) =>
                    Type::Infer(TypeInfer { underscore_token: Default::default() }),
                _ => full_ty,
            },
            _ => full_ty,
        })

    }
}