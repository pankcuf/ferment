use syn::Type;
use crate::composer::FFIAspect;
use crate::ext::GenericNestedArg;
use crate::kind::{GenericTypeKind, TypeKind};

#[derive(Clone, Copy, Debug)]
pub struct ConversionAspect {
    pub aspect: FFIAspect,
    pub kind: ConversionExpressionKind
}

impl ConversionAspect {
    pub fn new(aspect: FFIAspect, kind: ConversionExpressionKind) -> Self {
        Self { aspect, kind}
    }
    pub fn kind_from(kind: ConversionExpressionKind) -> Self {
        Self::new(FFIAspect::From, kind)
    }
    pub fn kind_to(kind: ConversionExpressionKind) -> Self {
        Self::new(FFIAspect::To, kind)
    }
    pub fn kind_drop(kind: ConversionExpressionKind) -> Self {
        Self::new(FFIAspect::Drop, kind)
    }


    pub fn complex_from() -> Self {
        Self::new(FFIAspect::From, ConversionExpressionKind::Complex)
    }
    pub fn complex_from_opt() -> Self {
        Self::new(FFIAspect::From, ConversionExpressionKind::ComplexOpt)
    }
    pub fn primitive_from() -> Self {
        Self::new(FFIAspect::From, ConversionExpressionKind::Primitive)
    }
    pub fn primitive_from_opt() -> Self {
        Self::new(FFIAspect::From, ConversionExpressionKind::PrimitiveOpt)
    }
    pub fn primitive_group_from() -> Self {
        Self::new(FFIAspect::From, ConversionExpressionKind::PrimitiveGroup)
    }
    pub fn primitive_group_opt_from() -> Self {
        Self::new(FFIAspect::From, ConversionExpressionKind::PrimitiveOptGroup)
    }
    pub fn complex_group_from() -> Self {
        Self::new(FFIAspect::From, ConversionExpressionKind::ComplexGroup)
    }
    pub fn complex_group_opt_from() -> Self {
        Self::new(FFIAspect::From, ConversionExpressionKind::ComplexOptGroup)
    }
    pub fn opaque_group_from() -> Self {
        Self::new(FFIAspect::From, ConversionExpressionKind::OpaqueGroup)
    }
    pub fn opaque_group_opt_from() -> Self {
        Self::new(FFIAspect::From, ConversionExpressionKind::OpaqueOptGroup)
    }

    pub fn complex_to() -> Self {
        Self::new(FFIAspect::To, ConversionExpressionKind::Complex)
    }
    pub fn complex_to_opt() -> Self {
        Self::new(FFIAspect::To, ConversionExpressionKind::ComplexOpt)
    }
    pub fn primitive_to() -> Self {
        Self::new(FFIAspect::To, ConversionExpressionKind::Primitive)
    }
    pub fn primitive_to_opt() -> Self {
        Self::new(FFIAspect::To, ConversionExpressionKind::PrimitiveOpt)
    }
    pub fn primitive_group_to() -> Self {
        Self::new(FFIAspect::To, ConversionExpressionKind::PrimitiveGroup)
    }
    pub fn primitive_group_opt_to() -> Self {
        Self::new(FFIAspect::To, ConversionExpressionKind::PrimitiveOptGroup)
    }
    pub fn complex_group_to() -> Self {
        Self::new(FFIAspect::To, ConversionExpressionKind::ComplexGroup)
    }
    pub fn complex_group_opt_to() -> Self {
        Self::new(FFIAspect::To, ConversionExpressionKind::ComplexOptGroup)
    }
    pub fn opaque_group_to() -> Self {
        Self::new(FFIAspect::To, ConversionExpressionKind::OpaqueGroup)
    }
    pub fn opaque_group_opt_to() -> Self {
        Self::new(FFIAspect::To, ConversionExpressionKind::OpaqueOptGroup)
    }



    pub fn primitive_drop() -> Self {
        Self::new(FFIAspect::Drop, ConversionExpressionKind::Primitive)
    }
    pub fn primitive_opt_drop() -> Self {
        Self::new(FFIAspect::Drop, ConversionExpressionKind::PrimitiveOpt)
    }
    pub fn complex_drop() -> Self {
        Self::new(FFIAspect::Drop, ConversionExpressionKind::Complex)
    }
    pub fn complex_opt_drop() -> Self {
        Self::new(FFIAspect::Drop, ConversionExpressionKind::ComplexOpt)
    }

    pub fn primitive_group_drop() -> Self {
        Self::new(FFIAspect::Drop, ConversionExpressionKind::PrimitiveGroup)
    }
    pub fn complex_group_drop() -> Self {
        Self::new(FFIAspect::Drop, ConversionExpressionKind::ComplexGroup)
    }

}

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