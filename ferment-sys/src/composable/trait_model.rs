use std::fmt::{Debug, Display, Formatter};
use syn::{Path, Type};
use crate::composable::{TraitDecompositionPart1, TypeModel, TypeModeled};
use crate::ext::{AsType, ToType};

#[derive(Clone)]
pub struct TraitModel {
    pub ty: TypeModel,
    pub decomposition: TraitDecompositionPart1,
    pub bounds: Vec<Path>
}

impl TraitModel {
    pub fn new(ty: TypeModel, decomposition: TraitDecompositionPart1, bounds: Vec<Path>) -> Self {
        Self { ty, decomposition, bounds }
    }

}
impl Debug for TraitModel {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("$Trait({})", self.ty))
    }
}

impl Display for TraitModel {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}

impl<'a> AsType<'a> for TraitModel {
    fn as_type(&'a self) -> &'a Type {
        self.ty.as_type()
    }
}

impl ToType for TraitModel {
    fn to_type(&self) -> Type {
        self.as_type().clone()
    }
}

impl TypeModeled for TraitModel {
    fn type_model_mut(&mut self) -> &mut TypeModel {
        &mut self.ty
    }

    fn type_model_ref(&self) -> &TypeModel {
        &self.ty
    }
}


