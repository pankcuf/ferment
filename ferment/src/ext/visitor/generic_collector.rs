use std::collections::HashSet;
use quote::ToTokens;
use syn::{AngleBracketedGenericArguments, GenericArgument, Item, ParenthesizedGenericArguments, Path, PathArguments, Signature, TraitBound, Type, TypeArray, TypeImplTrait, TypeParamBound, TypePath, TypeReference, TypeSlice, TypeTraitObject, TypeTuple};
use crate::ast::TypeHolder;
use crate::conversion::ScopeItemConversion;
use crate::ext::item::segment_arguments_to_types;
use crate::ext::visitor::TypeCollector;

// pub trait Collector where Self: TypeCollector {
//     fn collect(&self) -> HashSet<TypeHolder> {
//         let compositions = self.collect_compositions();
//         //println!("find_generics: {}", format_type_holders(&HashSet::from_iter(compositions.clone().into_iter())));
//         // collect all types with generics and ensure their uniqueness
//         // since we don't want to implement interface multiple times for same object
//         let mut generics: HashSet<TypeHolder> = HashSet::new();
//         compositions
//             .iter()
//             .for_each(|TypeHolder(field_type)| field_type.collect_to(&mut generics));
//         generics
//     }
//     fn collect_to(&self, generics: &mut HashSet<TypeHolder>) {
//         generics.extend(self.find_generics());
//     }
//
// }

pub trait GenericCollector where Self: TypeCollector + ToTokens {
    fn find_generics(&self) -> HashSet<TypeHolder> {
        let compositions = self.collect_compositions();
        // println!("find_generics in [{}]: {}", self.to_token_stream(), format_type_holders(&HashSet::from_iter(compositions.clone().into_iter())));
        // collect all types with generics and ensure their uniqueness
        // since we don't want to implement interface multiple times for same object
        let mut generics: HashSet<TypeHolder> = HashSet::new();
        //println!("GenericCollector::compositions: {}\n{}", self.to_token_stream(), format_type_holders_vec(&compositions));
        compositions
            .iter()
            .for_each(|TypeHolder(field_type)| field_type.collect_to(&mut generics));
        //println!("GenericCollector::generics {}\n{}", self.to_token_stream(), format_type_holders(&generics));

        generics
    }

    fn collect_to(&self, generics: &mut HashSet<TypeHolder>) {
        generics.extend(self.find_generics());
    }
}
impl GenericCollector for ScopeItemConversion {}
impl GenericCollector for Item {}
impl GenericCollector for Signature {}

impl GenericCollector for Type {
    fn collect_to(&self, generics: &mut HashSet<TypeHolder>) {
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
                        has_nested_types && !matches!(seg.ident.to_string().as_str(), "Option")
                    }) {

                    generics.insert(TypeHolder(self.clone()));
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
                generics.insert(TypeHolder(self.clone()));
                elems.iter()
                    .for_each(|ty| ty.collect_to(generics));
            },
            Type::Array(TypeArray { elem, .. }) |
            Type::Slice(TypeSlice { elem, .. }) => {
                generics.insert(TypeHolder(self.clone()));
                elem.collect_to(generics);
            },
            _ => {}
        };
        // println!("GenericCollector:: {}", self.to_token_stream(), form);
        result
    }
}
impl GenericCollector for Path {
    fn collect_to(&self, generics: &mut HashSet<TypeHolder>) {
        self.segments
            .iter()
            .flat_map(segment_arguments_to_types)
            .for_each(|ty| ty.collect_to(generics));
    }
}


