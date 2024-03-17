use std::fmt::Formatter;
use std::hash::{Hash, Hasher};
use std::collections::HashSet;
use proc_macro2::TokenStream as TokenStream2;
use syn::{AngleBracketedGenericArguments, GenericArgument, Generics, Path, PathArguments, PathSegment, Type, TypePath};
use quote::{quote, ToTokens};
use crate::context::ScopeContext;
use crate::conversion::{ObjectConversion, TypeConversion};
use crate::holder::PathHolder;
use crate::presentation::ScopeContextPresentable;

#[derive(Clone, Debug)]
pub struct GenericConversion(pub ObjectConversion);

impl<'a> From<&'a ObjectConversion> for GenericConversion {
    fn from(value: &'a ObjectConversion) -> Self {
        GenericConversion::new(value.clone())
    }
}
impl std::fmt::Display for GenericConversion {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.0.to_token_stream().to_string().as_str())
    }
}

impl PartialEq for GenericConversion {
    fn eq(&self, other: &Self) -> bool {
        let self_tokens = [self.0.to_token_stream()];
        let other_tokens = [other.0.to_token_stream()];
        self_tokens.iter()
            .map(|t| t.to_string())
            .zip(other_tokens.iter().map(|t| t.to_string()))
            .all(|(a, b)| a == b)
    }
}

impl Eq for GenericConversion {}

impl Hash for GenericConversion {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.to_token_stream().to_string().hash(state);
    }
}

impl GenericConversion {
    pub fn new(full_type: ObjectConversion) -> Self {
        Self(full_type)
    }

    pub fn used_imports(&self) -> HashSet<PathHolder> {
        generic_imports(self.0.ty())
    }
}

impl ScopeContextPresentable for GenericConversion {
    type Presentation = TokenStream2;

    fn present(&self, source: &ScopeContext) -> Self::Presentation {
        let Self { 0: full_type } = self;
        println!("GenericConversion::present: {}", full_type.to_token_stream());
        match full_type {
            ObjectConversion::Type(type_conversion) |
            ObjectConversion::Item(type_conversion, _) => match TypeConversion::from(type_conversion.ty()) {
                TypeConversion::Generic(generic_conversion) =>
                    generic_conversion.expand(type_conversion, source),
                conversion =>
                    unimplemented!("non-generic GenericConversion: {}", conversion.to_token_stream())
            },
            ObjectConversion::Empty => {
                unimplemented!("expand: ObjectConversion::Empty")
            }
        }
    }
}

fn generic_imports(ty: Option<&Type>) -> HashSet<PathHolder> {
    match ty {
        Some(Type::Path(TypePath { path: Path { segments, .. }, .. })) => segments.iter()
            .flat_map(|PathSegment { arguments, .. }| match arguments {
                PathArguments::AngleBracketed(AngleBracketedGenericArguments { args, .. }) => args
                    .iter()
                    .map(|arg| match arg {
                        GenericArgument::Type(ty) => Some(ty),
                        _ => None
                    })
                    .flat_map(generic_imports)
                    .collect(),
                _ => HashSet::new(),
            })
            .collect(),
        _ => HashSet::new(),
    }
}


// pub fn collect_generic_types_in_type(field_type: &Type, generics: &mut HashSet<TypeHolder>) {
//     match field_type {
//         Type::Path(TypePath { path, .. }) => {
//             collect_generic_types_in_path(path, generics);
//             if path.segments.iter().any(|seg| !path_arguments_to_types(&seg.arguments).is_empty() && !matches!(seg.ident.to_string().as_str(), "Option")) {
//                 // println!("addd generic: {}", format_token_stream(field_type));
//                 generics.insert(TypeHolder(field_type.clone()));
//             }
//         },
//         Type::Reference(TypeReference { elem, .. }) => {
//             collect_generic_types_in_type(elem, generics);
//         },
//         Type::TraitObject(TypeTraitObject { bounds, .. }) => {
//             bounds.iter().for_each(|bound| match bound {
//                 TypeParamBound::Trait(TraitBound { path, .. }) => collect_generic_types_in_path(path, generics),
//                 _ => {}
//             })
//         },
//         Type::Tuple(TypeTuple { elems, .. }) => {
//             elems.iter()
//                 .for_each(|t| collect_generic_types_in_type(t, generics));
//         },
//         // Type::Array ??
//         _ => {}
//     }
// }
// fn collect_generic_types_in_type(field_type: &Type, generics: &mut HashSet<TypeAndPathHolder>) {
//     println!("collect_generic_types_in_type: {}", format_token_stream(field_type));
//     match field_type {
//         Type::Reference(TypeReference { mutability: _, elem, .. }) =>
//             Self::collect_generic_types_in_type(elem, generics),
//         Type::Path(TypePath { path, .. }) => {
//             match PathConversion::from(path) {
//                 PathConversion::Complex(path) => {
//                     println!("collect_generic_types_in_type: typepath complex: {}", format_token_stream(&path));
//                     if let Some(last_segment) = path.segments.last() {
//                         if last_segment.ident.to_string().as_str() == "Option" {
//                             Self::collect_generic_types_in_type(path_arguments_to_types(&last_segment.arguments)[0], generics);
//                         }
//                     }
//                 },
//                 PathConversion::Generic(GenericPathConversion::Result(path)) |
//                 PathConversion::Generic(GenericPathConversion::Vec(path)) |
//                 PathConversion::Generic(GenericPathConversion::Map(path)) => {
//                     println!("collect_generic_types_in_type: typepath generic: {}", format_token_stream(&path));
//                     path_arguments_to_types(&path.segments.last().unwrap().arguments)
//                         .iter()
//                         .for_each(|field_type|
//                             add_generic_type(field_type, generics));
//                     generics.insert(TypeAndPathHolder(field_type.clone(), path.clone()));
//                 },
//                 _ => {}
//             }
//         },
//         _ => {}
//     }
// }


// pub struct WhereComposition {
//     p
// }
//
#[derive(Clone, Debug)]
pub struct GenericsComposition {
    // pub qs: TypeComposition,
    pub generics: Generics,

    // pub where_composition: WhereComposition,
    // pub
}

impl ToTokens for GenericsComposition {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let Self { generics, .. } = self;
        quote!(#generics).to_tokens(tokens)
    }
}


//
// impl<'a> From<&'a Generics> for GenericComposition {
//     fn from(value: &'a Generics) -> Self {
//
//
//     }
//
// }