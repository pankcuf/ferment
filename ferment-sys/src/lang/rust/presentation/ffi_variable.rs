use std::marker::PhantomData;
use proc_macro2::{Ident, TokenStream};
use quote::ToTokens;
use syn::{parse_quote, AngleBracketedGenericArguments, Path, PathSegment, TraitBound, Type, TypeArray, TypeImplTrait, TypeParamBound, TypePath, TypePtr, TypeReference, TypeSlice, TypeTraitObject};
use crate::ast::AddPunctuated;
use crate::composable::{GenericBoundsModel, TypeModel};
use crate::context::ScopeContext;
use crate::ext::{Accessory, AsType, DictionaryType, GenericNestedArg, Mangle, MaybeAngleBracketedArgs, MaybeGenericType, Resolve, ToType};
use crate::kind::{DictFermentableModelKind, DictTypeModelKind, ObjectKind, SmartPointerModelKind, SpecialType, TypeKind, TypeModelKind};
use crate::lang::RustSpecification;
use crate::presentation::{FFIFullDictionaryPath, FFIFullPath, FFIVariable};

impl ToTokens for FFIVariable<RustSpecification, Type> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.to_type().to_tokens(tokens)
    }
}

impl ToType for FFIVariable<RustSpecification, Type> {
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

impl Accessory for FFIVariable<RustSpecification, Type> {
    fn joined_mut(&self) -> Self {
        FFIVariable::mut_ptr(self.to_type())
    }
    fn joined_const(&self) -> Self {
        FFIVariable::const_ptr(self.to_type())
    }
    fn joined_dyn(&self) -> Self {
        FFIVariable::r#dyn(self.to_type())
    }
    fn joined_ref(&self) -> Self {
        FFIVariable::r#ref(self.to_type())
    }
    fn joined_mut_ref(&self) -> Self {
        FFIVariable::mut_ref(self.to_type())
    }
    fn joined_ident(&self, ident: &Ident) -> Self {
        match self {
            FFIVariable::Direct { ty, .. } => FFIVariable::Direct { ty: parse_quote!(#ty::#ident), _marker: PhantomData },
            FFIVariable::ConstPtr { ty, .. } => FFIVariable::ConstPtr { ty: parse_quote!(#ty::#ident), _marker: PhantomData },
            FFIVariable::MutPtr { ty, .. } => FFIVariable::MutPtr { ty: parse_quote!(#ty::#ident), _marker: PhantomData },
            FFIVariable::Ref { ty, .. } =>  FFIVariable::Ref { ty: parse_quote!(#ty::#ident), _marker: PhantomData },
            FFIVariable::MutRef { ty, .. } => FFIVariable::MutRef { ty: parse_quote!(#ty::#ident), _marker: PhantomData },
            FFIVariable::Dyn { ty, .. } => FFIVariable::Dyn { ty: parse_quote!(#ty::#ident), _marker: PhantomData }
        }
    }
}

impl Resolve<FFIVariable<RustSpecification, Type>> for Type {
    fn maybe_resolve(&self, source: &ScopeContext) -> Option<FFIVariable<RustSpecification, Type>> {
        Some(self.resolve(source))
    }
    fn resolve(&self, source: &ScopeContext) -> FFIVariable<RustSpecification, Type> {
        let full_ty = Resolve::<Type>::resolve(self, source);
        let refined = source.maybe_special_or_regular_ffi_full_path::<RustSpecification>(&full_ty)
            .map(|ffi_path| ffi_path.to_type())
            .unwrap_or_else(|| parse_quote!(#self));
        resolve_type_variable(refined, source)
    }
}

impl Resolve<FFIVariable<RustSpecification, Type>> for TypeModelKind {
    fn maybe_resolve(&self, source: &ScopeContext) -> Option<FFIVariable<RustSpecification, Type>> {
        Some(self.resolve(source))
    }
    fn resolve(&self, source: &ScopeContext) -> FFIVariable<RustSpecification, Type> {
        let result = match self  {
            // TODO: For now we assume that every callback defined as fn pointer is opaque
            TypeModelKind::FnPointer(TypeModel { ty, .. }, ..) =>
                FFIVariable::direct(Resolve::<SpecialType<RustSpecification>>::maybe_resolve(ty, source)
                    .map(|special| special.to_type())
                    .unwrap_or_else(|| Resolve::<FFIFullPath<RustSpecification>>::resolve(ty, source)
                        .to_type())),
            TypeModelKind::Dictionary(DictTypeModelKind::LambdaFn(TypeModel { ty, .. }, ..)) =>
                FFIVariable::mut_ptr(Resolve::<FFIFullPath<RustSpecification>>::resolve(ty, source).to_type()),
            TypeModelKind::Dictionary(DictTypeModelKind::Primitive(composition)) =>
                FFIVariable::direct(composition.to_type()),
            TypeModelKind::Dictionary(DictTypeModelKind::NonPrimitiveFermentable(DictFermentableModelKind::I128(..) | DictFermentableModelKind::U128(..))) =>
                FFIVariable::mut_ptr(parse_quote!([u8; 16])),
            TypeModelKind::Dictionary(DictTypeModelKind::NonPrimitiveFermentable(DictFermentableModelKind::SmartPointer(SmartPointerModelKind::Box(TypeModel { ty, .. })))) |
            TypeModelKind::Dictionary(DictTypeModelKind::NonPrimitiveFermentable(DictFermentableModelKind::Cow(TypeModel { ty, .. }))) => match ty.maybe_first_nested_type_ref() {
                Some(nested_full_ty) =>
                    resolve_type_variable(match Resolve::<SpecialType<RustSpecification>>::maybe_resolve(nested_full_ty, source) {
                        Some(special) => special.to_type(),
                        None => source.maybe_ffi_full_path::<RustSpecification>(nested_full_ty)
                            .map(|full_path| full_path.to_type())
                            .unwrap_or_else(|| nested_full_ty.clone())
                    }, source),
                None =>
                    panic!("error: Arg kind ({}) not supported", ty.to_token_stream())
            },
            TypeModelKind::Bounds(bounds) =>
                bounds.resolve(source),
            TypeModelKind::Fn(TypeModel { ty, .. }, ..) =>
                panic!("error: Arg kind (Fn) ({}) not supported", ty.to_token_stream()),
            model_kind => {
                let ty = model_kind.as_type();
                let full_ty = Resolve::<SpecialType<RustSpecification>>::maybe_resolve(ty, source)
                    .map(|special_ty| FFIFullPath::from(special_ty).to_type())
                    .unwrap_or_else(|| Resolve::<ObjectKind>::maybe_resolve(ty, source)
                        .and_then(|external_type| external_type.maybe_fn_or_trait_or_same_kind(source)
                            .map(|kind| kind.to_type()))
                        .unwrap_or_else(|| ty.clone()));
                resolve_type_variable(full_ty, source)
            }
        };
        result
    }
}

impl Resolve<FFIVariable<RustSpecification, Type>> for GenericBoundsModel {
    fn maybe_resolve(&self, source: &ScopeContext) -> Option<FFIVariable<RustSpecification, Type>> {
        Some(self.resolve(source))
    }
    fn resolve(&self, _source: &ScopeContext) -> FFIVariable<RustSpecification, Type> {
        let ffi_name = self.mangle_ident_default();
        let ty = parse_quote!(crate::fermented::generics::#ffi_name);
        if self.is_lambda() {
            FFIVariable::direct(ty)
        } else {
            FFIVariable::mut_ptr(ty)
        }
    }
}

impl Resolve<FFIVariable<RustSpecification, Type>> for AddPunctuated<TypeParamBound> {
    fn maybe_resolve(&self, source: &ScopeContext) -> Option<FFIVariable<RustSpecification, Type>> {
        Some(self.resolve(source))
    }
    fn resolve(&self, source: &ScopeContext) -> FFIVariable<RustSpecification, Type> {
        let maybe_bound = self.iter().find_map(|bound| match bound {
            TypeParamBound::Trait(TraitBound { path, .. }) =>
                Some(path.to_type()),
            _ =>
                None
        });
        if let Some(bound) = maybe_bound {
            bound.resolve(source)
        } else {
            FFIVariable::mut_ptr(self.to_type())
        }
    }
}

impl Resolve<FFIVariable<RustSpecification, Type>> for Path {
    fn maybe_resolve(&self, source: &ScopeContext) -> Option<FFIVariable<RustSpecification, Type>> {
        Some(self.resolve(source))
    }
    fn resolve(&self, source: &ScopeContext) -> FFIVariable<RustSpecification, Type> {
        match (self.segments.first(), self.segments.last()) {
            (Some(PathSegment { ident: first_ident, ..}), Some(PathSegment { ident: last_ident, arguments})) => if last_ident.is_primitive() {
                Some(FFIVariable::direct(self.to_type()))
            } else if last_ident.is_128_digit() {
                Some(FFIVariable::mut_ptr(parse_quote!([u8; 16])))
            } else if last_ident.is_optional() {
                arguments.maybe_angle_bracketed_args()
                    .and_then(|AngleBracketedGenericArguments { args, .. }| args.iter().find_map(|arg| arg.maybe_generic_type().and_then(|ty| match TypeKind::from(ty) {
                    TypeKind::Primitive(ty) =>
                        Some(FFIVariable::mut_ptr(ty.clone())),
                    TypeKind::Generic(generic_ty) =>
                        Resolve::<FFIFullPath<RustSpecification>>::maybe_resolve(&generic_ty, source).map(|path| FFIVariable::mut_ptr(path.to_type())),
                    TypeKind::Complex(Type::Path(TypePath { path, .. })) =>
                        Resolve::<FFIVariable<RustSpecification, Type>>::maybe_resolve(&path, source),
                    _ => None
                })))
            } else if last_ident.is_special_generic() || last_ident.is_result() || (last_ident.eq("Map") && first_ident.eq("serde_json")) {
                Some(FFIVariable::mut_ptr(source.scope_type_for_path(self).map(|full_type| full_type.mangle_tokens_default().to_type()).unwrap_or_else(|| self.to_type())))
            } else {
                None
            },
            _ => None
        }.unwrap_or_else(|| FFIVariable::mut_ptr(self.to_type()))
    }
}


pub fn resolve_type_variable(ty: Type, source: &ScopeContext) -> FFIVariable<RustSpecification, Type> {
    match ty {
        Type::Path(TypePath { path, .. }) =>
            path.resolve(source),
        Type::Array(TypeArray { elem, len, .. }) =>
            FFIVariable::mut_ptr(parse_quote!([#elem; #len])),
        Type::Reference(TypeReference { elem, .. }) |
        Type::Slice(TypeSlice { elem, .. }) =>
            elem.resolve(source),
        Type::TraitObject(TypeTraitObject { bounds, .. }) |
        Type::ImplTrait(TypeImplTrait { bounds, .. }) =>
            bounds.resolve(source),
        Type::Ptr(TypePtr { const_token, mutability, elem, .. }) => match *elem {
            Type::Path(TypePath { path, .. }) => if let Some(PathSegment { ident, .. }) = path.segments.last() {
                let ty = if ident.is_void() {
                    FFIFullDictionaryPath::<RustSpecification>::Void.to_type()
                } else {
                    path.to_type()
                };
                if const_token.is_some() {
                    FFIVariable::const_ptr(ty)
                } else {
                    FFIVariable::mut_ptr(ty)
                }
            } else {
                FFIVariable::mut_ptr(path.to_type())
            },
            Type::Ptr(..) =>
                FFIVariable::mut_ptr(elem.to_type()),
            ty if mutability.is_some() =>
                FFIVariable::mut_ptr(ty.clone()),
            ty =>
                FFIVariable::const_ptr(ty.clone())
        },
        ty =>
            FFIVariable::direct(ty.mangle_ident_default().to_type())
    }
}

pub fn resolve_type_variable_via_type(ty: Type, source: &ScopeContext) -> FFIVariable<RustSpecification, Type> {
    resolve_type_variable(Resolve::<FFIFullPath<RustSpecification>>::maybe_resolve(&ty, source).map(|path| path.to_type()).unwrap_or(ty), source)
}

pub fn resolve_type_variable_via_ffi_full_path(ty_model_kind: TypeModelKind, source: &ScopeContext) -> FFIVariable<RustSpecification, Type> {
    resolve_type_variable_via_type(ty_model_kind.to_type(), source)
}

pub fn resolve_type_variable_via_maybe_object(
    maybe_object: Option<ObjectKind>,
    ty: &Type,
    source: &ScopeContext
) -> FFIVariable<RustSpecification, Type> {
    resolve_type_variable_via_ffi_full_path(
        maybe_object.and_then(|object_kind| object_kind.maybe_fn_or_trait_or_same_kind2(source))
            .unwrap_or_else(|| TypeModelKind::unknown_type_ref(ty)), source)
}
