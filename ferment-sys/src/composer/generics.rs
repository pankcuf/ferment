use std::marker::PhantomData;
use syn::{GenericParam, PredicateType, TraitBound, TypeParam, TypeParamBound, WherePredicate, Type};
use crate::ast::AddPunctuated;
use crate::composable::GenModel;
use crate::composer::{SourceComposable, Linkable};
use crate::context::ScopeContextLink;
use crate::ext::ToType;
use crate::lang::{LangGenSpecification, Specification};
use crate::shared::SharedAccess;

pub struct GenericsComposer<SPEC, Link>
    where Link: SharedAccess,
          SPEC: Specification {
    pub parent: Option<Link>,
    pub generics: GenModel,
    _marker: PhantomData<SPEC>,

}
impl<SPEC, Link> GenericsComposer<SPEC, Link>
    where Link: SharedAccess,
          SPEC: Specification {
    pub fn new(generics: GenModel) -> Self {
        Self { parent: None, generics, _marker: PhantomData }
    }
}

impl<SPEC, Link> Linkable<Link> for GenericsComposer<SPEC, Link>
    where Link: SharedAccess,
          SPEC: Specification {
    fn link(&mut self, parent: &Link) {
        self.parent = Some(parent.clone_container());
    }
}

impl<SPEC, Link> SourceComposable for GenericsComposer<SPEC, Link>
    where Link: SharedAccess,
          SPEC: Specification {
    type Source = ScopeContextLink;
    type Output = SPEC::Gen;
    fn compose(&self, context: &Self::Source) -> Self::Output {
        let context = context.borrow();
        SPEC::Gen::from_generics(self.generics.generics.as_ref().map(|generics| {
            let mut g = generics.clone();
            let update_bound = |type_path: &Type, bounds: &mut AddPunctuated<TypeParamBound>| {
                let lock = context.context.borrow();
                if let Some(refined_bounds) = lock.generics.maybe_generic_bounds(&context.scope, type_path) {
                    bounds.iter_mut()
                        .zip(refined_bounds)
                        .for_each(|(bound, refined_bound)| if let TypeParamBound::Trait(TraitBound { path, .. }) = bound {
                            *path = refined_bound.clone();
                        });
                }
            };
            g.params.iter_mut().for_each(|gp| if let GenericParam::Type(TypeParam { ident, bounds, .. }) = gp {
                update_bound(&ident.to_type(), bounds)
            });
            if let Some(ref mut wh) = g.where_clause {
                wh.predicates.iter_mut().for_each(|wp| if let WherePredicate::Type(PredicateType { bounded_ty, bounds, .. }) = wp {
                    update_bound(bounded_ty, bounds)
                })
            }
            g
        }))
    }
}
