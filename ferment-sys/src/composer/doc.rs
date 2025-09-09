use std::marker::PhantomData;
use quote::ToTokens;
use syn::__private::TokenStream2;
use crate::composer::{Linkable, SourceComposable};
use crate::context::ScopeContextLink;
use crate::lang::Specification;
use crate::presentation::default_doc;
use crate::shared::SharedAccess;

pub struct DocComposer<SPEC, Link>
where Link: SharedAccess,
      SPEC: Specification {
    pub parent: Option<Link>,
    pub ty: TokenStream2,
    _marker: PhantomData<SPEC>,
}

impl<SPEC, Link> From<&SPEC::TYC> for DocComposer<SPEC, Link>
where Link: SharedAccess,
      SPEC: Specification {
    fn from(value: &SPEC::TYC) -> Self {
        Self::new(value.to_token_stream())
    }
}


impl<SPEC, Link> DocComposer<SPEC, Link>
where Link: SharedAccess,
      SPEC: Specification {
    pub fn new(ty: TokenStream2) -> Self {
        Self { parent: None, ty, _marker: PhantomData }
    }
}

impl<SPEC, Link> Linkable<Link> for DocComposer<SPEC, Link>
where Link: SharedAccess,
      SPEC: Specification {
    fn link(&mut self, parent: &Link) {
        self.parent = Some(parent.clone_container());
    }
}

impl<SPEC, Link> SourceComposable for DocComposer<SPEC, Link>
where Link: SharedAccess,
      SPEC: Specification {
    type Source = ScopeContextLink;
    type Output = TokenStream2;
    fn compose(&self, _source: &Self::Source) -> Self::Output {
        default_doc(&self.ty)
    }
}
