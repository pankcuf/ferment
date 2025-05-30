use std::marker::PhantomData;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_quote, Path, TraitBound, Type, TypeArray, TypeImplTrait, TypeParamBound, TypePath, TypePtr, TypeReference, TypeSlice, TypeTraitObject};
use ferment_macro::Display;
use crate::ast::AddPunctuated;
use crate::composable::{GenericBoundsModel, TypeModel};
use crate::context::ScopeContext;
use crate::conversion::{DictFermentableModelKind, DictTypeModelKind, GroupModelKind, ObjectKind, ScopeItemKind, SmartPointerModelKind, TypeKind, TypeModelKind};
use crate::ext::{path_arguments_to_type_conversions, Accessory, AsType, DictionaryType, GenericNestedArg, Mangle, Resolve, ResolveTrait, SpecialType, ToType};
use crate::lang::{LangFermentable, RustSpecification, Specification};
use crate::presentation::{FFIFullDictionaryPath, FFIFullPath, RustFermentate};

#[derive(Clone, Display, Debug)]
pub enum FFIVariable<LANG, SPEC, T>
    where T: ToTokens,
          LANG: LangFermentable,
          SPEC: Specification<LANG> {
    Direct { ty: T, _marker: PhantomData<(LANG, SPEC)> },
    ConstPtr { ty: T, _marker: PhantomData<(LANG, SPEC)> },
    MutPtr { ty: T, _marker: PhantomData<(LANG, SPEC)> },
    Ref { ty: T, _marker: PhantomData<(LANG, SPEC)> },
    MutRef { ty: T, _marker: PhantomData<(LANG, SPEC)> },
    Dyn { ty: T, _marker: PhantomData<(LANG, SPEC)> },
}

impl<LANG, SPEC, T> FFIVariable<LANG, SPEC, T>
    where T: ToTokens,
          LANG: LangFermentable,
          SPEC: Specification<LANG> {
    pub(crate) fn direct(ty: T) -> Self {
        Self::Direct { ty, _marker: PhantomData }
    }
    pub(crate) fn const_ptr(ty: T) -> Self {
        Self::ConstPtr { ty, _marker: PhantomData }
    }
    pub(crate) fn mut_ptr(ty: T) -> Self {
        Self::MutPtr { ty, _marker: PhantomData }
    }
    pub(crate) fn r#ref(ty: T) -> Self {
        Self::Ref { ty, _marker: PhantomData }
    }
    pub(crate) fn mut_ref(ty: T) -> Self {
        Self::MutRef { ty, _marker: PhantomData }
    }
    pub(crate) fn r#dyn(ty: T) -> Self {
        Self::Dyn { ty, _marker: PhantomData }
    }
}

impl<SPEC> ToTokens for FFIVariable<RustFermentate, SPEC, Type>
    where SPEC: RustSpecification {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.to_type().to_tokens(tokens)
    }
}

impl<SPEC> ToType for FFIVariable<RustFermentate, SPEC, Type>
    where SPEC: RustSpecification {
    fn to_type(&self) -> Type {
        match self {
            FFIVariable::Direct { ty, .. } => ty.to_type(),
            FFIVariable::ConstPtr { ty, .. } => ty.joined_const(),
            FFIVariable::MutPtr { ty, .. } => ty.joined_mut(),
            FFIVariable::Ref { ty, .. } => ty.joined_ref(),
            FFIVariable::MutRef { ty, .. } => ty.joined_mut_ref(),
            FFIVariable::Dyn { ty, .. } => ty.joined_dyn()
        }
    }
}

impl<SPEC> Resolve<FFIVariable<RustFermentate, SPEC, Type>> for Path
    where SPEC: RustSpecification {
    fn maybe_resolve(&self, source: &ScopeContext) -> Option<FFIVariable<RustFermentate, SPEC, Type>> {
        Some(self.resolve(source))
    }
    fn resolve(&self, source: &ScopeContext) -> FFIVariable<RustFermentate, SPEC, Type> {
        // println!("Path::<FFIVariable>::resolve({})", self.to_token_stream());
        let first_segment = self.segments.first().unwrap();
        let first_ident = &first_segment.ident;
        let last_segment = self.segments.last().unwrap();
        let last_ident = &last_segment.ident;
        if last_ident.is_primitive() {
            FFIVariable::direct(self.to_type())
        } else if matches!(last_ident.to_string().as_str(), "i128" | "u128") {
            FFIVariable::mut_ptr(parse_quote!([u8; 16]))
        } else if last_ident.is_optional() {
            match path_arguments_to_type_conversions(&last_segment.arguments).first() {
                Some(TypeKind::Primitive(ty)) =>
                    FFIVariable::mut_ptr(ty.clone()),
                Some(TypeKind::Generic(generic_ty)) =>
                    FFIVariable::mut_ptr(Resolve::<FFIFullPath<RustFermentate, SPEC>>::resolve(generic_ty, source).to_type()),
                Some(TypeKind::Complex(Type::Path(TypePath { path, .. }))) =>
                    Resolve::<FFIVariable<RustFermentate, SPEC, Type>>::resolve(path, source),
                _ => unimplemented!("ffi_dictionary_variable_type:: Empty Optional")
            }
        } else if last_ident.is_special_generic() || (last_ident.is_result() /*&& path.segments.len() == 1*/) || (last_ident.to_string().eq("Map") && first_ident.to_string().eq("serde_json")) {
            FFIVariable::mut_ptr(source.scope_type_for_path(self).map_or(self.to_type(), |full_type| full_type.mangle_tokens_default().to_type()))
        } else {
            FFIVariable::mut_ptr(self.to_type())
        }
    }
}

pub fn resolve_type_variable<SPEC>(ty: Type, source: &ScopeContext) -> FFIVariable<RustFermentate, SPEC, Type>
    where SPEC: RustSpecification {
    match ty {
        Type::Path(TypePath { path, .. }) =>
            path.resolve(source),
        Type::Array(TypeArray { elem, len, .. }) => FFIVariable::mut_ptr(parse_quote!([#elem; #len])),
        Type::Reference(TypeReference { elem, .. }) |
        Type::Slice(TypeSlice { elem, .. }) =>
            elem.resolve(source),
        Type::Ptr(TypePtr { star_token, const_token, mutability, elem }) =>
            match *elem {
                Type::Path(TypePath { path, .. }) => match path.segments.last().unwrap().ident.to_string().as_str() {
                    "c_void" => match (star_token, const_token, mutability) {
                        (_, Some(_const_token), None) => FFIVariable::const_ptr(FFIFullDictionaryPath::<RustFermentate, SPEC>::Void.to_type()),
                        (_, None, Some(_mut_token)) => FFIVariable::mut_ptr(FFIFullDictionaryPath::<RustFermentate, SPEC>::Void.to_type()),
                        _ => panic!("Resolve::<FFIVariable>::resolve: c_void with {} {} not supported", quote!(#const_token), quote!(#mutability))
                    },
                    _ => {
                        if const_token.is_some() {
                            FFIVariable::const_ptr(path.to_type())
                        } else {
                            FFIVariable::mut_ptr(path.to_type())
                        }
                    }
                },
                Type::Ptr(..) =>
                    FFIVariable::mut_ptr(elem.to_type()),
                ty => mutability.as_ref()
                    .map_or( FFIVariable::const_ptr(ty.clone()), |_| FFIVariable::mut_ptr(ty.clone()))
            },
        Type::TraitObject(TypeTraitObject { dyn_token: _, bounds, .. }) => {
            bounds.resolve(source)
        }
        Type::ImplTrait(TypeImplTrait { impl_token: _, bounds, .. }) =>
            bounds.resolve(source),
        ty => FFIVariable::direct(ty.mangle_ident_default().to_type())
    }
}

impl<SPEC> Resolve<FFIVariable<RustFermentate, SPEC, Type>> for AddPunctuated<TypeParamBound>
    where SPEC: RustSpecification {
    fn maybe_resolve(&self, source: &ScopeContext) -> Option<FFIVariable<RustFermentate, SPEC, Type>> {
        Some(self.resolve(source))
    }
    fn resolve(&self, source: &ScopeContext) -> FFIVariable<RustFermentate, SPEC, Type> {
        // println!("AddPunctuated<TypeParamBound>::<FFIVariable>::resolve({})", self.to_token_stream());
        let bound = self.iter().find_map(|bound| match bound {
            TypeParamBound::Trait(TraitBound { path, .. }) => Some(path.to_type()),
            TypeParamBound::Lifetime(_) => None
        }).unwrap();
        bound.resolve(source)
    }
}

impl<SPEC> Resolve<FFIVariable<RustFermentate, SPEC, Type>> for Type where SPEC: RustSpecification {
    fn maybe_resolve(&self, source: &ScopeContext) -> Option<FFIVariable<RustFermentate, SPEC, Type>> {
        Some(self.resolve(source))
    }
    fn resolve(&self, source: &ScopeContext) -> FFIVariable<RustFermentate, SPEC, Type> {
        // println!("Type::<FFIVariable>::resolve.1({})", self.to_token_stream());
        let full_ty = Resolve::<Type>::resolve(self, source);
        // println!("Type::<FFIVariable>::resolve.2({})", full_ty.to_token_stream());
        let refined = source.maybe_special_or_regular_ffi_full_path::<RustFermentate, SPEC>(&full_ty)
            .map(|ffi_path| ffi_path.to_type())
            .unwrap_or(parse_quote!(#self))
            .to_type();
        resolve_type_variable(refined, source)
    }
}

impl<SPEC> Resolve<FFIVariable<RustFermentate, SPEC, Type>> for TypeModelKind
    where SPEC: RustSpecification {
    fn maybe_resolve(&self, source: &ScopeContext) -> Option<FFIVariable<RustFermentate, SPEC, Type>> {
        Some(self.resolve(source))
    }
    fn resolve(&self, source: &ScopeContext) -> FFIVariable<RustFermentate, SPEC, Type> {
        println!("TypeModelKind::<FFIVariable>::resolve({}) in {}", self, source.scope.fmt_short());
        let result = match self  {
            // TODO: For now we assume that every callback defined as fn pointer is opaque
            TypeModelKind::FnPointer(TypeModel { ty, .. }, ..) => FFIVariable::direct(Resolve::<SpecialType<RustFermentate, SPEC>>::maybe_resolve(ty, source)
                    .map(|special| special.to_type())
                    .unwrap_or(Resolve::<FFIFullPath<RustFermentate, SPEC>>::resolve(ty, source)
                        .to_type())),
            TypeModelKind::Dictionary(DictTypeModelKind::LambdaFn(TypeModel { ty, .. }, ..)) => FFIVariable::mut_ptr(Resolve::<FFIFullPath<RustFermentate, SPEC>>::resolve(ty, source).to_type()),
            TypeModelKind::Dictionary(DictTypeModelKind::Primitive(composition)) => FFIVariable::direct(composition.to_type()),
            TypeModelKind::Dictionary(
                DictTypeModelKind::NonPrimitiveFermentable(DictFermentableModelKind::I128(..) | DictFermentableModelKind::U128(..))) => {
                FFIVariable::mut_ptr(parse_quote!([u8; 16]))
            },
            TypeModelKind::Dictionary(DictTypeModelKind::NonPrimitiveFermentable(DictFermentableModelKind::SmartPointer(SmartPointerModelKind::Box(TypeModel { ty, .. })))) => {
                // println!("TypeModelKind::Boxed: {}", ty.to_token_stream());
                match ty.maybe_first_nested_type_ref() {
                    Some(nested_full_ty) => {
                        // println!("Nested: {}", nested_full_ty.to_token_stream());
                        resolve_type_variable(match Resolve::<SpecialType<RustFermentate, SPEC>>::maybe_resolve(nested_full_ty, source) {
                            Some(special) => special.to_type(),
                            None => {
                                source.maybe_ffi_full_path::<RustFermentate, SPEC>(nested_full_ty)
                                    .map(|full_path| full_path.to_type())
                                    .unwrap_or_else(|| nested_full_ty.clone())
                            }
                        }, source)
                    }
                    None => panic!("error: Arg conversion ({}) not supported", ty.to_token_stream())
                }
            },

            TypeModelKind::Dictionary(
                DictTypeModelKind::NonPrimitiveFermentable(
                    DictFermentableModelKind::SmartPointer(
                        SmartPointerModelKind::Arc(TypeModel { ty, .. }) |
                        SmartPointerModelKind::Mutex(TypeModel { ty, .. }) |
                        SmartPointerModelKind::Rc(TypeModel { ty, .. }) |
                        SmartPointerModelKind::RefCell(TypeModel { ty, .. }) |
                        SmartPointerModelKind::RwLock(TypeModel { ty, .. }) |
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
                    DictFermentableModelKind::Str(TypeModel { ty, .. }) |
                    DictFermentableModelKind::String(TypeModel { ty, .. }) |
                    DictFermentableModelKind::Other(TypeModel { ty, .. })
                ) |
                DictTypeModelKind::NonPrimitiveOpaque(TypeModel { ty, .. })
            ) |
            TypeModelKind::Trait(TypeModel { ty, .. }, ..) |
            TypeModelKind::TraitType(TypeModel { ty, .. }) |
            TypeModelKind::Object(TypeModel { ty, .. }) |
            TypeModelKind::Optional(TypeModel { ty, .. }) |
            TypeModelKind::Array(TypeModel { ty, .. }) |
            TypeModelKind::Slice(TypeModel { ty, .. }) |
            TypeModelKind::Tuple(TypeModel { ty, .. }) |
            TypeModelKind::Unknown(TypeModel { ty, .. })  => {
                Resolve::<SpecialType<RustFermentate, SPEC>>::maybe_resolve(ty, source)
                    .map(|ty| resolve_type_variable(FFIFullPath::from(ty).to_type(), source))
                    .unwrap_or_else(|| {
                        let maybe_object_kind = Resolve::<ObjectKind>::maybe_resolve(ty, source);
                        let ty_model_kind_to_resolve = maybe_object_kind
                            .and_then(|external_type| {
                                match external_type {
                                    ObjectKind::Item(.., ScopeItemKind::Fn(..)) => {
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
                                    ObjectKind::Type(ref ty_conversion) |
                                    ObjectKind::Item(ref ty_conversion, ..) => {
                                        match ty_conversion {
                                            TypeModelKind::Trait(ty, _decomposition, _super_bounds) => {
                                                // println!("Type::<TypeModelKind> It's a Trait So --> {}", external_type.type_conversion().to_token_stream());
                                                ty.as_type().maybe_trait_object_maybe_model_kind(source)
                                            },
                                            _ => {
                                                None
                                            },
                                        }.unwrap_or_else(|| {
                                            // println!("Type::<TypeModelKind> Not a Trait So --> {}", external_type.type_conversion().to_token_stream());
                                            external_type.maybe_type_model_kind_ref().cloned()
                                        })
                                    },
                                    ObjectKind::Empty => {
                                        // println!("Type::<TypeModelKind> Has no object --> {}", external_type.type_conversion().to_token_stream());
                                        None
                                    }
                                }
                            })
                            .unwrap_or(TypeModelKind::unknown_type_ref(ty));
                        let ty_to_resolve = ty_model_kind_to_resolve.to_type();
                        resolve_type_variable(ty_to_resolve, source)
                    })
            },
            TypeModelKind::Fn(TypeModel { ty, .. }, ..) => {
                // ty.to_path().popped()
                panic!("error: Arg conversion (Fn) ({}) not supported", ty.to_token_stream())
            },

            TypeModelKind::Bounds(bounds) => {
                // println!("TypeModelKind::Bounds: {}", bounds);
                bounds.resolve(source)
            },
            ty =>
                panic!("error: Arg conversion ({}) not supported", ty),
        };
        // println!("TypeModelKind::<FFIVariable>::resolve.2({}) --> {}", self, result.to_token_stream());
        result
    }
}

impl<SPEC> Resolve<FFIVariable<RustFermentate, SPEC, Type>> for GenericBoundsModel where SPEC: RustSpecification {
    fn maybe_resolve(&self, source: &ScopeContext) -> Option<FFIVariable<RustFermentate, SPEC, Type>> {
        Some(self.resolve(source))
    }
    fn resolve(&self, _source: &ScopeContext) -> FFIVariable<RustFermentate, SPEC, Type> {
        let ffi_name = self.mangle_ident_default();
        if self.is_lambda() {
            FFIVariable::direct(parse_quote!(crate::fermented::generics::#ffi_name))

        } else {
            FFIVariable::mut_ptr(parse_quote!(crate::fermented::generics::#ffi_name))
        }
        // println!("GenericBoundsModel::<FFIVariable>::resolve({})", self);
    }
}