use quote::{quote, ToTokens};
use syn::{Type, TypeArray, TypeImplTrait, TypePath, TypePtr, TypeReference, TypeSlice, TypeTraitObject, TypeTuple};
use syn::punctuated::Punctuated;
use crate::ast::Depunctuated;
use crate::composable::{FieldTypeComposition, FieldTypeConversionKind};
use crate::conversion::TypeConversion;
use crate::ext::{DictionaryType, Mangle};
use crate::ext::item::path_arguments_to_type_conversions;
use crate::naming::{DictionaryExpr, FFIConversionMethodExpr, Name};
use crate::presentable::{Expression, OwnedItemPresentableContext, SequenceOutput};

pub trait Conversion {
    fn conversion_from(&self, expr: Expression) -> Expression;
    fn conversion_to(&self, expr: Expression) -> Expression;
    fn conversion_destroy(&self, expr: Expression) -> Expression;
}

impl Conversion for FieldTypeComposition {
    fn conversion_from(&self, expr: Expression) -> Expression {
        self.ty().conversion_from(expr)
    }

    fn conversion_to(&self, expr: Expression) -> Expression {
        self.ty().conversion_to(expr)
    }

    fn conversion_destroy(&self, expr: Expression) -> Expression {
        self.ty().conversion_destroy(expr)
    }
}

impl Conversion for Type {
    fn conversion_from(&self, expr: Expression) -> Expression {
        // println!("Type::conversion_from: {}", field_path);
        let resutl = match self {
            Type::Array(ty) =>
                ty.conversion_from(expr),
            Type::Path(ty) =>
                ty.conversion_from(expr),
            Type::Ptr(ty) =>
                ty.conversion_from(expr),
            Type::Reference(ty) =>
                ty.conversion_from(expr),
            Type::Slice(ty) =>
                ty.conversion_from(expr),
            Type::Tuple(ty) =>
                ty.conversion_from(expr),
            Type::TraitObject(ty) =>
                ty.conversion_from(expr),
            Type::ImplTrait(ty) =>
                ty.conversion_from(expr),
            _ => unimplemented!("No conversions for {}", self.to_token_stream())
        };
        // println!("Type::conversion_from ---> {}", resutl);
        resutl
    }

    fn conversion_to(&self, expr: Expression) -> Expression {
        match self {
            Type::Array(ty) =>
                ty.conversion_to(expr),
            Type::Path(ty) =>
                ty.conversion_to(expr),
            Type::Ptr(ty) =>
                ty.conversion_to(expr),
            Type::Reference(ty) =>
                ty.conversion_to(expr),
            Type::Slice(ty) =>
                ty.conversion_to(expr),
            Type::TraitObject(ty) =>
                ty.conversion_to(expr),
            Type::Tuple(ty) =>
                ty.conversion_to(expr),
            Type::ImplTrait(ty) =>
                ty.conversion_to(expr),
            _ => unimplemented!("No conversions for {}", self.to_token_stream())
        }
    }

    fn conversion_destroy(&self, expr: Expression) -> Expression {
        match self {
            Type::Array(ty) =>
                ty.conversion_destroy(expr),
            Type::Path(ty) =>
                ty.conversion_destroy(expr),
            Type::Ptr(ty) =>
                ty.conversion_destroy(expr),
            Type::Reference(ty) =>
                ty.conversion_destroy(expr),
            Type::Slice(ty) =>
                ty.conversion_destroy(expr),
            Type::TraitObject(ty) =>
                ty.conversion_destroy(expr),
            Type::Tuple(ty) =>
                ty.conversion_destroy(expr),
            Type::ImplTrait(ty) =>
                ty.conversion_destroy(expr),
            _ => unimplemented!("No conversions for {}", self.to_token_stream())
        }
    }
}

impl Conversion for TypeArray {
    fn conversion_from(&self, expr: Expression) -> Expression {
        Expression::From(expr.into())
    }

    fn conversion_to(&self, expr: Expression) -> Expression {
        Expression::To(expr.into())
    }

    fn conversion_destroy(&self, expr: Expression) -> Expression {
        Expression::UnboxAny(expr.into())
    }
}

impl Conversion for TypeSlice {
    fn conversion_from(&self, expr: Expression) -> Expression {
        let ty = &*self.elem;
        let ffi_type = self.mangle_ident_default();
        Expression::AsSlice(
            Expression::CastFrom(
                expr.into(),
                quote!(Vec<#ty>),
                quote!(crate::fermented::generics::#ffi_type)).into())
    }

    fn conversion_to(&self, expr: Expression) -> Expression {
        match &*self.elem {
            Type::Path(..) =>
                Expression::To(Expression::ToVec(expr.into()).into()),
            Type::Tuple(..) =>
                Expression::To(Expression::ToVec(expr.into()).into()),
            Type::Array(..) =>
                Expression::To(Expression::ToVec(expr.into()).into()),
            Type::Slice(..) =>
                Expression::To(Expression::ToVec(expr.into()).into()),
            Type::Reference(..) =>
                Expression::To(Expression::ToVec(expr.into()).into()),
            _ => panic!("<TypeSlice as Conversion>::conversion_to: Unknown type {} === {:?}", quote!(#self), self),
        }
    }

    fn conversion_destroy(&self, expr: Expression) -> Expression {
        Expression::UnboxAny(expr.into())
    }
}
impl Conversion for TypePtr {
    fn conversion_from(&self, expr: Expression) -> Expression {
        match &*self.elem {
            Type::Ptr(type_ptr) => match &*type_ptr.elem {
                Type::Path(_type_path) => Expression::FromOffsetMap,
                _ => Expression::From(expr.into()),
            },
            Type::Path(type_path) =>
                Expression::FromRawParts(type_path
                    .path
                    .segments
                    .last()
                    .unwrap()
                    .ident
                    .to_token_stream()),
            _ => Expression::From(expr.into()),
        }
    }

    fn conversion_to(&self, expr: Expression) -> Expression {
        match &*self.elem {
            Type::Array(TypeArray { elem, .. }) => elem.conversion_to(expr),
            Type::Path(type_path) => type_path.conversion_to(expr),
            Type::Ptr(TypePtr { elem, .. }) => match &**elem {
                Type::Path(type_path) =>
                    type_path.conversion_to(Expression::DerefContext(Expression::Add(expr.into(), quote!(i)).into())),
                Type::Array(_type_arr) => Expression::ToVecPtr,
                _ => panic!("to_ptr: Unknown type inside Type::Ptr {}", quote!(#self)),
            },
            _ => panic!("to_ptr: Unknown type {}", quote!(#self)),
        }
    }

    fn conversion_destroy(&self, expr: Expression) -> Expression {
        match &*self.elem {
            Type::Ptr(type_ptr) => type_ptr.conversion_destroy(expr),
            Type::Path(type_path) => type_path.conversion_destroy(expr),
            _ => panic!("Can't destroy_ptr: of type: {}", quote!(#self)),
        }
    }
}

impl Conversion for TypeReference {
    fn conversion_from(&self, expr: Expression) -> Expression {
        match &*self.elem {
            Type::Path(type_path) => match type_path.path.segments.last().unwrap().ident.to_string().as_str() {
                "str" =>
                    type_path.conversion_from(expr),
                _ if self.mutability.is_some() =>
                    Expression::AsMutRef(type_path.conversion_from(expr).into()),
                _ =>
                    Expression::AsRef(type_path.conversion_from(expr).into())
            },
            Type::Slice(type_slice) =>
                type_slice.conversion_from(expr),
            Type::Array(type_array) if self.mutability.is_some() =>
                Expression::AsMutRef(type_array.conversion_from(expr).into()),
            Type::Array(type_array) =>
                Expression::AsRef(type_array.conversion_from(expr).into()),
            Type::Tuple(type_tuple) if self.mutability.is_some() =>
                Expression::AsMutRef(type_tuple.conversion_from(expr).into()),
            Type::Tuple(type_tuple) =>
                Expression::AsRef(type_tuple.conversion_from(expr).into()),
            _ => panic!("TypeReference::conversion_from: unsupported type: {}", quote!(#self)),
        }
    }

    fn conversion_to(&self, expr: Expression) -> Expression {
        self.elem.conversion_to(expr)
    }

    fn conversion_destroy(&self, expr: Expression) -> Expression {
        match &*self.elem {
            Type::Path(type_path) => type_path.conversion_destroy(expr),
            Type::Slice(type_slice) => type_slice.conversion_destroy(expr),
            _ => panic!("conversion_from::conversion_destroy: unsupported type: {}", quote!(#self)),
        }
    }
}

impl Conversion for TypePath {
    fn conversion_from(&self, expr: Expression) -> Expression {
        let last_segment = self.path.segments.last().unwrap();
        let last_ident = &last_segment.ident;
        if last_ident.is_primitive() {
            expr
        } else if last_ident.is_optional() {
            match path_arguments_to_type_conversions(&last_segment.arguments).first() {
                None => unimplemented!("TypePath::conversion_from: Empty Optional: {}", self.to_token_stream()),
                Some(TypeConversion::Primitive(_)) => Expression::FromOptPrimitive(expr.into()),
                Some(_) => Expression::FromOpt(expr.into()),
            }
        } else if last_ident.is_box() {
            Expression::IntoBox(Expression::From(expr.into()).into())
        } else {
            Expression::From(expr.into())
        }
    }

    fn conversion_to(&self, expr: Expression) -> Expression {
        let last_segment = self.path.segments.last().unwrap();
        let last_ident = &last_segment.ident;
        if last_ident.is_primitive() {
            expr
        } else if last_ident.is_optional() {
            match path_arguments_to_type_conversions(&last_segment.arguments).first() {
                Some(TypeConversion::Primitive(_)) => Expression::ToOptPrimitive(expr.into()),
                Some(TypeConversion::Generic(_)) => Expression::OwnerIteratorPresentation(
                    SequenceOutput::MatchFields((expr.into(), Punctuated::from_iter([
                        OwnedItemPresentableContext::Lambda(quote!(Some(vec)), FFIConversionMethodExpr::FfiTo(quote!(vec)).to_token_stream(), Depunctuated::new()),
                        OwnedItemPresentableContext::Lambda(quote!(None), DictionaryExpr::NullMut.to_token_stream(), Depunctuated::new())
                    ])))),
                Some(_) => Expression::ToOpt(expr.into()),
                None => unimplemented!("TypePath::conversion_to: Empty Optional"),
            }
        } else {
            Expression::To(expr.into())
        }
    }

    fn conversion_destroy(&self, expr: Expression) -> Expression {
        let last_segment = self.path.segments.last().unwrap();
        let last_ident = &last_segment.ident;
        if last_ident.is_primitive() {
            Expression::Empty
        } else if last_ident.is_optional() {
            match path_arguments_to_type_conversions(&last_segment.arguments).first() {
                Some(TypeConversion::Primitive(_)) => Expression::Empty,
                Some(_) => Expression::DestroyOpt(expr.into()),
                None => unimplemented!("TypePath::conversion_destroy: Empty Optional"),
            }
        } else if last_ident.is_string() {
            Expression::DestroyString(expr.into(), self.path.to_token_stream())
        } else if last_ident.is_str() {
            Expression::DestroyString(expr.into(), quote!(&#self))
        } else {
            Expression::UnboxAnyTerminated(expr.into())
        }
    }
}

impl Conversion for TypeTuple {
    fn conversion_from(&self, expr: Expression) -> Expression {
        Expression::FromTuple(expr.into(), self.elems.iter()
            .enumerate()
            .map(|(index, elem)|
                elem.conversion_from(
                    Expression::FfiRefWithConversion(
                        FieldTypeComposition::unnamed(
                            Name::UnnamedArg(index),
                            FieldTypeConversionKind::Type(elem.clone())))
                        .into()))
            .collect())
    }

    fn conversion_to(&self, expr: Expression) -> Expression {
        Expression::To(expr.into())
    }

    fn conversion_destroy(&self, expr: Expression) -> Expression {
        Expression::UnboxAny(expr.into())
    }
}

impl Conversion for TypeTraitObject {
    fn conversion_from(&self, expr: Expression) -> Expression {
        Expression::AsRef(expr.into())
    }

    fn conversion_to(&self, expr: Expression) -> Expression {
        Expression::To(expr.into())
    }

    fn conversion_destroy(&self, expr: Expression) -> Expression {
        Expression::UnboxAny(expr.into())
    }
}

impl Conversion for TypeImplTrait {
    fn conversion_from(&self, expr: Expression) -> Expression {
        Expression::AsRef(expr.into())
    }

    fn conversion_to(&self, _expr: Expression) -> Expression {
        todo!()
    }

    fn conversion_destroy(&self, _expr: Expression) -> Expression {
        todo!()
    }
}
