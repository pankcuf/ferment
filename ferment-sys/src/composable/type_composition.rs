use std::fmt::{Debug, Display, Formatter};
use quote::ToTokens;
use syn::{Generics, Lifetime, Path, TraitBound, Type, TypeParamBound, TypePtr, TypeReference, TypeTraitObject};
use syn::punctuated::{IterMut, Punctuated};
use crate::composable::nested_arg::NestedArgument;
use crate::composer::CommaPunctuatedNestedArguments;
use crate::context::ScopeContext;
use crate::conversion::TypeModelKind;
use crate::ext::{AsType, CrateExtension, refine_ty_with_import_path, RefineMut, RefineWithNestedArgs, ResolveTrait, ToPath, ToType, LifetimeProcessor};

pub trait TypeModeled {
    fn type_model_mut(&mut self) -> &mut TypeModel;
    fn type_model_ref(&self) -> &TypeModel;
    fn type_model_and_nested_arguments_mut(&mut self) -> (&mut Type, &mut CommaPunctuatedNestedArguments) {
        let model = self.type_model_mut();
        (&mut model.ty, &mut model.nested_arguments)
    }
    fn nested_arguments_mut(&mut self) -> &mut CommaPunctuatedNestedArguments {
        &mut self.type_model_mut().nested_arguments
    }
    fn nested_arguments_iter_mut(&mut self) -> IterMut<NestedArgument> {
        self.nested_arguments_mut().iter_mut()
    }
    fn nested_arguments_ref(&self) -> &CommaPunctuatedNestedArguments {
        &self.type_model_ref().nested_arguments
    }
    fn ty_mut(&mut self) -> &mut Type {
        &mut self.type_model_mut().ty
    }
    fn replace_model_type(&mut self, with_ty: Type) {
        *self.ty_mut() = with_ty;
    }
}

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
        Self::new(value.clone(), None, CommaPunctuatedNestedArguments::new())
    }
}
impl From<Type> for TypeModel {
    fn from(value: Type) -> Self {
        Self::new(value, None, CommaPunctuatedNestedArguments::new())
    }
}

impl TypeModel {
    pub fn first_nested_argument(&self) -> Option<&NestedArgument> {
        self.nested_arguments_ref().first()
    }
    pub(crate) fn maybe_trait_object_maybe_model_kind(&self, source: &ScopeContext) -> Option<Option<TypeModelKind>> {
        self.as_type().maybe_trait_object_maybe_model_kind(source)
    }

    pub fn refine(&mut self, import_path: &Path) {
        let _ = self.ty.refine_with_nested_args(&self.nested_arguments);
        let _ = refine_ty_with_import_path(&mut self.ty, import_path);
    }
    pub fn new_non_gen(ty: Type, generics: Option<Generics>) -> Self {
        Self { ty, generics, nested_arguments: Punctuated::new() }
    }
    pub fn new(ty: Type, generics: Option<Generics>, nested_arguments: CommaPunctuatedNestedArguments) -> Self {
        Self { ty, generics, nested_arguments }
    }
    pub fn nested_argument_at_index(&self, index: usize) -> &NestedArgument {
        &self.nested_arguments[index]
    }

    pub fn pointer_less(&self) -> Path {
        let p = match &self.ty {
            Type::Reference(TypeReference { elem, .. }) |
            Type::Ptr(TypePtr { elem, .. }) => elem.to_path(),
            Type::TraitObject(TypeTraitObject { bounds, .. }) => {
                bounds.iter().find_map(|b| match b {
                    TypeParamBound::Trait(TraitBound { path, .. }) =>
                        Some(path.arg_less()),
                    TypeParamBound::Lifetime(_) =>
                        None
                }).unwrap()
            }
            other =>
                other.to_path()
        };
        // println!("pointer_less: {} --- {}", self.ty.to_token_stream(), p.to_token_stream());
        p
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
