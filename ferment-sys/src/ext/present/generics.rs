use syn::{AngleBracketedGenericArguments, GenericArgument, PathArguments, PathSegment, Type};
use crate::ext::maybe_generic_type::MaybeGenericType;
use crate::kind::TypeKind;

#[allow(unused)]
pub trait GenericNestedArg {
    fn maybe_first_nested_type_ref(&self) -> Option<&Type>;
    fn maybe_first_nested_type(&self) -> Option<Type> {
        self.maybe_first_nested_type_ref().cloned()
    }
    fn nested_types(&self) -> Vec<&Type>;
    fn maybe_first_nested_type_kind(&self) -> Option<TypeKind> {
        self.maybe_first_nested_type_ref()
            .map(TypeKind::from)
    }
}

impl GenericNestedArg for Type {
    fn maybe_first_nested_type_ref(&self) -> Option<&Type> {
        match self {
            Type::Array(type_array) => Some(&type_array.elem),
            Type::Slice(type_slice) => Some(&type_slice.elem),
            Type::Reference(type_reference) => Some(&type_reference.elem),
            Type::Path(type_path) => type_path.maybe_generic_type(),
            _ => None
        }
    }

    fn nested_types(&self) -> Vec<&Type> {
        match self {
            Type::Array(type_array) => vec![&type_array.elem],
            Type::Slice(type_slice) => vec![&type_slice.elem],
            Type::Reference(type_reference) => vec![&type_reference.elem],
            Type::Path(type_path) => {
                let mut vec = Vec::<&Type>::new();
                if let Some(PathSegment { arguments: PathArguments::AngleBracketed(AngleBracketedGenericArguments { args, .. }), .. }) = type_path.path.segments.last() {
                    args.iter().for_each(|arg| match arg {
                        GenericArgument::Type(ty) => vec.push(ty),
                        _ => {}
                    });
                }
                vec
            },
            Type::Tuple(type_tuple) => type_tuple.elems.iter().collect(),
            _ => vec![]
        }
    }
}

