use quote::{quote, ToTokens};
use syn::{Path, Type, TypeArray, TypeImplTrait, TypePath, TypePtr, TypeReference, TypeSlice, TypeTraitObject, TypeTuple};
use syn::punctuated::Punctuated;
use crate::conversion::{FieldTypeConversion, FieldTypeConversionKind, TypeConversion};
use crate::ext::{DictionaryType, Mangle, ToPath};
use crate::helper::path_arguments_to_type_conversions;
use crate::naming::{DictionaryExpr, FFIConversionMethodExpr, Name};
use crate::presentation::context::{FieldContext, OwnedItemPresentableContext, OwnerIteratorPresentationContext};

pub trait Conversion {
    fn conversion_from(&self, field_path: FieldContext) -> FieldContext;
    fn conversion_to(&self, field_path: FieldContext) -> FieldContext;
    fn conversion_destroy(&self, field_path: FieldContext) -> FieldContext;
}

impl Conversion for FieldTypeConversion {
    fn conversion_from(&self, field_path: FieldContext) -> FieldContext {
        self.ty().conversion_from(field_path)
    }

    fn conversion_to(&self, field_path: FieldContext) -> FieldContext {
        self.ty().conversion_to(field_path)
    }

    fn conversion_destroy(&self, field_path: FieldContext) -> FieldContext {
        self.ty().conversion_destroy(field_path)
    }
}

impl Conversion for Type {
    fn conversion_from(&self, field_path: FieldContext) -> FieldContext {
        // println!("Type::conversion_from: {}", field_path);
        let resutl = match self {
            Type::Array(ty) =>
                ty.conversion_from(field_path),
            Type::Path(ty) =>
                ty.conversion_from(field_path),
            Type::Ptr(ty) =>
                ty.conversion_from(field_path),
            Type::Reference(ty) =>
                ty.conversion_from(field_path),
            Type::Slice(ty) =>
                ty.conversion_from(field_path),
            Type::Tuple(ty) =>
                ty.conversion_from(field_path),
            Type::TraitObject(ty) =>
                ty.conversion_from(field_path),
            Type::ImplTrait(ty) =>
                ty.conversion_from(field_path),
            _ => unimplemented!("No conversions for {}", self.to_token_stream())
        };
        // println!("Type::conversion_from ---> {}", resutl);
        resutl
    }

    fn conversion_to(&self, field_path: FieldContext) -> FieldContext {
        match self {
            Type::Array(ty) =>
                ty.conversion_to(field_path),
            Type::Path(ty) =>
                ty.conversion_to(field_path),
            Type::Ptr(ty) =>
                ty.conversion_to(field_path),
            Type::Reference(ty) =>
                ty.conversion_to(field_path),
            Type::Slice(ty) =>
                ty.conversion_to(field_path),
            Type::TraitObject(ty) =>
                ty.conversion_to(field_path),
            Type::Tuple(ty) =>
                ty.conversion_to(field_path),
            Type::ImplTrait(ty) =>
                ty.conversion_to(field_path),
            _ => unimplemented!("No conversions for {}", self.to_token_stream())
        }
    }

    fn conversion_destroy(&self, field_path: FieldContext) -> FieldContext {
        match self {
            Type::Array(ty) =>
                ty.conversion_destroy(field_path),
            Type::Path(ty) =>
                ty.conversion_destroy(field_path),
            Type::Ptr(ty) =>
                ty.conversion_destroy(field_path),
            Type::Reference(ty) =>
                ty.conversion_destroy(field_path),
            Type::Slice(ty) =>
                ty.conversion_destroy(field_path),
            Type::TraitObject(ty) =>
                ty.conversion_destroy(field_path),
            Type::Tuple(ty) =>
                ty.conversion_destroy(field_path),
            Type::ImplTrait(ty) =>
                ty.conversion_destroy(field_path),
            _ => unimplemented!("No conversions for {}", self.to_token_stream())
        }
    }
}

impl Conversion for TypeArray {
    fn conversion_from(&self, field_path: FieldContext) -> FieldContext {
        // println!("Conversion for TypeArray: {} -- {:?}", self.to_token_stream(), field_path);
        // let arg_type = handle_arg_type(&type_array.elem, pat, context);
        // let len = &type_array.len;
        match &*self.elem {
            Type::Path(TypePath { path: Path { segments: _, .. }, .. }) =>
                // if segments.last().unwrap().ident.is_primitive() {
                    // FieldTypePresentableContext::DerefContext(field_path.into())
                    FieldContext::From(field_path.into()),
                // } else {
                //     panic!("<TypeArray as Conversion>::conversion_from: {}", quote!(#segments))
                // }
            Type::Tuple(..) => {
                // FieldTypePresentableContext::From(field_path.into())
                FieldContext::From(field_path.into())
                // FieldTypePresentableContext::DerefContext(field_path.into())
            },
            _ => panic!("<TypeArray as Conversion>::conversion_from: {}", quote!(#self)),
        }
    }

    fn conversion_to(&self, field_path: FieldContext) -> FieldContext {
        match &*self.elem {
            Type::Path(..) =>
                FieldContext::To(field_path.into()),

            // type_path.conversion_to(FieldTypePresentableContext::Boxed(field_path.into())),
            _ => panic!("<TypeArray as Conversion>::conversion_to: Unknown type {}", quote!(#self)),
        }
    }

    fn conversion_destroy(&self, field_path: FieldContext) -> FieldContext {
        FieldContext::UnboxAny(field_path.into())
    }
}

impl Conversion for TypeSlice {
    fn conversion_from(&self, field_path: FieldContext) -> FieldContext {
        let ty = &*self.elem;
        let ffi_type = self.mangle_ident_default();
        FieldContext::AsSlice(
            FieldContext::CastFrom(
                field_path.into(),
                quote!(Vec<#ty>),
                quote!(crate::fermented::generics::#ffi_type)).into())
    }

    fn conversion_to(&self, field_path: FieldContext) -> FieldContext {

        match &*self.elem {
            Type::Path(..) =>
                FieldContext::To(FieldContext::ToVec(field_path.into()).into()),
            Type::Tuple(..) =>
                FieldContext::To(FieldContext::ToVec(field_path.into()).into()),
            Type::Array(..) =>
                FieldContext::To(FieldContext::ToVec(field_path.into()).into()),
            Type::Slice(..) =>
                FieldContext::To(FieldContext::ToVec(field_path.into()).into()),
            Type::Reference(..) =>
                FieldContext::To(FieldContext::ToVec(field_path.into()).into()),
            _ => panic!("<TypeSlice as Conversion>::conversion_to: Unknown type {} === {:?}", quote!(#self), self),
        }
    }

    fn conversion_destroy(&self, field_path: FieldContext) -> FieldContext {
        // TODO: fix it TypeConversion::from
        FieldContext::UnboxAny(field_path.into())

        // TypeConversion::from()
        // todo!()
    }
}
impl Conversion for TypePtr {
    fn conversion_from(&self, field_path: FieldContext) -> FieldContext {
        match &*self.elem {
            Type::Ptr(type_ptr) => match &*type_ptr.elem {
                Type::Path(_type_path) => FieldContext::FromOffsetMap,
                _ => FieldContext::From(field_path.into()),
            },
            Type::Path(type_path) =>
                FieldContext::FromRawParts(type_path
                    .path
                    .segments
                    .last()
                    .unwrap()
                    .ident
                    .to_token_stream()),
            _ => FieldContext::From(field_path.into()),
        }
    }

    fn conversion_to(&self, field_path: FieldContext) -> FieldContext {
        match &*self.elem {
            Type::Array(TypeArray { elem, .. }) => match &**elem {
                Type::Path(type_path) => type_path.conversion_to(field_path),
                _ => panic!("to_ptr: Unknown type inside Type::Array {}", quote!(#self)),
            },
            Type::Ptr(TypePtr { elem, .. }) => match &**elem {
                Type::Path(type_path) =>
                    type_path.conversion_to(FieldContext::DerefContext(FieldContext::Add(field_path.into(), quote!(i)).into())),
                Type::Array(_type_arr) => FieldContext::ToVecPtr,
                _ => panic!("to_ptr: Unknown type inside Type::Ptr {}", quote!(#self)),
            },
            Type::Path(type_path) => type_path.conversion_to(field_path),
            _ => panic!("to_ptr: Unknown type {}", quote!(#self)),
        }
    }

    fn conversion_destroy(&self, field_path: FieldContext) -> FieldContext {
        match &*self.elem {
            Type::Ptr(type_ptr) => type_ptr.conversion_destroy(field_path),
            Type::Path(type_path) => type_path.conversion_destroy(field_path),
            _ => panic!("Can't destroy_ptr: of type: {}", quote!(#self)),
        }
    }
}

impl Conversion for TypeReference {
    fn conversion_from(&self, field_path: FieldContext) -> FieldContext {
        match &*self.elem {
            Type::Path(type_path) => {
                match type_path.path.segments.last().unwrap().ident.to_string().as_str() {
                    "str" => type_path.conversion_from(field_path),
                    _ => if self.mutability.is_some() {
                        FieldContext::AsMutRef(type_path.conversion_from(field_path).into())
                    } else {
                        FieldContext::AsRef(type_path.conversion_from(field_path).into())
                    }
                }
            },
            Type::Slice(type_slice) => type_slice.conversion_from(field_path),
            Type::Array(type_array) => if self.mutability.is_some() {
                FieldContext::AsMutRef(type_array.conversion_from(field_path).into())
            } else {
                FieldContext::AsRef(type_array.conversion_from(field_path).into())
            },
            Type::Tuple(type_tuple) => if self.mutability.is_some() {
                FieldContext::AsMutRef(type_tuple.conversion_from(field_path).into())
            } else {
                FieldContext::AsRef(type_tuple.conversion_from(field_path).into())
            },
            _ => panic!("TypeReference::conversion_from: unsupported type: {}", quote!(#self)),
        }
    }

    fn conversion_to(&self, field_path: FieldContext) -> FieldContext {
        self.elem.conversion_to(field_path)
    }

    fn conversion_destroy(&self, field_path: FieldContext) -> FieldContext {
        match &*self.elem {
            Type::Path(type_path) => type_path.conversion_destroy(field_path),
            Type::Slice(type_slice) => type_slice.conversion_destroy(field_path),
            _ => panic!("conversion_from::conversion_destroy: unsupported type: {}", quote!(#self)),
        }
    }
}

impl Conversion for TypePath {
    fn conversion_from(&self, field_path: FieldContext) -> FieldContext {
        let last_segment = self.path.segments.last().unwrap();
        let last_ident = &last_segment.ident;
        if last_ident.is_primitive() {
            field_path
        } else if last_ident.is_optional() {
            match path_arguments_to_type_conversions(&last_segment.arguments).first() {
                None => unimplemented!("TypePath::conversion_from: Empty Optional: {}", self.to_token_stream()),
                Some(conversion) => match conversion {
                    TypeConversion::Callback(ty) => unimplemented!("Optional Callback: {}", ty.to_token_stream()),
                    TypeConversion::Primitive(ty) => {
                        let ty_path = ty.to_path();
                        let last_segment = ty_path.segments.last().unwrap();
                        FieldContext::IfThen(field_path.into(), if last_segment.ident.is_bool() { quote!() } else { quote!(> 0) })
                    }
                    _ => FieldContext::FromOpt(field_path.into()),
                }
            }
        } else if last_ident.is_box() {
            FieldContext::IntoBox(FieldContext::From(field_path.into()).into())
        // } else if last_ident.is_smart_ptr() {
        //     match path_arguments_to_type_conversions(&last_segment.arguments).first() {
        //         None => unimplemented!("TypePath::conversion_from: Empty Smart pointer: {}", self.to_token_stream()),
        //         Some(conversion) => match conversion {
        //             TypeConversion::Callback(ty) => unimplemented!("TypePath::conversion_from: Optional Callback: {}", ty.to_token_stream()),
        //             TypeConversion::Primitive(ty) => {
        //                 let ty_path = ty.to_path();
        //                 let last_segment = ty_path.segments.last().unwrap();
        //                 let opt_ident = &last_segment.ident;
        //                 FieldContext::CastFrom(FieldContext::From(field_path.into()).into(), quote!(#self), opt_ident.to_token_stream())
        //             },
        //             TypeConversion::Complex(ty) => {
        //                 let ffi_type = ty.mangle_ident_default();
        //
        //                 FieldContext::CastFrom(FieldContext::From(field_path.into()).into(), quote!(#self), quote!(crate::fermented::generics::#ffi_type))
        //             }
        //             _ => {
        //                 let ffi_type = self.mangle_ident_default();
        //                 FieldContext::CastFrom(FieldContext::From(field_path.into()).into(), quote!(#self), quote!(crate::fermented::generics::#ffi_type))
        //             },
        //         }
        //     }
        //     // <std::os::raw::c_char as ferment_interfaces::FFIConversion<std::sync::Arc<String>>>::ffi_from(ffi_ref.opt_arc)
        //     // FieldTypePresentableContext::AsSlice(
        //     //     FieldTypePresentableContext::CastFrom(
        //     //         field_path.into(),
        //     //         quote!(Vec<#ty>),
        //     //         quote!(crate::fermented::generics::#ffi_type)).into())
        //     // FieldTypePresentableContext::Into(FieldTypePresentableContext::From(field_path.into()).into())
        } else {
            FieldContext::From(field_path.into())
        }
    }

    fn conversion_to(&self, field_path: FieldContext) -> FieldContext {
        let last_segment = self.path.segments.last().unwrap();
        let last_ident = &last_segment.ident;
        if last_ident.is_primitive() {
            field_path
        } else if last_ident.is_optional() {
            match path_arguments_to_type_conversions(&last_segment.arguments).first() {
                None => unimplemented!("TypePath::conversion_to: Empty Optional"),
                Some(conversion) => match conversion {
                    TypeConversion::Callback(ty) => unimplemented!("Optional Callback: {}", ty.to_token_stream()),
                    TypeConversion::Primitive(ty) => {
                        let ty_path = ty.to_path();
                        let last_segment = ty_path.segments.last().unwrap();
                        let opt_ident = &last_segment.ident;
                        if opt_ident.is_bool() {
                            FieldContext::UnwrapOr(field_path.into(), quote!(false))
                        } else {
                            FieldContext::UnwrapOr(field_path.into(), quote!(0))
                        }
                    },
                    TypeConversion::Generic(_ty) => FieldContext::OwnerIteratorPresentation(
                        OwnerIteratorPresentationContext::MatchFields((field_path.into(), Punctuated::from_iter([
                            OwnedItemPresentableContext::Lambda(quote!(Some(vec)), FFIConversionMethodExpr::FfiTo(quote!(vec)).to_token_stream(), quote!()),
                            OwnedItemPresentableContext::Lambda(quote!(None), DictionaryExpr::NullMut.to_token_stream(), quote!())
                        ])))),
                    _ => FieldContext::ToOpt(field_path.into())
                }
            }
        } else {
            FieldContext::To(field_path.into())
        }
    }

    fn conversion_destroy(&self, field_path: FieldContext) -> FieldContext {
        let last_segment = self.path.segments.last().unwrap();
        let last_ident = &last_segment.ident;
        if last_ident.is_primitive() {
            FieldContext::Empty
        } else if last_ident.is_optional() {
            match path_arguments_to_type_conversions(&last_segment.arguments).first() {
                None => unimplemented!("TypePath::conversion_destroy: Empty Optional"),
                Some(conversion) => match conversion {
                    TypeConversion::Callback(ty) => unimplemented!("TypePath::conversion_destroy: Optional Callback: {}", ty.to_token_stream()),
                    TypeConversion::Primitive(_) => FieldContext::Empty,
                    _ => FieldContext::DestroyOpt(field_path.into())
                }
            }
        } else if last_ident.is_string() {
            FieldContext::DestroyString(field_path.into(), self.path.to_token_stream())
        } else if last_ident.is_str() {
            FieldContext::DestroyString(field_path.into(), quote!(&#self))
        } else {
            FieldContext::UnboxAnyTerminated(field_path.into())
        }
    }
}

impl Conversion for TypeTuple {
    fn conversion_from(&self, field_path: FieldContext) -> FieldContext {
        FieldContext::FromTuple(field_path.into(), self.elems.iter()
            .enumerate()
            .map(|(index, elem)|
                elem.conversion_from(
                    FieldContext::FfiRefWithConversion(
                        FieldTypeConversion::unnamed(
                            Name::UnnamedArg(index),
                            FieldTypeConversionKind::Type(elem.clone())))
                        .into()))
            .collect())
    }

    fn conversion_to(&self, field_path: FieldContext) -> FieldContext {
        FieldContext::To(field_path.into())
    }

    fn conversion_destroy(&self, field_path: FieldContext) -> FieldContext {
        FieldContext::UnboxAny(field_path.into())
    }
}

impl Conversion for TypeTraitObject {
    fn conversion_from(&self, field_path: FieldContext) -> FieldContext {
        FieldContext::AsRef(field_path.into())
    }

    fn conversion_to(&self, _field_path: FieldContext) -> FieldContext {
        todo!()
    }

    fn conversion_destroy(&self, _field_path: FieldContext) -> FieldContext {
        todo!()
    }
}

impl Conversion for TypeImplTrait {
    fn conversion_from(&self, field_path: FieldContext) -> FieldContext {
        FieldContext::AsRef(field_path.into())
    }

    fn conversion_to(&self, _field_path: FieldContext) -> FieldContext {
        todo!()
    }

    fn conversion_destroy(&self, _field_path: FieldContext) -> FieldContext {
        todo!()
    }
}
