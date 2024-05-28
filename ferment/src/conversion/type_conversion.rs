use std::fmt;
use std::fmt::{Debug, Formatter};
use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{PathArguments, Type, TypeImplTrait, TypePath, TypeReference, TypeTraitObject};
use crate::conversion::GenericTypeConversion;

#[derive(Clone, Eq)]
pub enum TypeConversion {
    Primitive(Type),
    Complex(Type),
    Callback(Type),
    Generic(GenericTypeConversion),
}

impl Debug for TypeConversion {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.write_str(
        match self {
            TypeConversion::Primitive(_) => format!("Primitive({})", self.to_token_stream()),
            TypeConversion::Complex(_) => format!("Complex({})", self.to_token_stream()),
            TypeConversion::Callback(_) => format!("Callback({})", self.to_token_stream()),
            TypeConversion::Generic(_) => format!("Generic({})", self.to_token_stream()),
        }.as_str())
        // f.debug_list()
        //     .entries(self.to_token_stream())
        //     .finish()
    }
}

impl PartialEq for TypeConversion {
    fn eq(&self, other: &TypeConversion) -> bool {
        self.to_token_stream().to_string() == other.to_token_stream().to_string()
    }
}

impl ToTokens for TypeConversion {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            TypeConversion::Primitive(ty) |
            TypeConversion::Complex(ty) |
            TypeConversion::Callback(ty) => ty.to_tokens(tokens),
            TypeConversion::Generic(generic) => generic.to_tokens(tokens),
        }
    }
}
impl From<&Box<Type>> for TypeConversion {
    fn from(value: &Box<Type>) -> Self {
        TypeConversion::from(*value.clone())
    }
}
impl From<&Type> for TypeConversion {
    fn from(value: &Type) -> Self {
        TypeConversion::from(value.clone())
    }
}
impl From<Type> for TypeConversion {
    fn from(ty: Type) -> Self {
        // let dbg = ty.to_token_stream();
        let result = match ty {
            Type::Path(TypePath { ref path , ..}) => {
                let first_segment = path.segments.first().unwrap();
                let last_segment = path.segments.last().unwrap();
                let first_ident = &first_segment.ident;
                let last_ident = &last_segment.ident;
                match &last_segment.arguments {
                    PathArguments::AngleBracketed(..) => {
                        match last_ident.to_string().as_str() {
                            "Box" => TypeConversion::Generic(GenericTypeConversion::Box(ty)),
                            "Arc" | "Rc" | "Cell" | "RefCell" | "Mutex" | "RwLock" | "Pin" => TypeConversion::Generic(GenericTypeConversion::AnyOther(ty)),
                            "BTreeMap" | "HashMap" => TypeConversion::Generic(GenericTypeConversion::Map(ty)),
                            "IndexMap" => TypeConversion::Generic(GenericTypeConversion::IndexMap(ty)),
                            "BTreeSet" => TypeConversion::Generic(GenericTypeConversion::BTreeSet(ty)),
                            "HashSet" => TypeConversion::Generic(GenericTypeConversion::HashSet(ty)),
                            "Vec" => TypeConversion::Generic(GenericTypeConversion::Vec(ty)),
                            "Result" if path.segments.len() == 1 => TypeConversion::Generic(GenericTypeConversion::Result(ty)),
                            "Map" if first_ident.to_string().eq("serde_json") => TypeConversion::Generic(GenericTypeConversion::SerdeJsonMap(ty)),
                            "Option" => TypeConversion::Generic(GenericTypeConversion::Optional(ty)),
                            _ => path.segments.iter().find_map(|ff| match &ff.arguments {
                                PathArguments::AngleBracketed(_) =>
                                    Some(TypeConversion::Generic(GenericTypeConversion::AnyOther(ty.clone()))),
                                _ => None
                            }).unwrap_or(TypeConversion::Complex(ty))
                        }
                    },
                    _ => match last_ident.to_string().as_str() {
                        // std convertible
                        "i8" | "u8" | "i16" | "u16" | "i32" | "u32" | "i64" | "u64" | "f64" | "i128" | "u128"
                        | "isize" | "usize" | "bool" => TypeConversion::Primitive(ty),
                        "Box" => TypeConversion::Generic(GenericTypeConversion::Box(ty)),
                        "Arc" | "Rc" | "Cell" | "RefCell" | "Mutex" | "RwLock" | "Pin" => TypeConversion::Generic(GenericTypeConversion::AnyOther(ty)),
                        "BTreeMap" | "HashMap" => TypeConversion::Generic(GenericTypeConversion::Map(ty)),
                        "IndexMap" => TypeConversion::Generic(GenericTypeConversion::IndexMap(ty)),
                        "BTreeSet" => TypeConversion::Generic(GenericTypeConversion::BTreeSet(ty)),
                        "HashSet" => TypeConversion::Generic(GenericTypeConversion::HashSet(ty)),
                        "Vec" => TypeConversion::Generic(GenericTypeConversion::Vec(ty)),
                        "Result" if path.segments.len() == 1 => TypeConversion::Generic(GenericTypeConversion::Result(ty)),
                        "Map" if first_ident.to_string().eq("serde_json") => TypeConversion::Generic(GenericTypeConversion::SerdeJsonMap(ty)),
                        "Option" => TypeConversion::Generic(GenericTypeConversion::Optional(ty)),
                        _ => {
                            path.segments.iter().find_map(|ff| match &ff.arguments {
                                PathArguments::AngleBracketed(_) =>
                                    Some(TypeConversion::Generic(GenericTypeConversion::AnyOther(ty.clone()))),
                                _ => None
                            }).unwrap_or(TypeConversion::Complex(ty))
                        },
                    }
                }
            },
            Type::Tuple(..) =>
                TypeConversion::Generic(GenericTypeConversion::Tuple(ty.clone())),
            Type::Array(..) =>
                TypeConversion::Generic(GenericTypeConversion::Array(ty.clone())),
            Type::Slice(..) =>
                TypeConversion::Generic(GenericTypeConversion::Slice(ty.clone())),
            Type::BareFn(..) => TypeConversion::Callback(ty.clone()),
            // Type::Ptr(_) => {}
            Type::Reference(TypeReference { elem, .. }) => TypeConversion::from(*elem),
            Type::ImplTrait(TypeImplTrait { bounds, .. }) |
            Type::TraitObject(TypeTraitObject { bounds, .. }) =>
                TypeConversion::Generic(GenericTypeConversion::TraitBounds(bounds)),
            ty => unimplemented!("TypeConversion: Unknown type: {}", ty.to_token_stream())
        };
        // println!("TypeConversion::from.222({}) --- {:?}", dbg, result);

        result
    }
}
