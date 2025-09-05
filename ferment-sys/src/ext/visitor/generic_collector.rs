use std::collections::HashSet;
use quote::ToTokens;
use syn::{AngleBracketedGenericArguments, GenericArgument, Item, ParenthesizedGenericArguments, Path, PathArguments, Signature, TraitBound, Type, TypeArray, TypeImplTrait, TypeParamBound, TypePath, TypeReference, TypeSlice, TypeTraitObject, TypeTuple};
use crate::kind::ScopeItemKind;
use crate::ext::item::segment_arguments_to_types;
use crate::ext::visitor::TypeCollector;

pub trait GenericCollector where Self: TypeCollector + ToTokens {
    fn find_generics(&self) -> HashSet<Type> {
        let compositions = self.collect_compositions();
        // collect all types with generics and ensure their uniqueness
        // since we don't want to implement interface multiple times for same object
        let mut generics = HashSet::<Type>::new();
        compositions
            .iter()
            .for_each(|field_type| field_type.collect_to(&mut generics));
        generics
    }

    fn collect_to(&self, generics: &mut HashSet<Type>) {
        generics.extend(self.find_generics());
    }
}
impl GenericCollector for ScopeItemKind {}
impl GenericCollector for Item {}
impl GenericCollector for Signature {}

impl GenericCollector for Type {
    fn collect_to(&self, generics: &mut HashSet<Type>) {
        let result = match self {
            Type::Path(TypePath { path, .. }) => {
                path.collect_to(generics);
                if path.segments
                    .iter()
                    .any(|seg| {
                        let has_nested_types = match &seg.arguments {
                            PathArguments::AngleBracketed(AngleBracketedGenericArguments { args, .. }) =>
                                args.iter().any(|arg| match arg {
                                    GenericArgument::Type(..) => true,
                                    _ => false
                                }),
                            PathArguments::Parenthesized(ParenthesizedGenericArguments { .. }) => true,
                            _ => false,
                        };
                        has_nested_types && seg.ident.ne("Option")
                    }) {

                    generics.insert(self.clone());
                }
            },
            Type::Reference(TypeReference { elem, .. }) =>
                elem.collect_to(generics),
            Type::ImplTrait(TypeImplTrait { bounds, .. }) |
            Type::TraitObject(TypeTraitObject { bounds, .. }) => {
                bounds.iter().for_each(|bound| match bound {
                    TypeParamBound::Trait(TraitBound { path, .. }) =>
                        path.collect_to(generics),
                    _ => {}
                })
            },
            Type::Tuple(TypeTuple { elems, .. }) => {
                generics.insert(self.clone());
                elems.iter()
                    .for_each(|ty| ty.collect_to(generics));
            },
            Type::Array(TypeArray { elem, .. }) |
            Type::Slice(TypeSlice { elem, .. }) => {
                generics.insert(self.clone());
                elem.collect_to(generics);
            },
            _ => {}
        };
        result
    }
}
impl GenericCollector for Path {
    fn collect_to(&self, generics: &mut HashSet<Type>) {
        self.segments
            .iter()
            .flat_map(segment_arguments_to_types)
            .for_each(|ty| ty.collect_to(generics));
    }
}


