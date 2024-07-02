use syn::{GenericParam, Generics, parse_quote, PredicateType, TraitBound, TypeParam, TypeParamBound, WherePredicate};
use crate::ast::{AddPunctuated, TypePathHolder};
use crate::composer::{Composer, Linkable, ParentComposer};
use crate::context::ScopeContext;
use crate::shared::SharedAccess;

pub struct GenericsComposer<Parent: SharedAccess> {
    pub parent: Option<Parent>,
    pub generics: Option<Generics>,
}
impl<Parent: SharedAccess> GenericsComposer<Parent> {
    pub fn new(generics: Option<Generics>) -> GenericsComposer<Parent> {
        Self { parent: None, generics }
    }
}

impl<Parent: SharedAccess> Linkable<Parent> for GenericsComposer<Parent> {
    fn link(&mut self, parent: &Parent) {
        self.parent = Some(parent.clone_container());
    }
}

impl<'a, Parent: SharedAccess> Composer<'a> for GenericsComposer<Parent> {
    type Source = ParentComposer<ScopeContext>;
    type Result = Option<Generics>;
    fn compose(&self, context: &Self::Source) -> Self::Result {
        let context = context.borrow();
        self.generics.as_ref().map(|generics| {
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
        })
    }
}
