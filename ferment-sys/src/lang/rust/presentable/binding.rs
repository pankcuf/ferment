use quote::{quote, ToTokens};
use syn::{parse_quote, ReturnType};
use syn::token::RArrow;
use crate::composer::{NameKind, SourceComposable, CommaPunctuatedArgs};
use crate::context::ScopeContext;
use crate::kind::SmartPointerKind;
use crate::ext::{Accessory, Mangle, Primitive, ToPath, ToType};
use crate::lang::{RustSpecification, Specification};
use crate::presentable::{ArgKind, BindingPresentableContext, ScopeContextPresentable, SmartPointerPresentableContext};
use crate::presentation::{present_pub_function, ArgPresentation, BindingPresentation, DictionaryExpr, InterfacePresentation, InterfacesMethodExpr, Name};

impl ScopeContextPresentable for BindingPresentableContext<RustSpecification> {
    type Presentation = BindingPresentation;

    fn present(&self, source: &ScopeContext) -> Self::Presentation {
        match self {
            Self::Constructor(aspect, signature_aspect, name_kind, args, body) => {
                let ty = aspect.present(source);
                let body = body.present(source);
                let body_presentation = match name_kind {
                    NameKind::Unnamed => quote!((#body)),
                    _ => quote!({#body})
                };
                BindingPresentation::Constructor {
                    aspect: signature_aspect.clone(),
                    name: Name::<RustSpecification>::Constructor(ty.clone()).mangle_tokens_default(),
                    ty: ty.clone(),
                    ctor_arguments: args.present(&source),
                    body_presentation
                }
            },
            Self::VariantConstructor(aspect, signature_aspect, name_kind, args, body) => {
                let ty = aspect.present(source);
                let body = body.present(source);
                let body_presentation = match name_kind {
                    NameKind::Unnamed => quote!((#body)),
                    _ => quote!({#body})
                };


                BindingPresentation::VariantConstructor {
                    aspect: signature_aspect.clone(),
                    name: Name::<RustSpecification>::Constructor(ty.clone()).mangle_tokens_default(),
                    ty: ty.clone(),
                    ctor_arguments: args.present(&source),
                    body_presentation
                }
            },
            Self::Destructor(aspect, signature_aspect, _name_kind) => {
                let ty = aspect.present(source);
                BindingPresentation::Destructor {
                    aspect: signature_aspect.clone(),
                    name: Name::<RustSpecification>::Destructor(ty.clone()).mangle_tokens_default(),
                    ty: ty.clone(),
                }
            },
            Self::Getter(obj_aspect, signature_aspect, field_type, field_name) => {
                let obj_type = obj_aspect.present(source);

                BindingPresentation::Getter {
                    aspect: signature_aspect.clone(),
                    name: Name::<RustSpecification>::getter(obj_type.to_path(), &field_name).mangle_tokens_default(),
                    field_name: field_name.clone(),
                    obj_type: obj_type.clone(),
                    field_type: field_type.compose(source).to_type(),
                }
            },
            Self::Setter(obj_aspect, signature_aspect, field_type, field_name, ) => {
                let obj_type = obj_aspect.present(source);
                BindingPresentation::Setter {
                    aspect: signature_aspect.clone(),
                    name: Name::<RustSpecification>::setter(obj_type.to_path(), &field_name).mangle_tokens_default(),
                    field_name: field_name.clone(),
                    obj_type: obj_type.clone(),
                    field_type: field_type.compose(source).to_type(),
                }
            },
            Self::RegFn(path, signature_aspect, is_async, arguments, return_type, input_conversions, return_type_conversion) => BindingPresentation::RegularFunction {
                aspect: signature_aspect.clone(),
                is_async: *is_async,
                arguments: arguments.present(&source),
                name: Name::<RustSpecification>::ModFn(path.clone()).mangle_tokens_default(),
                input_conversions: input_conversions.present(&source),
                return_type: return_type.clone(),
                output_conversions: <<RustSpecification as Specification>::Expr as ScopeContextPresentable>::present(return_type_conversion, source).to_token_stream()
            },
            Self::RegFn2(path, signature_aspect, is_async, argument_names, arguments, return_type, full_fn_path, input_conversions, return_type_conversion) => BindingPresentation::RegularFunction2 {
                aspect: signature_aspect.clone(),
                is_async: *is_async,
                argument_names: argument_names.clone(),
                arguments: arguments.present(&source),
                name: Name::<RustSpecification>::ModFn(path.clone()).mangle_tokens_default(),
                full_fn_path: full_fn_path.clone(),
                input_conversions: input_conversions.present(&source),
                return_type: return_type.clone(),
                output_conversions: <<RustSpecification as Specification>::Expr as ScopeContextPresentable>::present(return_type_conversion, source).to_token_stream()
            },
            Self::TraitVTableInnerFn((attrs, ..), ident, name_and_args, return_type_conversion) => {
                let arguments = name_and_args.present(source);
                BindingPresentation::TraitVTableInnerFn {
                    attrs: attrs.clone(),
                    name: Name::<RustSpecification>::VTableInnerFn(ident.clone()).mangle_tokens_default(),
                    name_and_args: quote!(unsafe extern "C" fn (#arguments)),
                    output_expression: return_type_conversion.clone(),
                }
            },
            Self::SmartPointer(aspect, signature_aspect, SmartPointerKind::Mutex(..), SmartPointerPresentableContext::Ctor(ctor_arg_composer, from_arg_conversion)) => {
                let ty = aspect.present(source);
                let body = ArgKind::DefaultFieldByValueConversion(ctor_arg_composer.clone(), <RustSpecification as Specification>::Expr::boxed(from_arg_conversion.clone())).present(source);
                BindingPresentation::Constructor {
                    aspect: signature_aspect.clone(),
                    name: Name::<RustSpecification>::Constructor(ty.clone()).mangle_tokens_default(),
                    ty: ty.clone(),
                    ctor_arguments: CommaPunctuatedArgs::from_iter([ArgKind::inherited_named_by_ref(ctor_arg_composer).present(&source)]),
                    body_presentation: quote!({#body})
                }
            },
            Self::SmartPointer(aspect, signature_aspect, SmartPointerKind::Mutex(..), SmartPointerPresentableContext::Dtor(_name_kind)) => {
                let ty = aspect.present(source);
                BindingPresentation::Destructor {
                    aspect: signature_aspect.clone(),
                    name: Name::<RustSpecification>::Destructor(ty.clone()).mangle_tokens_default(),
                    ty,
                }
            },
            Self::SmartPointer(aspect, signature_aspect, SmartPointerKind::Mutex(..), SmartPointerPresentableContext::Read(arg_field_composer, arg_type, from_root_conversion, to_inner_conversion)) => {
                let from_root_conversion = from_root_conversion.present(source);
                let to_inner_conversion = to_inner_conversion.present(source);
                BindingPresentation::Any {
                    attrs: signature_aspect.0.clone(),
                    body: present_pub_function(
                        signature_aspect,
                        Name::<RustSpecification>::Read(aspect.present(source)).mangle_tokens_default(),
                        CommaPunctuatedArgs::from_iter([ArgKind::inherited_named_by_ref(arg_field_composer).present(&source)]),
                        ReturnType::Type(RArrow::default(), arg_type.clone().into()),
                        quote!(
                            let lock = #from_root_conversion;
                            let obj = lock.lock().unwrap();
                            #to_inner_conversion
                        )
                    )
                }
            },
            Self::SmartPointer(aspect, signature_aspect, SmartPointerKind::Mutex(..), SmartPointerPresentableContext::Write(arg_field_composer, arg_var_composer, from_root_conversion, from_inner_conversion)) => {
                let from_root_conversion = from_root_conversion.present(source);
                let from_inner_conversion = from_inner_conversion.present(source);
                BindingPresentation::Any {
                    attrs: signature_aspect.0.clone(),
                    body: present_pub_function(
                        signature_aspect,
                        Name::<RustSpecification>::Write(aspect.present(source)).mangle_tokens_default(),
                        CommaPunctuatedArgs::from_iter([
                            ArgKind::inherited_named_by_ref(arg_field_composer).present(&source),
                            ArgKind::inherited_named_by_ref(arg_var_composer).present(source)
                        ]),
                        ReturnType::Default,
                        quote! {
                            let lock = #from_root_conversion;
                            let mut obj = lock.lock().unwrap();
                            *obj = #from_inner_conversion;
                        }
                    )
                }
            },
            Self::SmartPointer(aspect, signature_aspect, SmartPointerKind::RwLock(..), SmartPointerPresentableContext::Ctor(ctor_arg_composer, from_arg_conversion)) => {
                let ty = aspect.present(source);
                let body = ArgKind::DefaultFieldByValueConversion(ctor_arg_composer.clone(), <RustSpecification as Specification>::Expr::boxed(from_arg_conversion.clone())).present(source);
                BindingPresentation::Constructor {
                    aspect: signature_aspect.clone(),
                    name: Name::<RustSpecification>::Constructor(ty.clone()).mangle_tokens_default(),
                    ty: ty.clone(),
                    ctor_arguments: CommaPunctuatedArgs::from_iter([ArgKind::inherited_named_by_ref(ctor_arg_composer).present(&source)]),
                    body_presentation: quote!({#body})
                }
            },
            Self::SmartPointer(aspect, signature_aspect, SmartPointerKind::RwLock(..), SmartPointerPresentableContext::Dtor(_name_kind)) => {
                let ty = aspect.present(source);
                BindingPresentation::Destructor {
                    aspect: signature_aspect.clone(),
                    name: Name::<RustSpecification>::Destructor(ty.clone()).mangle_tokens_default(),
                    ty,
                }
            },
            Self::SmartPointer(aspect, signature_aspect, SmartPointerKind::RwLock(..), SmartPointerPresentableContext::Read(arg_field_composer, arg_type, from_root_conversion, to_inner_conversion)) => {
                let from_root_conversion = from_root_conversion.present(source);
                let to_inner_conversion = to_inner_conversion.present(source);
                BindingPresentation::Any {
                    attrs: signature_aspect.0.clone(),
                    body: present_pub_function(
                        signature_aspect,
                        Name::<RustSpecification>::Read(aspect.present(source)).mangle_tokens_default(),
                        CommaPunctuatedArgs::from_iter([ArgKind::inherited_named_by_ref(arg_field_composer).present(&source)]),
                        ReturnType::Type(RArrow::default(), arg_type.clone().into()),
                        quote!(
                            let lock = #from_root_conversion;
                            let obj = lock.read().unwrap();
                            #to_inner_conversion
                        )
                    )
                }
            },
            Self::SmartPointer(aspect, signature_aspect, SmartPointerKind::RwLock(..), SmartPointerPresentableContext::Write(arg_field_composer, arg_var_composer, from_root_conversion, from_inner_conversion)) => {
                let from_root_conversion = from_root_conversion.present(source);
                let from_inner_conversion = from_inner_conversion.present(source);
                BindingPresentation::Any {
                    attrs: signature_aspect.0.clone(),
                    body: present_pub_function(
                        signature_aspect,
                        Name::<RustSpecification>::Write(aspect.present(source)).mangle_tokens_default(),
                        CommaPunctuatedArgs::from_iter([
                            ArgKind::inherited_named_by_ref(arg_field_composer).present(&source),
                            ArgKind::inherited_named_by_ref(arg_var_composer).present(source)
                        ]),
                        ReturnType::Default,
                        quote! {
                            let lock = #from_root_conversion;
                            let mut obj = lock.write().unwrap();
                            *obj = #from_inner_conversion;
                        }
                    )
                }
            },
            Self::SmartPointer(aspect, signature_aspect, SmartPointerKind::OnceLock(..), SmartPointerPresentableContext::Ctor(ctor_arg_composer, from_arg_conversion)) => {
                let ty = aspect.present(source);
                let body = ArgKind::DefaultFieldByValueConversion(ctor_arg_composer.clone(), <RustSpecification as Specification>::Expr::boxed(from_arg_conversion.clone())).present(source);
                BindingPresentation::Constructor {
                    aspect: signature_aspect.clone(),
                    name: Name::<RustSpecification>::Constructor(ty.clone()).mangle_tokens_default(),
                    ty: ty.clone(),
                    ctor_arguments: CommaPunctuatedArgs::new(),
                    body_presentation: quote!({#body})
                }
            },
            Self::SmartPointer(aspect, signature_aspect, SmartPointerKind::OnceLock(..), SmartPointerPresentableContext::Dtor(_name_kind)) => {
                let ty = aspect.present(source);
                BindingPresentation::Destructor {
                    aspect: signature_aspect.clone(),
                    name: Name::<RustSpecification>::Destructor(ty.clone()).mangle_tokens_default(),
                    ty,
                }
            },
            Self::SmartPointer(aspect, signature_aspect, SmartPointerKind::OnceLock(..), SmartPointerPresentableContext::Read(arg_field_composer, arg_type, from_root_conversion, to_inner_conversion)) => {
                let from_root_conversion = from_root_conversion.present(source);
                let to_inner_conversion = to_inner_conversion.present(source);
                BindingPresentation::Any {
                    attrs: signature_aspect.0.clone(),
                    body: present_pub_function(
                        signature_aspect,
                        Name::<RustSpecification>::Read(aspect.present(source)).mangle_tokens_default(),
                        CommaPunctuatedArgs::from_iter([ArgKind::inherited_named_by_ref(arg_field_composer).present(&source)]),
                        ReturnType::Type(RArrow::default(), arg_type.clone().into()),
                        quote!(
                            let lock = #from_root_conversion;
                            let obj = lock.get().unwrap();
                            #to_inner_conversion
                        )
                    )
                }
            },
            Self::SmartPointer(aspect, signature_aspect, SmartPointerKind::OnceLock(..), SmartPointerPresentableContext::Write(arg_field_composer, arg_var_composer, from_root_conversion, from_inner_conversion)) => {
                let from_root_conversion = from_root_conversion.present(source);
                let from_inner_conversion = from_inner_conversion.present(source);
                BindingPresentation::Any {
                    attrs: signature_aspect.0.clone(),
                    body: present_pub_function(
                        signature_aspect,
                        Name::<RustSpecification>::Write(aspect.present(source)).mangle_tokens_default(),
                        CommaPunctuatedArgs::from_iter([
                            ArgKind::inherited_named_by_ref(arg_field_composer).present(&source),
                            ArgKind::inherited_named_by_ref(arg_var_composer).present(source)
                        ]),
                        ReturnType::Default,
                        quote! {
                            let lock = #from_root_conversion;
                            _ = lock.set(#from_inner_conversion);
                        }
                    )
                }
            },
            Self::SmartPointer(aspect, signature_aspect, SmartPointerKind::Cell(..), SmartPointerPresentableContext::Ctor(ctor_arg_composer, from_arg_conversion)) => {
                let ty = aspect.present(source);
                let body = ArgKind::DefaultFieldByValueConversion(ctor_arg_composer.clone(), <RustSpecification as Specification>::Expr::boxed(from_arg_conversion.clone())).present(source);
                BindingPresentation::Constructor {
                    aspect: signature_aspect.clone(),
                    name: Name::<RustSpecification>::Constructor(ty.clone()).mangle_tokens_default(),
                    ty: ty.clone(),
                    ctor_arguments: CommaPunctuatedArgs::from_iter([ArgKind::inherited_named_by_ref(ctor_arg_composer).present(&source)]),
                    body_presentation: quote!({#body})
                }
            },
            Self::SmartPointer(aspect, signature_aspect, SmartPointerKind::Cell(..), SmartPointerPresentableContext::Dtor(_name_kind)) => {
                let ty = aspect.present(source);
                BindingPresentation::Destructor {
                    aspect: signature_aspect.clone(),
                    name: Name::<RustSpecification>::Destructor(ty.clone()).mangle_tokens_default(),
                    ty,
                }
            },
            Self::SmartPointer(aspect, signature_aspect, SmartPointerKind::Cell(..), SmartPointerPresentableContext::Read(arg_field_composer, arg_type, from_root_conversion, to_inner_conversion)) => {
                let from_root_conversion = from_root_conversion.present(source);
                let to_inner_conversion = to_inner_conversion.present(source);
                BindingPresentation::Any {
                    attrs: signature_aspect.0.clone(),
                    body: present_pub_function(
                        signature_aspect,
                        Name::<RustSpecification>::Read(aspect.present(source)).mangle_tokens_default(),
                        CommaPunctuatedArgs::from_iter([ArgKind::inherited_named_by_ref(arg_field_composer).present(&source)]),
                        ReturnType::Type(RArrow::default(), arg_type.clone().into()),
                        quote!(
                            let lock = #from_root_conversion;
                            let obj = lock.get();
                            #to_inner_conversion
                        )
                    )
                }
            },
            Self::SmartPointer(aspect, signature_aspect, SmartPointerKind::Cell(..), SmartPointerPresentableContext::Write(arg_field_composer, arg_var_composer, from_root_conversion, from_inner_conversion)) => {
                let from_root_conversion = from_root_conversion.present(source);
                let from_inner_conversion = from_inner_conversion.present(source);
                BindingPresentation::Any {
                    attrs: signature_aspect.0.clone(),
                    body: present_pub_function(
                        signature_aspect,
                        Name::<RustSpecification>::Write(aspect.present(source)).mangle_tokens_default(),
                        CommaPunctuatedArgs::from_iter([
                            ArgKind::inherited_named_by_ref(arg_field_composer).present(&source),
                            ArgKind::inherited_named_by_ref(arg_var_composer).present(source)
                        ]),
                        ReturnType::Default,
                        quote! {
                            let lock = #from_root_conversion;
                            lock.set(#from_inner_conversion);
                        }
                    )
                }
            },
            Self::SmartPointer(aspect, signature_aspect, SmartPointerKind::RefCell(..), SmartPointerPresentableContext::Ctor(ctor_arg_composer, from_arg_conversion)) => {
                let ty = aspect.present(source);
                let body = ArgKind::DefaultFieldByValueConversion(ctor_arg_composer.clone(), <RustSpecification as Specification>::Expr::boxed(from_arg_conversion.clone())).present(source);
                BindingPresentation::Constructor {
                    aspect: signature_aspect.clone(),
                    name: Name::<RustSpecification>::Constructor(ty.clone()).mangle_tokens_default(),
                    ty: ty.clone(),
                    ctor_arguments: CommaPunctuatedArgs::from_iter([ArgKind::inherited_named_by_ref(ctor_arg_composer).present(&source)]),
                    body_presentation: quote!({#body})
                }
            },
            Self::SmartPointer(aspect, signature_aspect, SmartPointerKind::RefCell(..), SmartPointerPresentableContext::Dtor(_name_kind)) => {
                let ty = aspect.present(source);
                BindingPresentation::Destructor {
                    aspect: signature_aspect.clone(),
                    name: Name::<RustSpecification>::Destructor(ty.clone()).mangle_tokens_default(),
                    ty,
                }
            },
            Self::SmartPointer(aspect, signature_aspect, SmartPointerKind::RefCell(..), SmartPointerPresentableContext::Read(arg_field_composer, arg_type, from_root_conversion, to_inner_conversion)) => {
                let from_root_conversion = from_root_conversion.present(source);
                let to_inner_conversion = to_inner_conversion.present(source);
                BindingPresentation::Any {
                    attrs: signature_aspect.0.clone(),
                    body: present_pub_function(
                        signature_aspect,
                        Name::<RustSpecification>::Read(aspect.present(source)).mangle_tokens_default(),
                        CommaPunctuatedArgs::from_iter([ArgKind::inherited_named_by_ref(arg_field_composer).present(&source)]),
                        ReturnType::Type(RArrow::default(), arg_type.clone().into()),
                        quote!(
                            let lock = #from_root_conversion;
                            let obj = lock.borrow();
                            #to_inner_conversion
                        )
                    )
                }
            },
            Self::SmartPointer(aspect, signature_aspect, SmartPointerKind::RefCell(..), SmartPointerPresentableContext::Write(arg_field_composer, arg_var_composer, from_root_conversion, from_inner_conversion)) => {
                let from_root_conversion = from_root_conversion.present(source);
                let from_inner_conversion = from_inner_conversion.present(source);
                BindingPresentation::Any {
                    attrs: signature_aspect.0.clone(),
                    body: present_pub_function(
                        signature_aspect,
                        Name::<RustSpecification>::Write(aspect.present(source)).mangle_tokens_default(),
                        CommaPunctuatedArgs::from_iter([
                            ArgKind::inherited_named_by_ref(arg_field_composer).present(&source),
                            ArgKind::inherited_named_by_ref(arg_var_composer).present(source)
                        ]),
                        ReturnType::Default,
                        quote! {
                            let lock = #from_root_conversion;
                            match lock.try_borrow_mut() {
                                Ok(mut obj) => { *obj = #from_inner_conversion; },
                                Err(_) => {},
                            };

                        }
                    )
                }
            },
            BindingPresentableContext::SmartPointer(aspect, signature_aspect, SmartPointerKind::UnsafeCell(..), SmartPointerPresentableContext::Ctor(ctor_arg_composer, from_arg_conversion)) => {
                let ty = aspect.present(source);
                let body = ArgKind::DefaultFieldByValueConversion(ctor_arg_composer.clone(), <RustSpecification as Specification>::Expr::boxed(from_arg_conversion.clone())).present(source);
                BindingPresentation::Constructor {
                    aspect: signature_aspect.clone(),
                    name: Name::<RustSpecification>::Constructor(ty.clone()).mangle_tokens_default(),
                    ty: ty.clone(),
                    ctor_arguments: CommaPunctuatedArgs::from_iter([ArgKind::inherited_named_by_ref(ctor_arg_composer).present(&source)]),
                    body_presentation: quote!({#body})
                }
            },
            Self::SmartPointer(aspect, signature_aspect, SmartPointerKind::UnsafeCell(..), SmartPointerPresentableContext::Dtor(_name_kind)) => {
                let ty = aspect.present(source);
                BindingPresentation::Destructor {
                    aspect: signature_aspect.clone(),
                    name: Name::<RustSpecification>::Destructor(ty.clone()).mangle_tokens_default(),
                    ty,
                }
            },
            Self::SmartPointer(aspect, signature_aspect, SmartPointerKind::UnsafeCell(..), SmartPointerPresentableContext::Read(arg_field_composer, arg_type, from_root_conversion, to_inner_conversion)) => {
                let from_root_conversion = from_root_conversion.present(source);
                let to_inner_conversion = to_inner_conversion.present(source);
                BindingPresentation::Any {
                    attrs: signature_aspect.0.clone(),
                    body: present_pub_function(
                        signature_aspect,
                        Name::<RustSpecification>::Read(aspect.present(source)).mangle_tokens_default(),
                        CommaPunctuatedArgs::from_iter([ArgKind::inherited_named_by_ref(arg_field_composer).present(&source)]),
                        ReturnType::Type(RArrow::default(), arg_type.clone().into()),
                        quote!(
                            let lock = #from_root_conversion;
                            let obj = &*lock.get();
                            #to_inner_conversion
                        )
                    )
                }
            },
            Self::SmartPointer(aspect, signature_aspect, SmartPointerKind::UnsafeCell(..), SmartPointerPresentableContext::Write(arg_field_composer, arg_var_composer, from_root_conversion, from_inner_conversion)) => {
                let from_root_conversion = from_root_conversion.present(source);
                let from_inner_conversion = from_inner_conversion.present(source);
                BindingPresentation::Any {
                    attrs: signature_aspect.0.clone(),
                    body: present_pub_function(
                        signature_aspect,
                        Name::<RustSpecification>::Write(aspect.present(source)).mangle_tokens_default(),
                        CommaPunctuatedArgs::from_iter([
                            ArgKind::inherited_named_by_ref(arg_field_composer).present(&source),
                            ArgKind::inherited_named_by_ref(arg_var_composer).present(source)
                        ]),
                        ReturnType::Default,
                        quote! {
                            let obj = #from_root_conversion;
                            *obj.get() = #from_inner_conversion;
                        }
                    )
                }
            },
            Self::SmartPointer(..) => panic!(""),
            Self::Callback(aspect, signature_aspect, ident, arg_target_fields, return_type, arg_to_conversions, post_processing, ffi_return_type, ffi_args) =>
                BindingPresentation::Callback {
                    aspect: signature_aspect.clone(),
                    name: ident.clone(),
                    ffi_args: ffi_args.clone(),
                    result: ffi_return_type.clone(),
                    conversion: InterfacePresentation::callback(
                        &signature_aspect.0,
                        &signature_aspect.1,
                        aspect.present(source),
                        arg_target_fields.clone(),
                        return_type.clone(),
                        arg_to_conversions.present(source),
                        post_processing
                    ),
                },
            Self::ArrayGetAtIndex(aspect, signature_aspect, arr_type, nested_type, to_conversion_expr_value) =>
                BindingPresentation::RegularFunctionWithBody {
                    aspect: signature_aspect.clone(),
                    name: Name::<RustSpecification>::GetValueAtIndex(aspect.present(source)).mangle_tokens_default(),
                    arguments: CommaPunctuatedArgs::from_iter([
                        ArgPresentation::no_attr_tokens(quote!(ffi: *const #arr_type)),
                        ArgPresentation::no_attr_tokens(quote!(index: usize))
                    ]),
                    return_type: ReturnType::Type(RArrow::default(), Box::new(nested_type.clone())),
                    body: to_conversion_expr_value.to_token_stream(),
                },
            Self::ArraySetAtIndex(aspect, signature_aspect, map_type, nested_type, from_conversion_expr_value) =>
                BindingPresentation::RegularFunctionWithBody {
                    aspect: signature_aspect.clone(),
                    name: Name::<RustSpecification>::SetValueAtIndex(aspect.present(source)).mangle_tokens_default(),
                    arguments: CommaPunctuatedArgs::from_iter([
                        ArgPresentation::no_attr_tokens(quote!(ffi: *mut #map_type)),
                        ArgPresentation::no_attr_tokens(quote!(index: usize)),
                        ArgPresentation::no_attr_tokens(quote!(value: #nested_type)),
                    ]),
                    return_type: ReturnType::Default,
                    body: from_conversion_expr_value.to_token_stream(),
                },
            Self::ValueByKey(aspect, signature_aspect, map_type, key_type, value_type, to_conversion_expr_value) => {
                let return_type = value_type.is_primitive().then(|| value_type.joined_mut()).unwrap_or_else(|| value_type.clone());

                BindingPresentation::RegularFunctionWithBody {
                    aspect: signature_aspect.clone(),
                    name: Name::<RustSpecification>::GetValueByKey(aspect.present(source)).mangle_tokens_default(),
                    arguments: CommaPunctuatedArgs::from_iter([
                        ArgPresentation::no_attr_tokens(quote!(ffi: *const #map_type)),
                        ArgPresentation::no_attr_tokens(quote!(key: #key_type))
                    ]),
                    return_type: ReturnType::Type(RArrow::default(), Box::new(return_type)),
                    body: to_conversion_expr_value.to_token_stream(),
                }
            },
            Self::SetValueForKey(aspect, signature_aspect, map_type, key_type, value_type, from_conversion_expr_value) => {
                BindingPresentation::RegularFunctionWithBody {
                    aspect: signature_aspect.clone(),
                    name: Name::<RustSpecification>::SetValueForKey(aspect.present(source)).mangle_tokens_default(),
                    arguments: CommaPunctuatedArgs::from_iter([
                        ArgPresentation::no_attr_tokens(quote!(ffi: *mut #map_type)),
                        ArgPresentation::no_attr_tokens(quote!(key: #key_type)),
                        ArgPresentation::no_attr_tokens(quote!(value: #value_type)),
                    ]),
                    return_type: ReturnType::Default,
                    body: from_conversion_expr_value.to_token_stream(),
                }
            },
            Self::KeyByValue(aspect, signature_aspect, map_type, key_type, value_type, to_conversion_expr_value) => {
                let return_type = key_type.is_primitive().then(|| key_type.joined_mut()).unwrap_or_else(|| key_type.clone());
                BindingPresentation::RegularFunctionWithBody {
                    aspect: signature_aspect.clone(),
                    name: Name::<RustSpecification>::GetKeyByValue(aspect.present(source)).mangle_tokens_default(),
                    arguments: CommaPunctuatedArgs::from_iter([
                        ArgPresentation::no_attr_tokens(quote!(ffi: *const #map_type)),
                        ArgPresentation::no_attr_tokens(quote!(value: #value_type))
                    ]),
                    return_type: ReturnType::Type(RArrow::default(), Box::new(return_type)),
                    body: to_conversion_expr_value.to_token_stream(),
                }
            },
            Self::SetKeyForValue(aspect, signature_aspect, map_type, key_type, value_type, from_conversion_expr_value) =>
                BindingPresentation::RegularFunctionWithBody {
                    aspect: signature_aspect.clone(),
                    name: Name::<RustSpecification>::SetKeyForValue(aspect.present(source)).mangle_tokens_default(),
                    arguments: CommaPunctuatedArgs::from_iter([
                        ArgPresentation::no_attr_tokens(quote!(ffi: *mut #map_type)),
                        ArgPresentation::no_attr_tokens(quote!(key: #key_type)),
                        ArgPresentation::no_attr_tokens(quote!(value: #value_type)),
                    ]),
                    return_type: ReturnType::Default,
                    body: from_conversion_expr_value.to_token_stream(),
                },
            Self::ResultOk(signature_aspect, result_type, ok_type) => {
                let null = DictionaryExpr::NullMut;
                BindingPresentation::RegularFunctionWithBody {
                    aspect: signature_aspect.clone(),
                    name: Name::<RustSpecification>::Constructor(parse_quote!(#result_type::Ok)).mangle_tokens_default(),
                    arguments: CommaPunctuatedArgs::from_iter([
                        ArgPresentation::no_attr_tokens(quote!(ok: #ok_type)),
                    ]),
                    return_type: ReturnType::Type(RArrow::default(), Box::new(result_type.joined_mut())),
                    body: InterfacesMethodExpr::Boxed(quote!(#result_type { ok, error: #null })).to_token_stream(),
                }
            }
            Self::ResultError(signature_aspect, result_type, error_type) => {
                let null = DictionaryExpr::NullMut;
                BindingPresentation::RegularFunctionWithBody {
                    aspect: signature_aspect.clone(),
                    name: Name::<RustSpecification>::Constructor(parse_quote!(#result_type::Error)).mangle_tokens_default(),
                    arguments: CommaPunctuatedArgs::from_iter([
                        ArgPresentation::no_attr_tokens(quote!(error: #error_type)),
                    ]),
                    return_type: ReturnType::Type(RArrow::default(), Box::new(result_type.joined_mut())),
                    body: InterfacesMethodExpr::Boxed(quote!(#result_type { ok: #null, error })).to_token_stream(),
                }
            }
        }
    }
}

