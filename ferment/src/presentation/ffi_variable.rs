use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_quote, Path, TraitBound, Type, TypeArray, TypeParamBound, TypePath, TypePtr, TypeReference, TypeSlice, TypeTraitObject};
use crate::composable::{GenericBoundComposition, TypeComposition};
use crate::context::ScopeContext;
use crate::conversion::{GenericTypeConversion, TypeCompositionConversion, TypeConversion};
use crate::ext::{Accessory, DictionaryType, FFIFullPathResolve, FFISpecialTypeResolve, Mangle, path_arguments_to_type_conversions, Resolve, ToPath, ToType};
use crate::presentation::{FFIFullDictionaryPath, FFIFullPath};

pub enum FFIVariable {
    Direct { ty: Type },
    ConstPtr { ty: Type },
    MutPtr { ty: Type },
}

impl ToTokens for FFIVariable {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.to_type().to_tokens(tokens)
    }
}

impl ToType for FFIVariable {
    fn to_type(&self) -> Type {
        match self {
            FFIVariable::Direct { ty } => ty.to_type(),
            FFIVariable::ConstPtr { ty } => ty.joined_const(),
            FFIVariable::MutPtr { ty } => ty.joined_mut()
        }
    }
}

impl Resolve<FFIVariable> for Path {
    fn resolve(&self, source: &ScopeContext) -> FFIVariable {
        let first_segment = self.segments.first().unwrap();
        let first_ident = &first_segment.ident;
        let last_segment = self.segments.last().unwrap();
        let last_ident = &last_segment.ident;
        if last_ident.is_primitive() {
            FFIVariable::Direct { ty: self.to_type() }
        } else if last_ident.is_optional() {
            match path_arguments_to_type_conversions(&last_segment.arguments).first() {
                Some(TypeConversion::Primitive(ty)) => FFIVariable::MutPtr {
                    ty: ty.clone()
                },
                Some(TypeConversion::Generic(generic_ty)) => FFIVariable::MutPtr {
                    ty: <GenericTypeConversion as Resolve<FFIFullPath>>::resolve(generic_ty, source).to_type()
                },
                Some(TypeConversion::Complex(Type::Path(TypePath { path, .. }))) =>
                    path.resolve(source),
                _ => unimplemented!("ffi_dictionary_variable_type:: Empty Optional")
            }
        } else if last_ident.is_special_generic() || (last_ident.is_result() /*&& path.segments.len() == 1*/) || (last_ident.to_string().eq("Map") && first_ident.to_string().eq("serde_json")) {
            FFIVariable::MutPtr {
                ty: source.scope_type_for_path(self)
                    .map_or(self.to_token_stream(), |full_type| full_type.mangle_tokens_default())
                    .to_type()
            }
        } else {
            FFIVariable::MutPtr {
                ty: self.to_type()
            }
        }
    }
}

impl Resolve<FFIVariable> for Type {
    fn resolve(&self, source: &ScopeContext) -> FFIVariable {
        match self.maybe_special_or_trait_ffi_full_path(source)
            .map(|ffi_path| ffi_path.to_type())
            .unwrap_or(parse_quote!(#self))
            .to_type() {
            Type::Path(TypePath { path, .. }) =>
                path.resolve(source),
            Type::Array(TypeArray { elem, len, .. }) => FFIVariable::MutPtr {
                ty: parse_quote!([#elem; #len])
            },
            Type::Reference(TypeReference { elem, .. }) |
            Type::Slice(TypeSlice { elem, .. }) =>
                elem.resolve(source),
            Type::Ptr(TypePtr { star_token, const_token, mutability, elem }) =>
                match *elem {
                    Type::Path(TypePath { path, .. }) => match path.segments.last().unwrap().ident.to_string().as_str() {
                        "c_void" => match (star_token, const_token, mutability) {
                            (_, Some(_const_token), None) => FFIVariable::ConstPtr { ty: FFIFullDictionaryPath::Void.to_type() },
                            (_, None, Some(_mut_token)) => FFIVariable::MutPtr { ty: FFIFullDictionaryPath::Void.to_type() },
                            _ => panic!("<Type as Resolve<FFIVariable>>::resolve: c_void with {} {} not supported", quote!(#const_token), quote!(#mutability))
                        },
                        _ => FFIVariable::MutPtr {
                            ty: path.to_type()
                        }
                    },
                    Type::Ptr(..) => FFIVariable::MutPtr {
                        ty: elem.to_type(),
                    },
                    ty => mutability.as_ref()
                        .map_or( FFIVariable::ConstPtr { ty: ty.clone() }, |_| FFIVariable::MutPtr { ty: ty.clone() })
                },
            Type::TraitObject(TypeTraitObject { bounds, .. }) => {
                let bound = bounds.iter().find_map(|bound| match bound {
                    TypeParamBound::Trait(TraitBound { path, .. }) => Some(path.to_type()),
                    TypeParamBound::Lifetime(_) => None
                }).unwrap();
                bound.resolve(source)
            },
            ty => FFIVariable::Direct { ty: ty.mangle_ident_default().to_type() }
        }
    }
}

impl Resolve<FFIVariable> for TypeCompositionConversion {
    fn resolve(&self, source: &ScopeContext) -> FFIVariable {
        match self {
            // TODO: For now we assume that every callback defined as fn pointer is opaque
            TypeCompositionConversion::FnPointer(TypeComposition { ty, .. }) => FFIVariable::Direct {
                ty: ty
                    .maybe_special_type(source)
                    .unwrap_or(<Type as Resolve::<FFIFullPath>>::resolve(ty, source)
                        .to_type())
            },
            TypeCompositionConversion::Primitive(ty) =>
                FFIVariable::Direct { ty: ty.ty.clone() },
            TypeCompositionConversion::Trait(TypeComposition { ty, .. }, _, _) |
            TypeCompositionConversion::TraitType(TypeComposition { ty, .. }) |
            TypeCompositionConversion::Object(TypeComposition { ty, .. }) |
            TypeCompositionConversion::Optional(TypeComposition { ty, .. }) |
            TypeCompositionConversion::Array(TypeComposition { ty, .. }) |
            TypeCompositionConversion::Slice(TypeComposition { ty, .. }) |
            TypeCompositionConversion::Tuple(TypeComposition { ty, .. }) |
            TypeCompositionConversion::Unknown(TypeComposition { ty, .. }) |
            TypeCompositionConversion::LocalOrGlobal(TypeComposition { ty, .. }) =>
                <Type as Resolve<Type>>::resolve(ty, source)
                    .maybe_special_type(source)
                    .map(|ty| FFIFullPath::External { path: ty.to_path() })
                    .or(<Type as Resolve<TypeCompositionConversion>>::resolve(ty, source)
                        .to_type()
                        .resolve(source))
                    .map(|ffi_path| ffi_path.to_type())
                    .unwrap_or(parse_quote!(#ty))
                    .to_type()
                    .resolve(source),
            TypeCompositionConversion::Bounds(bounds) =>
                bounds.resolve(source),
            ty =>
                panic!("error: Arg conversion ({}) not supported", ty),
        }
    }
}

impl Resolve<FFIVariable> for GenericBoundComposition {
    fn resolve(&self, _source: &ScopeContext) -> FFIVariable {
        let ffi_name = self.mangle_ident_default();
        FFIVariable::MutPtr { ty: parse_quote!(crate::fermented::generics::#ffi_name) }
    }
}