use quote::{quote, ToTokens};
use syn::{Lifetime, PathSegment};
use crate::ast::{CommaPunctuated, Depunctuated, SemiPunctuated};
use crate::composer::{AspectPresentable, AttrComposable, GenericComposerInfo, SourceComposable, VarComposer, AnyOtherComposer};
use crate::context::ScopeContext;
use crate::kind::{DictFermentableModelKind, DictTypeModelKind, FieldTypeKind, GenericTypeKind, ObjectKind, SmartPointerModelKind, TypeKind, TypeModelKind};
use crate::ext::{ArgsTransform, GenericNestedArg, LifetimeProcessor, Mangle, MaybeLambdaArgs, PunctuateOne, ToPath, ToType};
use crate::lang::{RustSpecification, Specification};
use crate::presentable::{Aspect, Expression, ScopeContextPresentable};
use crate::presentation::{DictionaryExpr, InterfacePresentation};
impl SourceComposable for AnyOtherComposer<RustSpecification> {
    type Source = ScopeContext;
    type Output = Option<GenericComposerInfo<RustSpecification>>;

    fn compose(&self, source: &Self::Source) -> Self::Output {
        let mut lifetimes = Vec::<Lifetime>::new();
        let ffi_name = self.ty.mangle_tokens_default();
        let arg_0_name = <RustSpecification as Specification>::Name::obj();

        let path = self.ty.to_path();
        let ctor_path = path.arg_less();

        // Arc/Rc: primitive arg: to: "*obj"
        // Arc/Rc: complex arg: to: "(*obj).clone()"
        // Mutex/RwLock: primitive/complex arg: to: "obj.into_inner().expect("Err")"
        // Arc<RwLock>>: to: obj.borrow().clone()
        // RefCell: primitive/complex arg: to: "obj.into_inner()"
        // let obj_by_value = source.maybe_object_by_value(&self.ty);
        let nested_ty = self.ty.maybe_first_nested_type_ref()?;
        lifetimes.extend(nested_ty.unique_lifetimes());
        // let maybe_opaque = source.maybe_opaque_object::<RustFermentate, SPEC>(nested_ty);
        // let nested_obj_by_value = source.maybe_object_by_value(nested_ty);
        // println!("AnyOther.ty: {}", nested_ty.to_token_stream());
        // println!("AnyOther.nested.ty: {}", nested_ty.to_token_stream());
        // println!("AnyOther by_value: {}", obj_by_value.as_ref().map_or("None".to_string(), |o| format!("{o}")));
        // println!("AnyOther nested: by_value: {}", nested_obj_by_value.as_ref().map_or("None".to_string(), |o| format!("{o}")));
        // println!("AnyOther opaque: {}", maybe_opaque.to_token_stream());
        // let _maybe_nested_nested_ty = nested_ty.maybe_first_nested_type_ref();
        // let compose = |arg_name: &Name, ty: &Type| {
        // };
        // let arg_name = &arg_0_name;
        // let ty = nested_ty;
        // compose(&arg_0_name, nested_ty)

        let ffi_var = VarComposer::<RustSpecification>::value(nested_ty)
            .compose(source)
            .to_type();
        let maybe_obj = source.maybe_object_by_value(nested_ty);
        let maybe_opaque = source.maybe_opaque_object::<RustSpecification>(nested_ty);
        let is_opaque = maybe_opaque.is_some();
        // println!("compose ffi_type: {}", ffi_var.to_token_stream());
        let to_expr = {
            match &path.segments.last() {
                Some(PathSegment { ident, .. }) => match ident.to_string().as_str() {
                    "Arc" | "Rc" => {
                        match TypeKind::from(nested_ty) {
                            TypeKind::Primitive(_) =>
                                DictionaryExpr::deref(&arg_0_name).to_token_stream(),
                            TypeKind::Complex(_) => {
                                if maybe_opaque.is_some() {
                                    quote!(#ctor_path::into_raw(#arg_0_name).cast_mut())
                                } else {
                                    quote!((*#arg_0_name).clone())
                                }
                            },
                            TypeKind::Generic(GenericTypeKind::AnyOther(ty)) => {
                                match &ty.to_path().segments.last() {
                                    Some(PathSegment { ident, .. }) => match ident.to_string().as_str() {
                                        "RwLock" | "Mutex" => quote!(std::sync::#ident::new(obj.read().expect("Poisoned").clone())),
                                        _ => quote!((*#arg_0_name).clone())
                                    },
                                    None => quote!((*#arg_0_name).clone())
                                }
                            },
                            TypeKind::Generic(..) =>
                                quote!((*#arg_0_name).clone()),
                        }
                    },
                    "Mutex" | "RwLock" => {
                        // let expr = ConversionToComposer::<RustSpecification>::value_ref(&arg_0_name, nested_ty).compose(source);
                        quote!(#arg_0_name.into_inner().expect("Err"))
                    },
                    "RefCell" => quote!(#arg_0_name.into_inner()),
                    "Pin" => quote!(&**#arg_0_name),
                    _ => quote!((*#arg_0_name).clone())
                }
                None => quote!((*#arg_0_name).clone())
            }
        };
        let (from_conversion, to_conversion, destroy_conversion) = match maybe_obj.as_ref().and_then(ObjectKind::maybe_type_model_kind_ref) {
            Some(ty_model_kind) => match ty_model_kind {
                TypeModelKind::Dictionary(DictTypeModelKind::Primitive(..)) => (
                    Expression::<RustSpecification>::from_primitive_tokens(DictionaryExpr::ffi_ref_prop(&arg_0_name)),
                    Some(Expression::<RustSpecification>::ffi_to_primitive_tokens(to_expr)),
                    Expression::<RustSpecification>::destroy_primitive_tokens(DictionaryExpr::self_prop(&arg_0_name))
                ),
                TypeModelKind::Dictionary(DictTypeModelKind::LambdaFn(..)) => {
                    if let Some(lambda_args) = MaybeLambdaArgs::<RustSpecification>::maybe_lambda_arg_names(ty_model_kind) {
                        (
                            Expression::from_lambda(Expression::Simple(quote!((&*ffi_ref.#arg_0_name))), lambda_args),
                            None,
                            Expression::destroy_primitive_tokens(DictionaryExpr::self_prop(&arg_0_name))
                        )
                    } else {
                        (
                            Expression::from_primitive(Expression::<RustSpecification>::FfiRefWithName(arg_0_name.clone())),
                            Some(Expression::ffi_to_primitive_tokens(to_expr)),
                            Expression::destroy_primitive_tokens(DictionaryExpr::self_prop(&arg_0_name))
                        )
                    }
                },
                TypeModelKind::Dictionary(DictTypeModelKind::NonPrimitiveOpaque(..)) => (
                    Expression::from_primitive_tokens(DictionaryExpr::ffi_ref_prop(&arg_0_name)),
                    Some(Expression::ffi_to_primitive_tokens(to_expr)),
                    Expression::destroy_complex_tokens(DictionaryExpr::self_prop(&arg_0_name))
                ),
                TypeModelKind::Dictionary(DictTypeModelKind::NonPrimitiveFermentable(kind)) => match kind {
                    DictFermentableModelKind::SmartPointer(SmartPointerModelKind::Box(..)) => (
                        Expression::from_complex_opt(Expression::FfiRefWithName(arg_0_name.clone())),
                        Some(Expression::ffi_to_complex_opt_tokens(to_expr)),
                        Expression::destroy_complex_opt_tokens(DictionaryExpr::self_prop(&arg_0_name))
                    ),
                    _ if is_opaque => (
                        Expression::from_primitive_tokens(DictionaryExpr::ffi_ref_prop(&arg_0_name)),
                        Some(Expression::ffi_to_primitive_tokens(to_expr)),
                        Expression::destroy_complex_tokens(DictionaryExpr::self_prop(&arg_0_name))
                    ),
                    _ => (
                        Expression::from_complex_tokens(DictionaryExpr::ffi_ref_prop(&arg_0_name)),
                        Some(Expression::ffi_to_complex_tokens(to_expr)),
                        Expression::destroy_complex_tokens(DictionaryExpr::self_prop(&arg_0_name))
                    )
                },
                TypeModelKind::Optional(model) => match model.first_nested_argument() {
                    Some(nested_arg) => match nested_arg.maybe_type_model_kind_ref() {
                        Some(nested_ty_model_kind) => match nested_ty_model_kind {
                            TypeModelKind::Dictionary(DictTypeModelKind::Primitive(..)) => (
                                Expression::from_primitive_opt_tokens(DictionaryExpr::ffi_ref_prop(&arg_0_name)),
                                Some(Expression::ffi_to_primitive_opt_tokens(to_expr)),
                                Expression::destroy_primitive_opt_tokens(DictionaryExpr::self_prop(&arg_0_name))
                            ),
                            TypeModelKind::Dictionary(DictTypeModelKind::NonPrimitiveFermentable(DictFermentableModelKind::SmartPointer(SmartPointerModelKind::Box(..)))) => (
                                Expression::map_into_box(Expression::from_complex_opt_tokens(DictionaryExpr::ffi_ref_prop(&arg_0_name))),
                                Some(Expression::ffi_to_complex_opt_tokens(to_expr)),
                                Expression::destroy_complex_opt_tokens(DictionaryExpr::self_prop(&arg_0_name))
                            ),
                            _ => (
                                Expression::from_complex_opt_tokens(DictionaryExpr::ffi_ref_prop(&arg_0_name)),
                                Some(Expression::ffi_to_complex_opt_tokens(to_expr)),
                                Expression::destroy_complex_opt_tokens(DictionaryExpr::self_prop(&arg_0_name))
                            ),
                        },
                        _ => (
                            Expression::from_primitive_tokens(DictionaryExpr::ffi_ref_prop(&arg_0_name)),
                            Some(Expression::ffi_to_primitive_tokens(to_expr)),
                            Expression::destroy_primitive_tokens(DictionaryExpr::self_prop(&arg_0_name))
                        ),
                    },
                    _ => (
                        Expression::from_complex_opt_tokens(DictionaryExpr::ffi_ref_prop(&arg_0_name)),
                        Some(Expression::ffi_to_complex_opt_tokens(to_expr)),
                        Expression::destroy_complex_opt_tokens(DictionaryExpr::self_prop(&arg_0_name))
                    ),
                },
                _ if is_opaque => (
                    Expression::from_primitive_tokens(DictionaryExpr::ffi_ref_prop(&arg_0_name)),
                    Some(Expression::ffi_to_primitive_tokens(to_expr)),
                    Expression::destroy_complex_tokens(DictionaryExpr::self_prop(&arg_0_name))
                ),
                _ => (
                    Expression::from_complex_tokens(DictionaryExpr::ffi_ref_prop(&arg_0_name)),
                    Some(Expression::ffi_to_complex_tokens(to_expr)),
                    Expression::destroy_complex_tokens(DictionaryExpr::self_prop(&arg_0_name))
                ),
            },
            None => (
                Expression::from_primitive_tokens(DictionaryExpr::ffi_ref_prop(&arg_0_name)),
                Some(Expression::ffi_to_primitive_tokens(to_expr)),
                Expression::destroy_primitive_tokens(DictionaryExpr::self_prop(&arg_0_name))
            )
        };

        let types = (self.present_ffi_aspect(), self.present_target_aspect());
        let attrs = self.compose_attributes();
        let mut interfaces = Depunctuated::new();
        let from_body = {
            let conversion = from_conversion.present(source);
            let from = maybe_opaque.as_ref().map_or(quote!(new), |_| quote!(from_raw));
            quote!(#ctor_path::#from(#conversion))
        };
        interfaces.push(InterfacePresentation::conversion_from_root(&attrs, &types, from_body, &None, &lifetimes));
        if let Some(to_conversion) = to_conversion {
            let expr_to_iter = [
                arg_0_name.field_composer(FieldTypeKind::Conversion(to_conversion.present(source)))
            ];
            let to_body = CommaPunctuated::from_iter(expr_to_iter).present(source);
            interfaces.push(InterfacePresentation::conversion_to_boxed_self_destructured(&attrs, &types, to_body, &None, &lifetimes));
        }
        let field_composers = arg_0_name.field_composer(FieldTypeKind::Type(ffi_var)).punctuate_one();
        let expr_destroy_iterator = [
            destroy_conversion.present(source)
        ];
        interfaces.push(InterfacePresentation::drop(&attrs, ffi_name.to_type(), SemiPunctuated::from_iter(expr_destroy_iterator)));
        let aspect = Aspect::raw_struct_ident(self.ty.mangle_ident_default());
        Some(GenericComposerInfo::<RustSpecification>::default(aspect, &attrs, field_composers, interfaces))
    }
}

