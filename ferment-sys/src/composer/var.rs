use std::marker::PhantomData;
use syn::{parse_quote, Type};
use crate::composable::TypeModel;
use crate::composer::SourceComposable;
use crate::context::{ScopeChain, ScopeContext, ScopeSearch, ScopeSearchKey};
use crate::conversion::{DictFermentableModelKind, DictTypeModelKind, GroupModelKind, ObjectKind, ScopeItemKind, SmartPointerModelKind, TypeModelKind};
use crate::ext::{AsType, FFISpecialTypeResolve, GenericNestedArg, Resolve, ResolveTrait, SpecialType, ToType};
use crate::lang::{RustSpecification, Specification};
use crate::presentation::{resolve_type_variable, FFIFullPath, FFIVariable, ToFFIVariable};

#[derive(Clone, Debug)]
pub struct VarComposer<'a, SPEC>
    where SPEC: Specification {
    pub search: ScopeSearch<'a>,
    _marker: PhantomData<SPEC>,
}

impl<'a, SPEC> VarComposer<'a, SPEC>
    where SPEC: Specification {
    pub fn new(search: ScopeSearch<'a>) -> Self {
        Self { search, _marker: PhantomData }
    }
    pub fn key_in_scope(ty: &'a Type, scope: &'a ScopeChain) -> Self {
        Self::new(ScopeSearch::KeyInScope(ScopeSearchKey::maybe_from_ref(ty).unwrap(), scope))
    }

    pub fn value(ty: &'a Type) -> Self {
        Self::new(ScopeSearch::Value(ScopeSearchKey::maybe_from_ref(ty).unwrap()))
    }
}

impl<'a> SourceComposable for VarComposer<'a, RustSpecification> {
    type Source = ScopeContext;
    type Output = <RustSpecification as Specification>::Var;

    fn compose(&self, source: &Self::Source) -> Self::Output {
        let search_key = self.search.search_key();
        let ptr_composer = search_key.ptr_composer();
        let maybe_obj = source.maybe_object_by_predicate_ref(&self.search);
        let full_ty = maybe_obj
            .as_ref()
            .and_then(ObjectKind::maybe_type)
            .unwrap_or_else(|| search_key.to_type());
        let maybe_special = Resolve::<SpecialType<RustSpecification>>::maybe_resolve(&full_ty, source);
        let result = match maybe_special {
            Some(special) => match maybe_obj {
                Some(ObjectKind::Item(_fn_ty_conversion, ScopeItemKind::Fn(..))) =>
                    ptr_composer(source.maybe_to_fn_type().unwrap_or_else(|| search_key.to_type())),
                Some(ObjectKind::Item(TypeModelKind::FnPointer(..), ..) |
                     ObjectKind::Type(TypeModelKind::FnPointer(..), ..)) =>
                    special.to_direct_var(),
                Some(ObjectKind::Item(TypeModelKind::Trait(..), ..) |
                     ObjectKind::Type(TypeModelKind::TraitType(..) |
                                      TypeModelKind::Dictionary(DictTypeModelKind::LambdaFn(..)), ..)) => {
                    let ty = special.to_type();
                    ptr_composer(parse_quote!(dyn #ty))
                },
                Some(ObjectKind::Type(TypeModelKind::Bounds(bounds))) =>
                    bounds.resolve(source),
                _ => ptr_composer(special.to_type())
            }
            None => {
                match maybe_obj {
                    Some(ObjectKind::Item(_fn_ty_conversion, ScopeItemKind::Fn(..))) => {
                        let ty = source.maybe_to_trait_fn_type::<RustSpecification>()
                            .unwrap_or_else(|| search_key.to_type());
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
                            TypeModelKind::Dictionary(DictTypeModelKind::NonPrimitiveFermentable(DictFermentableModelKind::SmartPointer(SmartPointerModelKind::Box(model)))) => {
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
                                                 ObjectKind::Type(TypeModelKind::Dictionary(DictTypeModelKind::LambdaFn(..)), ..)) => {
                                                let ty = special.to_type();
                                                ptr_composer(parse_quote!(dyn #ty))
                                            },
                                            _ => {
                                                ptr_composer(special.to_type())
                                            }
                                        }
                                    }
                                    None => {
                                        let object = source.maybe_object_by_value(full_nested_ty);
                                        let ty_model_kind = match object {
                                            Some(ObjectKind::Item(.., ScopeItemKind::Fn(..))) =>
                                                source.maybe_trait_or_regular_model_kind(),
                                            Some(ObjectKind::Type(ref kind) |
                                                 ObjectKind::Item(ref kind, ..)) =>
                                                kind.maybe_trait_model_kind_or_same(source),
                                            _ => None,
                                        }.unwrap_or_else(|| TypeModelKind::unknown_type_ref(full_nested_ty));
                                        let var_c_type = ty_model_kind.to_type();
                                        let ffi_path: Option<FFIFullPath<RustSpecification>> = var_c_type.maybe_resolve(source);
                                        let var_ty = ffi_path.map(|p| p.to_type()).unwrap_or_else(|| parse_quote!(#var_c_type));
                                        let result = resolve_type_variable(var_ty, source);
                                        result
                                    }
                                }
                            },
                            TypeModelKind::Unknown(TypeModel { ty, .. }) => {
                                FFIVariable::mut_ptr(ty)
                            },
                            TypeModelKind::Dictionary(DictTypeModelKind::Primitive(TypeModel { ty, .. })) => {
                                FFIVariable::direct(ty)
                            },
                            TypeModelKind::Dictionary(DictTypeModelKind::NonPrimitiveFermentable(DictFermentableModelKind::I128(..))) =>
                                FFIVariable::mut_ptr(parse_quote!([u8; 16])),
                            TypeModelKind::Dictionary(DictTypeModelKind::NonPrimitiveFermentable(DictFermentableModelKind::U128(..))) =>
                                FFIVariable::mut_ptr(parse_quote!([u8; 16])),
                            TypeModelKind::FnPointer(TypeModel { ty, .. }, ..) => {
                                FFIVariable::direct(
                                    Resolve::<SpecialType<RustSpecification>>::maybe_resolve(&ty, source)
                                        .map(|special| special.to_type())
                                        .unwrap_or_else(|| Resolve::<FFIFullPath<RustSpecification>>::resolve(&ty, source)
                                            .to_type())
                                )
                            },
                            TypeModelKind::Dictionary(DictTypeModelKind::LambdaFn(TypeModel { ty, .. }, ..)) => {
                                FFIVariable::mut_ptr(Resolve::<FFIFullPath<RustSpecification>>::resolve(&ty, source).to_type())
                            },
                            TypeModelKind::Dictionary(
                                DictTypeModelKind::NonPrimitiveFermentable(
                                    DictFermentableModelKind::SmartPointer(
                                        SmartPointerModelKind::Arc(TypeModel { ty, .. }) |
                                        SmartPointerModelKind::Rc(TypeModel { ty, .. }) |
                                        SmartPointerModelKind::Mutex(TypeModel { ty, .. }) |
                                        SmartPointerModelKind::RwLock(TypeModel { ty, .. }) |
                                        SmartPointerModelKind::RefCell(TypeModel { ty, .. }) |
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
                                let result: FFIVariable<RustSpecification, Type> = conversion.resolve(source);
                                result
                            },
                            TypeModelKind::Bounds(bounds) => {
                                bounds.resolve(source)
                            },

                            ref cnv => {

                                if cnv.is_optional() {
                                    let nested_ty = full_ty.maybe_first_nested_type_kind().unwrap();
                                    let maybe_special = <Type as FFISpecialTypeResolve<RustSpecification>>::maybe_special_type(&nested_ty.to_type(), source);
                                    match maybe_special {
                                        Some(SpecialType::Custom(special_ty) | SpecialType::Opaque(special_ty)) => {
                                            return FFIVariable::mut_ptr(special_ty.to_type());
                                        },
                                        _ => {}
                                    }
                                }

                                let var_ty = match maybe_obj {
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
                    },

                    _ => {
                        let maybe_special: Option<SpecialType<RustSpecification>> = ScopeSearchKey::maybe_resolve(search_key, source);
                        maybe_special
                            .map(FFIFullPath::from)
                            .or_else(|| Resolve::<TypeModelKind>::resolve(search_key, source)
                                .to_type()
                                .maybe_resolve(source))
                            .map(|ffi_path| ffi_path.to_type())
                            .unwrap_or_else(|| search_key.to_type())
                            .resolve(source)
                    }
                }
            }
        };
        result
    }
}