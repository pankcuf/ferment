use syn::{parse_quote, Type};
use crate::composable::TypeModel;
use crate::context::ScopeContext;
use crate::kind::{DictFermentableModelKind, DictTypeModelKind, ObjectKind, SmartPointerModelKind, SpecialType, TypeModelKind};
use crate::ext::{AsType, GenericNestedArg, Resolve, ToType};
use crate::lang::RustSpecification;
use crate::lang::rust::presentation::resolve_type_variable;
use crate::presentation::{FFIFullPath, FFIVariable, ToFFIVariable};

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
                match ty.maybe_first_nested_type_ref() {
                    Some(full_nested_ty) => match Resolve::<SpecialType<RustSpecification>>::maybe_resolve(full_nested_ty, source) {
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
                    None => ty.to_direct_var()
                }
            },
            _ => panic!()
        }
    }
}