use std::marker::PhantomData;
use syn::{GenericParam, parse_quote, PredicateType, TraitBound, TypeParam, TypeParamBound, WherePredicate};
use crate::ast::{AddPunctuated, TypePathHolder};
use crate::composable::GenModel;
use crate::composer::{Composer, Linkable, ComposerLink};
use crate::context::ScopeContext;
use crate::lang::LangGenSpecification;
use crate::shared::SharedAccess;

pub struct GenericsComposer<Link, LANG, Gen>
    where Link: SharedAccess,
          LANG: Clone,
          Gen: LangGenSpecification<LANG> {
    pub parent: Option<Link>,
    pub generics: GenModel,
    _phantom_data: PhantomData<(LANG, Gen)>,

}
impl<Link, LANG, Gen> GenericsComposer<Link, LANG, Gen>
    where Link: SharedAccess, LANG: Clone, Gen: LangGenSpecification<LANG> {
    pub fn new(generics: GenModel) -> Self {
        Self { parent: None, generics, _phantom_data: PhantomData }
    }
}

impl<Link, LANG, Gen> Linkable<Link> for GenericsComposer<Link, LANG, Gen>
    where Link: SharedAccess, LANG: Clone, Gen: LangGenSpecification<LANG>  {
    fn link(&mut self, parent: &Link) {
        self.parent = Some(parent.clone_container());
    }
}

impl<'a, Link, LANG, Gen> Composer<'a> for GenericsComposer<Link, LANG, Gen>
    where Link: SharedAccess,
          LANG: Clone,
          Gen: LangGenSpecification<LANG> {
    type Source = ComposerLink<ScopeContext>;
    type Output = Gen;
    fn compose(&self, context: &Self::Source) -> Self::Output {
        let context = context.borrow();
        Gen::from_generics(self.generics.generics.as_ref().map(|generics| {
            let mut g = generics.clone();
            let update_bound = |type_path: &TypePathHolder, bounds: &mut AddPunctuated<TypeParamBound>| {
                if let Some(refined_bounds) = context.context.read().unwrap().generics.maybe_generic_bounds(&context.scope, type_path) {
                    bounds.iter_mut()
                        .zip(refined_bounds)
                        .for_each(|(b, rb)| match b {
                            TypeParamBound::Trait(TraitBound { path, .. }) => {
                                *path = rb.clone();
                            },
                            TypeParamBound::Lifetime(_) => {}
                        });
                }
            };
            g.params.iter_mut().for_each(|gp| match gp {
                GenericParam::Type(TypeParam { ident, bounds, .. }) => {
                    let ident_path: TypePathHolder = parse_quote!(#ident);
                    update_bound(&ident_path, bounds);
                }
                GenericParam::Lifetime(_) |
                GenericParam::Const(_) => {},
            });
            if let Some(ref mut wh) = g.where_clause {
                wh.predicates.iter_mut().for_each(|wp| match wp {
                    WherePredicate::Type(PredicateType { bounded_ty, bounds, .. }) => {
                        let ident_path: TypePathHolder = parse_quote!(#bounded_ty);
                        update_bound(&ident_path, bounds);
                    }
                    WherePredicate::Lifetime(_) => {}
                    WherePredicate::Eq(_) => {}
                })
            }
            g
        }))
    }
}
