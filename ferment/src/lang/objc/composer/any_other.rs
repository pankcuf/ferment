use quote::{quote, ToTokens};
use syn::{Generics, PathSegment};
use crate::composer::{AnyOtherComposer, Composer, VarComposer};
use crate::context::{ScopeContext, ScopeSearch, ScopeSearchKey};
use crate::conversion::{DESTROY_COMPLEX, DESTROY_OPT_COMPLEX, DESTROY_OPT_PRIMITIVE, DESTROY_PRIMITIVE, DictFermentableModelKind, DictTypeModelKind, FROM_COMPLEX, FROM_OPT_COMPLEX, FROM_OPT_PRIMITIVE, FROM_PRIMITIVE, GenericTypeKind, RustExpressionComposer, SmartPointerModelKind, TO_COMPLEX, TO_OPT_COMPLEX, TO_OPT_PRIMITIVE, TO_PRIMITIVE, TypeKind, TypeModelKind};
use crate::ext::{CrateExtension, GenericNestedArg, Mangle, ToPath, ToType};
use crate::lang::objc::composers::AttrWrapper;
use crate::lang::objc::ObjCFermentate;
use crate::presentable::Expression;
use crate::presentation::{DictionaryExpr, DictionaryName, FFIConversionFromMethod, InterfacesMethodExpr, Name};

impl<'a> Composer<'a> for AnyOtherComposer<ObjCFermentate, AttrWrapper, Option<Generics>>  {
    type Source = ScopeContext;
    type Output = ObjCFermentate;

    #[allow(unused_variables)]
    fn compose(&self, source: &'a Self::Source) -> Self::Output {
        let ffi_name = self.ty.mangle_ident_default();
        let ffi_type = ffi_name.to_type();
        let arg_0_name = Name::Dictionary(DictionaryName::Obj);

        let path = self.ty.to_path();
        let ctor_path = path.arg_less();

        // Arc/Rc: primitive arg: to: "*obj"
        // Arc/Rc: complex arg: to: "(*obj).clone()"
        // Mutex/RwLock: primitive/complex arg: to: "obj.into_inner().expect("Err")"
        // Arc<RwLock>>: to: obj.borrow().clone()
        // RefCell: primitive/complex arg: to: "obj.into_inner()"
        let obj_by_value = source.maybe_object_by_value(&self.ty);
        let nested_ty = self.ty.first_nested_type().unwrap();
        let maybe_opaque = source.maybe_opaque_object(nested_ty);
        let nested_obj_by_value = source.maybe_object_by_value(nested_ty);
        println!("AnyOther.ty: {}", nested_ty.to_token_stream());
        println!("AnyOther.nested.ty: {}", nested_ty.to_token_stream());
        println!("AnyOther by_value: {}", obj_by_value.as_ref().map_or("None".to_string(), |o| format!("{o}")));
        println!("AnyOther nested: by_value: {}", nested_obj_by_value.as_ref().map_or("None".to_string(), |o| format!("{o}")));
        println!("AnyOther opaque: {}", maybe_opaque.to_token_stream());

        // let compose = |arg_name: &Name, ty: &Type| {
        // };
        let arg_name = &arg_0_name;
        // let ty = nested_ty;
        // compose(&arg_0_name, nested_ty)

        // let search = ScopeSearch::Value(ScopeSearchKey::TypeRef(ty));
        let search = ScopeSearch::Value(ScopeSearchKey::maybe_from_ref(nested_ty).unwrap());
        let ffi_var = VarComposer::new(search.clone()).compose(source).to_type();
        let maybe_obj = source.maybe_object_by_value(nested_ty);
        let maybe_opaque = source.maybe_opaque_object(nested_ty);
        // println!("compose ffi_type: {}", ffi_var.to_token_stream());
        let default_composer_set = maybe_opaque.as_ref().map_or((FROM_COMPLEX(quote!(ffi_ref.#arg_0_name)), Some(TO_COMPLEX), DESTROY_COMPLEX), |_| (FROM_PRIMITIVE(quote!(ffi_ref.#arg_0_name)), Some(TO_PRIMITIVE), DESTROY_COMPLEX));

        let (from_conversion, to_composer, destroy_composer) = match maybe_obj.as_ref().and_then(|o| o.maybe_type_model_kind_ref()) {
            Some(ty_model_kind) => match ty_model_kind {
                TypeModelKind::Dictionary(DictTypeModelKind::Primitive(..)) =>
                    (FROM_PRIMITIVE(quote!(ffi_ref.#arg_0_name)), Some(TO_PRIMITIVE), DESTROY_PRIMITIVE),
                TypeModelKind::Dictionary(DictTypeModelKind::LambdaFn(..)) => {
                    if let Some(lambda_args) = ty_model_kind.maybe_lambda_args() {
                        (Expression::FromLambda(Expression::Simple(quote!((&*ffi_ref.#arg_0_name))).into(), lambda_args), None, DESTROY_PRIMITIVE)
                    } else {
                        (FROM_PRIMITIVE(quote!(ffi_ref.#arg_0_name)), Some(TO_PRIMITIVE), DESTROY_PRIMITIVE)
                    }

                },
                TypeModelKind::Dictionary(DictTypeModelKind::NonPrimitiveOpaque(..)) =>
                    (FROM_PRIMITIVE(quote!(ffi_ref.#arg_0_name)), Some(TO_PRIMITIVE), DESTROY_COMPLEX),
                TypeModelKind::Dictionary(DictTypeModelKind::NonPrimitiveFermentable(kind)) => match kind {
                    DictFermentableModelKind::SmartPointer(SmartPointerModelKind::Box(..)) => (FROM_OPT_COMPLEX(quote!(ffi_ref.#arg_0_name)), Some(TO_OPT_COMPLEX), DESTROY_OPT_COMPLEX),
                    _ => default_composer_set
                },
                TypeModelKind::Optional(model) => match model.first_nested_argument() {
                    Some(nested_arg) => match nested_arg.maybe_type_model_kind_ref() {
                        Some(nested_ty_model_kind) => match nested_ty_model_kind {
                            TypeModelKind::Dictionary(DictTypeModelKind::Primitive(..)) =>
                                (FROM_OPT_PRIMITIVE(quote!(ffi_ref.#arg_0_name)), Some(TO_OPT_PRIMITIVE), DESTROY_OPT_PRIMITIVE),
                            TypeModelKind::Dictionary(DictTypeModelKind::NonPrimitiveFermentable(DictFermentableModelKind::SmartPointer(SmartPointerModelKind::Box(..)))) => {
                                let boxed_comp: RustExpressionComposer = |expr| Expression::MapIntoBox(Expression::InterfacesExpr(InterfacesMethodExpr::FFIConversionFrom(FFIConversionFromMethod::FfiFromOpt, expr)).into());
                                (boxed_comp(quote!(ffi_ref.#arg_0_name)), Some(TO_OPT_COMPLEX), DESTROY_OPT_COMPLEX)
                            },
                            _ => (FROM_OPT_COMPLEX(quote!(ffi_ref.#arg_0_name)), Some(TO_OPT_COMPLEX), DESTROY_OPT_COMPLEX),
                        },
                        _ => (FROM_PRIMITIVE(quote!(ffi_ref.#arg_0_name)), Some(TO_PRIMITIVE), DESTROY_PRIMITIVE),
                    },
                    _ => (FROM_OPT_COMPLEX(quote!(ffi_ref.#arg_0_name)), Some(TO_OPT_COMPLEX), DESTROY_OPT_COMPLEX),
                },
                _ => default_composer_set,
            },
            None => (FROM_PRIMITIVE(quote!(ffi_ref.#arg_0_name)), Some(TO_PRIMITIVE), DESTROY_PRIMITIVE)
        };
        let to_expr = {
            match &path.segments.last() {
                Some(PathSegment { ident, .. }) => match ident.to_string().as_str() {
                    "Arc" | "Rc" => {
                        match TypeKind::from(nested_ty) {
                            TypeKind::Primitive(_) => DictionaryExpr::Deref(arg_0_name.to_token_stream()).to_token_stream(),
                            TypeKind::Complex(_) => {
                                if maybe_opaque.is_some() {
                                    quote!(#ctor_path::into_raw(#arg_0_name).cast_mut())
                                } else {
                                    quote!((*#arg_0_name).clone())
                                }
                            },
                            TypeKind::Generic(nested_generic_ty) => {
                                println!("GENERIC inside Arc/Rc: {}", nested_generic_ty);
                                match nested_generic_ty {
                                    GenericTypeKind::AnyOther(ty) => {
                                        println!("GENERIC (AnyOther) inside Arc/Rc: {}", ty.to_token_stream());
                                        let path = ty.to_path();
                                        match &path.segments.last() {
                                            Some(PathSegment { ident, .. }) => match ident.to_string().as_str() {
                                                "RwLock" | "Mutex" => quote!(std::sync::#ident::new(obj.read().expect("Poisoned").clone())),
                                                _ => quote!((*#arg_0_name).clone())
                                            },
                                            None => quote!((*#arg_0_name).clone())
                                        }
                                    },
                                    _ => quote!((*#arg_0_name).clone())
                                }
                            },
                        }
                    },
                    "Mutex" | "RwLock" => quote!(#arg_0_name.into_inner().expect("Err")),
                    "RefCell" => quote!(#arg_0_name.into_inner()),
                    "Pin" => quote!(&**#arg_0_name),
                    _ => quote!((*#arg_0_name).clone())
                }
                None => quote!((*#arg_0_name).clone())
            }
        };
        let types = (ffi_type.clone(), self.ty.clone());

        // let mut presentations = Depunctuated::new();
        // presentations.push(InterfacePresentation::ConversionFrom {
        //     attrs: self.attrs,
        //     types: types.clone(),
        //     conversions: (
        //         {
        //             let conversion = from_conversion.present(source);
        //             let from = maybe_opaque.as_ref().map_or(quote!(new), |_| quote!(from_raw));
        //             quote! {
        //             let ffi_ref = &*ffi;
        //             #ctor_path::#from(#conversion)
        //         }
        //         },
        //         None
        //     )
        // });
        // match to_composer {
        //     None => {}
        //     Some(to_composer) => {
        //         presentations.push(InterfacePresentation::ConversionTo {
        //             attrs: self.attrs.clone(),
        //             types: types.clone(),
        //             conversions: (
        //                 Expression::<RustFermentate, Vec<Attribute>>::InterfacesExpr(
        //                     InterfacesMethodExpr::Boxed(
        //                         DictionaryExpr::SelfDestructuring(
        //                             CommaPunctuated::from_iter([
        //                                 RustFieldComposer::named(arg_0_name.clone(), FieldTypeKind::Conversion(to_composer(to_expr).present(source)))
        //                             ])
        //                                 .to_token_stream())
        //                             .to_token_stream()))
        //                     .present(source),
        //                 None
        //             )
        //         });
        //     }
        // };
        // presentations.push(InterfacePresentation::ConversionDestroy {
        //     attrs: self.attrs.clone(),
        //     types,
        //     conversions: (
        //         InterfacesMethodExpr::UnboxAny(DictionaryName::Ffi.to_token_stream()).to_token_stream().terminated(),
        //         None
        //     )
        // });

        ObjCFermentate::default()

        // compose_generic_presentation(
        //     ffi_name,
        //     self.attrs.clone(),
        //     Depunctuated::from_iter([
        //         FieldComposer::named(arg_0_name.clone(), FieldTypeKind::Type(ffi_var))
        //     ]),
        //     presentations,
        //     Depunctuated::from_iter([destroy_composer(DictionaryExpr::SelfProp(arg_name.to_token_stream()).to_token_stream()).present(source).terminated()]),
        //     source
        // )
    }
}