use std::marker::PhantomData;
use quote::ToTokens;
use syn::__private::TokenStream2;
use syn::PatType;
use crate::composer::{ConversionFromComposer, SourceComposable};
use crate::context::ScopeContext;
use crate::ext::{LifetimeProcessor, ToType};
use crate::lang::{LangAttrSpecification, LangLifetimeSpecification, Specification};
use crate::presentable::{ArgKind, Expression, ScopeContextPresentable};
use crate::presentation::{FFIFullDictionaryPath, FFIFullPath, Name};

pub struct PatTypeComposer<'a, SPEC> {
    pub pat_type: &'a PatType,
    _phantom_data: PhantomData<SPEC>,
}
impl<'a, SPEC> PatTypeComposer<'a, SPEC>
where SPEC: Specification {
    pub fn new(pat_type: &'a PatType) -> Self {
        Self {
            pat_type,
            _phantom_data: PhantomData,
        }
    }
}

impl<SPEC> SourceComposable for PatTypeComposer<'_, SPEC>
where SPEC: Specification<Name=Name<SPEC>, Expr=Expression<SPEC>>,
      SPEC::Expr: ScopeContextPresentable,
      SPEC::Name: ToTokens,
      FFIFullPath<SPEC>: ToType,
      FFIFullDictionaryPath<SPEC>: ToType {
    type Source = ScopeContext;
    type Output = (SPEC::Lt, TokenStream2, ArgKind<SPEC>, ArgKind<SPEC>);

    fn compose(&self, source: &Self::Source) -> Self::Output {
        let PatType { ty, attrs, pat, .. } = self.pat_type;
        let name = Name::pat(pat);
        (
            SPEC::Lt::from_lifetimes(ty.unique_lifetimes()),
            name.to_token_stream(),
            ArgKind::inherited_named_type(name.clone(), ty, SPEC::Attr::from_cfg_attrs(attrs)),
            ArgKind::expr(ConversionFromComposer::<SPEC>::key_in_composer_scope(name, ty).compose(source))
        )
    }
}

