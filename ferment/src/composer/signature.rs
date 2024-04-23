use std::cell::RefCell;
use std::rc::Rc;
use proc_macro2::Ident;
use quote::{quote, ToTokens};
use syn::{Attribute, FnArg, Generics, ItemFn, parse_quote, Path, Receiver, Signature, TypeBareFn};
use syn::punctuated::Punctuated;
use syn::token::{Comma, Paren};
use crate::composer::{AttrsComposer, BindingComposer, Composer, constants, Depunctuated, ParentComposer, SigParentComposer, TypeContextComposer};
use crate::composer::basic::BasicComposer;
use crate::composer::composable::{BasicComposable, SourceExpandable, NameContext};
use crate::composer::r#type::TypeComposer;
use crate::composition::{AttrsComposition, FnArgComposition, FnSignatureContext};
use crate::context::{ScopeChain, ScopeContext};
use crate::conversion::FieldTypeConversion;
use crate::ext::{Mangle, Resolve};
use crate::naming::{DictionaryFieldName, Name};
use crate::presentation::{BindingPresentation, DocPresentation, Expansion, ScopeContextPresentable};
use crate::presentation::context::{FieldTypePresentableContext, name, OwnedItemPresentableContext};
use crate::presentation::context::name::Context;
use crate::shared::ParentLinker;
use crate::wrapped::Wrapped;

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
        let root = Rc::new(RefCell::new(Self {
            base: BasicComposer::new(
                AttrsComposer::new(attrs),
                doc_composer,
                TypeComposer::new(Context::Fn { path, sig_context, }),
                generics,
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
                let source= composer.base.context.borrow();
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
    fn name_context_ref(&self) -> &name::Context {
        self.base.name_context_ref()
    }
}

impl SourceExpandable for SigComposer {
    fn context(&self) -> &ParentComposer<ScopeContext> {
        self.base.context()
    }
    fn expand(&self) -> Expansion {
        let source = self.context().borrow();
        let ffi_name_context = self.ffi_name_aspect();
        let target_name_context = self.target_name_aspect();
        // TODO: source.scope or local_scope?
        let scope = source.scope.self_path_holder_ref();
        let binding = match self.name_context_ref() {
            Context::Fn { path: full_fn_path, sig_context } => {
                println!("Context::Fn: {}: {:?}", full_fn_path.to_token_stream(), sig_context);
                match sig_context {
                    FnSignatureContext::ModFn(ItemFn { sig, .. }) => {
                        // println!("FnSignatureContext::ModFn: {}")
                        let Signature { output, ident, inputs, generics, .. } = sig;
                        let return_type = output.compose(&(false, &source));
                        let argument_comps = inputs
                            .iter()
                            .map(|arg| match arg {
                                FnArg::Receiver(Receiver { mutability, reference, .. }) => {
                                    panic!("receiver in mod fn")
                                },
                                FnArg::Typed(pat_ty) =>
                                    pat_ty.compose(&source),
                            })
                            .collect::<Vec<_>>();
                        let arguments = argument_comps
                            .iter()
                            .map(|arg| arg.name_type_original.clone())
                            .collect::<Punctuated<_, Comma>>();
                        let argument_conversions = argument_comps
                            .iter()
                            .map(|arg| OwnedItemPresentableContext::Conversion(arg.name_type_conversion.present(&source)))
                            .collect::<Punctuated<_, _>>();
                        let fields_presenter = constants::ROUND_BRACES_FIELDS_PRESENTER((target_name_context.clone(), argument_conversions));
                        BindingPresentation::RegularFunction {
                            is_async: sig.asyncness.is_some(),
                            arguments: arguments.present(&source),
                            name: Name::ModFn(full_fn_path.clone()),
                            input_conversions: fields_presenter.present(&source),
                            return_type: return_type.presentation.clone(),
                            output_conversions: return_type.conversion.present(&source)
                        }

                    },
                    FnSignatureContext::Impl(self_ty, maybe_trait_ty, sig) => {
                        let Signature { output, ident, inputs, generics, .. } = sig;
                        let return_type = output.compose(&(false, &source));
                        let argument_comps = inputs
                            .iter()
                            .map(|arg| match arg {
                                FnArg::Receiver(Receiver { mutability, reference, .. }) => {
                                    let (mangled_ident, name_type_conversion) = match maybe_trait_ty {
                                        Some(trait_ty) => (
                                            trait_ty.resolve(&source).mangle_ident_default(),
                                            FieldTypePresentableContext::SelfAsTrait(self_ty.resolve(&source).to_token_stream())
                                        ),
                                        None => (
                                            self_ty.resolve(&source).mangle_ident_default(),
                                            FieldTypePresentableContext::From(FieldTypePresentableContext::Self_.into())
                                        )
                                    };
                                    let access = mutability.as_ref().map_or(quote!(const), ToTokens::to_token_stream);
                                    let name_type_original = OwnedItemPresentableContext::Named(
                                        FieldTypeConversion::Named(
                                            Name::Dictionary(DictionaryFieldName::Self_),
                                            parse_quote!(* #access #mangled_ident)),
                                        false);
                                    let name_type_conversion = if reference.is_some() {
                                        FieldTypePresentableContext::AsRef(name_type_conversion.into())
                                    } else {
                                        name_type_conversion
                                    };
                                    FnArgComposition {
                                        name: None,
                                        name_type_original,
                                        name_type_conversion
                                    }
                                },
                                FnArg::Typed(pat_ty) =>
                                    pat_ty.compose(&source),
                            })
                            .collect::<Vec<_>>();
                        let arguments = argument_comps
                            .iter()
                            .map(|arg| arg.name_type_original.clone())
                            .collect::<Punctuated<_, Comma>>();
                        let argument_conversions = argument_comps
                            .iter()
                            .map(|arg| OwnedItemPresentableContext::Conversion(arg.name_type_conversion.present(&source)))
                            .collect::<Punctuated<_, _>>();
                        let fields_presenter = constants::ROUND_BRACES_FIELDS_PRESENTER((ffi_name_context.clone(), argument_conversions));
                        BindingPresentation::RegularFunction {
                            is_async: sig.asyncness.is_some(),
                            arguments: arguments.present(&source),
                            name: Name::ModFn(full_fn_path.clone()),
                            input_conversions: fields_presenter.present(&source),
                            return_type: return_type.presentation.clone(),
                            output_conversions: return_type.conversion.present(&source)
                        }
                    },
                    FnSignatureContext::TraitInner(self_ty, maybe_trait_ty, sig) => {
                        let Signature { output, ident, inputs, generics, .. } = sig;
                        let return_type = output.compose(&(false, &source));
                        let argument_comps = inputs
                            .iter()
                            .map(|arg| match arg {
                                FnArg::Receiver(Receiver { mutability, reference, .. }) => {
                                    let (mangled_ident, name_type_conversion) = match maybe_trait_ty {
                                        Some(trait_ty) => (
                                            trait_ty.resolve(&source).mangle_ident_default(),
                                            FieldTypePresentableContext::SelfAsTrait(self_ty.resolve(&source).to_token_stream())
                                        ),
                                        None => (
                                            self_ty.resolve(&source).mangle_ident_default(),
                                            FieldTypePresentableContext::From(FieldTypePresentableContext::Self_.into())
                                        )
                                    };
                                    let access = mutability.as_ref().map_or(quote!(const), ToTokens::to_token_stream);
                                    let name_type_original = OwnedItemPresentableContext::Named(
                                        FieldTypeConversion::Named(
                                            Name::Dictionary(DictionaryFieldName::Self_),
                                            parse_quote!(* #access #mangled_ident)),
                                        false);
                                    let name_type_conversion = if reference.is_some() {
                                        FieldTypePresentableContext::AsRef(name_type_conversion.into())
                                    } else {
                                        name_type_conversion
                                    };
                                    FnArgComposition {
                                        name: None,
                                        name_type_original,
                                        name_type_conversion
                                    }
                                },
                                FnArg::Typed(pat_ty) =>
                                    pat_ty.compose(&source),
                            })
                            .collect::<Vec<_>>();

                        let arguments = argument_comps.iter()
                            .map(|arg| arg.name_type_original.clone())
                            .collect::<Punctuated<_, Comma>>();
                        let presentation = Wrapped::<_, Paren>::new(arguments.present(&source));
                        let name_and_args = quote!(unsafe extern "C" fn #presentation);
                        let output_expression = return_type.presentation;

                        BindingPresentation::TraitVTableInnerFn {
                            name: Name::VTableInnerFn(sig.ident.clone()),
                            name_and_args,
                            output_expression
                        }

                        // FnSignatureComposition::from_signature(sig_context, sig, scope, &source)
                        //     .present(FnSignatureCompositionContext::TraitVTableInner, &source)
                    },
                    FnSignatureContext::Bare(target_name, type_bare_fn) => {
                        let TypeBareFn { inputs, output, .. } = type_bare_fn;
                        let argument_comps = inputs.compose(&source);
                        let return_type = output.compose(&(true, &source));
                        // FnSignatureComposition {
                        //     is_async: false,
                        //     ident: Some(ident.clone()),
                        //     scope: scope.clone(),
                        //     arguments,
                        //     return_type,
                        //     impl_context: FnSignatureContext::Bare(ident.clone(), bare_fn.clone()),
                        //     generics: None
                        // }

                        let arguments = argument_comps
                            .iter()
                            .map(|arg| arg.name_type_original.present(&source))
                            .collect::<Punctuated<_, _>>();
                        let output_expression = return_type.presentation;
                        // let name = self.ident.clone().unwrap().to_token_stream();
                        BindingPresentation::Callback {
                            name: target_name.to_token_stream(),
                            arguments,
                            output_expression
                        }


                        // FnSignatureComposition::from_bare_fn(type_bare_fn, target_name, scope, &source)
                        //     .present(FnSignatureCompositionContext::FFIObjectCallback, &source)
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
