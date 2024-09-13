use std::fmt::Debug;
use quote::{quote, ToTokens};
use syn::{Type, TypeArray, TypeImplTrait, TypePath, TypePtr, TypeReference, TypeSlice, TypeTraitObject, TypeTuple};
use syn::punctuated::Punctuated;
use crate::composable::{FieldComposer, GenericBoundsModel};
use crate::composer::{Composer, DestroyConversionComposer, FromConversionFullComposer, ToConversionComposer};
use crate::context::{ScopeContext, ScopeSearch, ScopeSearchKey};
use crate::conversion::TypeKind;
use crate::ext::{DictionaryType, Mangle, path_arguments_to_type_conversions};
use crate::lang::LangAttrSpecification;
use crate::presentable::{Expression, OwnedItemPresentableContext, SequenceOutput};
use crate::presentation::{DictionaryExpr, FFIConversionToMethodExpr, Name};

#[derive(Clone, Debug)]
pub enum ConversionType<LANG, SPEC>
    where LANG: Clone,
          SPEC: LangAttrSpecification<LANG> {
    From(Name, Type, Option<Expression<LANG, SPEC>>),
    To(ToConversionComposer<LANG, SPEC>),
    Destroy(DestroyConversionComposer<LANG, SPEC>),
    // Variable(VariableComposer)
}

// #[allow(unused)]
// impl ConversionType {
//     pub fn expr(&self) -> &Option<Expression> {
//         match self {
//             ConversionType::From(composer) => &composer.expr,
//             ConversionType::To(composer) => &composer.expr,
//             ConversionType::Destroy(composer) => &composer.expr,
//             // ConversionType::Variable(composer) => &None,
//         }
//     }
// }

impl<'a, LANG, SPEC> Composer<'a> for ConversionType<LANG, SPEC>
    where LANG: Clone + Debug,
          SPEC: LangAttrSpecification<LANG> + Debug {
    type Source = ScopeContext;
    type Output = Expression<LANG, SPEC>;

    fn compose(&self, source: &'a Self::Source) -> Self::Output {
        match self {
            ConversionType::From(name, ty, expr) => {
                FromConversionFullComposer::new(name.clone(), ScopeSearch::KeyInScope(ScopeSearchKey::maybe_from_ref(ty).unwrap(), &source.scope), expr.clone()).compose(source)
                // composer.compose(source)
            },
            ConversionType::To(composer) =>
                composer.compose(source),
            ConversionType::Destroy(composer) =>
                composer.compose(source),
            // ConversionType::Variable(composer) =>
            //     composer.compose(source)
        }
    }
}

pub trait ConversionTrait<LANG, SPEC>
    where LANG: Clone,
          SPEC: LangAttrSpecification<LANG> {
    fn conversion_from(&self, expr: Expression<LANG, SPEC>) -> Expression<LANG, SPEC>;
    fn conversion_to(&self, expr: Expression<LANG, SPEC>) -> Expression<LANG, SPEC>;
    fn conversion_destroy(&self, expr: Expression<LANG, SPEC>) -> Expression<LANG, SPEC>;
}

impl<LANG, SPEC> ConversionTrait<LANG, SPEC> for FieldComposer<LANG, SPEC>
    where LANG: Clone + Debug,
          SPEC: LangAttrSpecification<LANG> + Debug {
    fn conversion_from(&self, expr: Expression<LANG, SPEC>) -> Expression<LANG, SPEC> {
        self.ty().conversion_from(expr)
    }

    fn conversion_to(&self, expr: Expression<LANG, SPEC>) -> Expression<LANG, SPEC> {
        self.ty().conversion_to(expr)
    }

    fn conversion_destroy(&self, expr: Expression<LANG, SPEC>) -> Expression<LANG, SPEC> {
        self.ty().conversion_destroy(expr)
    }
}

impl<LANG, SPEC> ConversionTrait<LANG, SPEC> for Type
    where LANG: Clone + Debug,
          SPEC: LangAttrSpecification<LANG> + Debug {
    fn conversion_from(&self, expr: Expression<LANG, SPEC>) -> Expression<LANG, SPEC> {
        //println!("Type::conversion_from: {}", expr);
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

    fn conversion_to(&self, expr: Expression<LANG, SPEC>) -> Expression<LANG, SPEC> {
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

    fn conversion_destroy(&self, expr: Expression<LANG, SPEC>) -> Expression<LANG, SPEC> {
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

impl<LANG, SPEC> ConversionTrait<LANG, SPEC> for TypeArray
    where LANG: Clone,
          SPEC: LangAttrSpecification<LANG> {
    fn conversion_from(&self, expr: Expression<LANG, SPEC>) -> Expression<LANG, SPEC> {
        Expression::From(expr.into())
    }

    fn conversion_to(&self, expr: Expression<LANG, SPEC>) -> Expression<LANG, SPEC> {
        Expression::To(expr.into())
    }

    fn conversion_destroy(&self, expr: Expression<LANG, SPEC>) -> Expression<LANG, SPEC> {
        Expression::UnboxAny(expr.into())
    }
}

impl<LANG, SPEC> ConversionTrait<LANG, SPEC> for TypeSlice
    where LANG: Clone,
          SPEC: LangAttrSpecification<LANG> {
    fn conversion_from(&self, expr: Expression<LANG, SPEC>) -> Expression<LANG, SPEC> {
        let ty = &*self.elem;
        let ffi_type = self.mangle_ident_default();
        Expression::AsSlice(
            Expression::CastFrom(
                expr.into(),
                quote!(Vec<#ty>),
                quote!(crate::fermented::generics::#ffi_type)).into())
    }

    fn conversion_to(&self, expr: Expression<LANG, SPEC>) -> Expression<LANG, SPEC> {
        Expression::To(Expression::ToVec(expr.into()).into())
    }

    fn conversion_destroy(&self, expr: Expression<LANG, SPEC>) -> Expression<LANG, SPEC> {
        Expression::UnboxAny(expr.into())
    }
}
impl<LANG, SPEC> ConversionTrait<LANG, SPEC> for TypePtr
    where LANG: Clone + Debug,
          SPEC: LangAttrSpecification<LANG> + Debug {
    fn conversion_from(&self, expr: Expression<LANG, SPEC>) -> Expression<LANG, SPEC> {
        println!("TypePtr::conversion_from: {} === {}", self.to_token_stream(), expr);
        match &*self.elem {
            Type::Ptr(type_ptr) => match &*type_ptr.elem {
                Type::Path(_type_path) => Expression::FromOffsetMap,
                _ => Expression::From(expr.into()),
            },
            Type::Path(..) => expr,
                // Expression::FromRawParts(type_path
                //     .path
                //     .segments
                //     .last()
                //     .unwrap()
                //     .ident
                //     .to_token_stream()),
            _ => Expression::From(expr.into()),
        }
    }

    fn conversion_to(&self, expr: Expression<LANG, SPEC>) -> Expression<LANG, SPEC> {
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

    fn conversion_destroy(&self, expr: Expression<LANG, SPEC>) -> Expression<LANG, SPEC> {
        match &*self.elem {
            Type::Ptr(type_ptr) => type_ptr.conversion_destroy(expr),
            Type::Path(type_path) => type_path.conversion_destroy(expr),
            _ => panic!("Can't destroy_ptr: of type: {}", quote!(#self)),
        }
    }
}

impl<LANG, SPEC> ConversionTrait<LANG, SPEC> for TypeReference
    where LANG: Clone + Debug,
          SPEC: LangAttrSpecification<LANG> + Debug {
    fn conversion_from(&self, expr: Expression<LANG, SPEC>) -> Expression<LANG, SPEC> {
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

    fn conversion_to(&self, expr: Expression<LANG, SPEC>) -> Expression<LANG, SPEC> {
        self.elem.conversion_to(expr)
    }

    fn conversion_destroy(&self, expr: Expression<LANG, SPEC>) -> Expression<LANG, SPEC> {
        match &*self.elem {
            Type::Path(type_path) => type_path.conversion_destroy(expr),
            Type::Slice(type_slice) => type_slice.conversion_destroy(expr),
            _ => panic!("conversion_from::conversion_destroy: unsupported type: {}", quote!(#self)),
        }
    }
}

impl<LANG, SPEC> ConversionTrait<LANG, SPEC> for TypePath
    where LANG: Clone,
          SPEC: LangAttrSpecification<LANG>,
          Box<Expression<LANG, SPEC>>: From<Expression<LANG, SPEC>> {
    fn conversion_from(&self, expr: Expression<LANG, SPEC>) -> Expression<LANG, SPEC> {
        let last_segment = self.path.segments.last().unwrap();
        let last_ident = &last_segment.ident;
        if last_ident.is_primitive() {
            expr
        } else if last_ident.is_optional() {
            match path_arguments_to_type_conversions(&last_segment.arguments).first() {
                None => unimplemented!("TypePath::conversion_from: Empty Optional: {}", self.to_token_stream()),
                Some(TypeKind::Primitive(_)) => Expression::FromOptPrimitive(expr.into()),
                Some(_) => Expression::FromOpt(expr.into()),
            }
        } else if last_ident.is_box() {
            Expression::IntoBox(Expression::From(expr.into()).into())
        } else {
            Expression::From(expr.into())
        }
    }

    fn conversion_to(&self, expr: Expression<LANG, SPEC>) -> Expression<LANG, SPEC> {
        let last_segment = self.path.segments.last().unwrap();
        let last_ident = &last_segment.ident;
        if last_ident.is_primitive() {
            expr
        } else if last_ident.is_optional() {
            match path_arguments_to_type_conversions(&last_segment.arguments).first() {
                Some(TypeKind::Primitive(_)) => Expression::ToOptPrimitive(expr.into()),
                Some(TypeKind::Generic(_)) =>
                    // Expression::Expr(Expr::Match(ExprMatch {
                    //     attrs: vec![],
                    //     match_token: Default::default(),
                    //     expr: Box::new(Expr::),
                    //     brace_token: Default::default(),
                    //     arms: vec![
                    //         Arm {
                    //             attrs: vec![],
                    //             pat: Pat::Verbatim(quote!(Some(vec))),
                    //             guard: None,
                    //             fat_arrow_token: Default::default(),
                    //             body: Box::new(Expr::Verbatim(FFIConversionMethodExpr::FfiTo(quote!(vec)).to_token_stream())),
                    //             comma: Some(Default::default()),
                    //         },
                    //         Arm {
                    //             attrs: vec![],
                    //             pat: Pat::Verbatim(quote!(None)),
                    //             guard: None,
                    //             fat_arrow_token: Default::default(),
                    //             body: Box::new(Expr::Verbatim(DictionaryExpr::NullMut.to_token_stream())),
                    //             comma: None,
                    //         }
                    //     ],
                    // }))

                    Expression::OwnerIteratorPresentation(
                    SequenceOutput::MatchFields((expr.into(), Punctuated::from_iter([
                        OwnedItemPresentableContext::Lambda(quote!(Some(vec)), FFIConversionToMethodExpr::FfiTo(quote!(vec)).to_token_stream(), Default::default()),
                        OwnedItemPresentableContext::Lambda(quote!(None), DictionaryExpr::NullMut.to_token_stream(), Default::default())
                    ])))),
                Some(_) => Expression::ToOpt(expr.into()),
                None => unimplemented!("TypePath::conversion_to: Empty Optional"),
            }
        } else {
            Expression::To(expr.into())
        }
    }

    fn conversion_destroy(&self, expr: Expression<LANG, SPEC>) -> Expression<LANG, SPEC> {
        let last_segment = self.path.segments.last().unwrap();
        let last_ident = &last_segment.ident;
        if last_ident.is_primitive() {
            Expression::Empty
        } else if last_ident.is_optional() {
            match path_arguments_to_type_conversions(&last_segment.arguments).first() {
                Some(TypeKind::Primitive(_)) => Expression::Empty,
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

impl<LANG, SPEC> ConversionTrait<LANG, SPEC> for TypeTuple
    where LANG: Clone + Debug,
          SPEC: LangAttrSpecification<LANG> + Debug {
    fn conversion_from(&self, expr: Expression<LANG, SPEC>) -> Expression<LANG, SPEC> {
        Expression::FromTuple(expr.into(), self.elems.iter()
            .enumerate()
            .map(|(index, elem)|
                elem.conversion_from(Expression::FfiRefWithName(Name::UnnamedArg(index))))
            .collect())
    }

    fn conversion_to(&self, expr: Expression<LANG, SPEC>) -> Expression<LANG, SPEC> {
        Expression::To(expr.into())
    }

    fn conversion_destroy(&self, expr: Expression<LANG, SPEC>) -> Expression<LANG, SPEC> {
        Expression::UnboxAny(expr.into())
    }
}

impl<LANG, SPEC> ConversionTrait<LANG, SPEC> for TypeTraitObject
    where LANG: Clone,
          SPEC: LangAttrSpecification<LANG> {
    fn conversion_from(&self, expr: Expression<LANG, SPEC>) -> Expression<LANG, SPEC> {
        Expression::AsRef(expr.into())
    }

    fn conversion_to(&self, expr: Expression<LANG, SPEC>) -> Expression<LANG, SPEC> {
        Expression::To(expr.into())
    }

    fn conversion_destroy(&self, expr: Expression<LANG, SPEC>) -> Expression<LANG, SPEC> {
        Expression::UnboxAny(expr.into())
    }
}

impl<LANG, SPEC> ConversionTrait<LANG, SPEC> for TypeImplTrait
    where LANG: Clone,
          SPEC: LangAttrSpecification<LANG> {
    fn conversion_from(&self, expr: Expression<LANG, SPEC>) -> Expression<LANG, SPEC> {
        Expression::AsRef(expr.into())
    }

    fn conversion_to(&self, expr: Expression<LANG, SPEC>) -> Expression<LANG, SPEC> {
        Expression::To(expr.into())
    }

    fn conversion_destroy(&self, expr: Expression<LANG, SPEC>) -> Expression<LANG, SPEC> {
        Expression::UnboxAny(expr.into())
    }
}

impl<LANG, SPEC> ConversionTrait<LANG, SPEC> for GenericBoundsModel
    where LANG: Clone,
          SPEC: LangAttrSpecification<LANG> {
    fn conversion_from(&self, expr: Expression<LANG, SPEC>) -> Expression<LANG, SPEC> {
        expr
    }

    fn conversion_to(&self, expr: Expression<LANG, SPEC>) -> Expression<LANG, SPEC> {
        Expression::To(expr.into())
    }

    fn conversion_destroy(&self, expr: Expression<LANG, SPEC>) -> Expression<LANG, SPEC> {
        Expression::UnboxAny(expr.into())
    }
}