use std::fmt::{Debug, Display, Formatter};
use syn::{parse_quote, Type};
use crate::composable::{TypeModel, TypeModeled};
use crate::context::ScopeContext;
use crate::conversion::{DictFermentableModelKind, ObjectKind, SmartPointerModelKind, TypeModelKind};
use crate::ext::{AsType, GenericNestedArg, Resolve, SpecialType, ToType};
use crate::lang::RustSpecification;
use crate::presentation::{resolve_type_variable, FFIFullPath, FFIVariable, ToFFIVariable};

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
        f.write_str(match self {
            DictTypeModelKind::Primitive(ty) =>
                format!("Primitive({})", ty),
            DictTypeModelKind::NonPrimitiveFermentable(ty) =>
                format!("NonPrimitiveFermentable({})", ty),
            DictTypeModelKind::NonPrimitiveOpaque(ty) =>
                format!("NonPrimitiveOpaque({})", ty),
            DictTypeModelKind::LambdaFn(ty) =>
                format!("LambdaFn({})", ty),
        }.as_str())
    }
}

impl Display for DictTypeModelKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}

impl Resolve<FFIVariable<RustSpecification, Type>> for DictTypeModelKind {
    fn maybe_resolve(&self, source: &ScopeContext) -> Option<FFIVariable<RustSpecification, Type>> {
        Some(self.resolve(source))
    }

    fn resolve(&self, source: &ScopeContext) -> FFIVariable<RustSpecification, Type> {
        match self {
            DictTypeModelKind::Primitive(TypeModel { ty, .. }) =>
                FFIVariable::direct(ty.clone()),
            DictTypeModelKind::LambdaFn(TypeModel { ty, .. }) =>
                FFIVariable::mut_ptr(Resolve::<FFIFullPath<RustSpecification>>::resolve(ty, source).to_type()),
            // Dictionary generics should be fermented
            // Others should be treated as opaque
            DictTypeModelKind::NonPrimitiveOpaque(TypeModel { ty, .. }) =>
                Resolve::<SpecialType<RustSpecification>>::maybe_resolve(ty, source)
                    .map(|ty| resolve_type_variable(FFIFullPath::from(ty).to_type(), source))
                    .unwrap_or_else(|| {
                        let ty_model_kind_to_resolve = Resolve::<ObjectKind>::maybe_resolve(ty, source)
                            .and_then(|external_type| external_type.maybe_fn_or_trait_or_same_kind(source))
                            .unwrap_or_else(|| TypeModelKind::unknown_type_ref(ty));
                        resolve_type_variable(ty_model_kind_to_resolve.to_type(), source)
                    }),
            DictTypeModelKind::NonPrimitiveFermentable(DictFermentableModelKind::I128(..) | DictFermentableModelKind::U128(..)) =>
                FFIVariable::mut_ptr(parse_quote!([u8; 16])),
            DictTypeModelKind::NonPrimitiveFermentable(DictFermentableModelKind::SmartPointer(SmartPointerModelKind::Box(model))) |
            DictTypeModelKind::NonPrimitiveFermentable(DictFermentableModelKind::Cow(model)) => {
                let ty = model.as_type();
                let full_nested_ty = ty.maybe_first_nested_type_ref().unwrap();
                match Resolve::<SpecialType<RustSpecification>>::maybe_resolve(full_nested_ty, source) {
                    Some(special) => {
                        match source.maybe_object_by_value(full_nested_ty) {
                            Some(ObjectKind::Item(TypeModelKind::FnPointer(..), ..) |
                                 ObjectKind::Type(TypeModelKind::FnPointer(..), ..)) =>
                                special.to_direct_var(),
                            Some(ObjectKind::Item(TypeModelKind::Trait(..), ..) |
                                 ObjectKind::Type(TypeModelKind::TraitType(..), ..) |
                                 ObjectKind::Type(TypeModelKind::Dictionary(DictTypeModelKind::LambdaFn(..)), ..)) =>
                                special.to_dyn_var(),
                            _ => special.to_direct_var()
                        }
                    }
                    None => {
                        let object = source.maybe_object_by_value(full_nested_ty);
                        let ty_model_kind = object.and_then(|object_kind| object_kind.maybe_fn_or_trait_or_same_kind2(source))
                            .unwrap_or_else(|| TypeModelKind::unknown_type_ref(full_nested_ty));
                        let var_c_type = ty_model_kind.to_type();
                        let ffi_path: Option<FFIFullPath<RustSpecification>> = var_c_type.maybe_resolve(source);
                        let var_ty = ffi_path.map(|p| p.to_type()).unwrap_or_else(|| parse_quote!(#var_c_type));
                        let result = resolve_type_variable(var_ty, source);
                        result
                    }
                }
            },
            _ => panic!()
        }
    }
}