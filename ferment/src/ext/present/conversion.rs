use quote::{quote, ToTokens};
use syn::{Path, Type, TypeArray, TypeImplTrait, TypePath, TypePtr, TypeReference, TypeSlice, TypeTraitObject, TypeTuple};
use syn::punctuated::Punctuated;
use crate::conversion::FieldTypeConversion;
use crate::ext::Mangle;
use crate::helper::path_arguments_to_paths;
use crate::interface::ffi_to_conversion;
use crate::naming::Name;
use crate::presentation::context::{FieldTypePresentableContext, OwnedItemPresentableContext, OwnerIteratorPresentationContext};

pub enum ConversionMethod {
    From,
    To,
    Destroy
}

pub trait Conversion {
    fn conversion_from(&self, field_path: FieldTypePresentableContext) -> FieldTypePresentableContext;
    fn conversion_to(&self, field_path: FieldTypePresentableContext) -> FieldTypePresentableContext;
    fn conversion_destroy(&self, field_path: FieldTypePresentableContext) -> FieldTypePresentableContext;
}

impl Conversion for FieldTypeConversion {
    fn conversion_from(&self, field_path: FieldTypePresentableContext) -> FieldTypePresentableContext {
        self.ty().conversion_from(field_path)
    }

    fn conversion_to(&self, field_path: FieldTypePresentableContext) -> FieldTypePresentableContext {
        self.ty().conversion_to(field_path)
    }

    fn conversion_destroy(&self, field_path: FieldTypePresentableContext) -> FieldTypePresentableContext {
        self.ty().conversion_destroy(field_path)
    }
}

impl Conversion for Type {
    fn conversion_from(&self, field_path: FieldTypePresentableContext) -> FieldTypePresentableContext {
        match self {
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
        }
    }

    fn conversion_to(&self, field_path: FieldTypePresentableContext) -> FieldTypePresentableContext {
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

    fn conversion_destroy(&self, field_path: FieldTypePresentableContext) -> FieldTypePresentableContext {
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
    fn conversion_from(&self, field_path: FieldTypePresentableContext) -> FieldTypePresentableContext {
        //println!("Conversion for TypeArray: {} -- {:?}", self.to_token_stream(), field_path);
        // let arg_type = handle_arg_type(&type_array.elem, pat, context);
        // let len = &type_array.len;
        match &*self.elem {
            Type::Path(TypePath { path: Path { segments, .. }, .. }) =>
                if matches!(segments.last().unwrap().ident.to_string().as_str(), "i8" | "u8" | "i16" | "u16" | "i32" | "u32" | "i64" | "u64" | "f64" | "i128" | "u128" | "isize" | "usize" | "bool") {
                    // FieldTypePresentableContext::DerefContext(field_path.into())
                    FieldTypePresentableContext::From(field_path.into())
                } else {
                    panic!("<TypeArray as Conversion>::conversion_from: {}", quote!(#segments))
                }
            Type::Tuple(..) => {
                // FieldTypePresentableContext::From(field_path.into())
                FieldTypePresentableContext::From(field_path.into())
                // FieldTypePresentableContext::DerefContext(field_path.into())
            },
            _ => panic!("<TypeArray as Conversion>::conversion_from: {}", quote!(#self)),
        }
    }

    fn conversion_to(&self, field_path: FieldTypePresentableContext) -> FieldTypePresentableContext {
        match &*self.elem {
            Type::Path(type_path) =>
                FieldTypePresentableContext::To(field_path.into()),

            // type_path.conversion_to(FieldTypePresentableContext::Boxed(field_path.into())),
            _ => panic!("<TypeArray as Conversion>::conversion_to: Unknown type {}", quote!(#self)),
        }
    }

    fn conversion_destroy(&self, field_path: FieldTypePresentableContext) -> FieldTypePresentableContext {
        FieldTypePresentableContext::UnboxAny(field_path.into())
    }
}

impl Conversion for TypeSlice {
    fn conversion_from(&self, field_path: FieldTypePresentableContext) -> FieldTypePresentableContext {
        let ty = &*self.elem;
        let ffi_type = self.mangle_ident_default();
        FieldTypePresentableContext::AsSlice(
            FieldTypePresentableContext::CastFrom(
                field_path.into(),
                quote!(Vec<#ty>),
                quote!(crate::fermented::generics::#ffi_type)).into())
    }

    fn conversion_to(&self, field_path: FieldTypePresentableContext) -> FieldTypePresentableContext {
        // TODO: fix it TypeConversion::from
        match &*self.elem {
            Type::Path(type_path) =>
                FieldTypePresentableContext::To(FieldTypePresentableContext::ToVec(field_path.into()).into()),

            // type_path.conversion_to(FieldTypePresentableContext::Boxed(field_path.into())),
            // match segments.last().unwrap().ident.to_string().as_str() {
            //     "u8" => FieldTypePresentableContext::DerefContext(field_path.into()),
            //     _ => panic!("from_slice: unsupported segments {}", quote!(#segments))
            // },
            Type::Tuple(type_tuple) =>
                FieldTypePresentableContext::To(FieldTypePresentableContext::ToVec(field_path.into()).into()),
            Type::Array(type_array) =>
                FieldTypePresentableContext::To(FieldTypePresentableContext::ToVec(field_path.into()).into()),
            Type::Slice(type_slice) =>
                FieldTypePresentableContext::To(FieldTypePresentableContext::ToVec(field_path.into()).into()),
            Type::Reference(type_reference) =>
                FieldTypePresentableContext::To(FieldTypePresentableContext::ToVec(field_path.into()).into()),
            _ => panic!("<TypeSlice as Conversion>::conversion_to: Unknown type {} === {:?}", quote!(#self), self),
        }
    }

    fn conversion_destroy(&self, field_path: FieldTypePresentableContext) -> FieldTypePresentableContext {
        // TODO: fix it TypeConversion::from
        FieldTypePresentableContext::UnboxAny(field_path.into())

        // TypeConversion::from()
        // todo!()
    }
}
impl Conversion for TypePtr {
    fn conversion_from(&self, field_path: FieldTypePresentableContext) -> FieldTypePresentableContext {
        match &*self.elem {
            Type::Ptr(type_ptr) => match &*type_ptr.elem {
                Type::Path(_type_path) => FieldTypePresentableContext::FromOffsetMap,
                _ => FieldTypePresentableContext::From(field_path.into()),
            },
            Type::Path(type_path) =>
                FieldTypePresentableContext::FromRawParts(type_path
                    .path
                    .segments
                    .last()
                    .unwrap()
                    .ident
                    .to_token_stream()),
            _ => FieldTypePresentableContext::From(field_path.into()),
        }
    }

    fn conversion_to(&self, field_path: FieldTypePresentableContext) -> FieldTypePresentableContext {
        match &*self.elem {
            Type::Array(TypeArray { elem, .. }) => match &**elem {
                Type::Path(type_path) => type_path.conversion_to(field_path),
                _ => panic!("to_ptr: Unknown type inside Type::Array {}", quote!(#self)),
            },
            Type::Ptr(TypePtr { elem, .. }) => match &**elem {
                Type::Path(type_path) =>
                    type_path.conversion_to(FieldTypePresentableContext::DerefContext(FieldTypePresentableContext::Add(field_path.into(), quote!(i)).into())),
                Type::Array(_type_arr) => FieldTypePresentableContext::ToVecPtr,
                _ => panic!("to_ptr: Unknown type inside Type::Ptr {}", quote!(#self)),
            },
            Type::Path(type_path) => type_path.conversion_to(field_path),
            _ => panic!("to_ptr: Unknown type {}", quote!(#self)),
        }
    }

    fn conversion_destroy(&self, field_path: FieldTypePresentableContext) -> FieldTypePresentableContext {
        match &*self.elem {
            Type::Ptr(type_ptr) => type_ptr.conversion_destroy(field_path),
            Type::Path(type_path) => type_path.conversion_destroy(field_path),
            _ => panic!("Can't destroy_ptr: of type: {}", quote!(#self)),
        }
    }
}

impl Conversion for TypeReference {
    fn conversion_from(&self, field_path: FieldTypePresentableContext) -> FieldTypePresentableContext {
        match &*self.elem {
            Type::Path(type_path) => {
                match type_path.path.segments.last().unwrap().ident.to_string().as_str() {
                    "str" => type_path.conversion_from(field_path),
                    _ => {
                        if self.mutability.is_some() {
                            FieldTypePresentableContext::AsMutRef(type_path.conversion_from(field_path).into())
                        } else {
                            FieldTypePresentableContext::AsRef(type_path.conversion_from(field_path).into())
                        }
                    }
                }
                // if self.mutability.is_some() {
                //     FieldTypePresentableContext::AsMutRef(type_path.conversion_from(field_path).into())
                // } else {
                //     FieldTypePresentableContext::AsRef(type_path.conversion_from(field_path).into())
                // }
                //type_path.conversion_from(field_path)
            },
            Type::Slice(type_slice) => {
                println!("SLICE IN REF: {}", self.to_token_stream());
                type_slice.conversion_from(field_path)
                // if self.mutability.is_some() {
                //     FieldTypePresentableContext::AsMutRef(type_slice.conversion_from(field_path).into())
                // } else {
                //     FieldTypePresentableContext::AsRef(type_slice.conversion_from(field_path).into())
                // }
            },
            Type::Array(type_array) => {
                if self.mutability.is_some() {
                    FieldTypePresentableContext::AsMutRef(type_array.conversion_from(field_path).into())
                } else {
                    FieldTypePresentableContext::AsRef(type_array.conversion_from(field_path).into())
                }
            },
            Type::Tuple(type_tuple) => {
                if self.mutability.is_some() {
                    FieldTypePresentableContext::AsMutRef(type_tuple.conversion_from(field_path).into())
                } else {
                    FieldTypePresentableContext::AsRef(type_tuple.conversion_from(field_path).into())
                }
            },
            _ => panic!("from_reference: unsupported type: {}", quote!(#self)),
        }
    }

    fn conversion_to(&self, field_path: FieldTypePresentableContext) -> FieldTypePresentableContext {
        //
        self.elem.conversion_to(field_path)
        // match &*self.elem {
        //     Type::Path(type_path) => type_path.conversion_to(field_path),
        //     _ => panic!("to_reference: Unknown type {}", quote!(#self)),
        // }
    }

    fn conversion_destroy(&self, field_path: FieldTypePresentableContext) -> FieldTypePresentableContext {
        match &*self.elem {
            Type::Path(type_path) => type_path.conversion_destroy(field_path),
            Type::Slice(type_slice) => type_slice.conversion_destroy(field_path),
            _ => panic!("destroy_reference: unsupported type: {}", quote!(#self)),
        }
    }
}

impl Conversion for TypePath {
    fn conversion_from(&self, field_path: FieldTypePresentableContext) -> FieldTypePresentableContext {
        let last_segment = self.path.segments.last().unwrap();
        match last_segment.ident.to_string().as_str() {
            "i8" | "u8" | "i16" | "u16" | "i32" | "u32" | "i64" | "u64" | "f64" | "i128" | "u128" | "isize" | "usize" | "bool" => field_path,
            // TODO: redo this
            "Option" => match path_arguments_to_paths(&last_segment.arguments).first().unwrap().segments.last().unwrap().ident.to_string().as_str() {
                // std convertible
                // TODO: what to use? 0 or ::MAX
                "i8" | "u8" | "i16" | "u16" | "i32" | "u32" | "i64" | "u64" | "f64" | "i128" | "u128"
                | "isize" | "usize" => FieldTypePresentableContext::IfThen(field_path.into(), quote!(> 0)),
                // TODO: mmm shit that's incorrect
                "bool" => FieldTypePresentableContext::IfThen(field_path.into(), quote!()),
                _ => FieldTypePresentableContext::FromOpt(field_path.into()),
            },
            _ => FieldTypePresentableContext::From(field_path.into()),
        }
    }

    fn conversion_to(&self, field_path: FieldTypePresentableContext) -> FieldTypePresentableContext {
        let last_segment = self.path.segments.last().unwrap();
        match last_segment.ident.to_string().as_str() {
            "i8" | "u8" | "i16" | "u16" | "i32" | "u32" | "i64" | "u64" | "f64" | "i128" | "u128" | "isize"
            | "usize" | "bool" => field_path,
            "Option" => match path_arguments_to_paths(&last_segment.arguments).first().unwrap().segments.last().unwrap().ident.to_string().as_str() {
                // TODO: MAX/MIN? use optional primitive?
                "i8" | "u8" | "i16" | "u16" | "i32" | "u32" | "i64" | "u64" | "f64" | "i128" | "u128" | "isize" | "usize" =>
                    FieldTypePresentableContext::UnwrapOr(field_path.into(), quote!(0)),
                "bool" =>
                    FieldTypePresentableContext::UnwrapOr(field_path.into(), quote!(false)),
                "Vec" =>
                    FieldTypePresentableContext::OwnerIteratorPresentation(
                        OwnerIteratorPresentationContext::MatchFields((field_path.into(), Punctuated::from_iter([
                            OwnedItemPresentableContext::Lambda(quote!(Some(vec)), ffi_to_conversion(quote!(vec))),
                            OwnedItemPresentableContext::Lambda(quote!(None), quote!(std::ptr::null_mut()))
                        ])))),
                _ => FieldTypePresentableContext::ToOpt(field_path.into()),
            },
            _ => FieldTypePresentableContext::To(field_path.into()),
        }
    }

    fn conversion_destroy(&self, field_path: FieldTypePresentableContext) -> FieldTypePresentableContext {
        let last_segment = self.path.segments.last().unwrap();
        match last_segment.ident.to_string().as_str() {
            "i8" | "u8" | "i16" | "u16" | "i32" | "u32" | "i64" | "u64" | "f64" | "i128" | "u128" | "isize"
            | "usize" | "bool" => FieldTypePresentableContext::Empty,
            "Option" => match path_arguments_to_paths(&self.path.segments.last().unwrap().arguments).first().unwrap().segments.last().unwrap().ident.to_string().as_str() {
                "i8" | "u8" | "i16" | "u16" | "i32" | "u32" | "i64" | "u64" | "f64" | "i128" | "u128" | "isize" | "usize" | "bool" =>
                    FieldTypePresentableContext::Empty,
                _ =>
                    FieldTypePresentableContext::IsNull(field_path.into())
            },
            "String" => FieldTypePresentableContext::DestroyConversion(field_path.into(), self.path.to_token_stream()),
            "str" => FieldTypePresentableContext::DestroyConversion(field_path.into(), quote!(&#self)),
            _ => FieldTypePresentableContext::UnboxAnyTerminated(field_path.into()),
        }
    }
}

impl Conversion for TypeTuple {
    fn conversion_from(&self, field_path: FieldTypePresentableContext) -> FieldTypePresentableContext {
        FieldTypePresentableContext::FromTuple(field_path.into(), self.elems.iter()
            .enumerate()
            .map(|(index, elem)|
                elem.conversion_from(
                        FieldTypePresentableContext::FfiRefWithConversion(FieldTypeConversion::Unnamed(Name::UnnamedArg(index), elem.clone())).into()))
            .collect())
    }

    fn conversion_to(&self, field_path: FieldTypePresentableContext) -> FieldTypePresentableContext {
        FieldTypePresentableContext::To(field_path.into())
    }

    fn conversion_destroy(&self, field_path: FieldTypePresentableContext) -> FieldTypePresentableContext {
        FieldTypePresentableContext::UnboxAny(field_path.into())
    }
}

impl Conversion for TypeTraitObject {
    fn conversion_from(&self, field_path: FieldTypePresentableContext) -> FieldTypePresentableContext {
        FieldTypePresentableContext::AsRef(field_path.into())
    }

    fn conversion_to(&self, _field_path: FieldTypePresentableContext) -> FieldTypePresentableContext {
        todo!()
    }

    fn conversion_destroy(&self, _field_path: FieldTypePresentableContext) -> FieldTypePresentableContext {
        todo!()
    }
}

impl Conversion for TypeImplTrait {
    fn conversion_from(&self, field_path: FieldTypePresentableContext) -> FieldTypePresentableContext {
        FieldTypePresentableContext::AsRef(field_path.into())
    }

    fn conversion_to(&self, _field_path: FieldTypePresentableContext) -> FieldTypePresentableContext {
        todo!()
    }

    fn conversion_destroy(&self, _field_path: FieldTypePresentableContext) -> FieldTypePresentableContext {
        todo!()
    }
}
