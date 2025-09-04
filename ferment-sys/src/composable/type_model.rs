use std::fmt::{Debug, Display, Formatter};
use quote::ToTokens;
use syn::{Generics, Lifetime, Path, TraitBound, Type, TypeParamBound, TypePtr, TypeReference, TypeTraitObject};
use crate::composable::{NestedArgument, TypeModeled};
use crate::composer::CommaPunctuatedNestedArguments;
use crate::ext::{AsType, CrateExtension, refine_ty_with_import_path, RefineMut, RefineWithNestedArgs, ToPath, ToType, LifetimeProcessor};

#[derive(Clone)]
pub struct TypeModel {
    pub ty: Type,
    pub generics: Option<Generics>,
    pub nested_arguments: CommaPunctuatedNestedArguments,
}

impl TypeModeled for TypeModel {
    fn type_model_mut(&mut self) -> &mut TypeModel {
        self
    }

    fn type_model_ref(&self) -> &TypeModel {
        self
    }
}

impl RefineMut for TypeModel {

    type Refinement = CommaPunctuatedNestedArguments;

    fn refine_with(&mut self, refined: Self::Refinement) {
        self.ty.refine_with(refined);
    }
}

impl From<&Type> for TypeModel {
    fn from(value: &Type) -> Self {
        Self::new_default(value.clone())
    }
}
impl From<Type> for TypeModel {
    fn from(value: Type) -> Self {
        Self::new_default(value)
    }
}

impl TypeModel {
    pub fn new_default(ty: Type) -> Self {
        Self::new(ty, None, CommaPunctuatedNestedArguments::new())
    }
    pub fn new_non_nested(ty: Type, generics: Option<Generics>) -> Self {
        Self::new(ty, generics, CommaPunctuatedNestedArguments::new())
    }
    pub fn new_nested(ty: Type, nested_arguments: CommaPunctuatedNestedArguments) -> Self {
        Self::new(ty, None, nested_arguments)
    }
    pub fn new_generic(ty: Type, generics: Generics, nested_arguments: CommaPunctuatedNestedArguments) -> Self {
        Self::new(ty, Some(generics), nested_arguments)
    }
    pub fn new_generic_non_nested(ty: Type, generics: Generics) -> Self {
        Self::new_non_nested(ty, Some(generics))
    }
    fn new(ty: Type, generics: Option<Generics>, nested_arguments: CommaPunctuatedNestedArguments) -> Self {
        Self { ty, generics, nested_arguments }
    }
    pub fn first_nested_argument(&self) -> Option<&NestedArgument> {
        self.nested_arguments_ref().first()
    }
    pub fn refine(&mut self, import_path: &Path) {
        let _ = self.ty.refine_with_nested_args(&self.nested_arguments);
        let _ = refine_ty_with_import_path(&mut self.ty, import_path);
    }
    pub fn nested_argument_at_index(&self, index: usize) -> &NestedArgument {
        &self.nested_arguments[index]
    }

    pub fn pointer_less(&self) -> Path {
        match &self.ty {
            Type::Reference(TypeReference { elem, .. }) |
            Type::Ptr(TypePtr { elem, .. }) => elem.to_path(),
            Type::TraitObject(TypeTraitObject { bounds, .. }) => {
                if let Some(bound) = bounds.iter().find_map(|b| match b {
                    TypeParamBound::Trait(TraitBound { path, .. }) =>
                        Some(path.arg_less()),
                    _ =>
                        None
                }) {
                    bound
                } else {
                    bounds.to_path()
                }
            }
            other =>
                other.to_path()
        }
    }
}

impl LifetimeProcessor for TypeModel {
    fn clean_lifetimes(&mut self) {
        self.ty.clean_lifetimes();
    }

    fn unique_lifetimes(&self) -> Vec<Lifetime> {
        self.ty.unique_lifetimes()
    }
}

impl<'a> AsType<'a> for TypeModel {
    fn as_type(&'a self) -> &'a Type {
        &self.ty
    }
}

impl ToType for TypeModel {
    fn to_type(&self) -> Type {
        self.as_type().clone()
    }
}

impl Debug for TypeModel {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            format!("$Ty({}, {:?})",
                    self.ty.to_token_stream(),
                    self.nested_arguments,
                    // self.generics.as_ref().map_or(format!("None"), format_token_stream)
                ).as_str())
    }
}

impl Display for TypeModel {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}
