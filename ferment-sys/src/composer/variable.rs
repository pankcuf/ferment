use std::fmt::Debug;
use std::marker::PhantomData;
use syn::{parse_quote, Type, TypePtr};
use crate::composable::TypeModel;
use crate::composer::SourceComposable;
use crate::context::{ScopeContext, ScopeSearch, ScopeSearchKey};
use crate::conversion::{DictFermentableModelKind, DictTypeModelKind, GroupModelKind, ObjectKind, ScopeItemKind, SmartPointerModelKind, TypeModelKind};
use crate::ext::{AsType, FFISpecialTypeResolve, GenericNestedArg, Resolve, ResolveTrait, SpecialType, ToType};
use crate::lang::{RustSpecification, Specification};
use crate::presentation::{resolve_type_variable, FFIFullPath, FFIVariable, ToFFIVariable};


#[derive(Clone, Debug)]
pub struct VariableComposer<SPEC>
    where SPEC: Specification {
    pub ty: Type,
    _marker: PhantomData<SPEC>
}

impl<SPEC> VariableComposer<SPEC>
    where SPEC: Specification {
    pub fn new(ty: Type) -> Self {
        Self { ty, _marker: PhantomData }
    }
}
impl<SPEC> From<&Type> for VariableComposer<SPEC>
    where SPEC: Specification {
    fn from(value: &Type) -> Self {
        Self { ty: value.clone(), _marker: PhantomData }
    }
}

impl SourceComposable for VariableComposer<RustSpecification> {
    type Source = ScopeContext;
    type Output = FFIVariable<RustSpecification, Type>;

    fn compose(&self, source: &Self::Source) -> Self::Output {
        let is_const_ptr = match self.ty {
            Type::Ptr(TypePtr { const_token, .. }) => const_token.is_some(),
            _ => false
        };
        let ptr_composer = if is_const_ptr {
            FFIVariable::const_ptr
        } else {
            FFIVariable::mut_ptr
        };
        println!("VariableComposer::search_key: {:?}", self.ty);

        let full_ty: Type = Resolve::resolve(&self.ty, source);
        let maybe_obj = source.maybe_object_by_predicate(ScopeSearch::KeyInScope(ScopeSearchKey::TypeRef(&self.ty, None), &source.scope));
        let result = match Resolve::<SpecialType<RustSpecification>>::maybe_resolve(&full_ty, source) {
            Some(special) => match maybe_obj {
                Some(ObjectKind::Item(_fn_ty_conversion, ScopeItemKind::Fn(..))) => {
                    let ty = match &source.scope.parent_object().unwrap() {
                        ObjectKind::Type(ref ty_model_kind) |
                        ObjectKind::Item(ref ty_model_kind, ..) => {
                            let parent_scope = source.scope.parent_scope().unwrap();
                            let context = source.context.read().unwrap();
                            context.maybe_scope_ref_obj_first(parent_scope.self_path())
                                .and_then(|parent_obj_scope| {
                                    context.maybe_object_ref_by_tree_key(ty_model_kind.as_type(), parent_obj_scope)
                                        .and_then(ObjectKind::maybe_type)
                                }).unwrap_or_else(|| parent_scope.to_type())
                        },
                        _ => {
                            self.ty.clone()
                        }
                    };
                    ptr_composer(ty)
                }
                Some(ObjectKind::Item(TypeModelKind::FnPointer(..), ..) |
                     ObjectKind::Type(TypeModelKind::FnPointer(..), ..)) =>
                    special.to_direct_var(),
                Some(ObjectKind::Item(TypeModelKind::Trait(..), ..) |
                     ObjectKind::Type(TypeModelKind::TraitType(..), ..) |
                     ObjectKind::Type(TypeModelKind::Dictionary(DictTypeModelKind::LambdaFn(..)), ..)) => {
                    let ty = special.to_type();
                    ptr_composer(parse_quote!(dyn #ty))
                },
                Some(ObjectKind::Type(TypeModelKind::Bounds(bounds))) => {
                    bounds.resolve(source)
                },
                _ => {
                    ptr_composer(special.to_type())
                }
            }
            None => {
                match maybe_obj {
                    Some(ObjectKind::Item(_fn_ty_conversion, ScopeItemKind::Fn(..))) => {
                        let ty = match &source.scope.parent_object().unwrap() {
                            ObjectKind::Type(ref ty_conversion) |
                            ObjectKind::Item(ref ty_conversion, ..) => {
                                let full_parent_ty: Type = Resolve::resolve(ty_conversion.as_type(), source);
                                match Resolve::<SpecialType<RustSpecification>>::maybe_resolve(&full_parent_ty, source) {
                                    Some(special) => special.to_type(),
                                    None => {
                                        match ty_conversion {
                                            TypeModelKind::Trait(ty, _decomposition, _super_bounds) =>
                                                ty.as_type()
                                                    .maybe_trait_object(source)
                                                    .and_then(|oc| oc.maybe_type_model_kind_ref().map(|c| c.to_type()))
                                                    .unwrap_or_else(|| ty_conversion.to_type()),
                                            _ => ty_conversion.to_type()
                                        }
                                    }
                                }
                            },
                            _ => self.ty.clone()
                        };
                        ptr_composer(ty)
                    },
                    Some(ObjectKind::Type(ref ty_model_kind)) |
                    Some(ObjectKind::Item(ref ty_model_kind, ..)) => {
                        let conversion = match ty_model_kind {
                            TypeModelKind::Trait(ty, ..) => {
                                ty.as_type()
                                    .maybe_trait_object_model_kind(source)
                            },
                            _ => Some(ty_model_kind.clone()),
                        }.unwrap_or_else(|| ty_model_kind.clone());
                        match conversion {
                            TypeModelKind::Dictionary(DictTypeModelKind::NonPrimitiveFermentable(DictFermentableModelKind::SmartPointer(SmartPointerModelKind::Box(model)))) |
                            TypeModelKind::Dictionary(DictTypeModelKind::NonPrimitiveFermentable(DictFermentableModelKind::Cow(model))) => {
                                let ty = model.as_type();
                                let nested_ty = self.ty.maybe_first_nested_type_ref().unwrap();
                                let full_nested_ty = ty.maybe_first_nested_type_ref().unwrap();
                                match Resolve::<SpecialType<RustSpecification>>::maybe_resolve(full_nested_ty, source) {
                                    Some(special) => {
                                        match source.maybe_object_by_key(nested_ty) {
                                            Some(ObjectKind::Item(TypeModelKind::FnPointer(..), ..) |
                                                 ObjectKind::Type(TypeModelKind::FnPointer(..), ..)) =>
                                                special.to_direct_var(),
                                            Some(ObjectKind::Item(TypeModelKind::Trait(..), ..) |
                                                 ObjectKind::Type(TypeModelKind::TraitType(..), ..) |
                                                 ObjectKind::Type(TypeModelKind::Dictionary(DictTypeModelKind::LambdaFn(..)), ..)) => {
                                                let ty = special.to_type();
                                                ptr_composer(parse_quote!(dyn #ty))
                                            },
                                            _ => {
                                                let ty = special.to_type();
                                                ptr_composer(ty)
                                            }
                                        }
                                    }
                                    None => {
                                        let object = Resolve::<ObjectKind>::maybe_resolve(nested_ty, source);
                                        let ty_model_kind = match object {
                                            Some(ObjectKind::Item(.., ScopeItemKind::Fn(..))) =>
                                                source.maybe_trait_or_regular_model_kind(),
                                            Some(ObjectKind::Type(ref kind) |
                                                 ObjectKind::Item(ref kind, ..)) =>
                                                kind.maybe_trait_model_kind_or_same(source),
                                            _ => None,
                                        }.unwrap_or_else(|| TypeModelKind::unknown_type_ref(nested_ty));
                                        let var_c_type = ty_model_kind.to_type();
                                        let ffi_path: Option<FFIFullPath<RustSpecification>> = var_c_type.maybe_resolve(source);
                                        let var_ty = ffi_path.map(|p| p.to_type()).unwrap_or_else(|| parse_quote!(#var_c_type));
                                        let result = resolve_type_variable(var_ty, source);
                                        result
                                    }
                                }
                            }
                            TypeModelKind::Unknown(TypeModel { ty, .. }) => {
                                FFIVariable::mut_ptr(ty)
                            },
                            TypeModelKind::Dictionary(DictTypeModelKind::Primitive(TypeModel { ty, .. })) => {
                                FFIVariable::direct(ty)
                            },
                            TypeModelKind::Dictionary(DictTypeModelKind::NonPrimitiveFermentable(DictFermentableModelKind::I128(..) | DictFermentableModelKind::U128(..))) =>
                                FFIVariable::mut_ptr(parse_quote!([u8; 16])),
                            TypeModelKind::FnPointer(TypeModel { ty, .. }, ..) =>
                                FFIVariable::direct(
                                    Resolve::<SpecialType<RustSpecification>>::maybe_resolve(&ty, source)
                                        .map(|special| special.to_type())
                                        .unwrap_or_else(|| Resolve::<FFIFullPath<RustSpecification>>::resolve(&ty, source)
                                            .to_type())
                                ),
                            TypeModelKind::Dictionary(DictTypeModelKind::LambdaFn(TypeModel { ty, .. }, ..)) => {
                                FFIVariable::mut_ptr(
                                    Resolve::<FFIFullPath<RustSpecification>>::resolve(&ty, source).to_type())
                            },
                            TypeModelKind::Dictionary(
                                DictTypeModelKind::NonPrimitiveFermentable(
                                    DictFermentableModelKind::SmartPointer(
                                        SmartPointerModelKind::Arc(TypeModel { ty, .. }) |
                                        SmartPointerModelKind::Rc(TypeModel { ty, .. }) |
                                        SmartPointerModelKind::Mutex(TypeModel { ty, .. }) |
                                        SmartPointerModelKind::OnceLock(TypeModel { ty, .. }) |
                                        SmartPointerModelKind::RwLock(TypeModel { ty, .. }) |
                                        SmartPointerModelKind::Cell(TypeModel { ty, .. }) |
                                        SmartPointerModelKind::RefCell(TypeModel { ty, .. }) |
                                        SmartPointerModelKind::UnsafeCell(TypeModel { ty, .. }) |
                                        SmartPointerModelKind::Pin(TypeModel { ty, .. })
                                    ) |
                                    DictFermentableModelKind::Group(
                                        GroupModelKind::BTreeSet(TypeModel { ty, .. }) |
                                        GroupModelKind::HashSet(TypeModel { ty, .. }) |
                                        GroupModelKind::Map(TypeModel { ty, .. }) |
                                        GroupModelKind::Result(TypeModel { ty, .. }) |
                                        GroupModelKind::Vec(TypeModel { ty, .. }) |
                                        GroupModelKind::IndexMap(TypeModel { ty, .. }) |
                                        GroupModelKind::IndexSet(TypeModel { ty, .. })
                                    ) |
                                    DictFermentableModelKind::Other(TypeModel { ty, .. }) |
                                    DictFermentableModelKind::Str(TypeModel { ty, .. }) |
                                    DictFermentableModelKind::String(TypeModel { ty, .. }))) => {
                                // Dictionary generics and strings should be fermented
                                // Others should be treated as opaque
                                let maybe_ffi_full_path: Option<FFIFullPath<RustSpecification>> = ty.maybe_resolve(source);
                                let result = resolve_type_variable(maybe_ffi_full_path.map(|path| path.to_type()).unwrap_or_else(|| parse_quote!(#ty)), source);
                                result
                            },
                            TypeModelKind::Dictionary(DictTypeModelKind::NonPrimitiveOpaque(..)) => {
                                // Dictionary generics should be fermented
                                // Others should be treated as opaque
                                let result = Resolve::<FFIVariable<RustSpecification, Type>>::resolve(&conversion, source);
                                result
                            },
                            TypeModelKind::Bounds(bounds) => {
                                bounds.resolve(source)
                            }

                            ref cnv => {
                                if cnv.is_optional() {
                                    let nested_ty = full_ty.maybe_first_nested_type_kind().unwrap();
                                    let maybe_special = <Type as FFISpecialTypeResolve<RustSpecification>>::maybe_special_type(&nested_ty.to_type(), source);
                                    match maybe_special {
                                        Some(SpecialType::Custom(custom_ty)) => {
                                            return FFIVariable::mut_ptr(custom_ty.to_type());
                                        },
                                        _ => {}
                                    }
                                }
                                let object = Resolve::<ObjectKind>::maybe_resolve(&self.ty, source);
                                let var_ty = match object {
                                    Some(ObjectKind::Item(.., ScopeItemKind::Fn(..))) => {
                                        let parent_object = &source.scope.parent_object().unwrap();
                                        match parent_object {
                                            ObjectKind::Type(ref ty_conversion) |
                                            ObjectKind::Item(ref ty_conversion, ..) => {
                                                match ty_conversion {
                                                    TypeModelKind::Trait(ty, _decomposition, _super_bounds) => {
                                                        ty.as_type().maybe_trait_object_maybe_model_kind(source)
                                                    },
                                                    _ => {
                                                        None
                                                    },
                                                }.unwrap_or_else(|| {
                                                    parent_object.maybe_type_model_kind_ref().cloned()
                                                })
                                            },
                                            ObjectKind::Empty => {
                                                None
                                            }
                                        }
                                    },
                                    Some(ObjectKind::Type(..) |
                                         ObjectKind::Item(..)) => {
                                        match &cnv {
                                            TypeModelKind::Trait(ty, _decomposition, _super_bounds) => {
                                                ty.as_type().maybe_trait_object_maybe_model_kind(source)
                                            },
                                            _ => {
                                                None
                                            },
                                        }.unwrap_or_else(|| {
                                            Some(cnv.clone())
                                        })

                                    },
                                    _ => None,
                                }.unwrap_or_else(|| {
                                    cnv.clone()
                                });
                                let var_c_type = var_ty.to_type();
                                let ffi_path: Option<FFIFullPath<RustSpecification>> = var_c_type.maybe_resolve(source);
                                let var_ty = ffi_path.map(|p| p.to_type()).unwrap_or_else(|| parse_quote!(#var_c_type));
                                let result = resolve_type_variable(var_ty, source);
                                result
                            }
                        }
                    }
                    _ => {
                        source.maybe_special_or_regular_ffi_full_path::<RustSpecification>(&self.ty)
                            .map(|ffi_path| ffi_path.to_type())
                            .unwrap_or_else(|| self.ty.clone())
                            .resolve(source)
                    }
                }
            }
        };
        result
    }
}

