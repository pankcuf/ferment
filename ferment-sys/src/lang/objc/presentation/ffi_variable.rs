use std::marker::PhantomData;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::__private::TokenStream2;
use syn::{parse_quote, TraitBound, Type, TypeArray, TypeImplTrait, TypeParamBound, TypePath, TypePtr, TypeReference, TypeSlice, TypeTraitObject};
use crate::ast::AddPunctuated;
use crate::composable::{GenericBoundsModel, TypeModel};
use crate::context::ScopeContext;
use crate::conversion::{DictFermentableModelKind, DictTypeModelKind, GroupModelKind, ObjectKind, ScopeItemKind, SmartPointerModelKind, TypeModelKind};
use crate::ext::{Accessory, AsType, GenericNestedArg, Mangle, Resolve, ResolveTrait, SpecialType, ToType};
use crate::lang::objc::ObjCSpecification;
use crate::presentation::{FFIFullPath, FFIVariable};

impl ToType for FFIVariable<ObjCSpecification, TokenStream2> {
    fn to_type(&self) -> Type {
        match self {
            FFIVariable::Direct { ty, .. } => ty.to_type(),
            FFIVariable::ConstPtr { ty, .. } => ty.joined_const().to_type(),
            FFIVariable::MutPtr { ty, .. } => ty.joined_mut().to_type(),
            FFIVariable::Ref { ty, .. } => ty.joined_ref().to_type(),
            FFIVariable::MutRef { ty, .. } => ty.joined_mut_ref().to_type(),
            FFIVariable::Dyn { ty, .. } => ty.joined_dyn().to_type()
        }
    }
}

impl ToTokens for FFIVariable<ObjCSpecification, TokenStream2> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            FFIVariable::Direct { ty, .. } => ty.to_tokens(tokens),
            FFIVariable::ConstPtr { ty, .. } => quote!(const #ty *).to_tokens(tokens),
            FFIVariable::MutPtr { ty, .. } => quote!(#ty *).to_tokens(tokens),
            FFIVariable::Ref { ty, .. } => quote!(&#ty).to_tokens(tokens),
            FFIVariable::MutRef { ty, .. } => quote!(&#ty *).to_tokens(tokens),
            FFIVariable::Dyn { ty, .. } => quote!(#ty *).to_tokens(tokens),
        }
    }
}

impl Accessory for FFIVariable<ObjCSpecification, TokenStream2> {
    fn joined_mut(&self) -> Self {
        match self {
            FFIVariable::Direct { ty, .. } => FFIVariable::MutPtr { ty: ty.clone(), _marker: PhantomData },
            FFIVariable::ConstPtr { ty, .. } => FFIVariable::MutPtr { ty: ty.clone(), _marker: PhantomData },
            FFIVariable::MutPtr { ty, .. } => FFIVariable::MutPtr { ty: ty.clone(), _marker: PhantomData },
            FFIVariable::Ref { ty , .. } => FFIVariable::MutPtr { ty: ty.clone(), _marker: PhantomData },
            FFIVariable::MutRef { ty, .. } => FFIVariable::MutPtr { ty: ty.clone(), _marker: PhantomData },
            FFIVariable::Dyn { ty, .. } => FFIVariable::MutPtr { ty: ty.clone(), _marker: PhantomData },
        }
    }

    fn joined_const(&self) -> Self {
        match self {
            FFIVariable::Direct { ty, .. } => FFIVariable::MutPtr { ty: ty.clone(), _marker: PhantomData },
            FFIVariable::ConstPtr { ty, .. } => FFIVariable::MutPtr { ty: ty.clone(), _marker: PhantomData },
            FFIVariable::MutPtr { ty, .. } => FFIVariable::MutPtr { ty: ty.clone(), _marker: PhantomData },
            FFIVariable::Ref { ty , .. } => FFIVariable::MutPtr { ty: ty.clone(), _marker: PhantomData },
            FFIVariable::MutRef { ty, .. } => FFIVariable::MutPtr { ty: ty.clone(), _marker: PhantomData },
            FFIVariable::Dyn { ty, .. } => FFIVariable::MutPtr { ty: ty.clone(), _marker: PhantomData },
        }
    }

    fn joined_dyn(&self) -> Self {
        self.clone()
    }

    fn joined_ref(&self) -> Self {
        todo!()
    }

    fn joined_mut_ref(&self) -> Self {
        todo!()
    }
}

impl Resolve<FFIVariable<ObjCSpecification, TokenStream2>> for GenericBoundsModel {
    fn maybe_resolve(&self, source: &ScopeContext) -> Option<FFIVariable<ObjCSpecification, TokenStream2>> {
        Some(self.resolve(source))
    }

    fn resolve(&self, _source: &ScopeContext) -> FFIVariable<ObjCSpecification, TokenStream2> {
        let ffi_name = self.mangle_tokens_default();
        if self.is_lambda() {
            FFIVariable::direct(ffi_name)
        } else {
            FFIVariable::mut_ptr(ffi_name)
        }
    }
}

impl Resolve<FFIVariable<ObjCSpecification, TokenStream2>> for Type {
    fn maybe_resolve(&self, source: &ScopeContext) -> Option<FFIVariable<ObjCSpecification, TokenStream2>> {
        Some(self.resolve(source))
    }
    fn resolve(&self, source: &ScopeContext) -> FFIVariable<ObjCSpecification, TokenStream2> {
        let full_ty = Resolve::<Type>::resolve(self, source);
        let maybe_special = Resolve::<SpecialType<ObjCSpecification>>::maybe_resolve(&full_ty, source);
        let refined = maybe_special
            .map(FFIFullPath::from)
            .or_else(|| source.maybe_ffi_full_path(self))
            .map(|ffi_path| ffi_path.to_type())
            .unwrap_or_else(|| parse_quote!(#self))
            .to_type();
        resolve_type_variable(refined, source)
    }
}
impl Resolve<FFIVariable<ObjCSpecification, TokenStream2>> for AddPunctuated<TypeParamBound> {
    fn maybe_resolve(&self, source: &ScopeContext) -> Option<FFIVariable<ObjCSpecification, TokenStream2>> {
        Some(self.resolve(source))
    }
    fn resolve(&self, source: &ScopeContext) -> FFIVariable<ObjCSpecification, TokenStream2> {
        let bound = self.iter().find_map(|bound| match bound {
            TypeParamBound::Trait(TraitBound { path, .. }) => Some(path.to_type()),
            _ => None
        }).unwrap();
        bound.resolve(source)
    }
}


pub fn resolve_type_variable(ty: Type, source: &ScopeContext) -> FFIVariable<ObjCSpecification, TokenStream2> {
    //println!("resolve_type_variable: {}", ty.to_token_stream());
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
                        (_, Some(_const_token), None) => FFIVariable::const_ptr(quote!(void)),
                        (_, None, Some(_mut_token)) => FFIVariable::mut_ptr(quote!(void)),
                        _ => panic!("Resolve::<FFIVariable>::resolve: c_void with {} {} not supported", quote!(#const_token), quote!(#mutability))
                    },
                    _ => {
                        if const_token.is_some() {
                            FFIVariable::const_ptr(path.to_token_stream())
                        } else {
                            FFIVariable::mut_ptr(path.to_token_stream())
                        }
                    }
                },
                Type::Ptr(..) => {
                    FFIVariable::mut_ptr(elem.to_token_stream())
                },
                ty => mutability.as_ref()
                    .map_or( FFIVariable::const_ptr(ty.to_token_stream()), |_| FFIVariable::mut_ptr(ty.to_token_stream()))
            },
        Type::TraitObject(TypeTraitObject { dyn_token: _, bounds, .. }) => {
            bounds.resolve(source)
        }
        Type::ImplTrait(TypeImplTrait { impl_token: _, bounds, .. }) =>
            bounds.resolve(source),
        ty => FFIVariable::direct(ty.mangle_tokens_default())
    }
}

impl Resolve<FFIVariable<ObjCSpecification, TokenStream2>> for TypeModelKind {
    fn maybe_resolve(&self, source: &ScopeContext) -> Option<FFIVariable<ObjCSpecification, TokenStream2>> {
        Some(self.resolve(source))
    }

    fn resolve(&self, source: &ScopeContext) -> FFIVariable<ObjCSpecification, TokenStream2> {
        println!("TypeModelKind::<FFIVariable>::resolve({}) in {}", self, source.scope.fmt_short());
        let result = match self  {
            // TODO: For now we assume that every callback defined as fn pointer is opaque
            TypeModelKind::FnPointer(TypeModel { ty, .. }, ..) =>
                FFIVariable::direct(
                    Resolve::<SpecialType<ObjCSpecification>>::maybe_resolve(ty, source)
                        .map(|special| special.to_token_stream())
                        .unwrap_or_else(|| Resolve::<FFIFullPath<ObjCSpecification>>::resolve(ty, source)
                            .to_token_stream())),
            TypeModelKind::Dictionary(DictTypeModelKind::LambdaFn(TypeModel { ty, .. }, ..)) =>
                FFIVariable::mut_ptr(Resolve::<FFIFullPath<ObjCSpecification>>::resolve(ty, source).to_token_stream()),
            TypeModelKind::Dictionary(DictTypeModelKind::Primitive(composition)) =>
                FFIVariable::direct(composition.to_type().to_token_stream()),
            TypeModelKind::Dictionary(DictTypeModelKind::NonPrimitiveFermentable(DictFermentableModelKind::SmartPointer(SmartPointerModelKind::Box(TypeModel { ty, .. })))) => {
                // println!("TypeModelKind::Boxed: {}", ty.to_token_stream());
                match ty.maybe_first_nested_type_ref() {
                    Some(nested_full_ty) => {
                        // println!("Nested: {}", nested_full_ty.to_token_stream());
                        resolve_type_variable(match Resolve::<SpecialType<ObjCSpecification>>::maybe_resolve(nested_full_ty, source) {
                            Some(special) => special.to_type(),
                            None => {
                                source.maybe_ffi_full_path::<ObjCSpecification>(nested_full_ty)
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
                        SmartPointerModelKind::Cell(TypeModel { ty, .. }) |
                        SmartPointerModelKind::RefCell(TypeModel { ty, .. }) |
                        SmartPointerModelKind::UnsafeCell(TypeModel { ty, .. }) |
                        SmartPointerModelKind::OnceLock(TypeModel { ty, .. }) |
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
                    DictFermentableModelKind::I128(TypeModel { ty, .. }) |
                    DictFermentableModelKind::U128(TypeModel { ty, .. }) |
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
                Resolve::<SpecialType<ObjCSpecification>>::maybe_resolve(ty, source)
                    .map(|ty| resolve_type_variable(FFIFullPath::from(ty).to_type(), source))
                    .unwrap_or_else(|| {
                        resolve_type_variable(Resolve::<ObjectKind>::maybe_resolve(ty, source)
                          .and_then(|external_type| {
                              match external_type {
                                  ObjectKind::Item(.., ScopeItemKind::Fn(..)) => {
                                      let parent_object = &source.scope.parent_object().unwrap();
                                      match parent_object {
                                          ObjectKind::Type(ref ty_conversion) |
                                          ObjectKind::Item(ref ty_conversion, ..) => {
                                              match ty_conversion {
                                                  TypeModelKind::Trait(ty, ..) =>
                                                      ty.as_type().maybe_trait_object_maybe_model_kind(source),
                                                  _ => None,
                                              }.unwrap_or_else(|| parent_object.maybe_type_model_kind_ref().cloned())
                                          },
                                          ObjectKind::Empty => {
                                              None
                                          }
                                      }
                                  },
                                  ObjectKind::Type(ref ty_conversion) |
                                  ObjectKind::Item(ref ty_conversion, ..) => {
                                      match ty_conversion {
                                          TypeModelKind::Trait(ty, ..) =>
                                              ty.as_type().maybe_trait_object_maybe_model_kind(source),
                                          _ => None,
                                      }.unwrap_or_else(|| external_type.maybe_type_model_kind_ref().cloned())
                                  },
                                  ObjectKind::Empty => None
                              }
                          })
                          .unwrap_or_else(|| TypeModelKind::unknown_type_ref(ty))
                          .to_type(), source)
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