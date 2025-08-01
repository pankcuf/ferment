use std::marker::PhantomData;
use syn::{GenericParam, parse_quote, PredicateType, TraitBound, TypeParam, TypeParamBound, WherePredicate};
use crate::ast::{AddPunctuated, TypePathHolder};
use crate::composable::GenModel;
use crate::composer::{SourceComposable, Linkable};
use crate::context::ScopeContextLink;
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
            let update_bound = |type_path: &TypePathHolder, bounds: &mut AddPunctuated<TypeParamBound>| {
                if let Some(refined_bounds) = context.context.read().unwrap().generics.maybe_generic_bounds(&context.scope, type_path) {
                    bounds.iter_mut()
                        .zip(refined_bounds)
                        .for_each(|(b, rb)| match b {
                            TypeParamBound::Trait(TraitBound { path, .. }) => {
                                *path = rb.clone();
                            },
                            _ => {}
                        });
                }
            };
            g.params.iter_mut().for_each(|gp| match gp {
                GenericParam::Type(TypeParam { ident, bounds, .. }) => {
                    let ident_path: TypePathHolder = parse_quote!(#ident);
                    update_bound(&ident_path, bounds);
                }
                _ => {},
            });
            if let Some(ref mut wh) = g.where_clause {
                wh.predicates.iter_mut().for_each(|wp| match wp {
                    WherePredicate::Type(PredicateType { bounded_ty, bounds, .. }) => {
                        let ident_path: TypePathHolder = parse_quote!(#bounded_ty);
                        update_bound(&ident_path, bounds);
                    }
                    _ => {}
                })
            }
            g
        }))
    }
}
