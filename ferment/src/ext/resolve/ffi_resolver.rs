use syn::{Path, Type};
use crate::context::ScopeContext;
use crate::conversion::{GenericTypeConversion, TypeCompositionConversion};
use crate::ext::{Accessory, Resolve, ToPath, ToType};
use crate::presentation::FFIFullPath;

pub trait FFISpecialTypeResolve {
    /// Types that are exported with [ferment_macro::register] or [ferment_macro::opaque]
    /// so it's custom conversion or opaque pointer therefore we should use direct paths for ffi export
    fn maybe_special_type(&self, source: &ScopeContext) -> Option<Type>;
}
impl FFISpecialTypeResolve for Type {
    fn maybe_special_type(&self, source: &ScopeContext) -> Option<Type> {
        source.maybe_custom_conversion(self)
            .or_else(|| source.maybe_opaque_object(self))
    }
}
impl FFISpecialTypeResolve for GenericTypeConversion {
    fn maybe_special_type(&self, source: &ScopeContext) -> Option<Type> {
        self.ty()
            .and_then(|ty| ty.maybe_special_type(source))
    }
}

pub trait FFIFullPathResolve {
    fn maybe_special_or_trait_ffi_full_path(&self, source: &ScopeContext) -> Option<FFIFullPath>;
}
impl FFIFullPathResolve for Type {
    fn maybe_special_or_trait_ffi_full_path(&self, source: &ScopeContext) -> Option<FFIFullPath> {
        <Type as Resolve<Type>>::resolve(self, source)
            .maybe_special_type(source)
            .map(|ty| FFIFullPath::External { path: ty.to_path() })
            .or(<Type as Resolve<TypeCompositionConversion>>::resolve(self, source)
                .to_type()
                .resolve(source))
    }
}
impl FFIFullPathResolve for Path {
    fn maybe_special_or_trait_ffi_full_path(&self, source: &ScopeContext) -> Option<FFIFullPath> {
        <Type as Resolve<Type>>::resolve(&self.to_type(), source)
            .maybe_special_type(source)
            .map(|ty| FFIFullPath::External { path: ty.to_path() })
            .or(<Type as Resolve<TypeCompositionConversion>>::resolve(&self.to_type(), source)
                .to_type()
                .resolve(source))
    }

}

pub trait FFITypeResolve: FFISpecialTypeResolve + Resolve<FFIFullPath> {
    fn special_or_to_ffi_full_path_type(&self, source: &ScopeContext) -> Type {
        self.maybe_special_type(source)
            .unwrap_or(<Self as Resolve::<FFIFullPath>>::resolve(self, source).to_type())
    }
    fn special_or_to_ffi_full_path_variable_type(&self, source: &ScopeContext) -> Type {
        self.special_or_to_ffi_full_path_type(source)
            .joined_mut()
    }
}
impl FFITypeResolve for Type {}
impl FFITypeResolve for GenericTypeConversion {}

// pub trait FFIVariableResolve: FFIFullPathResolve {
//     fn to_ffi_variable(&self, source: &ScopeContext) -> Type;
//     fn to_full_ffi_variable(&self, source: &ScopeContext) -> Type where Self: ToTokens {
//         self.maybe_special_or_trait_ffi_full_path(source)
//             .map(|ffi_path| ffi_path.to_type())
//             .unwrap_or(parse_quote!(#self))
//             .to_type()
//             .to_ffi_variable(source)
//     }
// }

// impl FFIVariableResolve for Path {
//     fn to_ffi_variable(&self, source: &ScopeContext) -> Type {
//         let first_segment = self.segments.first().unwrap();
//         let first_ident = &first_segment.ident;
//         let last_segment = self.segments.last().unwrap();
//         let last_ident = &last_segment.ident;
//         if last_ident.is_primitive() {
//             self.to_type()
//         } else if last_ident.is_optional() {
//             match path_arguments_to_type_conversions(&last_segment.arguments).first() {
//                 Some(TypeConversion::Primitive(ty)) =>
//                     ty.to_path().to_full_ffi_variable(source)
//                         .joined_mut(),
//                 Some(TypeConversion::Generic(generic_ty)) =>
//                     <GenericTypeConversion as Resolve<FFIFullPath>>::resolve(generic_ty, source)
//                         .to_type()
//                         .joined_mut(),
//                 Some(TypeConversion::Complex(Type::Path(TypePath { path, .. }))) =>
//                     path.to_ffi_variable(source),
//                 _ => unimplemented!("ffi_dictionary_variable_type:: Empty Optional")
//             }
//         } else if last_ident.is_special_generic() || (last_ident.is_result() /*&& path.segments.len() == 1*/) || (last_ident.to_string().eq("Map") && first_ident.to_string().eq("serde_json")) {
//             source.scope_type_for_path(self)
//                 .map_or(self.to_token_stream(), |full_type| full_type.mangle_tokens_default())
//                 .to_type()
//                 .joined_mut()
//         } else {
//             self.to_type()
//                 .joined_mut()
//         }
//     }
// }
//
// impl FFIVariableResolve for Type {
//     fn to_ffi_variable(&self, source: &ScopeContext) -> Type {
//         match self {
//             Type::Path(TypePath { path, .. }) =>
//                 path.to_ffi_variable(source),
//             Type::Array(TypeArray { elem, len, .. }) =>
//                 parse_quote!(*mut [#elem; #len]),
//             Type::Reference(TypeReference { elem, .. }) |
//             Type::Slice(TypeSlice { elem, .. }) =>
//                 elem.to_ffi_variable(source),
//             Type::Ptr(TypePtr { star_token, const_token, mutability, elem }) =>
//                 match &**elem {
//                     Type::Path(TypePath { path, .. }) => match path.segments.last().unwrap().ident.to_string().as_str() {
//                         "c_void" => match (star_token, const_token, mutability) {
//                             (_, Some(_const_token), None) => parse_quote!(ferment_interfaces::OpaqueContext),
//                             (_, None, Some(_mut_token)) => parse_quote!(ferment_interfaces::OpaqueContextMut),
//                             _ => panic!("ffi_dictionary_type_presenter: c_void with {} {} not supported", quote!(#const_token), quote!(#mutability))
//                         },
//                         _ => parse_quote!(*mut #path)
//                     },
//                     Type::Ptr(type_ptr) =>
//                         parse_quote!(*mut #type_ptr),
//                     _ => self.clone()
//                 },
//             Type::TraitObject(TypeTraitObject { bounds, .. }) => {
//                 let bound = bounds.iter().find_map(|bound| match bound {
//                     TypeParamBound::Trait(TraitBound { path, .. }) => Some(path.to_type()),
//                     TypeParamBound::Lifetime(_) => None
//                 }).unwrap();
//                 bound.to_ffi_variable(source)
//             },
//             ty =>
//                 ty.mangle_ident_default().to_type()
//         }
//     }
// }
//

