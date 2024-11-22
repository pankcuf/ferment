use std::marker::PhantomData;
use syn::Type;
use crate::composer::SourceComposable;
use crate::context::{ScopeChain, ScopeContext, ScopeSearch, ScopeSearchKey};
use crate::conversion::ObjectKind;
use crate::ext::{Resolve, SpecialType, ToType};
use crate::lang::{LangFermentable, Specification};
use crate::presentation::FFIFullDictionaryPath;

#[derive(Clone)]
pub struct ScopeSearchComposer<'a, LANG, SPEC>
where LANG: LangFermentable,
      SPEC: Specification<LANG> {
    pub search: ScopeSearch<'a>,
    phantom_data: PhantomData<(LANG, SPEC)>
}

impl<'a, LANG, SPEC> ScopeSearchComposer<'a, LANG, SPEC>
where LANG: LangFermentable,
      SPEC: Specification<LANG> {
    fn new(search: ScopeSearch<'a>) -> Self {
        Self { search, phantom_data: PhantomData }
    }
    pub fn key_in_scope(ty: &'a Type, scope: &'a ScopeChain) -> Self {
        Self::new(ScopeSearch::KeyInScope(ScopeSearchKey::maybe_from_ref(ty).unwrap(), scope))
    }
    pub fn value(ty: &'a Type) -> Self {
        Self::new(ScopeSearch::Value(ScopeSearchKey::maybe_from_ref(ty).unwrap()))
    }
}

impl<'a, LANG, SPEC> SourceComposable for ScopeSearchComposer<'a, LANG, SPEC>
where LANG: LangFermentable,
      SPEC: Specification<LANG>,
      FFIFullDictionaryPath<LANG, SPEC>: ToType {

    type Source = ScopeContext;
    type Output = (Type, Option<ObjectKind>, Option<SpecialType<LANG, SPEC>>);

    fn compose(&self, source: &Self::Source) -> Self::Output {
        let search_key = self.search.search_key();
        let maybe_obj = source.maybe_object_by_predicate_ref(&self.search);
        let full_ty = maybe_obj.as_ref().and_then(ObjectKind::maybe_type).unwrap_or(search_key.to_type());
        let maybe_special: Option<SpecialType<LANG, SPEC>> = full_ty.maybe_resolve(source);
        (full_ty, maybe_obj, maybe_special)
    }
}