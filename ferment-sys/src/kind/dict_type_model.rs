use std::fmt::{Debug, Display, Formatter};
use syn::Type;
use crate::composable::{TypeModel, TypeModeled};
use crate::kind::DictFermentableModelKind;
use crate::ext::AsType;

#[derive(Clone)]
pub enum DictTypeModelKind {
    Primitive(TypeModel),
    LambdaFn(TypeModel),
    NonPrimitiveFermentable(DictFermentableModelKind),
    NonPrimitiveOpaque(TypeModel),
}

impl<'a> AsType<'a> for DictTypeModelKind {
    fn as_type(&'a self) -> &'a Type {
        match self {
            DictTypeModelKind::Primitive(model) |
            DictTypeModelKind::LambdaFn(model) |
            DictTypeModelKind::NonPrimitiveOpaque(model) => model.as_type(),
            DictTypeModelKind::NonPrimitiveFermentable(kind) => kind.as_type(),
        }
    }
}

impl TypeModeled for DictTypeModelKind {
    fn type_model_mut(&mut self) -> &mut TypeModel {
        match self {
            DictTypeModelKind::Primitive(model) |
            DictTypeModelKind::NonPrimitiveOpaque(model) |
            DictTypeModelKind::LambdaFn(model) => model,
            DictTypeModelKind::NonPrimitiveFermentable(kind) => kind.type_model_mut()
        }
    }
    fn type_model_ref(&self) -> &TypeModel {
        match self {
            DictTypeModelKind::Primitive(model) |
            DictTypeModelKind::NonPrimitiveOpaque(model) |
            DictTypeModelKind::LambdaFn(model) => model,
            DictTypeModelKind::NonPrimitiveFermentable(kind) => kind.type_model_ref()
        }
    }
}

impl Debug for DictTypeModelKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", match self {
            DictTypeModelKind::Primitive(ty) =>
                format!("Primitive({})", ty),
            DictTypeModelKind::NonPrimitiveFermentable(ty) =>
                format!("NonPrimitiveFermentable({})", ty),
            DictTypeModelKind::NonPrimitiveOpaque(ty) =>
                format!("NonPrimitiveOpaque({})", ty),
            DictTypeModelKind::LambdaFn(ty) =>
                format!("LambdaFn({})", ty),
        }))
    }
}

impl Display for DictTypeModelKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}
