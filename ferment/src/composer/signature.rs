use std::cell::RefCell;
use std::rc::Rc;
use proc_macro2::Ident;
use quote::{quote, ToTokens};
use syn::{Attribute, BareFnArg, Generics, ItemFn, Path, ReturnType, Signature, TypeBareFn};
use syn::__private::TokenStream2;
use syn::punctuated::Punctuated;
use syn::token::RArrow;
use crate::composer::{AttrsComposer, BindingComposer, CommaPunctuated, Composer, constants, Depunctuated, ParentComposer, ParenWrapped, SigParentComposer, TypeContextComposer};
use crate::composer::basic::BasicComposer;
use crate::composer::composable::{BasicComposable, SourceExpandable, NameContext, SourceAccessible};
use crate::composer::generics_composer::GenericsComposer;
use crate::composer::r#type::TypeComposer;
use crate::composition::{AttrsComposition, CfgAttributes, FnReturnTypeComposer, FnSignatureContext};
use crate::context::{ScopeChain, ScopeContext};
use crate::conversion::{GenericTypeConversion, TypeConversion};
use crate::ext::{Conversion, FFIResolveExtended, FFITypeResolve, Mangle};
use crate::naming::{DictionaryExpr, DictionaryName, FFIConversionMethodExpr, InterfacesMethodExpr, Name};
use crate::presentation::{BindingPresentation, DocPresentation, Expansion, InterfacePresentation, ScopeContextPresentable};
use crate::presentation::context::{FieldContext, name::Context, OwnedItemPresentableContext};
use crate::presentation::context::name::Aspect;
use crate::shared::ParentLinker;

pub struct SigComposer {
    pub base: BasicComposer<SigParentComposer>,
    pub binding_composer: BindingComposer<SigParentComposer>
}

impl SigComposer {

    fn new(
        path: Path,
        sig_context: FnSignatureContext,
        generics: Option<Generics>,
        attrs: AttrsComposition,
        doc_composer: TypeContextComposer<SigParentComposer>,
        binding_composer: BindingComposer<SigParentComposer>,
        context: &ParentComposer<ScopeContext>) -> SigParentComposer {
        let ty_context = Context::Fn { path, sig_context, attrs: attrs.cfg_attributes_expanded() };
        let root = Rc::new(RefCell::new(Self {
            base: BasicComposer::new(
                AttrsComposer::new(attrs),
                doc_composer,
                TypeComposer::new(ty_context),
                GenericsComposer::new(generics),
                Rc::clone(context)
            ),
            binding_composer,
        }));
        {
            let mut composer = root.borrow_mut();
            composer.base.link(&root);
        }
        root
    }
    pub fn with_context(
        path: Path,
        target_name: &Ident,
        sig_context: FnSignatureContext,
        generics: &Generics,
        attrs: &Vec<Attribute>,
        scope: &ScopeChain,
        context: &ParentComposer<ScopeContext>
    ) -> SigParentComposer {
        Self::new(
            path,
            sig_context,
            Some(generics.clone()),
            AttrsComposition::from(attrs, target_name, scope),
            constants::composer_doc_default(),
            |composer| {
                let composer = composer.borrow();
                let _source = composer.base.context.borrow();
                BindingPresentation::Empty
            },
            context)
    }
}


impl BasicComposable<SigParentComposer> for SigComposer {
    fn compose_attributes(&self) -> Depunctuated<Expansion> {
        self.base.compose_attributes()
    }
    fn compose_docs(&self) -> DocPresentation {
        DocPresentation::Direct(self.base.doc.compose(&()))
    }
}


impl NameContext for SigComposer {
    fn name_context_ref(&self) -> &Context {
        self.base.name_context_ref()
    }
}

fn regular_fn(path: Path, aspect: Aspect, attrs: TokenStream2, generics: Option<Generics>, sig: &Signature, sig_context: &FnSignatureContext, source: &ScopeContext) -> BindingPresentation {
    let Signature { output, inputs, asyncness, .. } = sig;
    // let return_type = output.compose(&(false, &source));
    // let (bare, source) = source;
    let return_type = match output {
        ReturnType::Default => FnReturnTypeComposer {
            presentation: ReturnType::Default,
            conversion: FieldContext::LineTermination
        },
        ReturnType::Type(_, ty) => FnReturnTypeComposer {
            presentation: ReturnType::Type(RArrow::default(), Box::new(ty.ffi_full_dictionary_type_presenter(source))),
            conversion: ty.conversion_to(FieldContext::Obj)
        }
    };

    let argument_comps = inputs
        .iter()
        .map(|arg| arg.compose(&(sig_context, &source)));
    let arguments = Punctuated::from_iter(argument_comps.clone()
        .map(|arg| arg.name_type_original.clone()));
    let argument_conversions = CommaPunctuated::from_iter(argument_comps
        .map(|arg| OwnedItemPresentableContext::Conversion(arg.name_type_conversion.present(&source), quote!())));
    let fields_presenter = constants::ROUND_BRACES_FIELDS_PRESENTER((aspect, argument_conversions));
    BindingPresentation::RegularFunction {
        attrs,
        is_async: asyncness.is_some(),
        arguments: arguments.present(&source),
        name: Name::ModFn(path),
        input_conversions: fields_presenter.present(&source),
        return_type: return_type.presentation.clone(),
        generics,
        output_conversions: return_type.conversion.present(&source)
    }
}

impl SourceAccessible for SigComposer {
    fn context(&self) -> &ParentComposer<ScopeContext> {
        self.base.context()
    }
}

impl SourceExpandable for SigComposer {
    fn expand(&self) -> Expansion {
        let source = self.context().borrow();
        // TODO: source.scope or local_scope?
        // let scope = source.scope.self_path_holder_ref();
        let binding = match self.name_context_ref() {
            Context::Fn { path: full_fn_path, sig_context, attrs } => {
                println!("SigComposer::expand: Fn: {:?}", sig_context);
                match sig_context {
                    FnSignatureContext::ModFn(ItemFn { sig, .. }) => regular_fn(
                        full_fn_path.clone(),
                        self.target_name_aspect(),
                        attrs.to_token_stream(),
                        None,
                        sig,
                        sig_context,
                        &source
                    ),
                    FnSignatureContext::Impl(_, _, sig) => regular_fn(
                        full_fn_path.clone(),
                        self.ffi_name_aspect(),
                        attrs.to_token_stream(),
                        None,
                        sig,
                        sig_context,
                        &source
                    ),
                    FnSignatureContext::TraitInner(_, _, sig) => {
                        let Signature { output, inputs, .. } = sig;
                        let return_type = match output {
                            ReturnType::Default => FnReturnTypeComposer {
                                presentation: ReturnType::Default,
                                conversion: FieldContext::LineTermination
                            },
                            ReturnType::Type(_, ty) => FnReturnTypeComposer {
                                presentation: ReturnType::Type(RArrow::default(), Box::new(ty.ffi_full_dictionary_type_presenter(&source))),
                                conversion: ty.conversion_to(FieldContext::Obj)
                            },
                        };
                        let argument_comps = inputs
                            .iter()
                            .map(|arg| arg.compose(&(sig_context, &source)));

                        let arguments = CommaPunctuated::from_iter(argument_comps
                            .map(|arg| arg.name_type_original.clone()));
                        // let presentation = ParenWrapped::new(arguments.present(&source));
                        let presentation = ParenWrapped::new(arguments).present(&source);
                        let output_expression = return_type.presentation;



                        BindingPresentation::TraitVTableInnerFn {
                            name: Name::VTableInnerFn(sig.ident.clone()),
                            name_and_args: quote!(unsafe extern "C" fn #presentation),
                            output_expression
                        }
                    },
                    FnSignatureContext::Bare(_target_name, type_bare_fn) => {
                        let TypeBareFn { inputs, output, .. } = type_bare_fn;
                        let ffi_result = DictionaryName::FFiResult;
                        let opt_conversion = |conversion: TokenStream2| quote!((!ffi_result.is_null()).then(|| { #conversion }));

                        let from_complex_result = || quote! {
                            let result = ferment_interfaces::FFIConversion::ffi_from(#ffi_result);
                            (self.destructor)(#ffi_result);
                            result
                        };
                        let from_primitive_result = || quote!(ffi_result);
                        let from_opt_primitive_result = || DictionaryExpr::Deref(ffi_result.to_token_stream()).to_token_stream();

                        let (return_type, ffi_return_type, post_processing) = match output {
                            ReturnType::Type(token, field_type) => (
                                ReturnType::Type(token.clone(), Box::new(source.full_type_for(field_type))),
                                ReturnType::Type(token.clone(), Box::new(field_type.ffi_full_dictionary_type_presenter(&source))),
                                match TypeConversion::from(field_type) {
                                    TypeConversion::Primitive(_) => from_primitive_result(),
                                    TypeConversion::Complex(_) =>  from_complex_result(),
                                    TypeConversion::Generic(generic_ty) => match generic_ty {
                                        GenericTypeConversion::TraitBounds(_) => unimplemented!("TODO: mixins+traits+generics"),
                                        GenericTypeConversion::Optional(ty) => {
                                            opt_conversion(match TypeConversion::from(ty) {
                                                TypeConversion::Primitive(_) => from_opt_primitive_result(),
                                                TypeConversion::Complex(_) |
                                                TypeConversion::Generic(_) => from_complex_result(),
                                            })
                                        }
                                        _ => from_complex_result()
                                    }
                                }
                            ),
                            ReturnType::Default => (ReturnType::Default, ReturnType::Default, from_primitive_result()),
                        };
                        let mut arg_names = CommaPunctuated::new();
                        let mut ffi_args = CommaPunctuated::new();
                        let mut arg_target_types = CommaPunctuated::new();
                        let mut arg_to_conversions = CommaPunctuated::new();
                        inputs
                            .iter()
                            .for_each(|BareFnArg { ty, name, .. }| {
                                let conversion = TypeConversion::from(ty);
                                let name = Name::Optional(name.as_ref().map(|(ident, ..)| ident.clone()));
                                arg_names.push(name.to_token_stream());

                                arg_target_types.push(ty.to_token_stream());
                                ffi_args.push(match &conversion {
                                    TypeConversion::Primitive(ty) => ty.clone(),
                                    TypeConversion::Complex(ty) => ty.to_custom_or_ffi_type_mut_ptr(&source),
                                    // TypeConversion::Callback(ty) => ty.to_custom_or_ffi_type(&source),
                                    TypeConversion::Generic(generic_ty) => generic_ty.to_custom_or_ffi_type_mut_ptr(&source),
                                }.to_token_stream());
                                arg_to_conversions.push(match &conversion {
                                    // TypeConversion::Callback(..) =>
                                    //     unimplemented!("Callback in callback => |#arg_names| ferment_interfaces::FFICallback::apply(&name, (#arg_names))"),
                                    TypeConversion::Primitive(..) => name.to_token_stream(),
                                    TypeConversion::Generic(generic_ty) => match generic_ty {
                                        GenericTypeConversion::TraitBounds(_) => unimplemented!("TODO: mixins+traits+generics"),
                                        GenericTypeConversion::Optional(ty) => match TypeConversion::from(ty) {
                                            // TypeConversion::Callback(_) => unimplemented!("TODO: optional callback inside callback"),
                                            TypeConversion::Primitive(_) => InterfacesMethodExpr::ToOptPrimitive(name.to_token_stream()).to_token_stream(),
                                            TypeConversion::Complex(_) |
                                            TypeConversion::Generic(_) => FFIConversionMethodExpr::FfiToOpt(name.to_token_stream()).to_token_stream(),
                                        }
                                        _ => FFIConversionMethodExpr::FfiTo(name.to_token_stream()).to_token_stream()
                                    },
                                    TypeConversion::Complex(..) => FFIConversionMethodExpr::FfiTo(name.to_token_stream()).to_token_stream(),
                                });
                            });

                        let attrs = self.compose_attributes();
                        let conversion = InterfacePresentation::Callback {
                            attrs: attrs.to_token_stream(),
                            ffi_type: self.ffi_name_aspect().present(&source),
                            inputs: arg_target_types,
                            output: return_type,
                            body: quote! {
                                |(#arg_names)| {
                                    let ffi_result = (self.caller)(#arg_to_conversions);
                                    #post_processing
                                }
                            }
                        };
                        BindingPresentation::Callback {
                            name: full_fn_path.mangle_ident_default(),
                            attrs: attrs.clone(),
                            ffi_args: ffi_args,
                            result: ffi_return_type,
                            conversion
                        }
                    }
                }

            }
            _ => panic!("Wrong name context for fn")
        };
        Expansion::Function {
            comment: self.compose_docs(),
            binding
        }
    }
}
