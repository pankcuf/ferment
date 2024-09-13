use std::marker::PhantomData;
use std::rc::Rc;
use quote::{quote, ToTokens};
use syn::{Attribute, Generics, PathSegment, Type};
use crate::ast::{CommaPunctuated, Depunctuated, SemiPunctuated};
use crate::composable::{AttrsModel, FieldComposer, FieldTypeKind, GenModel, RustFieldComposer};
use crate::composer::{Composer, GenericComposerInfo, NameComposable, ComposerLink, VarComposer, BasicComposer, constants, BasicComposerOwner, AttrComposable, SourceAccessible, NameContext};
use crate::context::{ScopeContext, ScopeSearch, ScopeSearchKey};
use crate::conversion::{DESTROY_COMPLEX, DESTROY_OPT_COMPLEX, DESTROY_OPT_PRIMITIVE, DESTROY_PRIMITIVE, DictFermentableModelKind, DictTypeModelKind, FROM_COMPLEX, FROM_OPT_COMPLEX, FROM_OPT_PRIMITIVE, FROM_PRIMITIVE, GenericTypeKind, RustExpressionComposer, SmartPointerModelKind, TO_COMPLEX, TO_OPT_COMPLEX, TO_OPT_PRIMITIVE, TO_PRIMITIVE, TypeKind, TypeModelKind};
use crate::ext::{CrateExtension, GenericNestedArg, Mangle, ToPath, ToType};
use crate::lang::{LangAttrSpecification, LangGenSpecification};
use crate::presentable::{Context, Expression, OwnedItemPresentableContext, ScopeContextPresentable, SequenceOutput};
use crate::presentation::{DictionaryExpr, DictionaryName, FFIConversionFromMethod, InterfacePresentation, InterfacesMethodExpr, Name, RustFermentate};

pub struct AnyOtherComposer<LANG, SPEC, Gen>
    where LANG: Clone + 'static,
          SPEC: LangAttrSpecification<LANG> + 'static,
          Gen: LangGenSpecification<LANG> + 'static,
          SequenceOutput<LANG, SPEC>: ScopeContextPresentable,
          OwnedItemPresentableContext<LANG, SPEC>: ScopeContextPresentable {
    pub ty: Type,
    base: BasicComposer<ComposerLink<Self>, LANG, SPEC, Gen>,
    phantom_data: PhantomData<LANG>,
}

impl<LANG, SPEC, Gen> AnyOtherComposer<LANG, SPEC, Gen>
    where LANG: Clone,
          SPEC: LangAttrSpecification<LANG>,
          Gen: LangGenSpecification<LANG>,
          SequenceOutput<LANG, SPEC>: ScopeContextPresentable,
          OwnedItemPresentableContext<LANG, SPEC>: ScopeContextPresentable {
    pub fn new(ty: &Type, context: Context, attrs: Vec<Attribute>, scope_context: &ComposerLink<ScopeContext>) -> Self {
        Self {
            base: BasicComposer::<ComposerLink<Self>, LANG, SPEC, Gen>::from(AttrsModel::from(&attrs), context, GenModel::default(), constants::composer_doc(), Rc::clone(scope_context)),
            ty: ty.clone(),
            phantom_data: PhantomData
        }
    }
}
impl<LANG, SPEC, Gen> NameContext<Context> for AnyOtherComposer<LANG, SPEC, Gen>
    where LANG: Clone,
          SPEC: LangAttrSpecification<LANG>,
          Gen: LangGenSpecification<LANG>,
          SequenceOutput<LANG, SPEC>: ScopeContextPresentable,
          OwnedItemPresentableContext<LANG, SPEC>: ScopeContextPresentable {
    fn name_context_ref(&self) -> &Context {
        self.base().name_context_ref()
    }
}
impl<LANG, SPEC, Gen> SourceAccessible for AnyOtherComposer<LANG, SPEC, Gen>
    where LANG: Clone,
          SPEC: LangAttrSpecification<LANG>,
          Gen: LangGenSpecification<LANG>,
          SequenceOutput<LANG, SPEC>: ScopeContextPresentable,
          OwnedItemPresentableContext<LANG, SPEC>: ScopeContextPresentable {
    fn context(&self) -> &ComposerLink<ScopeContext> {
        self.base().context()
    }
}

impl<LANG, SPEC, Gen> BasicComposerOwner<Context, LANG, SPEC, Gen> for AnyOtherComposer<LANG, SPEC, Gen>
    where LANG: Clone,
          SPEC: LangAttrSpecification<LANG>,
          Gen: LangGenSpecification<LANG>,
          SequenceOutput<LANG, SPEC>: ScopeContextPresentable,
          OwnedItemPresentableContext<LANG, SPEC>: ScopeContextPresentable {
    fn base(&self) -> &BasicComposer<ComposerLink<Self>, LANG, SPEC, Gen> {
        &self.base
    }
}

impl<'a> Composer<'a> for AnyOtherComposer<RustFermentate, Vec<Attribute>, Option<Generics>> {
    type Source = ScopeContext;
    type Output = Option<GenericComposerInfo<RustFermentate, Vec<Attribute>, Option<Generics>>>;

    fn compose(&self, source: &'a Self::Source) -> Self::Output {
        let ffi_name = self.ty.mangle_ident_default();
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
        let default_composer_set = maybe_opaque.as_ref()
            .map_or(
                (FROM_COMPLEX(quote!(ffi_ref.#arg_0_name)), Some(TO_COMPLEX), DESTROY_COMPLEX),
                |_| (FROM_PRIMITIVE(quote!(ffi_ref.#arg_0_name)), Some(TO_PRIMITIVE), DESTROY_COMPLEX));

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
        let types = (self.compose_ffi_name(), self.compose_target_name());
        let attrs = self.base.compose_attributes();
        let mut interfaces = Depunctuated::new();
        let from_body = {
            let conversion = from_conversion.present(source);
            let from = maybe_opaque.as_ref().map_or(quote!(new), |_| quote!(from_raw));
            quote!(#ctor_path::#from(#conversion))
        };
        interfaces.push(InterfacePresentation::conversion_from_root(&attrs, &types, from_body, &None));
        if let Some(to_composer) = to_composer {
            let expr_to_iter = [
                RustFieldComposer::named(arg_0_name.clone(), FieldTypeKind::Conversion(to_composer(to_expr).present(source)))
            ];
            let to_body = CommaPunctuated::from_iter(expr_to_iter).present(source);
            interfaces.push(InterfacePresentation::conversion_to_boxed_self_destructured(&attrs, &types, to_body, &None));
        }
        interfaces.push(InterfacePresentation::conversion_unbox_any_terminated(&attrs, &types, DictionaryName::Ffi, &None));
        let field_composers = Depunctuated::from_iter([
            FieldComposer::named(arg_0_name.clone(), FieldTypeKind::Type(ffi_var))
        ]);
        let expr_destroy_iterator = [
            destroy_composer(DictionaryExpr::SelfProp(arg_name.to_token_stream()).to_token_stream()).present(source)
        ];
        interfaces.push(InterfacePresentation::drop(&attrs, ffi_name.to_type(), SemiPunctuated::from_iter(expr_destroy_iterator)));
        Some(GenericComposerInfo::<RustFermentate, Vec<Attribute>, Option<Generics>>::default(ffi_name, &attrs, field_composers, interfaces))
    }
}
