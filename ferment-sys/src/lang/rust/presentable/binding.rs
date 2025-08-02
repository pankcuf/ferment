use quote::{quote, ToTokens};
use syn::ReturnType;
use syn::token::RArrow;
use crate::composer::{NameKind, SourceComposable, CommaPunctuatedArgs};
use crate::context::ScopeContext;
use crate::kind::SmartPointerKind;
use crate::ext::{Mangle, ToPath, ToType};
use crate::lang::{RustSpecification, Specification};
use crate::presentable::{ArgKind, BindingPresentableContext, ScopeContextPresentable, SmartPointerPresentableContext};
use crate::presentation::{present_pub_function, BindingPresentation, InterfacePresentation, Name};

impl ScopeContextPresentable for BindingPresentableContext<RustSpecification> {
    type Presentation = BindingPresentation;

    fn present(&self, source: &ScopeContext) -> Self::Presentation {
        match self {
            Self::Constructor(aspect, attrs, lifetimes, generics, name_kind, args, body) => {
                let ty = aspect.present(source);
                let body = body.present(source);
                let body_presentation = match name_kind {
                    NameKind::Unnamed => quote!((#body)),
                    _ => quote!({#body})
                };
                BindingPresentation::Constructor {
                    attrs: attrs.clone(),
                    lifetimes: lifetimes.clone(),
                    name: Name::<RustSpecification>::Constructor(ty.clone()).mangle_tokens_default(),
                    ty: ty.clone(),
                    generics: generics.clone(),
                    ctor_arguments: args.present(&source),
                    body_presentation
                }
            },
            Self::VariantConstructor(aspect, attrs, lifetimes, generics, name_kind, args, body) => {
                let ty = aspect.present(source);
                let body = body.present(source);
                let body_presentation = match name_kind {
                    NameKind::Unnamed => quote!((#body)),
                    _ => quote!({#body})
                };


                BindingPresentation::VariantConstructor {
                    attrs: attrs.clone(),
                    lifetimes: lifetimes.clone(),
                    name: Name::<RustSpecification>::Constructor(ty.clone()).mangle_tokens_default(),
                    ty: ty.clone(),
                    generics: generics.clone(),
                    ctor_arguments: args.present(&source),
                    body_presentation
                }
            },
            Self::Destructor(aspect, attrs, lifetimes, generics, _name_kind) => {
                let ty = aspect.present(source);
                BindingPresentation::Destructor {
                    attrs: attrs.clone(),
                    lifetimes: lifetimes.clone(),
                    name: Name::<RustSpecification>::Destructor(ty.clone()).mangle_tokens_default(),
                    ty: ty.clone(),
                    generics: generics.clone()
                }
            },
            Self::Getter(obj_aspect, attrs, lifetimes, generics, field_type, field_name) => {
                let obj_type = obj_aspect.present(source);

                BindingPresentation::Getter {
                    attrs: attrs.clone(),
                    lifetimes: lifetimes.clone(),
                    name: Name::<RustSpecification>::getter(obj_type.to_path(), &field_name).mangle_tokens_default(),
                    field_name: field_name.clone(),
                    obj_type: obj_type.clone(),
                    field_type: field_type.compose(source).to_type(),
                    generics: generics.clone(),
                }
            },
            Self::Setter(obj_aspect, attrs, lifetimes, generics, field_type, field_name, ) => {
                let obj_type = obj_aspect.present(source);
                BindingPresentation::Setter {
                    attrs: attrs.clone(),
                    lifetimes: lifetimes.clone(),
                    name: Name::<RustSpecification>::setter(obj_type.to_path(), &field_name).mangle_tokens_default(),
                    field_name: field_name.clone(),
                    obj_type: obj_type.clone(),
                    field_type: field_type.compose(source).to_type(),
                    generics: generics.clone(),
                }
            },
            Self::RegFn(path, is_async, arguments, return_type, input_conversions, return_type_conversion, attrs, lifetimes, generics) => BindingPresentation::RegularFunction {
                attrs: attrs.clone(),
                is_async: *is_async,
                arguments: arguments.present(&source),
                name: Name::<RustSpecification>::ModFn(path.clone()).mangle_tokens_default(),
                input_conversions: input_conversions.present(&source),
                return_type: return_type.clone(),
                generics: generics.clone(),
                lifetimes: lifetimes.clone(),
                output_conversions: <<RustSpecification as Specification>::Expr as ScopeContextPresentable>::present(return_type_conversion, source).to_token_stream()
            },
            Self::RegFn2(path, is_async, argument_names, arguments, return_type, full_fn_path, input_conversions, return_type_conversion, attrs, lifetimes, generics) => BindingPresentation::RegularFunction2 {
                attrs: attrs.clone(),
                is_async: *is_async,
                argument_names: argument_names.clone(),
                arguments: arguments.present(&source),
                name: Name::<RustSpecification>::ModFn(path.clone()).mangle_tokens_default(),
                full_fn_path: full_fn_path.clone(),
                input_conversions: input_conversions.present(&source),
                return_type: return_type.clone(),
                generics: generics.clone(),
                lifetimes: lifetimes.clone(),
                output_conversions: <<RustSpecification as Specification>::Expr as ScopeContextPresentable>::present(return_type_conversion, source).to_token_stream()
            },
            Self::TraitVTableInnerFn(attrs, ident, name_and_args, return_type_conversion) => {
                let arguments = name_and_args.present(source);
                BindingPresentation::TraitVTableInnerFn {
                    attrs: attrs.clone(),
                    name: Name::<RustSpecification>::VTableInnerFn(ident.clone()).mangle_tokens_default(),
                    name_and_args: quote!(unsafe extern "C" fn (#arguments)),
                    output_expression: return_type_conversion.clone(),
                }
            },
            // BindingPresentableContext::BareFn(attrs, ident, ffi_args, name_and_args, return_type_conversion) => {
            //     let arguments = name_and_args.present(source);
            //     let conversion = InterfacePresentation::callback(attrs, &ffi_type, arg_target_fields, return_type, &lifetimes, args_to, post_processing);
            //
            //     BindingPresentation::Callback {
            //         name: full_fn_path.mangle_ident_default(),
            //         attrs: attrs.clone(),
            //         ffi_args,
            //         result: return_type_conversion.clone(),
            //         conversion
            //     }
            // },

            Self::SmartPointer(SmartPointerKind::Mutex(..), aspect, attrs, lifetimes, SmartPointerPresentableContext::Ctor(ctor_arg_composer, from_arg_conversion)) => {
                let ty = aspect.present(source);
                let body = ArgKind::DefaultFieldByValueConversion(ctor_arg_composer.clone(), <RustSpecification as Specification>::Expr::boxed(from_arg_conversion.clone())).present(source);
                BindingPresentation::Constructor {
                    attrs: attrs.clone(),
                    lifetimes: lifetimes.clone(),
                    name: Name::<RustSpecification>::Constructor(ty.clone()).mangle_tokens_default(),
                    ty: ty.clone(),
                    generics: None,
                    ctor_arguments: CommaPunctuatedArgs::from_iter([ArgKind::inherited_named_by_ref(ctor_arg_composer).present(&source)]),
                    body_presentation: quote!({#body})
                }
            },
            Self::SmartPointer(SmartPointerKind::Mutex(..), aspect, attrs, lifetimes, SmartPointerPresentableContext::Dtor(generics, _name_kind)) => {
                let ty = aspect.present(source);
                BindingPresentation::Destructor {
                    attrs: attrs.clone(),
                    lifetimes: lifetimes.clone(),
                    name: Name::<RustSpecification>::Destructor(ty.clone()).mangle_tokens_default(),
                    ty,
                    generics: generics.clone()
                }
            },
            Self::SmartPointer(SmartPointerKind::Mutex(..), aspect, attrs, lifetimes, SmartPointerPresentableContext::Read(arg_field_composer, arg_type, from_root_conversion, to_inner_conversion)) => {
                let from_root_conversion = from_root_conversion.present(source);
                let to_inner_conversion = to_inner_conversion.present(source);
                BindingPresentation::Any {
                    attrs: attrs.clone(),
                    body: present_pub_function(
                        attrs,
                        Name::<RustSpecification>::Read(aspect.present(source)).mangle_tokens_default(),
                        CommaPunctuatedArgs::from_iter([ArgKind::inherited_named_by_ref(arg_field_composer).present(&source)]),
                        ReturnType::Type(RArrow::default(), arg_type.clone().into()),
                        None,
                        lifetimes.clone(),
                        quote!(
                            let lock = #from_root_conversion;
                            let obj = lock.lock().unwrap();
                            #to_inner_conversion
                        )
                    )
                }
            },
            Self::SmartPointer(SmartPointerKind::Mutex(..), aspect, attrs, lifetimes, SmartPointerPresentableContext::Write(arg_field_composer, arg_var_composer, from_root_conversion, from_inner_conversion)) => {
                let from_root_conversion = from_root_conversion.present(source);
                let from_inner_conversion = from_inner_conversion.present(source);
                BindingPresentation::Any {
                    attrs: attrs.clone(),
                    body: present_pub_function(
                        attrs,
                        Name::<RustSpecification>::Write(aspect.present(source)).mangle_tokens_default(),
                        CommaPunctuatedArgs::from_iter([
                            ArgKind::inherited_named_by_ref(arg_field_composer).present(&source),
                            ArgKind::inherited_named_by_ref(arg_var_composer).present(source)
                        ]),
                        ReturnType::Default,
                        None,
                        lifetimes.clone(),
                        quote! {
                            let lock = #from_root_conversion;
                            let mut obj = lock.lock().unwrap();
                            *obj = #from_inner_conversion;
                        }
                    )
                }
            },
            Self::SmartPointer(SmartPointerKind::RwLock(..), aspect, attrs, lifetimes, SmartPointerPresentableContext::Ctor(ctor_arg_composer, from_arg_conversion)) => {
                let ty = aspect.present(source);
                let body = ArgKind::DefaultFieldByValueConversion(ctor_arg_composer.clone(), <RustSpecification as Specification>::Expr::boxed(from_arg_conversion.clone())).present(source);
                BindingPresentation::Constructor {
                    attrs: attrs.clone(),
                    lifetimes: lifetimes.clone(),
                    name: Name::<RustSpecification>::Constructor(ty.clone()).mangle_tokens_default(),
                    ty: ty.clone(),
                    generics: None,
                    ctor_arguments: CommaPunctuatedArgs::from_iter([ArgKind::inherited_named_by_ref(ctor_arg_composer).present(&source)]),
                    body_presentation: quote!({#body})
                }
            },
            Self::SmartPointer(SmartPointerKind::RwLock(..), aspect, attrs, lifetimes, SmartPointerPresentableContext::Dtor(generics, _name_kind)) => {
                let ty = aspect.present(source);
                BindingPresentation::Destructor {
                    attrs: attrs.clone(),
                    lifetimes: lifetimes.clone(),
                    name: Name::<RustSpecification>::Destructor(ty.clone()).mangle_tokens_default(),
                    ty,
                    generics: generics.clone()
                }
            },
            Self::SmartPointer(SmartPointerKind::RwLock(..), aspect, attrs, lifetimes, SmartPointerPresentableContext::Read(arg_field_composer, arg_type, from_root_conversion, to_inner_conversion)) => {
                let from_root_conversion = from_root_conversion.present(source);
                let to_inner_conversion = to_inner_conversion.present(source);
                BindingPresentation::Any {
                    attrs: attrs.clone(),
                    body: present_pub_function(
                        attrs,
                        Name::<RustSpecification>::Read(aspect.present(source)).mangle_tokens_default(),
                        CommaPunctuatedArgs::from_iter([ArgKind::inherited_named_by_ref(arg_field_composer).present(&source)]),
                        ReturnType::Type(RArrow::default(), arg_type.clone().into()),
                        None,
                        lifetimes.clone(),
                        quote!(
                            let lock = #from_root_conversion;
                            let obj = lock.read().unwrap();
                            #to_inner_conversion
                        )
                    )
                }
            },
            Self::SmartPointer(SmartPointerKind::RwLock(..), aspect, attrs, lifetimes, SmartPointerPresentableContext::Write(arg_field_composer, arg_var_composer, from_root_conversion, from_inner_conversion)) => {
                let from_root_conversion = from_root_conversion.present(source);
                let from_inner_conversion = from_inner_conversion.present(source);
                BindingPresentation::Any {
                    attrs: attrs.clone(),
                    body: present_pub_function(
                        attrs,
                        Name::<RustSpecification>::Write(aspect.present(source)).mangle_tokens_default(),
                        CommaPunctuatedArgs::from_iter([
                            ArgKind::inherited_named_by_ref(arg_field_composer).present(&source),
                            ArgKind::inherited_named_by_ref(arg_var_composer).present(source)
                        ]),
                        ReturnType::Default,
                        None,
                        lifetimes.clone(),
                        quote! {
                            let lock = #from_root_conversion;
                            let mut obj = lock.write().unwrap();
                            *obj = #from_inner_conversion;
                        }
                    )
                }
            },
            Self::SmartPointer(SmartPointerKind::OnceLock(..), aspect, attrs, lifetimes, SmartPointerPresentableContext::Ctor(ctor_arg_composer, from_arg_conversion)) => {
                let ty = aspect.present(source);
                let body = ArgKind::DefaultFieldByValueConversion(ctor_arg_composer.clone(), <RustSpecification as Specification>::Expr::boxed(from_arg_conversion.clone())).present(source);
                BindingPresentation::Constructor {
                    attrs: attrs.clone(),
                    lifetimes: lifetimes.clone(),
                    name: Name::<RustSpecification>::Constructor(ty.clone()).mangle_tokens_default(),
                    ty: ty.clone(),
                    generics: None,
                    ctor_arguments: CommaPunctuatedArgs::new(),
                    body_presentation: quote!({#body})
                }
            },
            Self::SmartPointer(SmartPointerKind::OnceLock(..), aspect, attrs, lifetimes, SmartPointerPresentableContext::Dtor(generics, _name_kind)) => {
                let ty = aspect.present(source);
                BindingPresentation::Destructor {
                    attrs: attrs.clone(),
                    lifetimes: lifetimes.clone(),
                    name: Name::<RustSpecification>::Destructor(ty.clone()).mangle_tokens_default(),
                    ty,
                    generics: generics.clone()
                }
            },
            Self::SmartPointer(SmartPointerKind::OnceLock(..), aspect, attrs, lifetimes, SmartPointerPresentableContext::Read(arg_field_composer, arg_type, from_root_conversion, to_inner_conversion)) => {
                let from_root_conversion = from_root_conversion.present(source);
                let to_inner_conversion = to_inner_conversion.present(source);
                BindingPresentation::Any {
                    attrs: attrs.clone(),
                    body: present_pub_function(
                        attrs,
                        Name::<RustSpecification>::Read(aspect.present(source)).mangle_tokens_default(),
                        CommaPunctuatedArgs::from_iter([ArgKind::inherited_named_by_ref(arg_field_composer).present(&source)]),
                        ReturnType::Type(RArrow::default(), arg_type.clone().into()),
                        None,
                        lifetimes.clone(),
                        quote!(
                            let lock = #from_root_conversion;
                            let obj = lock.get().unwrap();
                            #to_inner_conversion
                        )
                    )
                }
            },
            Self::SmartPointer(SmartPointerKind::OnceLock(..), aspect, attrs, lifetimes, SmartPointerPresentableContext::Write(arg_field_composer, arg_var_composer, from_root_conversion, from_inner_conversion)) => {
                let from_root_conversion = from_root_conversion.present(source);
                let from_inner_conversion = from_inner_conversion.present(source);
                BindingPresentation::Any {
                    attrs: attrs.clone(),
                    body: present_pub_function(
                        attrs,
                        Name::<RustSpecification>::Write(aspect.present(source)).mangle_tokens_default(),
                        CommaPunctuatedArgs::from_iter([
                            ArgKind::inherited_named_by_ref(arg_field_composer).present(&source),
                            ArgKind::inherited_named_by_ref(arg_var_composer).present(source)
                        ]),
                        ReturnType::Default,
                        None,
                        lifetimes.clone(),
                        quote! {
                            let lock = #from_root_conversion;
                            _ = lock.set(#from_inner_conversion);
                        }
                    )
                }
            },
            Self::SmartPointer(SmartPointerKind::Cell(..), aspect, attrs, lifetimes, SmartPointerPresentableContext::Ctor(ctor_arg_composer, from_arg_conversion)) => {
                let ty = aspect.present(source);
                let body = ArgKind::DefaultFieldByValueConversion(ctor_arg_composer.clone(), <RustSpecification as Specification>::Expr::boxed(from_arg_conversion.clone())).present(source);
                BindingPresentation::Constructor {
                    attrs: attrs.clone(),
                    lifetimes: lifetimes.clone(),
                    name: Name::<RustSpecification>::Constructor(ty.clone()).mangle_tokens_default(),
                    ty: ty.clone(),
                    generics: None,
                    ctor_arguments: CommaPunctuatedArgs::from_iter([ArgKind::inherited_named_by_ref(ctor_arg_composer).present(&source)]),
                    body_presentation: quote!({#body})
                }
            },
            Self::SmartPointer(SmartPointerKind::Cell(..), aspect, attrs, lifetimes, SmartPointerPresentableContext::Dtor(generics, _name_kind)) => {
                let ty = aspect.present(source);
                BindingPresentation::Destructor {
                    attrs: attrs.clone(),
                    lifetimes: lifetimes.clone(),
                    name: Name::<RustSpecification>::Destructor(ty.clone()).mangle_tokens_default(),
                    ty,
                    generics: generics.clone()
                }
            },
            Self::SmartPointer(SmartPointerKind::Cell(..), aspect, attrs, lifetimes, SmartPointerPresentableContext::Read(arg_field_composer, arg_type, from_root_conversion, to_inner_conversion)) => {
                let from_root_conversion = from_root_conversion.present(source);
                let to_inner_conversion = to_inner_conversion.present(source);
                BindingPresentation::Any {
                    attrs: attrs.clone(),
                    body: present_pub_function(
                        attrs,
                        Name::<RustSpecification>::Read(aspect.present(source)).mangle_tokens_default(),
                        CommaPunctuatedArgs::from_iter([ArgKind::inherited_named_by_ref(arg_field_composer).present(&source)]),
                        ReturnType::Type(RArrow::default(), arg_type.clone().into()),
                        None,
                        lifetimes.clone(),
                        quote!(
                            let lock = #from_root_conversion;
                            let obj = lock.get();
                            #to_inner_conversion
                        )
                    )
                }
            },
            Self::SmartPointer(SmartPointerKind::Cell(..), aspect, attrs, lifetimes, SmartPointerPresentableContext::Write(arg_field_composer, arg_var_composer, from_root_conversion, from_inner_conversion)) => {
                let from_root_conversion = from_root_conversion.present(source);
                let from_inner_conversion = from_inner_conversion.present(source);
                BindingPresentation::Any {
                    attrs: attrs.clone(),
                    body: present_pub_function(
                        attrs,
                        Name::<RustSpecification>::Write(aspect.present(source)).mangle_tokens_default(),
                        CommaPunctuatedArgs::from_iter([
                            ArgKind::inherited_named_by_ref(arg_field_composer).present(&source),
                            ArgKind::inherited_named_by_ref(arg_var_composer).present(source)
                        ]),
                        ReturnType::Default,
                        None,
                        lifetimes.clone(),
                        quote! {
                            let lock = #from_root_conversion;
                            lock.set(#from_inner_conversion);
                        }
                    )
                }
            },
            Self::SmartPointer(SmartPointerKind::RefCell(..), aspect, attrs, lifetimes, SmartPointerPresentableContext::Ctor(ctor_arg_composer, from_arg_conversion)) => {
                let ty = aspect.present(source);
                let body = ArgKind::DefaultFieldByValueConversion(ctor_arg_composer.clone(), <RustSpecification as Specification>::Expr::boxed(from_arg_conversion.clone())).present(source);
                BindingPresentation::Constructor {
                    attrs: attrs.clone(),
                    lifetimes: lifetimes.clone(),
                    name: Name::<RustSpecification>::Constructor(ty.clone()).mangle_tokens_default(),
                    ty: ty.clone(),
                    generics: None,
                    ctor_arguments: CommaPunctuatedArgs::from_iter([ArgKind::inherited_named_by_ref(ctor_arg_composer).present(&source)]),
                    body_presentation: quote!({#body})
                }
            },
            Self::SmartPointer(SmartPointerKind::RefCell(..), aspect, attrs, lifetimes, SmartPointerPresentableContext::Dtor(generics, _name_kind)) => {
                let ty = aspect.present(source);
                BindingPresentation::Destructor {
                    attrs: attrs.clone(),
                    lifetimes: lifetimes.clone(),
                    name: Name::<RustSpecification>::Destructor(ty.clone()).mangle_tokens_default(),
                    ty,
                    generics: generics.clone()
                }
            },
            Self::SmartPointer(SmartPointerKind::RefCell(..), aspect, attrs, lifetimes, SmartPointerPresentableContext::Read(arg_field_composer, arg_type, from_root_conversion, to_inner_conversion)) => {
                let from_root_conversion = from_root_conversion.present(source);
                let to_inner_conversion = to_inner_conversion.present(source);
                BindingPresentation::Any {
                    attrs: attrs.clone(),
                    body: present_pub_function(
                        attrs,
                        Name::<RustSpecification>::Read(aspect.present(source)).mangle_tokens_default(),
                        CommaPunctuatedArgs::from_iter([ArgKind::inherited_named_by_ref(arg_field_composer).present(&source)]),
                        ReturnType::Type(RArrow::default(), arg_type.clone().into()),
                        None,
                        lifetimes.clone(),
                        quote!(
                            let lock = #from_root_conversion;
                            let obj = lock.borrow();
                            #to_inner_conversion
                        )
                    )
                }
            },
            Self::SmartPointer(SmartPointerKind::RefCell(..), aspect, attrs, lifetimes, SmartPointerPresentableContext::Write(arg_field_composer, arg_var_composer, from_root_conversion, from_inner_conversion)) => {
                let from_root_conversion = from_root_conversion.present(source);
                let from_inner_conversion = from_inner_conversion.present(source);
                BindingPresentation::Any {
                    attrs: attrs.clone(),
                    body: present_pub_function(
                        attrs,
                        Name::<RustSpecification>::Write(aspect.present(source)).mangle_tokens_default(),
                        CommaPunctuatedArgs::from_iter([
                            ArgKind::inherited_named_by_ref(arg_field_composer).present(&source),
                            ArgKind::inherited_named_by_ref(arg_var_composer).present(source)
                        ]),
                        ReturnType::Default,
                        None,
                        lifetimes.clone(),
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
            BindingPresentableContext::SmartPointer(SmartPointerKind::UnsafeCell(..), aspect, attrs, lifetimes, SmartPointerPresentableContext::Ctor(ctor_arg_composer, from_arg_conversion)) => {
                let ty = aspect.present(source);
                let body = ArgKind::DefaultFieldByValueConversion(ctor_arg_composer.clone(), <RustSpecification as Specification>::Expr::boxed(from_arg_conversion.clone())).present(source);
                BindingPresentation::Constructor {
                    attrs: attrs.clone(),
                    lifetimes: lifetimes.clone(),
                    name: Name::<RustSpecification>::Constructor(ty.clone()).mangle_tokens_default(),
                    ty: ty.clone(),
                    generics: None,
                    ctor_arguments: CommaPunctuatedArgs::from_iter([ArgKind::inherited_named_by_ref(ctor_arg_composer).present(&source)]),
                    body_presentation: quote!({#body})
                }
            },
            Self::SmartPointer(SmartPointerKind::UnsafeCell(..), aspect, attrs, lifetimes, SmartPointerPresentableContext::Dtor(generics, _name_kind)) => {
                let ty = aspect.present(source);
                BindingPresentation::Destructor {
                    attrs: attrs.clone(),
                    lifetimes: lifetimes.clone(),
                    name: Name::<RustSpecification>::Destructor(ty.clone()).mangle_tokens_default(),
                    ty,
                    generics: generics.clone()
                }
            },
            Self::SmartPointer(SmartPointerKind::UnsafeCell(..), aspect, attrs, lifetimes, SmartPointerPresentableContext::Read(arg_field_composer, arg_type, from_root_conversion, to_inner_conversion)) => {
                let from_root_conversion = from_root_conversion.present(source);
                let to_inner_conversion = to_inner_conversion.present(source);
                BindingPresentation::Any {
                    attrs: attrs.clone(),
                    body: present_pub_function(
                        attrs,
                        Name::<RustSpecification>::Read(aspect.present(source)).mangle_tokens_default(),
                        CommaPunctuatedArgs::from_iter([ArgKind::inherited_named_by_ref(arg_field_composer).present(&source)]),
                        ReturnType::Type(RArrow::default(), arg_type.clone().into()),
                        None,
                        lifetimes.clone(),
                        quote!(
                            let lock = #from_root_conversion;
                            let obj = &*lock.get();
                            #to_inner_conversion
                        )
                    )
                }
            },
            Self::SmartPointer(SmartPointerKind::UnsafeCell(..), aspect, attrs, lifetimes, SmartPointerPresentableContext::Write(arg_field_composer, arg_var_composer, from_root_conversion, from_inner_conversion)) => {
                let from_root_conversion = from_root_conversion.present(source);
                let from_inner_conversion = from_inner_conversion.present(source);
                BindingPresentation::Any {
                    attrs: attrs.clone(),
                    body: present_pub_function(
                        attrs,
                        Name::<RustSpecification>::Write(aspect.present(source)).mangle_tokens_default(),
                        CommaPunctuatedArgs::from_iter([
                            ArgKind::inherited_named_by_ref(arg_field_composer).present(&source),
                            ArgKind::inherited_named_by_ref(arg_var_composer).present(source)
                        ]),
                        ReturnType::Default,
                        None,
                        lifetimes.clone(),
                        quote! {
                            let obj = #from_root_conversion;
                            *obj.get() = #from_inner_conversion;
                        }
                    )
                }
            },
            Self::SmartPointer(..) => panic!(""),
            Self::Callback(aspect, attrs, lifetimes, generics, ident, arg_target_fields, return_type, arg_to_conversions, post_processing, ffi_return_type, ffi_args) => {
                let ffi_type = aspect.present(source);

                BindingPresentation::Callback {
                    name: ident.clone(),
                    attrs: attrs.clone(),
                    ffi_args: ffi_args.clone(),
                    result: ffi_return_type.clone(),
                    conversion: InterfacePresentation::callback(
                        attrs,
                        ffi_type,
                        arg_target_fields.clone(),
                        return_type.clone(),
                        lifetimes,
                        arg_to_conversions.present(source),
                        post_processing
                    ),
                    generics: generics.clone(),
                    lifetimes: lifetimes.clone(),
                }
            }
        }
    }
}
