use std::marker::PhantomData;
use syn::{Type, TypeInfer};
use crate::composer::SourceComposable;
use crate::context::{ScopeChain, ScopeContext, ScopeSearch, ScopeSearchKey};
use crate::conversion::{ObjectKind, TypeModelKind};
use crate::ext::{AsType, ResolveTrait, ToType};
use crate::lang::{RustSpecification, Specification};
use crate::presentation::FFIVariable;

#[derive(Clone, Debug)]
pub struct TargetVarComposer<'a, SPEC>
where SPEC: Specification {
    pub search: ScopeSearch<'a>,
    _marker: PhantomData<SPEC>,
}

impl<'a, SPEC> TargetVarComposer<'a, SPEC>
    where SPEC: Specification {
    pub fn new(search: ScopeSearch<'a>) -> Self {
        Self { search, _marker: PhantomData }
    }
    #[allow(unused)]
    pub fn key_in_scope(ty: &'a Type, scope: &'a ScopeChain) -> Self {
        Self::new(ScopeSearch::KeyInScope(ScopeSearchKey::maybe_from_ref(ty).unwrap(), scope))
    }

    pub fn value(ty: &'a Type) -> Self {
        Self::new(ScopeSearch::Value(ScopeSearchKey::maybe_from_ref(ty).unwrap()))
    }
}

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
            .unwrap_or(search_key.to_type());
        accessor_composer(match maybe_obj {
            Some(ObjectKind::Type(ref ty_model_kind)) |
            Some(ObjectKind::Item(ref ty_model_kind, ..)) => {
                let conversion = match ty_model_kind {
                    TypeModelKind::Trait(ty, ..) => {
                        ty.as_type()
                            .maybe_trait_object_model_kind(source)
                    },
                    _ => Some(ty_model_kind.clone()),
                }.unwrap_or(ty_model_kind.clone());
                match conversion {
                    TypeModelKind::Bounds(..) =>
                        Type::Infer(TypeInfer { underscore_token: Default::default() }),
                    _ => full_ty,
                }
            },
            _ => full_ty,
        })

    }
}