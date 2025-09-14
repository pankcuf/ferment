use std::fmt::{Debug, Display, Formatter};
use proc_macro2::Ident;
use quote::ToTokens;
use syn::{Generics, Lifetime, Path, TraitBound, Type, TypeImplTrait, TypePath, TypePtr, TypeReference, TypeTraitObject};
use crate::composable::{NestedArgument, TypeModeled};
use crate::composer::CommaPunctuatedNestedArguments;
use crate::context::ScopeChain;
use crate::ext::{AsType, refine_ty_with_import_path, RefineWithNestedArgs, ToPath, ToType, LifetimeProcessor, ArgsTransform, MaybeTraitBound};

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
    fn new(ty: Type, generics: Option<Generics>, nested_arguments: CommaPunctuatedNestedArguments) -> Self {
        Self { ty, generics, nested_arguments }
    }
    pub fn new_default(ty: Type) -> Self {
        Self::new(ty, None, CommaPunctuatedNestedArguments::new())
    }
    pub fn new_default_from_path(path: &Path) -> Self {
        Self::new(path.to_type(), None, CommaPunctuatedNestedArguments::new())
    }
    pub fn new_default_from_trait_bound(trait_bound: &TraitBound) -> Self {
        Self::new_default_from_path(&trait_bound.path)
    }
    pub fn new_nested(ty: Type, nested_arguments: CommaPunctuatedNestedArguments) -> Self {
        Self::new(ty, None, nested_arguments)
    }
    pub fn new_nested_ref(ty: &Type, nested_arguments: CommaPunctuatedNestedArguments) -> Self {
        Self::new_nested(ty.clone(), nested_arguments)
    }
    pub fn new_generic(ty: Type, generics: Generics, nested_arguments: CommaPunctuatedNestedArguments) -> Self {
        Self::new(ty, Some(generics), nested_arguments)
    }
    pub fn new_generic_ident(ident: &Ident, generics: Generics, nested_arguments: CommaPunctuatedNestedArguments) -> Self {
        Self::new_generic(ident.to_type(), generics, nested_arguments)
    }
    pub fn new_generic_non_nested(ty: Type, generics: &Generics) -> Self {
        Self::new(ty, Some(generics.clone()), CommaPunctuatedNestedArguments::new())
    }
    pub fn new_generic_ident_non_nested(ident: &Ident, generics: &Generics) -> Self {
        Self::new_generic_non_nested(ident.to_type(), generics)
    }
    pub fn new_generic_scope_non_nested(scope: &ScopeChain, generics: &Generics) -> Self {
        Self::new_generic_non_nested(scope.to_type(), generics)
    }
    pub fn new_generic_scope(scope: &ScopeChain, generics: &Generics, nested_arguments: CommaPunctuatedNestedArguments) -> Self {
        Self::new_generic(scope.to_type(), generics.clone(), nested_arguments)
    }
}
impl TypeModel {
    pub fn first_nested_argument(&self) -> Option<&NestedArgument> {
        self.nested_arguments_ref().first()
    }
    pub fn refine(&mut self, import_path: &Path) {
        let _ = self.ty.refine_with_nested_args(&self.nested_arguments);
        refine_ty_with_import_path(&mut self.ty, import_path);
    }
    pub fn nested_argument_at_index(&self, index: usize) -> &NestedArgument {
        &self.nested_arguments[index]
    }

    pub fn pointer_less(&self) -> Path {
        match &self.ty {
            Type::Reference(TypeReference { elem, .. }) |
            Type::Ptr(TypePtr { elem, .. }) => elem.to_path(),
            Type::TraitObject(TypeTraitObject { bounds, .. }) |
            Type::ImplTrait(TypeImplTrait { bounds, .. }) =>
                bounds.iter()
                    .find_map(MaybeTraitBound::maybe_trait_bound)
                    .map(|TraitBound { path, .. }| path.arg_less())
                    .unwrap_or_else(|| bounds.to_path()),
            Type::Path(TypePath { path, .. }) => path.clone(),
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
        f.write_fmt(format_args!("$Ty({}, {:?})", self.ty.to_token_stream(), self.nested_arguments))
    }
}

impl Display for TypeModel {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}
