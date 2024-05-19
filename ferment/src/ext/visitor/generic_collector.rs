use std::collections::HashSet;
use syn::{Item, Path, Signature, TraitBound, Type, TypeArray, TypeImplTrait, TypeParamBound, TypePath, TypeReference, TypeSlice, TypeTraitObject, TypeTuple};
use crate::conversion::ScopeItemConversion;
use crate::ext::visitor::TypeCollector;
use crate::helper::{path_arguments_to_types, segment_arguments_to_types};
use crate::holder::TypeHolder;


pub trait GenericCollector where Self: TypeCollector {
    fn find_generics(&self) -> HashSet<TypeHolder> {
        let compositions = self.collect_compositions();
        //println!("find_generics: {}", format_type_holders(&HashSet::from_iter(compositions.clone().into_iter())));
        // collect all types with generics and ensure their uniqueness
        // since we don't want to implement interface multiple times for same object
        let mut generics: HashSet<TypeHolder> = HashSet::new();
        compositions
            .iter()
            .for_each(|TypeHolder(field_type)| field_type.collect_to(&mut generics));
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
        match self {
            Type::Path(TypePath { path, .. }) => {
                path.collect_to(generics);
                if path.segments.iter().any(|seg| !path_arguments_to_types(&seg.arguments).is_empty() &&
                    !matches!(seg.ident.to_string().as_str(), "Option")) {
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
        }
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


