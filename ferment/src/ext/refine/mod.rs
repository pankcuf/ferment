use quote::ToTokens;
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{AngleBracketedGenericArguments, GenericArgument, parse_quote, PathArguments, Type, TypePath, TypeTuple};
use crate::composition::NestedArgument;
use crate::context::ScopeChain;

pub trait RefineMut: Sized {
    type Refinement;
    fn refine_with(&mut self, refined: Self::Refinement);
}

pub trait Unrefined: Sized {
    type Unrefinement;
    fn unrefined(&self) -> Self::Unrefinement;
}

pub trait RefineUnrefined: RefineMut + Unrefined<Unrefinement = Self::Refinement> {
    fn refine(&mut self) {
        let unrefined = self.unrefined();
        self.refine_with(unrefined);
    }
}

pub trait Refine: Sized {
    type Unrefined;
    fn refine_with(&self, refined: Self::Unrefined) -> Self;

    fn unrefined(&self) -> Self::Unrefined;

    fn refine_with_unrefined(&self) -> Self {
        let unrefined = self.unrefined();
        self.refine_with(unrefined)
    }
}

pub trait RefineAtScope: Sized {
    fn refine_at_scope(&self, scope: &ScopeChain) -> Self;
}

impl RefineMut for Type {
    type Refinement = Punctuated<NestedArgument, Comma>;

    fn refine_with(&mut self, refined: Self::Refinement) {
        if self == &parse_quote!(Option<get_identity_response_v0::Result>) ||
            self == &parse_quote!(get_identity_response_v0::Result) {
            println!("refine: {}\n\twith: {}", self.to_token_stream(), refined.to_token_stream())
        }
        match self {
            Type::Path(TypePath { path, .. }) => {
                path.segments.last_mut().unwrap().arguments.refine_with(refined);
            },
            Type::Tuple(TypeTuple { elems, .. }) => {
                let mut refinement = refined.clone();
                elems.iter_mut()
                    .rev()
                    .for_each(|inner_ty| {
                        match refinement.pop() {
                            None => {}
                            Some(nested_arg) => match nested_arg.into_value() {
                                NestedArgument::Object(obj) => {
                                    *inner_ty = obj.to_ty().unwrap();
                                }
                            }
                        }
                    });
            },
            _ => {}
        }
    }
}

impl RefineMut for PathArguments {
    type Refinement = Punctuated<NestedArgument, Comma>;

    fn refine_with(&mut self, refined: Self::Refinement) {
        let mut refinement = refined.clone();
        match self {
            PathArguments::None => {}
            PathArguments::AngleBracketed(AngleBracketedGenericArguments { args, .. }) => {
                args.iter_mut()
                    .rev()
                    .for_each(|arg| {
                        match arg {
                            GenericArgument::Type(inner_ty) => {
                                match refinement.pop() {
                                    None => {}
                                    Some(nested_arg) => match nested_arg.into_value() {
                                        NestedArgument::Object(obj) => {
                                            *inner_ty = obj.to_ty().unwrap();
                                        }
                                    }
                                }
                            }
                            GenericArgument::Lifetime(_) => {}
                            GenericArgument::Const(_) => {}
                            GenericArgument::Binding(_) => {}
                            GenericArgument::Constraint(_) => {}
                        }
                    });
            }
            PathArguments::Parenthesized(_) => {}
        }
    }
}