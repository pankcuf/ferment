use syn::{Path, Type};
use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};
use crate::composition::TypeComposition;
use crate::formatter::{format_path_vec, format_predicates_dict};

#[derive(Clone)]
pub struct GenericBoundComposition {
    pub type_composition: TypeComposition,
    pub bounds: Vec<Path>,
    pub predicates: HashMap<Type, Vec<Path>>
}

impl Debug for GenericBoundComposition {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let str = format!("GenericBoundComposition({}, {}, {})",
                            self.type_composition,
                            format_path_vec(&self.bounds),
                            format_predicates_dict(&self.predicates));
        f.write_str(str.as_str())
    }
}

impl Display for GenericBoundComposition {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}


impl GenericBoundComposition {
    // pub fn is_empty(&self) -> bool {
    //     self.bounds.is_empty()
    // }

    // pub fn from_generics(generics: &Generics) -> Self {
    //     let bounds
    //     generics.params.iter().for_each(|generic_param| {
    //         match generic_param {
    //             GenericParam::Type(TypeParam { ident: generic_ident, bounds, .. }) => {
    //                 let mut de_bounds: Vec<Path> =  vec![];
    //                 bounds.iter().for_each(|bound| {
    //                     match bound {
    //                         TypeParamBound::Trait(TraitBound { path, .. }) => {
    //                             de_bounds.push(path.clone());
    //                         },
    //                         TypeParamBound::Lifetime(_lifetime) => {}
    //                     }
    //                 });
    //                 // generics.insert(parse_quote!(#generic_ident), de_bounds);
    //             },
    //             GenericParam::Lifetime(_lifetime) => {},
    //             GenericParam::Const(ConstParam { ty, .. }) => {
    //
    //             },
    //         }
    //     }
    // }
}


