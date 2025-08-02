use std::fmt;
use std::fmt::{Debug, Formatter};
use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{GenericArgument, PathArguments, Type, TypeImplTrait, TypePath, TypeReference, TypeTraitObject};
use syn::parse::{Parse, ParseStream};
use crate::ast::CommaPunctuated;
use crate::kind::{CallbackKind, GenericTypeKind, SmartPointerKind};
use crate::ext::{GenericNestedArg, Primitive};
use crate::presentable::ConversionExpressionKind;

#[derive(Clone, Eq)]
pub enum TypeKind {
    Primitive(Type),
    Complex(Type),
    Generic(GenericTypeKind),
}

impl Debug for TypeKind {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.write_str(match self {
            TypeKind::Primitive(_) => format!("Primitive({})", self.to_token_stream()),
            TypeKind::Complex(_) => format!("Complex({})", self.to_token_stream()),
            TypeKind::Generic(_) => format!("Generic({})", self.to_token_stream())
        }.as_str())
    }
}

impl PartialEq for TypeKind {
    fn eq(&self, other: &TypeKind) -> bool {
        self.to_token_stream().to_string() == other.to_token_stream().to_string()
    }
}

impl Parse for TypeKind {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Type::parse(input).map(Self::from)
    }
}

impl ToTokens for TypeKind {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            TypeKind::Primitive(ty) |
            TypeKind::Complex(ty) => ty.to_tokens(tokens),
            TypeKind::Generic(generic) => generic.to_tokens(tokens),
        }
    }
}
impl Primitive for TypeKind {
    fn is_primitive(&self) -> bool {
        match self {
            TypeKind::Primitive(..) => true,
            _ => false
        }
    }
}
impl From<&Box<Type>> for TypeKind {
    fn from(value: &Box<Type>) -> Self {
        TypeKind::from(*value.clone())
    }
}
impl From<&Type> for TypeKind {
    fn from(value: &Type) -> Self {
        TypeKind::from(value.clone())
    }
}
impl From<Type> for TypeKind {
    fn from(ty: Type) -> Self {
        let result = match ty {
            Type::Path(TypePath { ref path , ..}) => {
                let first_segment = path.segments.first().unwrap();
                let last_segment = path.segments.last().unwrap();
                let first_ident = &first_segment.ident;
                let last_ident = &last_segment.ident;
                match &last_segment.arguments {
                    PathArguments::AngleBracketed(..) => {
                        match last_ident.to_string().as_str() {
                            "Box" => TypeKind::Generic(GenericTypeKind::Box(ty)),
                            "Cell" => TypeKind::Generic(GenericTypeKind::SmartPointer(SmartPointerKind::Cell(ty))),
                            "Rc" => TypeKind::Generic(GenericTypeKind::SmartPointer(SmartPointerKind::Rc(ty))),
                            "Arc" => TypeKind::Generic(GenericTypeKind::SmartPointer(SmartPointerKind::Arc(ty))),
                            "Cow" => TypeKind::Generic(GenericTypeKind::Cow(ty)),
                            "RefCell" => TypeKind::Generic(GenericTypeKind::SmartPointer(SmartPointerKind::RefCell(ty))),
                            "UnsafeCell" => TypeKind::Generic(GenericTypeKind::SmartPointer(SmartPointerKind::UnsafeCell(ty))),
                            "Mutex" => TypeKind::Generic(GenericTypeKind::SmartPointer(SmartPointerKind::Mutex(ty))),
                            "OnceLock" => TypeKind::Generic(GenericTypeKind::SmartPointer(SmartPointerKind::OnceLock(ty))),
                            "RwLock" => TypeKind::Generic(GenericTypeKind::SmartPointer(SmartPointerKind::RwLock(ty))),
                            "Pin" => TypeKind::Generic(GenericTypeKind::SmartPointer(SmartPointerKind::Pin(ty))),
                            "BTreeMap" | "HashMap" => TypeKind::Generic(GenericTypeKind::Map(ty)),
                            "IndexMap" => TypeKind::Generic(GenericTypeKind::Map(ty)),
                            "BTreeSet" => TypeKind::Generic(GenericTypeKind::Group(ty)),
                            "HashSet" => TypeKind::Generic(GenericTypeKind::Group(ty)),
                            "IndexSet" => TypeKind::Generic(GenericTypeKind::Group(ty)),
                            "Vec" => TypeKind::Generic(GenericTypeKind::Group(ty)),
                            "Result" if path.segments.len() == 1 => TypeKind::Generic(GenericTypeKind::Result(ty)),
                            "Map" if first_ident.to_string().eq("serde_json") => TypeKind::Generic(GenericTypeKind::Map(ty)),
                            "Option" => TypeKind::Generic(GenericTypeKind::Optional(ty)),
                            "FnOnce" => TypeKind::Generic(GenericTypeKind::Callback(CallbackKind::FnOnce(ty))),
                            "Fn" => TypeKind::Generic(GenericTypeKind::Callback(CallbackKind::Fn(ty))),
                            "FnMut" => TypeKind::Generic(GenericTypeKind::Callback(CallbackKind::FnMut(ty))),
                            _ => path.segments.iter().find_map(|ff| match &ff.arguments {
                                PathArguments::AngleBracketed(args) => {
                                    let non_lifetimes = CommaPunctuated::from_iter(args.args.iter().filter_map(|arg| if let GenericArgument::Lifetime(_) = arg { None } else { Some(arg) }));
                                    Some(if non_lifetimes.is_empty() {
                                        TypeKind::Complex(ty.clone())
                                    } else {
                                        TypeKind::Generic(GenericTypeKind::AnyOther(ty.clone()))
                                    })
                                },
                                _ => None
                            }).unwrap_or_else(|| TypeKind::Complex(ty))
                        }
                    },
                    _ => match last_ident.to_string().as_str() {
                        // std convertible
                        "i8" | "u8" | "i16" | "u16" | "i32" | "u32" | "i64" | "u64" | "f64"
                        | "isize" | "usize" | "bool" => TypeKind::Primitive(ty),
                        "Box" => TypeKind::Generic(GenericTypeKind::Box(ty)),
                        "Cell" => TypeKind::Generic(GenericTypeKind::SmartPointer(SmartPointerKind::Cell(ty))),
                        "Rc" => TypeKind::Generic(GenericTypeKind::SmartPointer(SmartPointerKind::Rc(ty))),
                        "Arc" => TypeKind::Generic(GenericTypeKind::SmartPointer(SmartPointerKind::Arc(ty))),
                        "Cow" => TypeKind::Generic(GenericTypeKind::Cow(ty)),
                        "RefCell" => TypeKind::Generic(GenericTypeKind::SmartPointer(SmartPointerKind::RefCell(ty))),
                        "UnsafeCell" => TypeKind::Generic(GenericTypeKind::SmartPointer(SmartPointerKind::UnsafeCell(ty))),
                        "Mutex" => TypeKind::Generic(GenericTypeKind::SmartPointer(SmartPointerKind::Mutex(ty))),
                        "OnceLock" => TypeKind::Generic(GenericTypeKind::SmartPointer(SmartPointerKind::OnceLock(ty))),
                        "RwLock" => TypeKind::Generic(GenericTypeKind::SmartPointer(SmartPointerKind::RwLock(ty))),
                        "Pin" => TypeKind::Generic(GenericTypeKind::SmartPointer(SmartPointerKind::Pin(ty))),

                        "BTreeMap" | "HashMap" => TypeKind::Generic(GenericTypeKind::Map(ty)),
                        "IndexMap" => TypeKind::Generic(GenericTypeKind::Map(ty)),
                        "IndexSet" => TypeKind::Generic(GenericTypeKind::Group(ty)),
                        "BTreeSet" => TypeKind::Generic(GenericTypeKind::Group(ty)),
                        "HashSet" => TypeKind::Generic(GenericTypeKind::Group(ty)),
                        "Vec" => TypeKind::Generic(GenericTypeKind::Group(ty)),
                        "Result" if path.segments.len() == 1 => TypeKind::Generic(GenericTypeKind::Result(ty)),
                        "Map" if first_ident.to_string().eq("serde_json") => TypeKind::Generic(GenericTypeKind::Map(ty)),
                        "Option" => TypeKind::Generic(GenericTypeKind::Optional(ty)),
                        "FnOnce" => TypeKind::Generic(GenericTypeKind::Callback(CallbackKind::FnOnce(ty))),
                        "Fn" => TypeKind::Generic(GenericTypeKind::Callback(CallbackKind::Fn(ty))),
                        "FnMut" => TypeKind::Generic(GenericTypeKind::Callback(CallbackKind::FnMut(ty))),
                        _ => {
                            path.segments.iter().find_map(|ff| match &ff.arguments {
                                PathArguments::AngleBracketed(_) =>
                                    Some(TypeKind::Generic(GenericTypeKind::AnyOther(ty.clone()))),
                                _ => None
                            }).unwrap_or_else(|| TypeKind::Complex(ty))
                        },
                    }
                }
            },
            Type::Tuple(..) =>
                TypeKind::Generic(GenericTypeKind::Tuple(ty.clone())),
            Type::Array(..) =>
                TypeKind::Generic(GenericTypeKind::Array(ty.clone())),
            Type::Slice(..) =>
                TypeKind::Generic(GenericTypeKind::Slice(ty.clone())),
            Type::BareFn(..) => TypeKind::Generic(GenericTypeKind::Callback(CallbackKind::FnPointer(ty.clone()))),
            // Type::Ptr(_) => {}
            Type::Reference(TypeReference { elem, .. }) => TypeKind::from(*elem),
            Type::ImplTrait(TypeImplTrait { bounds, .. }) |
            Type::TraitObject(TypeTraitObject { bounds, .. }) =>
                TypeKind::Generic(GenericTypeKind::TraitBounds(bounds)),
            // todo: actually it's just about of absence of the conversions for opaque types
            Type::Ptr(..) => TypeKind::Primitive(ty),
            ty => unimplemented!("TypeKind: Unknown type: {:?}", ty)
        };
        // println!("TypeKind::from({}) ==== {:?}", dbg, result);

        result
    }
}

impl From<TypeKind> for ConversionExpressionKind {
    fn from(value: TypeKind) -> Self {
        match value {
            TypeKind::Primitive(_) =>
                ConversionExpressionKind::Primitive,
            TypeKind::Generic(GenericTypeKind::Optional(ty)) => match ty.maybe_first_nested_type_kind() {
                Some(TypeKind::Primitive(_)) =>
                    ConversionExpressionKind::PrimitiveOpt,
                _ =>
                    ConversionExpressionKind::ComplexOpt,
            }
            _ =>
                ConversionExpressionKind::Complex,
        }
    }
}