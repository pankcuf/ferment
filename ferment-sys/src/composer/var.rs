use std::marker::PhantomData;
use syn::{parse_quote, Type};
use crate::composable::TypeModel;
use crate::composer::SourceComposable;
use crate::context::{ScopeChain, ScopeContext, ScopeSearch, ScopeSearchKey};
use crate::conversion::{DictFermentableModelKind, DictTypeModelKind, GroupModelKind, ObjectKind, ScopeItemKind, SmartPointerModelKind, TypeModelKind};
use crate::ext::{AsType, FFISpecialTypeResolve, GenericNestedArg, Resolve, SpecialType, ToType};
use crate::lang::{RustSpecification, Specification};
use crate::presentation::{resolve_type_variable_via_ffi_full_path, resolve_type_variable_via_maybe_object, resolve_type_variable_via_type, FFIFullPath, FFIVariable, ToFFIVariable};

// Dictionary generics and strings should be fermented
// Others should be treated as opaque

#[derive(Clone, Debug)]
pub struct VarComposer<'a, SPEC>
    where SPEC: Specification {
    pub search: ScopeSearch<'a>,
    _marker: PhantomData<SPEC>,
}

impl<'a, SPEC> VarComposer<'a, SPEC>
    where SPEC: Specification {
    fn new(search: ScopeSearch<'a>) -> Self {
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
                Some(ObjectKind::Item(_, ScopeItemKind::Fn(..))) =>
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
                _ =>
                    ptr_composer(special.to_type())
            }
            None => {
                match maybe_obj {
                    Some(ObjectKind::Item(.., ScopeItemKind::Fn(..))) =>
                        ptr_composer(source.maybe_to_trait_fn_type::<RustSpecification>().unwrap_or_else(|| search_key.to_type())),
                    Some(ObjectKind::Type(ref ty_model_kind)) |
                    Some(ObjectKind::Item(ref ty_model_kind, ..)) => {
                        let conversion = ty_model_kind.maybe_trait_object_maybe_model_kind_or_same(source);

                        match conversion {
                            TypeModelKind::Unknown(TypeModel { ty, .. }) =>
                                FFIVariable::mut_ptr(ty),
                            TypeModelKind::Dictionary(DictTypeModelKind::Primitive(TypeModel { ty, .. })) =>
                                ty.to_direct_var(),
                            TypeModelKind::Dictionary(DictTypeModelKind::NonPrimitiveFermentable(DictFermentableModelKind::I128(..) | DictFermentableModelKind::U128(..))) =>
                                FFIVariable::mut_ptr(parse_quote!([u8; 16])),
                            TypeModelKind::FnPointer(TypeModel { ty, .. }, ..) =>
                                Resolve::<SpecialType<RustSpecification>>::maybe_resolve(&ty, source)
                                    .map(|special| special.to_type())
                                    .unwrap_or_else(|| Resolve::<FFIFullPath<RustSpecification>>::resolve(&ty, source)
                                        .to_type())
                                    .to_direct_var(),
                            TypeModelKind::Dictionary(DictTypeModelKind::LambdaFn(TypeModel { ty, .. }, ..)) =>
                                FFIVariable::mut_ptr(Resolve::<FFIFullPath<RustSpecification>>::resolve(&ty, source).to_type()),
                            TypeModelKind::Dictionary(DictTypeModelKind::NonPrimitiveFermentable(DictFermentableModelKind::SmartPointer(SmartPointerModelKind::Box(model)))) |
                            TypeModelKind::Dictionary(DictTypeModelKind::NonPrimitiveFermentable(DictFermentableModelKind::Cow(model))) => {
                                let ty = model.as_type();
                                let full_nested_ty = ty.maybe_first_nested_type_ref().unwrap();
                                match Resolve::<SpecialType<RustSpecification>>::maybe_resolve(full_nested_ty, source) {
                                    Some(special) => match source.maybe_object_by_value(full_nested_ty) {
                                        Some(ObjectKind::Item(TypeModelKind::FnPointer(..), ..) |
                                             ObjectKind::Type(TypeModelKind::FnPointer(..), ..)) =>
                                            special.to_direct_var(),
                                        Some(ObjectKind::Item(TypeModelKind::Trait(..), ..) |
                                             ObjectKind::Type(TypeModelKind::TraitType(..), ..) |
                                             ObjectKind::Type(TypeModelKind::Dictionary(DictTypeModelKind::LambdaFn(..)), ..)) => {
                                            let ty = special.to_type();
                                            ptr_composer(parse_quote!(dyn #ty))
                                        },
                                        _ => ptr_composer(special.to_type())
                                    }
                                    None => resolve_type_variable_via_maybe_object(source.maybe_object_by_value(full_nested_ty), full_nested_ty, source)
                                }
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
                                resolve_type_variable_via_type(ty, source)
                            },
                            TypeModelKind::Dictionary(DictTypeModelKind::NonPrimitiveOpaque(..)) |
                            TypeModelKind::Bounds(..) =>
                                conversion.resolve(source),

                            ref cnv => {

                                if cnv.is_optional() {
                                    let nested_ty = full_ty.maybe_first_nested_type_kind().unwrap();
                                    match <Type as FFISpecialTypeResolve<RustSpecification>>::maybe_special_type(&nested_ty.to_type(), source) {
                                        Some(SpecialType::Custom(special_ty) | SpecialType::Opaque(special_ty)) => {
                                            return FFIVariable::mut_ptr(special_ty.to_type());
                                        },
                                        _ => {}
                                    }
                                }
                                maybe_obj.and_then(|obj|
                                    obj.maybe_fn_or_trait_or_same_kind(source)
                                        .map(|type_kind| resolve_type_variable_via_ffi_full_path(type_kind, source)))
                                    .unwrap_or_else(|| resolve_type_variable_via_ffi_full_path(cnv.clone(), source))

                            }
                        }
                    },

                    _ => search_key.resolve(source)
                }
            }
        };
        result
    }
}