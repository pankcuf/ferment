use quote::{quote, ToTokens};
use syn::{parse_quote, Expr, ExprAssign, ExprCall, ReturnType, Visibility};
use syn::token::RArrow;
use crate::composer::{NameKind, SourceComposable, CommaPunctuatedArgs, ConversionDropComposer, ConversionFromComposer};
use crate::context::ScopeContext;
use crate::kind::SmartPointerKind;
use crate::ext::{Accessory, Mangle, Primitive, PunctuateOne, Terminated, ToPath, ToType, WrapIntoCurlyBraces, WrapIntoRoundBraces};
use crate::lang::{RustSpecification, Specification};
use crate::presentable::{ArgKind, BindingPresentableContext, ScopeContextPresentable, SmartPointerPresentableContext};
use crate::presentation::{present_pub_function, present_signature, ArgPresentation, BindingPresentation, DictionaryExpr, DictionaryName, InterfacePresentation, InterfacesMethodExpr, Name};

impl ScopeContextPresentable for BindingPresentableContext<RustSpecification> {
    type Presentation = BindingPresentation;

    fn present(&self, source: &ScopeContext) -> Self::Presentation {
        match self {
            Self::Constructor(aspect, signature_aspect, name_kind, args, body) => {
                let ty = aspect.present(source);
                BindingPresentation::Constructor {
                    aspect: signature_aspect.clone(),
                    name: Name::<RustSpecification>::ctor(&ty).mangle_tokens_default(),
                    ty,
                    ctor_arguments: args.present(&source),
                    body_presentation: match name_kind {
                        NameKind::Unnamed => WrapIntoRoundBraces::wrap(body.present(source)),
                        _ => WrapIntoCurlyBraces::wrap(body.present(source))
                    }
                }
            },
            Self::VariantConstructor(aspect, signature_aspect, name_kind, args, body) => {
                let ty = aspect.present(source);
                BindingPresentation::VariantConstructor {
                    aspect: signature_aspect.clone(),
                    name: Name::<RustSpecification>::ctor(&ty).mangle_tokens_default(),
                    ty,
                    ctor_arguments: args.present(&source),
                    body_presentation: match name_kind {
                        NameKind::Unnamed => WrapIntoRoundBraces::wrap(body.present(source)),
                        _ => WrapIntoCurlyBraces::wrap(body.present(source))
                    }
                }
            },
            Self::Destructor(aspect, signature_aspect) => {
                let ty = aspect.present(source);
                BindingPresentation::Destructor {
                    aspect: signature_aspect.clone(),
                    var: ty.joined_mut(),
                    name: Name::<RustSpecification>::Destructor(ty).mangle_tokens_default(),
                }
            },
            Self::Getter(obj_aspect, signature_aspect, field_type, field_name) => {
                let obj_type = obj_aspect.present(source);
                BindingPresentation::Getter {
                    aspect: signature_aspect.clone(),
                    name: Name::<RustSpecification>::getter(obj_type.to_path(), &field_name).mangle_tokens_default(),
                    field_name: field_name.clone(),
                    obj_var: obj_type.joined_const(),
                    field_type: field_type.compose(source).to_type(),
                }
            },
            Self::Setter(obj_aspect, signature_aspect, field_type, field_name, ) => {
                let obj_type = obj_aspect.present(source);
                BindingPresentation::Setter {
                    aspect: signature_aspect.clone(),
                    name: Name::<RustSpecification>::setter(obj_type.to_path(), &field_name).mangle_tokens_default(),
                    field_name: field_name.clone(),
                    obj_var: obj_type.joined_mut(),
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
            Self::TraitVTableInnerFn((attrs, ..), ident, name_and_args, return_type_conversion) => BindingPresentation::TraitVTableInnerFn {
                attrs: attrs.clone(),
                name: Name::<RustSpecification>::VTableInnerFn(ident.clone()).mangle_tokens_default(),
                name_and_args: present_signature(Visibility::Inherited, WrapIntoRoundBraces::wrap(name_and_args.present(source))),
                output_expression: return_type_conversion.clone(),
            },
            Self::SmartPointer(aspect, signature_aspect, SmartPointerKind::Mutex(..), SmartPointerPresentableContext::Ctor(ctor_arg_composer, from_arg_conversion)) => {
                let ty = aspect.present(source);
                BindingPresentation::Constructor {
                    aspect: signature_aspect.clone(),
                    name: Name::<RustSpecification>::ctor(&ty).mangle_tokens_default(),
                    ty,
                    ctor_arguments: CommaPunctuatedArgs::from_iter([ArgKind::inherited_named_by_ref(ctor_arg_composer).present(&source)]),
                    body_presentation: WrapIntoCurlyBraces::wrap(ArgKind::DefaultFieldByValueConversion(ctor_arg_composer.clone(), <RustSpecification as Specification>::Expr::boxed(from_arg_conversion.clone())).present(source))
                }
            },
            Self::SmartPointer(aspect, signature_aspect, SmartPointerKind::Mutex(..), SmartPointerPresentableContext::Dtor(_name_kind)) => {
                let ty = aspect.present(source);
                BindingPresentation::Destructor {
                    aspect: signature_aspect.clone(),
                    var: ty.joined_mut(),
                    name: Name::<RustSpecification>::Destructor(ty).mangle_tokens_default(),
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
                    name: Name::<RustSpecification>::ctor(&ty).mangle_tokens_default(),
                    ty: ty.clone(),
                    ctor_arguments: CommaPunctuatedArgs::from_iter([ArgKind::inherited_named_by_ref(ctor_arg_composer).present(&source)]),
                    body_presentation: WrapIntoCurlyBraces::wrap(body)
                }
            },
            Self::SmartPointer(aspect, signature_aspect, SmartPointerKind::RwLock(..), SmartPointerPresentableContext::Dtor(_name_kind)) => {
                let ty = aspect.present(source);
                BindingPresentation::Destructor {
                    aspect: signature_aspect.clone(),
                    var: ty.joined_mut(),
                    name: Name::<RustSpecification>::Destructor(ty).mangle_tokens_default(),
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
                BindingPresentation::Constructor {
                    aspect: signature_aspect.clone(),
                    name: Name::<RustSpecification>::ctor(&ty).mangle_tokens_default(),
                    ty: ty.clone(),
                    ctor_arguments: CommaPunctuatedArgs::new(),
                    body_presentation: WrapIntoCurlyBraces::wrap(ArgKind::DefaultFieldByValueConversion(ctor_arg_composer.clone(), <RustSpecification as Specification>::Expr::boxed(from_arg_conversion.clone())).present(source))
                }
            },
            Self::SmartPointer(aspect, signature_aspect, SmartPointerKind::OnceLock(..), SmartPointerPresentableContext::Dtor(_name_kind)) => {
                let ty = aspect.present(source);
                BindingPresentation::Destructor {
                    aspect: signature_aspect.clone(),
                    var: ty.joined_mut(),
                    name: Name::<RustSpecification>::Destructor(ty).mangle_tokens_default(),
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
                    name: Name::<RustSpecification>::ctor(&ty).mangle_tokens_default(),
                    ty: ty.clone(),
                    ctor_arguments: CommaPunctuatedArgs::from_iter([ArgKind::inherited_named_by_ref(ctor_arg_composer).present(&source)]),
                    body_presentation: WrapIntoCurlyBraces::wrap(body)
                }
            },
            Self::SmartPointer(aspect, signature_aspect, SmartPointerKind::Cell(..), SmartPointerPresentableContext::Dtor(_name_kind)) => {
                let ty = aspect.present(source);
                BindingPresentation::Destructor {
                    aspect: signature_aspect.clone(),
                    var: ty.joined_mut(),
                    name: Name::<RustSpecification>::Destructor(ty).mangle_tokens_default(),
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
                    name: Name::<RustSpecification>::ctor(&ty).mangle_tokens_default(),
                    ty,
                    ctor_arguments: CommaPunctuatedArgs::from_iter([ArgKind::inherited_named_by_ref(ctor_arg_composer).present(&source)]),
                    body_presentation: WrapIntoCurlyBraces::wrap(body)
                }
            },
            Self::SmartPointer(aspect, signature_aspect, SmartPointerKind::RefCell(..), SmartPointerPresentableContext::Dtor(_name_kind)) => {
                let ty = aspect.present(source);
                BindingPresentation::Destructor {
                    aspect: signature_aspect.clone(),
                    var: ty.joined_mut(),
                    name: Name::<RustSpecification>::Destructor(ty).mangle_tokens_default(),
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
                BindingPresentation::Constructor {
                    aspect: signature_aspect.clone(),
                    name: Name::<RustSpecification>::ctor(&ty).mangle_tokens_default(),
                    ty,
                    ctor_arguments: CommaPunctuatedArgs::from_iter([ArgKind::inherited_named_by_ref(ctor_arg_composer).present(&source)]),
                    body_presentation: WrapIntoCurlyBraces::wrap(ArgKind::DefaultFieldByValueConversion(ctor_arg_composer.clone(), <RustSpecification as Specification>::Expr::boxed(from_arg_conversion.clone())).present(source))
                }
            },
            Self::SmartPointer(aspect, signature_aspect, SmartPointerKind::UnsafeCell(..), SmartPointerPresentableContext::Dtor(_name_kind)) => {
                let ty = aspect.present(source);
                BindingPresentation::Destructor {
                    aspect: signature_aspect.clone(),
                    var: ty.joined_mut(),
                    name: Name::<RustSpecification>::Destructor(ty).mangle_tokens_default(),
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
            Self::ArrayGetAtIndex(aspect, signature_aspect, arr_type, nested_type) => {
                let ffi = DictionaryName::Ffi;
                let values = DictionaryName::Values;
                let index = DictionaryName::Index;
                BindingPresentation::RegularFunctionWithBody {
                    aspect: signature_aspect.clone(),
                    name: Name::<RustSpecification>::GetValueAtIndex(aspect.present(source)).mangle_tokens_default(),
                    arguments: CommaPunctuatedArgs::from_iter([
                        ArgPresentation::no_attr_tokens(quote!(#ffi: *const #arr_type)),
                        ArgPresentation::no_attr_tokens(quote!(#index: usize))
                    ]),
                    return_type: ReturnType::Type(RArrow::default(), Box::new(nested_type.clone())),
                    body: Expr::Call(ExprCall {
                        attrs: vec![],
                        func: Box::new(Expr::Verbatim(quote!(*(*#ffi).#values.add))),
                        paren_token: Default::default(),
                        args: Expr::Verbatim(index.to_token_stream()).punctuate_one(),
                    }).to_token_stream(),
                }
            },
            Self::ArraySetAtIndex(aspect, signature_aspect, map_type, nested_type) => {
                let ffi = DictionaryName::Ffi;
                let value = DictionaryName::Value;
                let values = DictionaryName::Values;
                let index = DictionaryName::Index;
                BindingPresentation::RegularFunctionWithBody {
                    aspect: signature_aspect.clone(),
                    name: Name::<RustSpecification>::SetValueAtIndex(aspect.present(source)).mangle_tokens_default(),
                    arguments: CommaPunctuatedArgs::from_iter([
                        ArgPresentation::no_attr_tokens(quote!(#ffi: *mut #map_type)),
                        ArgPresentation::no_attr_tokens(quote!(#index: usize)),
                        ArgPresentation::no_attr_tokens(quote!(#value: #nested_type)),
                    ]),
                    return_type: ReturnType::Default,
                    body: Expr::Assign(ExprAssign {
                        attrs: vec![],
                        left: Box::new(Expr::Call(ExprCall {
                            attrs: vec![],
                            func: Box::new(Expr::Verbatim(quote!(*(*#ffi).#values.add))),
                            paren_token: Default::default(),
                            args: Expr::Verbatim(index.to_token_stream()).punctuate_one(),
                        })),
                        eq_token: Default::default(),
                        right: Box::new(Expr::Verbatim(value.to_token_stream())),
                    }).to_token_stream(),
                }
            },
            Self::ValueByKey(aspect, signature_aspect, map_type, key_var, value_var) => {
                let ffi = DictionaryName::Ffi;
                let ffi_ref = DictionaryName::FfiRef;
                let key = DictionaryName::Key;
                let keys = DictionaryName::Keys;
                let values = DictionaryName::Values;
                let value_is_primitive = value_var.is_primitive();
                let return_type = value_is_primitive
                    .then(|| value_var.joined_mut())
                    .unwrap_or_else(|| value_var.clone());
                let get_key = quote!(*#ffi_ref.#keys.add(i));
                let from_key_conversion = ConversionFromComposer::<RustSpecification>::value_pat_tokens(&key, key_var).compose(source).present(source);
                let from_key_2_conversion = ConversionFromComposer::<RustSpecification>::value_pat_tokens(&get_key, key_var).compose(source).present(source);
                let get_value = quote!(*#ffi_ref.#values.add(i));
                let return_value_expr = value_is_primitive
                    .then(|| InterfacesMethodExpr::Boxed(get_value.to_token_stream()).to_token_stream())
                    .unwrap_or(get_value);
                BindingPresentation::RegularFunctionWithBody {
                    aspect: signature_aspect.clone(),
                    name: Name::<RustSpecification>::GetValueByKey(aspect.present(source)).mangle_tokens_default(),
                    arguments: CommaPunctuatedArgs::from_iter([
                        ArgPresentation::no_attr_tokens(quote!(#ffi: *const #map_type)),
                        ArgPresentation::no_attr_tokens(quote!(#key: #key_var))
                    ]),
                    return_type: ReturnType::Type(RArrow::default(), Box::new(return_type)),
                    body: quote! {
                        let #ffi_ref = &*#ffi;
                        let key = #from_key_conversion;
                        for i in 0..#ffi_ref.count {
                            if key == #from_key_2_conversion {
                                return #return_value_expr;
                            }
                        }
                        std::ptr::null_mut()
                    }
                }
            }
            Self::SetValueForKey(aspect, signature_aspect, map_type, key_type, value_type, _) => {
                let ffi = DictionaryName::Ffi;
                let ffi_ref = DictionaryName::FfiRef;
                let key = DictionaryName::Key;
                let value = DictionaryName::Value;
                let keys = DictionaryName::Keys;
                let values = DictionaryName::Values;
                let old_value = DictionaryName::OldValue;
                let new_value = DictionaryName::NewValue;
                let from_key_conversion = ConversionFromComposer::<RustSpecification>::value_pat_tokens(&key, key_type).compose(source).present(source);
                let from_key_2_conversion = ConversionFromComposer::<RustSpecification>::value_pat_tokens(quote!(*#ffi_ref.#keys.add(i)), key_type).compose(source).present(source);
                let destroy_value = ConversionDropComposer::<RustSpecification>::value(Name::pat_tokens(&old_value), value_type)
                    .compose(source)
                    .map(|expr| DictionaryExpr::IfNotNull(old_value.to_token_stream(), expr.present(source).terminated()).to_token_stream())
                    .unwrap_or_default();


                BindingPresentation::RegularFunctionWithBody {
                    aspect: signature_aspect.clone(),
                    name: Name::<RustSpecification>::SetValueForKey(aspect.present(source)).mangle_tokens_default(),
                    arguments: CommaPunctuatedArgs::from_iter([
                        ArgPresentation::no_attr_tokens(quote!(#ffi: *mut #map_type)),
                        ArgPresentation::no_attr_tokens(quote!(#key: #key_type)),
                        ArgPresentation::no_attr_tokens(quote!(#value: #value_type)),
                    ]),
                    return_type: ReturnType::Default,
                    body: quote! {
                        let #ffi_ref = &*#ffi;
                        let target_key = #from_key_conversion;
                        for i in 0..#ffi_ref.count {
                            let candidate_key = #from_key_2_conversion;
                            if candidate_key.eq(&target_key) {
                                let #new_value = (*#ffi).#values.add(i);
                                let #old_value = *#new_value;
                                #destroy_value
                                *#new_value = #value;
                                break;
                            }
                        }
                    },
                }
            },
            Self::KeyByValue(aspect, signature_aspect, map_type, key_var, value_var) => {
                let ffi = DictionaryName::Ffi;
                let ffi_ref = DictionaryName::FfiRef;
                let value = DictionaryName::Value;
                let keys = DictionaryName::Keys;
                let values = DictionaryName::Values;
                let return_type = key_var.is_primitive().then(|| key_var.joined_mut()).unwrap_or_else(|| key_var.clone());
                let from_value_conversion = ConversionFromComposer::<RustSpecification>::value_pat_tokens(&value, value_var).compose(source).present(source);
                let from_value_2_conversion = ConversionFromComposer::<RustSpecification>::value_pat_tokens(quote!(*#ffi_ref.#values.add(i)), value_var).compose(source).present(source);
                let get_key = quote!(*#ffi_ref.#keys.add(i));
                let return_key_expr = key_var.is_primitive()
                    .then(|| InterfacesMethodExpr::Boxed(get_key.to_token_stream()).to_token_stream())
                    .unwrap_or(get_key);

                BindingPresentation::RegularFunctionWithBody {
                    aspect: signature_aspect.clone(),
                    name: Name::<RustSpecification>::GetKeyByValue(aspect.present(source)).mangle_tokens_default(),
                    arguments: CommaPunctuatedArgs::from_iter([
                        ArgPresentation::no_attr_tokens(quote!(#ffi: *const #map_type)),
                        ArgPresentation::no_attr_tokens(quote!(#value: #value_var))
                    ]),
                    return_type: ReturnType::Type(RArrow::default(), Box::new(return_type)),
                    body: quote! {
                        let #ffi_ref = &*#ffi;
                        let key = #from_value_conversion;
                        for i in 0..#ffi_ref.count {
                            if key == #from_value_2_conversion {
                                return #return_key_expr;
                            }
                        }
                        std::ptr::null_mut()
                    },
                }
            }
            Self::SetKeyForValue(aspect, signature_aspect, map_type, key_var, value_var, _) => {
                let ffi = DictionaryName::Ffi;
                let ffi_ref = DictionaryName::FfiRef;
                let key = DictionaryName::Key;
                let value = DictionaryName::Value;
                let keys = DictionaryName::Keys;
                let values = DictionaryName::Values;
                let old_value = DictionaryName::OldValue;
                let new_value = DictionaryName::NewValue;
                let from_value_conversion = ConversionFromComposer::<RustSpecification>::value_pat_tokens(&value, value_var).compose(source).present(source);
                let from_value_2_conversion = ConversionFromComposer::<RustSpecification>::value_pat_tokens(quote!(*ffi_ref.#values.add(i)), value_var).compose(source).present(source);
                let destroy_key = ConversionDropComposer::<RustSpecification>::value(Name::pat_tokens(&old_value), key_var)
                    .compose(source)
                    .map(|expr| DictionaryExpr::IfNotNull(old_value.to_token_stream(), expr.present(source).terminated()).to_token_stream())
                    .unwrap_or_default();

                BindingPresentation::RegularFunctionWithBody {
                    aspect: signature_aspect.clone(),
                    name: Name::<RustSpecification>::SetKeyForValue(aspect.present(source)).mangle_tokens_default(),
                    arguments: CommaPunctuatedArgs::from_iter([
                        ArgPresentation::no_attr_tokens(quote!(#ffi: *mut #map_type)),
                        ArgPresentation::no_attr_tokens(quote!(#key: #key_var)),
                        ArgPresentation::no_attr_tokens(quote!(#value: #value_var)),
                    ]),
                    return_type: ReturnType::Default,
                    body: quote! {
                        let #ffi_ref = &*#ffi;
                        let target_key = #from_value_conversion;
                        for i in 0..#ffi_ref.count {
                            let candidate_key = #from_value_2_conversion;
                            if candidate_key.eq(&target_key) {
                                let #new_value = (*#ffi).#keys.add(i);
                                let #old_value = *#new_value;
                                #destroy_key
                                *#new_value = #key;
                                break;
                            }
                        }
                    },
                }
            }
            Self::ResultOk(signature_aspect, result_type, ok_type) => {
                let ok = DictionaryName::Ok;
                let error = DictionaryName::Error;
                let null = DictionaryExpr::NullMut;
                BindingPresentation::RegularFunctionWithBody {
                    aspect: signature_aspect.clone(),
                    name: Name::<RustSpecification>::Constructor(parse_quote!(#result_type::Ok)).mangle_tokens_default(),
                    arguments: ArgPresentation::no_attr_tokens(quote!(#ok: #ok_type)).punctuate_one(),
                    return_type: ReturnType::Type(RArrow::default(), Box::new(result_type.joined_mut())),
                    body: InterfacesMethodExpr::Boxed(quote!(#result_type { #ok, #error: #null })).to_token_stream(),
                }
            }
            Self::ResultError(signature_aspect, result_type, error_type) => {
                let ok = DictionaryName::Ok;
                let error = DictionaryName::Error;
                let null = DictionaryExpr::NullMut;
                BindingPresentation::RegularFunctionWithBody {
                    aspect: signature_aspect.clone(),
                    name: Name::<RustSpecification>::Constructor(parse_quote!(#result_type::Error)).mangle_tokens_default(),
                    arguments: ArgPresentation::no_attr_tokens(quote!(#error: #error_type)).punctuate_one(),
                    return_type: ReturnType::Type(RArrow::default(), Box::new(result_type.joined_mut())),
                    body: InterfacesMethodExpr::Boxed(quote!(#result_type { #ok: #null, #error })).to_token_stream(),
                }
            }
        }
    }
}

