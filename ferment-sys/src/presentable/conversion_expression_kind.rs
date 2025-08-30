use syn::Type;
use crate::ext::GenericNestedArg;
use crate::kind::{GenericTypeKind, TypeKind};

#[derive(Clone, Copy, Debug)]
pub enum ConversionExpressionKind {
    Primitive,
    PrimitiveOpt,
    Complex,
    ComplexOpt,
    OpaqueOpt,
    PrimitiveGroup,
    PrimitiveOptGroup,
    ComplexGroup,
    ComplexOptGroup,
    OpaqueGroup,
    OpaqueOptGroup,
}

impl From<&Type> for ConversionExpressionKind {
    fn from(value: &Type) -> Self {
        Self::from(value.clone())
    }
}

impl From<Type> for ConversionExpressionKind {
    fn from(value: Type) -> Self {
        match TypeKind::from(value) {
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