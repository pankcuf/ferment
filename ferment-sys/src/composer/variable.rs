use std::fmt::Debug;
use std::marker::PhantomData;
use quote::ToTokens;
use syn::{parse_quote, Type, TypePtr};
use crate::composable::TypeModel;
use crate::composer::SourceComposable;
use crate::context::{ScopeChain, ScopeContext, ScopeSearch, ScopeSearchKey};
use crate::conversion::{DictFermentableModelKind, DictTypeModelKind, GroupModelKind, ObjectKind, ScopeItemKind, SmartPointerModelKind, TypeModelKind};
use crate::ext::{AsType, FFISpecialTypeResolve, GenericNestedArg, Resolve, ResolveTrait, SpecialType, ToType};
use crate::lang::{LangFermentable, RustSpecification, Specification};
use crate::presentation::{resolve_type_variable, FFIFullPath, FFIVariable, RustFermentate};

pub trait VarComposable<LANG, SPEC>: Clone + Debug + ToTokens + ToType
    where LANG: LangFermentable,
          SPEC: Specification<LANG> {}
impl<LANG, SPEC, T> VarComposable<LANG, SPEC> for FFIVariable<LANG, SPEC, T>
    where Self: ToTokens + ToType,
          T: Clone + Debug + ToTokens,
          LANG: LangFermentable,
          SPEC: Specification<LANG> {}

#[derive(Clone, Debug)]
pub struct VarComposer<'a, LANG, SPEC>
    where LANG: LangFermentable,
          SPEC: Specification<LANG> {
    pub search: ScopeSearch<'a>,
    _marker: PhantomData<(LANG, SPEC)>,
}

impl<'a, LANG, SPEC> VarComposer<'a, LANG, SPEC>
    where LANG: LangFermentable,
          SPEC: Specification<LANG> {
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

#[derive(Clone, Debug)]
pub struct TargetVarComposer<'a, LANG, SPEC>
where LANG: LangFermentable,
      SPEC: Specification<LANG> {
    pub search: ScopeSearch<'a>,
    _marker: PhantomData<(LANG, SPEC)>,
}

impl<'a, LANG, SPEC> TargetVarComposer<'a, LANG, SPEC>
    where LANG: LangFermentable,
          SPEC: Specification<LANG> {
    pub fn new(search: ScopeSearch<'a>) -> Self {
        Self { search, _marker: PhantomData }
    }
    #[allow(unused)]
    pub fn key_in_scope(ty: &'a Type, scope: &'a ScopeChain) -> Self {
        Self::new(ScopeSearch::KeyInScope(ScopeSearchKey::maybe_from_ref(ty).unwrap(), scope))
    }

    pub fn value(ty: &'a Type) -> Self {
        Self::new(ScopeSearch::Value(ScopeSearchKey::maybe_from_ref(ty).unwrap()))
    }
}

impl<'a, SPEC> SourceComposable for VarComposer<'a, RustFermentate, SPEC>
    where SPEC: RustSpecification {
    type Source = ScopeContext;
    type Output = SPEC::Var;

    fn compose(&self, source: &Self::Source) -> Self::Output {
        let search_key = self.search.search_key();
        let ptr_composer = search_key.ptr_composer();
        let maybe_obj = source.maybe_object_by_predicate_ref(&self.search);
        let full_ty = maybe_obj
            .as_ref()
            .and_then(ObjectKind::maybe_type)
            .unwrap_or(search_key.to_type());
        println!("VarComposer:: {} --- {} --- {}", self.search, full_ty.to_token_stream(), maybe_obj.as_ref().map_or("None".to_string(), ObjectKind::to_string));
        let maybe_special = Resolve::<SpecialType<RustFermentate, SPEC>>::maybe_resolve(&full_ty, source);
        println!("VarComposer:: (Maybe Special?) {}", maybe_special.as_ref().map_or("None".to_string(), |o| format!("{o}")));
        let result = match maybe_special {
            Some(special) => match maybe_obj {
                Some(ObjectKind::Item(_fn_ty_conversion, ScopeItemKind::Fn(..))) =>
                    ptr_composer(source.maybe_to_fn_type().unwrap_or(search_key.to_type())),
                Some(ObjectKind::Item(TypeModelKind::FnPointer(..), ..) |
                     ObjectKind::Type(TypeModelKind::FnPointer(..), ..)) =>
                    FFIVariable::direct(special.to_type()),
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
                println!("VarComposer (NonSpecial): {} in {} ({:?})", full_ty.to_token_stream(), source.scope.fmt_short(), maybe_obj);
                match maybe_obj {
                    Some(ObjectKind::Item(_fn_ty_conversion, ScopeItemKind::Fn(..))) => {
                        let ty = source.maybe_to_trait_fn_type::<RustFermentate, SPEC>()
                            .unwrap_or(search_key.to_type());
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
                        }.unwrap_or(ty_model_kind.clone());
                        match conversion {
                            // TypeModelKind::Optional(_) |
                            TypeModelKind::Dictionary(DictTypeModelKind::NonPrimitiveFermentable(DictFermentableModelKind::SmartPointer(SmartPointerModelKind::Box(model)))) => {
                                // println!("VariableComposer (Boxed conversion): {}", conversion);
                                // let nested_ty = ty.first_nested_type().unwrap();
                                let ty = model.as_type();
                                // full_ty.first_nested_type()

                                // let nested_ty = self.ty.first_nested_type().unwrap();
                                let full_nested_ty = ty.maybe_first_nested_type_ref().unwrap();
                                match Resolve::<SpecialType<RustFermentate, SPEC>>::maybe_resolve(full_nested_ty, source) {
                                    Some(special) => {
                                        // println!("VariableComposer (Special Boxed conversion): Nested Type: {}", special.to_token_stream());
                                        match source.maybe_object_by_value(full_nested_ty) {
                                            Some(ObjectKind::Item(TypeModelKind::FnPointer(..), ..) |
                                                 ObjectKind::Type(TypeModelKind::FnPointer(..), ..)) => {
                                                // println!("VariableComposer (Special Boxed conversion): Nested Special FnPointer: {}", nested_ty.to_token_stream());
                                                FFIVariable::direct(special.to_type())
                                            }
                                            Some(ObjectKind::Item(TypeModelKind::Trait(..), ..) |
                                                 ObjectKind::Type(TypeModelKind::TraitType(..), ..) |
                                                 ObjectKind::Type(TypeModelKind::Dictionary(DictTypeModelKind::LambdaFn(..)), ..)) => {
                                                // println!("VariableComposer (Special Boxed conversion): Nested Special Trait: {}", nested_ty.to_token_stream());
                                                let ty = special.to_type();
                                                ptr_composer(parse_quote!(dyn #ty))
                                            },
                                            _ => {
                                                // println!("VariableComposer (Boxed conversion): Nested Special MutPtr: {}", nested_ty.to_token_stream());
                                                ptr_composer(special.to_type())
                                            }
                                        }
                                    }
                                    None => {
                                        // println!("VariableComposer (Nested Boxed ty): {}", nested_ty.to_token_stream());
                                        // let nested_conversion = Resolve::<TypeModelKind>::resolve(nested_ty, source);
                                        // // println!("VariableComposer (Nested Boxed conversion): {}", nested_conversion);
                                        // let result = Resolve::<FFIVariable>::resolve(&nested_conversion, source);
                                        // println!("VariableComposer (Nested Boxed variable): {}", result.to_token_stream());



                                        // let conversion_ty = conversion.ty();
                                        let object = source.maybe_object_by_value(full_nested_ty);
                                        // let object = Resolve::<Option<ObjectKind>>::resolve(nested_ty, source);
                                        // println!("VarComposer (Nested Boxed Type Conversion (Object?)): {}", object.as_ref().map_or("None".to_string(), |o| format!("{}", o)));
                                        let ty_model_kind = match object {
                                            Some(ObjectKind::Item(.., ScopeItemKind::Fn(..))) =>
                                                source.maybe_trait_or_regular_model_kind(),
                                            Some(ObjectKind::Type(ref kind) |
                                                 ObjectKind::Item(ref kind, ..)) =>
                                                kind.maybe_trait_model_kind_or_same(source),
                                            _ => None,
                                        }.unwrap_or(TypeModelKind::unknown_type_ref(full_nested_ty));
                                        let var_c_type = ty_model_kind.to_type();
                                        let ffi_path: Option<FFIFullPath<RustFermentate, SPEC>> = var_c_type.maybe_resolve(source);
                                        let var_ty = ffi_path.map(|p| p.to_type()).unwrap_or(parse_quote!(#var_c_type));
                                        let result = resolve_type_variable(var_ty, source);

                                        // let result = resolve_type_variable(var_ty.to_type(), source);

                                        result
                                    }
                                }
                            },
                            TypeModelKind::Unknown(TypeModel { ty, .. }) => {
                                // println!("VarComposer (Unknown): {}", ty.to_token_stream());
                                FFIVariable::mut_ptr(ty)
                            },
                            TypeModelKind::Dictionary(DictTypeModelKind::Primitive(TypeModel { ty, .. })) => {
                                // println!("VarComposer (Dictionary Primitive): {}", ty.to_token_stream());
                                FFIVariable::direct(ty)
                            },
                            TypeModelKind::Dictionary(DictTypeModelKind::NonPrimitiveFermentable(DictFermentableModelKind::I128(..))) =>
                                FFIVariable::mut_ptr(parse_quote!([u8; 16])),
                            TypeModelKind::Dictionary(DictTypeModelKind::NonPrimitiveFermentable(DictFermentableModelKind::U128(..))) =>
                                FFIVariable::mut_ptr(parse_quote!([u8; 16])),
                            TypeModelKind::FnPointer(TypeModel { ty, .. }, ..) => {
                                // println!("VarComposer (FnPointer Conversion): {}", ty.to_token_stream());
                                let result = FFIVariable::direct(
                                    Resolve::<SpecialType<RustFermentate, SPEC>>::maybe_resolve(&ty, source)
                                        .map(|special| special.to_type())
                                        .unwrap_or(Resolve::<FFIFullPath<RustFermentate, SPEC>>::resolve(&ty, source)
                                            .to_type())
                                );
                                // println!("VarComposer (FnPointer Variable): {}", result.to_token_stream());
                                result
                            },
                            TypeModelKind::Dictionary(DictTypeModelKind::LambdaFn(TypeModel { ty, .. }, ..)) => {
                                // println!("VarComposer (LambdaFn Conversion): {}", ty.to_token_stream());
                                let result = FFIVariable::mut_ptr(Resolve::<FFIFullPath<RustFermentate, SPEC>>::resolve(&ty, source).to_type());
                                // println!("VarComposer (LambdaFn Variable): {}", result.to_token_stream());
                                result
                            },
                            // TypeModelKind::Dictionary(DictTypeModelKind::NonPrimitiveFermentable(TypeModel { ty, .. })) => {
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
                                    // DictFermentableModelKind::I128(TypeModel { ty, .. }) |
                                    // DictFermentableModelKind::U128(TypeModel { ty, .. }) |
                                    DictFermentableModelKind::Other(TypeModel { ty, .. }) |
                                    DictFermentableModelKind::Str(TypeModel { ty, .. }) |
                                    DictFermentableModelKind::String(TypeModel { ty, .. }))) => {
                                // Dictionary generics and strings should be fermented
                                // Others should be treated as opaque
                                // println!("VarComposer (Dictionary NonPrimitiveFermentable Conversion): {}", ty.to_token_stream());
                                let maybe_ffi_full_path: Option<FFIFullPath<RustFermentate, SPEC>> = ty.maybe_resolve(source);
                                // println!("VarComposer (Dictionary NonPrimitiveFermentable Conversion FFIFULLPATH?): {}", maybe_ffi_full_path.to_token_stream());
                                let result = resolve_type_variable(maybe_ffi_full_path.map(|path| path.to_type()).unwrap_or(parse_quote!(#ty)), source);
                                // println!("VarComposer (Dictionary NonPrimitiveFermentable Variable): {}", result.to_token_stream());
                                result
                            },
                            TypeModelKind::Dictionary(DictTypeModelKind::NonPrimitiveOpaque(..)) => {
                                // Dictionary generics should be fermented
                                // Others should be treated as opaque
                                // println!("VarComposer (Dictionary NonPrimitiveOpaque Conversion): {}", conversion.to_token_stream());
                                let result: FFIVariable<RustFermentate, SPEC, Type> = conversion.resolve(source);
                                // println!("VarComposer (Dictionary NonPrimitiveOpaque Variable): {}", result.to_token_stream());
                                result
                            },
                            TypeModelKind::Bounds(bounds) => {
                                bounds.resolve(source)
                            },

                            ref cnv => {

                                if cnv.is_optional() {
                                    let nested_ty = full_ty.maybe_first_nested_type_kind().unwrap();
                                    let maybe_special = <Type as FFISpecialTypeResolve<RustFermentate, SPEC>>::maybe_special_type(&nested_ty.to_type(), source);
                                    match maybe_special {
                                        Some(SpecialType::Custom(custom_ty)) => {
                                            return FFIVariable::mut_ptr(custom_ty.to_type());
                                        },
                                        _ => {}
                                    }
                                }

                                // println!("VarComposer (Regular Fermentable Conversion): {}", conversion);
                                // let result: FFIVariable = conversion.resolve(source);
                                // let conversion_ty = conversion.ty();
                                // let object = Resolve::<Option<ObjectKind>>::resolve(&self.ty, source);
                                // println!("VarComposer (Regular Fermentable Conversion (Object?)): {}", maybe_obj.as_ref().map_or("None".to_string(), |o| format!("{}", o)));
                                let var_ty = match maybe_obj {
                                    Some(ObjectKind::Item(.., ScopeItemKind::Fn(..))) => {
                                        let parent_object = &source.scope.parent_object().unwrap();
                                        match parent_object {
                                            ObjectKind::Type(ref ty_conversion) |
                                            ObjectKind::Item(ref ty_conversion, ..) => {
                                                match ty_conversion {
                                                    TypeModelKind::Trait(ty, _decomposition, _super_bounds) => {
                                                        // println!("VarComposer (Regular Fermentable Trait Fn Conversion): {}", conversion);
                                                        ty.as_type().maybe_trait_object_maybe_model_kind(source)
                                                    },
                                                    _ => {
                                                        None
                                                    },
                                                }.unwrap_or_else(|| {
                                                    // println!("VarComposer (Regular Fermentable Non-Trait Fn Conversion): {}", conversion);
                                                    parent_object.maybe_type_model_kind_ref().cloned()
                                                })
                                            },
                                            ObjectKind::Empty => {
                                                // println!("Type::<TypeModelKind> Has no object2 --> {}", parent_object.type_conversion().to_token_stream());
                                                None
                                            }
                                        }
                                    },
                                    Some(ObjectKind::Type(..) |
                                         ObjectKind::Item(..)) => {
                                        // cnv
                                        match &cnv {
                                            TypeModelKind::Trait(ty, _decomposition, _super_bounds) => {
                                                // println!("VarComposer (Regular Fermentable Trait Conversion): {}", conversion);
                                                ty.as_type().maybe_trait_object_maybe_model_kind(source)
                                            },
                                            // TypeModelKind::Bounds(bounds) =>
                                            //     bounds.resolve(source),

                                            _ => {
                                                // println!("VariableComposer (Regular Fermentable Non-Trait Conversion): {}", conversion);
                                                None
                                            },
                                        }.unwrap_or_else(|| {
                                            // println!("VarComposer (Regular Fermentable Non Trait Conversion): {}", cnv);
                                            Some(cnv.clone())
                                        })

                                    },
                                    _ => None,
                                }.unwrap_or_else(|| {
                                    // println!("VarComposer (Regular Fermentable Unknown Conversion): {}", cnv);
                                    cnv.clone()
                                    // TypeModelKind::Unknown(TypeComposition::new(conversion_ty.clone(), None, Punctuated::new()))
                                });
                                println!("VarComposer (Regular Fermentable Conversion): {}", var_ty.to_token_stream());
                                let var_c_type = var_ty.to_type();
                                let ffi_path: Option<FFIFullPath<RustFermentate, SPEC>> = var_c_type.maybe_resolve(source);
                                let var_ty = ffi_path.map(|p| p.to_type()).unwrap_or(parse_quote!(#var_c_type));
                                let result = resolve_type_variable(var_ty, source);

                                // println!("VarComposer (Regular Fermentable Variable): {}", result.to_token_stream());
                                result
                            }
                        }
                    },

                    _ => {
                        let maybe_special: Option<SpecialType<RustFermentate, SPEC>> = ScopeSearchKey::maybe_resolve(search_key, source);
                        maybe_special
                            .map(FFIFullPath::from)
                            .or_else(|| Resolve::<TypeModelKind>::resolve(search_key, source)
                                .to_type()
                                .maybe_resolve(source))
                            .map(|ffi_path| ffi_path.to_type())
                            .unwrap_or(search_key.to_type())
                            .resolve(source)
                    }
                }
            }
        };
        println!("VarComposer (compose) RESULT: {}", result.to_token_stream());
        result
    }
}

#[derive(Clone, Debug)]
pub struct VariableComposer<LANG, SPEC>
    where LANG: LangFermentable,
          SPEC: Specification<LANG> {
    pub ty: Type,
    _marker: PhantomData<(LANG, SPEC)>
}

impl<LANG, SPEC> VariableComposer<LANG, SPEC>
    where LANG: LangFermentable,
          SPEC: Specification<LANG> {
    pub fn new(ty: Type) -> Self {
        Self { ty, _marker: PhantomData }
    }
}
impl<LANG, SPEC> From<&Type> for VariableComposer<LANG, SPEC>
    where LANG: LangFermentable,
          SPEC: Specification<LANG> {
    fn from(value: &Type) -> Self {
        Self { ty: value.clone(), _marker: PhantomData }
    }
}

impl<SPEC> SourceComposable for VariableComposer<RustFermentate, SPEC>
    where SPEC: RustSpecification {
    type Source = ScopeContext;
    type Output = FFIVariable<RustFermentate, SPEC, Type>;

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

        let full_ty: Type = Resolve::resolve(&self.ty, source);
        // println!("VariableComposer (compose): {} ({}) in {}", self.ty.to_token_stream(), full_ty.to_token_stream(), source.scope.fmt_short());

        let maybe_obj = source.maybe_object_by_predicate(ScopeSearch::KeyInScope(ScopeSearchKey::TypeRef(&self.ty, None), &source.scope));
        let result = match Resolve::<SpecialType<RustFermentate, SPEC>>::maybe_resolve(&full_ty, source) {
            Some(special) => match maybe_obj {
                Some(ObjectKind::Item(_fn_ty_conversion, ScopeItemKind::Fn(..))) => {
                    // println!("VariableComposer (Special Function): {} in {}", fn_ty_conversion.to_token_stream(), source.scope.fmt_short());
                    let ty = match &source.scope.parent_object().unwrap() {
                        ObjectKind::Type(ref ty_model_kind) |
                        ObjectKind::Item(ref ty_model_kind, ..) => {
                            let parent_scope = source.scope.parent_scope().unwrap();
                            // println!("VariableComposer (Special Function Parent TYC): {} in {}", ty_model_kind, parent_scope.fmt_short());
                            let context = source.context.read().unwrap();
                            context.maybe_scope_ref_obj_first(parent_scope.self_path())
                                .and_then(|parent_obj_scope| {
                                    // println!("VariableComposer (Special Function Parent OBJ SCOPE): {}", parent_obj_scope.fmt_short());
                                    context.maybe_object_ref_by_tree_key(ty_model_kind.as_type(), parent_obj_scope)
                                        .and_then(|o| {
                                            // println!("VariableComposer (Special Function Parent OBJ FULL): {} in {}", o, o.maybe_type().to_token_stream());
                                            o.maybe_type()
                                        })
                                }).unwrap_or(parent_scope.to_type())
                        },
                        _ => {
                            // println!("VariableComposer (Special Function Unknown TYC): {} in {}", self.ty.to_token_stream(), source.scope.fmt_short());
                            self.ty.clone()
                        }
                    };
                    ptr_composer(ty)
                }
                Some(ObjectKind::Item(TypeModelKind::FnPointer(..), ..) |
                     ObjectKind::Type(TypeModelKind::FnPointer(..), ..)) => {
                    // println!("VariableComposer (Special FnPointer): {}", special.to_token_stream());
                    FFIVariable::direct(special.to_type())
                }
                Some(ObjectKind::Item(TypeModelKind::Trait(..), ..) |
                     ObjectKind::Type(TypeModelKind::TraitType(..), ..) |
                     ObjectKind::Type(TypeModelKind::Dictionary(DictTypeModelKind::LambdaFn(..)), ..)) => {
                    // println!("VariableComposer (Special Trait): {}", special.to_token_stream());
                    let ty = special.to_type();
                    ptr_composer(parse_quote!(dyn #ty))
                },
                Some(ObjectKind::Type(TypeModelKind::Bounds(bounds))) => {
                    // println!("VariableComposer (Bounds): {}", bounds);
                    bounds.resolve(source)
                },
                _ => {
                    // println!("VariableComposer (Special MutPtr): {}", special.to_token_stream());
                    ptr_composer(special.to_type())
                }
            }
            None => {
                // println!("VariableComposer (NonSpecial): {} in {}", full_ty.to_token_stream(), source.scope.fmt_short());
                match maybe_obj {
                    Some(ObjectKind::Item(_fn_ty_conversion, ScopeItemKind::Fn(..))) => {
                        // println!("VariableComposer (Function): {} in {}", fn_ty_conversion.to_token_stream(), source.scope.fmt_short());
                        let ty = match &source.scope.parent_object().unwrap() {
                            ObjectKind::Type(ref ty_conversion) |
                            ObjectKind::Item(ref ty_conversion, ..) => {
                                let full_parent_ty: Type = Resolve::resolve(ty_conversion.as_type(), source);
                                // println!("VariableComposer (Function Parent): {} ({}) in {}", ty_conversion.to_token_stream(), full_parent_ty.to_token_stream(), source.scope.fmt_short());
                                match Resolve::<SpecialType<RustFermentate, SPEC>>::maybe_resolve(&full_parent_ty, source) {
                                    Some(special) => special.to_type(),
                                    None => {
                                        match ty_conversion {
                                            TypeModelKind::Trait(ty, _decomposition, _super_bounds) =>
                                                ty.as_type()
                                                    .maybe_trait_object(source)
                                                    .and_then(|oc| oc.maybe_type_model_kind_ref().map(|c| c.to_type()))
                                                    .unwrap_or(ty_conversion.to_type()),
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
                        }.unwrap_or(ty_model_kind.clone());
                        match conversion {
                            // TypeModelKind::Optional(_) |
                            TypeModelKind::Dictionary(DictTypeModelKind::NonPrimitiveFermentable(DictFermentableModelKind::SmartPointer(SmartPointerModelKind::Box(model)))) => {
                                // println!("VariableComposer (Boxed conversion): {}", conversion);
                                // let nested_ty = ty.first_nested_type().unwrap();
                                let ty = model.as_type();
                                let nested_ty = self.ty.maybe_first_nested_type_ref().unwrap();
                                let full_nested_ty = ty.maybe_first_nested_type_ref().unwrap();
                                match Resolve::<SpecialType<RustFermentate, SPEC>>::maybe_resolve(full_nested_ty, source) {
                                    Some(special) => {
                                        // println!("VariableComposer (Special Boxed conversion): Nested Type: {}", special.to_token_stream());
                                        match source.maybe_object_by_key(nested_ty) {
                                            Some(ObjectKind::Item(TypeModelKind::FnPointer(..), ..) |
                                                 ObjectKind::Type(TypeModelKind::FnPointer(..), ..)) => {
                                                // println!("VariableComposer (Special Boxed conversion): Nested Special FnPointer: {}", nested_ty.to_token_stream());
                                                FFIVariable::direct(special.to_type())
                                            }
                                            Some(ObjectKind::Item(TypeModelKind::Trait(..), ..) |
                                                 ObjectKind::Type(TypeModelKind::TraitType(..), ..) |
                                                 ObjectKind::Type(TypeModelKind::Dictionary(DictTypeModelKind::LambdaFn(..)), ..)) => {
                                                // println!("VariableComposer (Special Boxed conversion): Nested Special Trait: {}", nested_ty.to_token_stream());
                                                let ty = special.to_type();
                                                ptr_composer(parse_quote!(dyn #ty))
                                            },
                                            _ => {
                                                // println!("VariableComposer (Boxed conversion): Nested Special MutPtr: {}", nested_ty.to_token_stream());
                                                let ty = special.to_type();
                                                ptr_composer(ty)
                                            }
                                        }
                                    }
                                    None => {
                                        // println!("VariableComposer (Nested Boxed ty): {}", nested_ty.to_token_stream());
                                        // let nested_conversion = Resolve::<TypeModelKind>::resolve(nested_ty, source);
                                        // // println!("VariableComposer (Nested Boxed conversion): {}", nested_conversion);
                                        // let result = Resolve::<FFIVariable>::resolve(&nested_conversion, source);
                                        // println!("VariableComposer (Nested Boxed variable): {}", result.to_token_stream());



                                        // let conversion_ty = conversion.ty();
                                        let object = Resolve::<ObjectKind>::maybe_resolve(nested_ty, source);
                                        // println!("VariableComposer (Nested Boxed Type Conversion (Object?)): {}", object.as_ref().map_or("None".to_string(), |o| format!("{}", o)));
                                        let ty_model_kind = match object {
                                            Some(ObjectKind::Item(.., ScopeItemKind::Fn(..))) =>
                                                source.maybe_trait_or_regular_model_kind(),
                                            Some(ObjectKind::Type(ref kind) |
                                                 ObjectKind::Item(ref kind, ..)) =>
                                                kind.maybe_trait_model_kind_or_same(source),
                                            _ => None,
                                        }.unwrap_or(TypeModelKind::unknown_type_ref(nested_ty));
                                        let var_c_type = ty_model_kind.to_type();
                                        let ffi_path: Option<FFIFullPath<RustFermentate, SPEC>> = var_c_type.maybe_resolve(source);
                                        let var_ty = ffi_path.map(|p| p.to_type()).unwrap_or(parse_quote!(#var_c_type));
                                        let result = resolve_type_variable(var_ty, source);

                                        // let result = resolve_type_variable(var_ty.to_type(), source);

                                        result
                                    }
                                }
                            }
                            TypeModelKind::Unknown(TypeModel { ty, .. }) => {
                                // println!("VariableComposer (Unknown): {}", ty.to_token_stream());
                                FFIVariable::mut_ptr(ty)
                            },
                            TypeModelKind::Dictionary(DictTypeModelKind::Primitive(TypeModel { ty, .. })) => {
                                // println!("VariableComposer (Dictionary Primitive): {}", ty.to_token_stream());
                                FFIVariable::direct(ty)
                            },
                            TypeModelKind::Dictionary(DictTypeModelKind::NonPrimitiveFermentable(DictFermentableModelKind::I128(..))) =>
                                FFIVariable::mut_ptr(parse_quote!([u8; 16])),
                            TypeModelKind::Dictionary(DictTypeModelKind::NonPrimitiveFermentable(DictFermentableModelKind::U128(..))) =>
                                FFIVariable::mut_ptr(parse_quote!([u8; 16])),
                            TypeModelKind::FnPointer(TypeModel { ty, .. }, ..) =>
                                FFIVariable::direct(
                                    Resolve::<SpecialType<RustFermentate, SPEC>>::maybe_resolve(&ty, source)
                                        .map(|special| special.to_type())
                                        .unwrap_or(Resolve::<FFIFullPath<RustFermentate, SPEC>>::resolve(&ty, source)
                                            .to_type())
                                ),
                            TypeModelKind::Dictionary(DictTypeModelKind::LambdaFn(TypeModel { ty, .. }, ..)) => {
                                // println!("VariableComposer (LambdaFn Conversion): {}", ty.to_token_stream());
                                let result = FFIVariable::mut_ptr(
                                    Resolve::<FFIFullPath<RustFermentate, SPEC>>::resolve(&ty, source).to_type());
                                // println!("VariableComposer (LambdaFn Variable): {}", result.to_token_stream());
                                result
                            },
                            // TypeModelKind::Dictionary(DictTypeModelKind::NonPrimitiveFermentable(TypeModel { ty, .. })) => {
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
                                    // DictFermentableModelKind::I128(TypeModel { ty, .. }) |
                                    // DictFermentableModelKind::U128(TypeModel { ty, .. }) |
                                    DictFermentableModelKind::Str(TypeModel { ty, .. }) |
                                    DictFermentableModelKind::String(TypeModel { ty, .. }))) => {
                                // Dictionary generics and strings should be fermented
                                // Others should be treated as opaque
                                // println!("VariableComposer (Dictionary NonPrimitiveFermentable Conversion): {}", ty.to_token_stream());
                                let maybe_ffi_full_path: Option<FFIFullPath<RustFermentate, SPEC>> = ty.maybe_resolve(source);
                                // println!("VariableComposer (Dictionary NonPrimitiveFermentable Conversion FFIFULLPATH?): {}", maybe_ffi_full_path.to_token_stream());
                                let result = resolve_type_variable(maybe_ffi_full_path.map(|path| path.to_type()).unwrap_or(parse_quote!(#ty)), source);
                                // println!("VariableComposer (Dictionary NonPrimitiveFermentable Variable): {}", result.to_token_stream());
                                result
                            },
                            TypeModelKind::Dictionary(DictTypeModelKind::NonPrimitiveOpaque(..)) => {
                                // Dictionary generics should be fermented
                                // Others should be treated as opaque
                                // println!("VariableComposer (Dictionary NonPrimitiveOpaque Conversion): {}", conversion.to_token_stream());
                                let result = Resolve::<FFIVariable<RustFermentate, SPEC, Type>>::resolve(&conversion, source);
                                // println!("VariableComposer (Dictionary NonPrimitiveOpaque Variable): {}", result.to_token_stream());
                                result
                            },
                            TypeModelKind::Bounds(bounds) => {
                                bounds.resolve(source)
                            }

                            ref cnv => {
                                if cnv.is_optional() {
                                    let nested_ty = full_ty.maybe_first_nested_type_kind().unwrap();
                                    let maybe_special = <Type as FFISpecialTypeResolve<RustFermentate, SPEC>>::maybe_special_type(&nested_ty.to_type(), source);
                                    match maybe_special {
                                        Some(SpecialType::Custom(custom_ty)) => {
                                            return FFIVariable::mut_ptr(custom_ty.to_type());
                                        },
                                        _ => {}
                                    }
                                }

                                // println!("VariableComposer (Regular Fermentable Conversion): {}", conversion);
                                // let result: FFIVariable = conversion.resolve(source);
                                // let conversion_ty = conversion.ty();
                                let object = Resolve::<ObjectKind>::maybe_resolve(&self.ty, source);
                                // println!("VariableComposer (Regular Fermentable Conversion (Object?)): {}", object.as_ref().map_or("None".to_string(), |o| format!("{}", o)));
                                let var_ty = match object {
                                    Some(ObjectKind::Item(.., ScopeItemKind::Fn(..))) => {
                                        let parent_object = &source.scope.parent_object().unwrap();
                                        match parent_object {
                                            ObjectKind::Type(ref ty_conversion) |
                                            ObjectKind::Item(ref ty_conversion, ..) => {
                                                match ty_conversion {
                                                    TypeModelKind::Trait(ty, _decomposition, _super_bounds) => {
                                                        // println!("VariableComposer (Regular Fermentable Trait Fn Conversion): {}", conversion);
                                                        ty.as_type().maybe_trait_object_maybe_model_kind(source)
                                                    },
                                                    _ => {
                                                        None
                                                    },
                                                }.unwrap_or_else(|| {
                                                    // println!("VariableComposer (Regular Fermentable Non-Trait Fn Conversion): {}", conversion);
                                                    parent_object.maybe_type_model_kind_ref().cloned()
                                                })
                                            },
                                            ObjectKind::Empty => {
                                                // println!("Type::<TypeModelKind> Has no object2 --> {}", parent_object.type_conversion().to_token_stream());
                                                None
                                            }
                                        }
                                    },
                                    Some(ObjectKind::Type(..) |
                                         ObjectKind::Item(..)) => {
                                        // cnv
                                        match &cnv {
                                            TypeModelKind::Trait(ty, _decomposition, _super_bounds) => {
                                                // println!("VariableComposer (Regular Fermentable Trait Conversion): {}", conversion);
                                                ty.as_type().maybe_trait_object_maybe_model_kind(source)
                                            },
                                            // TypeModelKind::Bounds(bounds) =>
                                            //     bounds.resolve(source),

                                            _ => {
                                                // println!("VariableComposer (Regular Fermentable Non-Trait Conversion): {}", conversion);
                                                None
                                            },
                                        }.unwrap_or_else(|| {
                                            // println!("VariableComposer (Regular Fermentable Non Trait Conversion): {}", cnv);
                                            Some(cnv.clone())
                                        })

                                    },
                                    _ => None,
                                }.unwrap_or_else(|| {
                                    // println!("VariableComposer (Regular Fermentable Unknown Conversion): {}", cnv);
                                    cnv.clone()
                                    // TypeModelKind::Unknown(TypeComposition::new(conversion_ty.clone(), None, Punctuated::new()))
                                });
                                // println!("VariableComposer (Regular Fermentable Conversion): {}", var_ty.to_token_stream());
                                let var_c_type = var_ty.to_type();
                                let ffi_path: Option<FFIFullPath<RustFermentate, SPEC>> = var_c_type.maybe_resolve(source);
                                let var_ty = ffi_path.map(|p| p.to_type()).unwrap_or(parse_quote!(#var_c_type));
                                let result = resolve_type_variable(var_ty, source);

                                // println!("VariableComposer (Regular Fermentable Variable): {}", result.to_token_stream());
                                result
                            }
                        }
                    }
                    _ => {
                        source.maybe_special_or_regular_ffi_full_path::<RustFermentate, SPEC>(&self.ty)
                            .map(|ffi_path| ffi_path.to_type())
                            .unwrap_or(self.ty.clone())
                            .resolve(source)

                    }
                }

                // let conversion = Resolve::<TypeModelKind>::resolve(&self.ty, source);

            }
        };
        // println!("VariableComposer (compose) RESULT: {}", result.to_token_stream());
        result
    }
}

