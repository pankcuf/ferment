use syn::{AngleBracketedGenericArguments, GenericArgument, Type, TypePath};
use crate::ext::MaybeAngleBracketedArgs;

pub trait MaybeGenericType {
    fn maybe_generic_type(&self) -> Option<&Type>;
    fn maybe_generic_type_mut(&mut self) -> Option<&mut Type>;
}

impl MaybeGenericType for GenericArgument {
    fn maybe_generic_type(&self) -> Option<&Type> {
        match self {
            GenericArgument::Type(ty) => Some(ty),
            _ => None
        }
    }

    fn maybe_generic_type_mut(&mut self) -> Option<&mut Type> {
        match self {
            GenericArgument::Type(ty) => Some(ty),
            _ => None
        }
    }
}

impl MaybeGenericType for AngleBracketedGenericArguments {
    fn maybe_generic_type(&self) -> Option<&Type> {
        self.args.iter().find_map(GenericArgument::maybe_generic_type)
    }

    fn maybe_generic_type_mut(&mut self) -> Option<&mut Type> {
        self.args.iter_mut().find_map(GenericArgument::maybe_generic_type_mut)
    }
}

impl MaybeGenericType for TypePath {
    fn maybe_generic_type(&self) -> Option<&Type> {
        self.path.segments.last()
            .and_then(MaybeAngleBracketedArgs::maybe_angle_bracketed_args)
            .and_then(MaybeGenericType::maybe_generic_type)
    }

    fn maybe_generic_type_mut(&mut self) -> Option<&mut Type> {
        self.path.segments.last_mut()
            .and_then(MaybeAngleBracketedArgs::maybe_angle_bracketed_args_mut)
            .and_then(MaybeGenericType::maybe_generic_type_mut)
    }
}