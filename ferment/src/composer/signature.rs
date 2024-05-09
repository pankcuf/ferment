use std::cell::RefCell;
use std::rc::Rc;
use proc_macro2::Ident;
use quote::{quote, ToTokens};
use syn::{Attribute, Generics, ItemFn, Path, Signature, TypeBareFn};
use syn::punctuated::Punctuated;
use syn::token::{Comma, Paren};
use crate::composer::{AttrsComposer, BindingComposer, Composer, constants, Depunctuated, ParentComposer, SigParentComposer, TypeContextComposer};
use crate::composer::basic::BasicComposer;
use crate::composer::composable::{BasicComposable, SourceExpandable, NameContext};
use crate::composer::r#type::TypeComposer;
use crate::composition::{AttrsComposition, CfgAttributes, FnSignatureContext};
use crate::context::{ScopeChain, ScopeContext};
use crate::naming::Name;
use crate::presentation::{BindingPresentation, DocPresentation, Expansion, ScopeContextPresentable};
use crate::presentation::context::{name::Context, OwnedItemPresentableContext};
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
        let cfg_attrs = attrs.cfg_attributes();
        let root = Rc::new(RefCell::new(Self {
            base: BasicComposer::new(
                AttrsComposer::new(attrs),
                doc_composer,
                TypeComposer::new(Context::Fn { path, sig_context, attrs: cfg_attrs }),
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
    fn name_context_ref(&self) -> &Context {
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
        // let scope = source.scope.self_path_holder_ref();
        let binding = match self.name_context_ref() {
            Context::Fn { path: full_fn_path, sig_context, attrs } => {
                // println!("Context::Fn: {}: {:?}", full_fn_path.to_token_stream(), sig_context);
                match sig_context {
                    FnSignatureContext::ModFn(ItemFn { sig, .. }) => {
                        let Signature { output, inputs, .. } = sig;
                        let return_type = output.compose(&(false, &source));
                        let argument_comps = inputs
                            .iter()
                            .map(|arg| arg.compose(&(sig_context, &source)));
                        let arguments = argument_comps.clone()
                            .map(|arg| arg.name_type_original.clone())
                            .collect::<Punctuated<_, _>>();
                        let argument_conversions = argument_comps
                            .map(|arg| OwnedItemPresentableContext::Conversion(arg.name_type_conversion.present(&source), quote! {}))
                            .collect::<Punctuated<_, _>>();
                        let fields_presenter = constants::ROUND_BRACES_FIELDS_PRESENTER((target_name_context.clone(), argument_conversions));
                        BindingPresentation::RegularFunction {
                            attrs: attrs.to_token_stream(),
                            is_async: sig.asyncness.is_some(),
                            arguments: arguments.present(&source),
                            name: Name::ModFn(full_fn_path.clone()),
                            input_conversions: fields_presenter.present(&source),
                            return_type: return_type.presentation.clone(),
                            output_conversions: return_type.conversion.present(&source)
                        }
                    },
                    FnSignatureContext::Impl(_, _, sig) => {
                        let Signature { output, inputs, .. } = sig;
                        let return_type = output.compose(&(false, &source));
                        let argument_comps = inputs
                            .iter()
                            .map(|arg| arg.compose(&(sig_context, &source)));
                        let arguments = argument_comps.clone()
                            .map(|arg| arg.name_type_original.clone())
                            .collect::<Punctuated<_, _>>();
                        let argument_conversions = argument_comps
                            .map(|arg| OwnedItemPresentableContext::Conversion(arg.name_type_conversion.present(&source), quote! {}))
                            .collect::<Punctuated<_, _>>();
                        let fields_presenter = constants::ROUND_BRACES_FIELDS_PRESENTER((ffi_name_context.clone(), argument_conversions));
                        BindingPresentation::RegularFunction {
                            attrs: attrs.to_token_stream(),
                            is_async: sig.asyncness.is_some(),
                            arguments: arguments.present(&source),
                            name: Name::ModFn(full_fn_path.clone()),
                            input_conversions: fields_presenter.present(&source),
                            return_type: return_type.presentation.clone(),
                            output_conversions: return_type.conversion.present(&source)
                        }
                    },
                    FnSignatureContext::TraitInner(_, _, sig) => {
                        let Signature { output, inputs, .. } = sig;
                        let return_type = output.compose(&(false, &source));
                        let argument_comps = inputs
                            .iter()
                            .map(|arg| arg.compose(&(sig_context, &source)));

                        let arguments = argument_comps
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
                    },
                    FnSignatureContext::Bare(target_name, type_bare_fn) => {
                        let TypeBareFn { inputs, output, .. } = type_bare_fn;
                        let argument_comps = inputs.compose(&source);
                        let return_type = output.compose(&(true, &source));
                        let arguments = argument_comps
                            .iter()
                            .map(|arg| arg.name_type_original.present(&source))
                            .collect::<Punctuated<_, _>>();
                        let output_expression = return_type.presentation;
                        BindingPresentation::Callback {
                            name: target_name.to_token_stream(),
                            arguments,
                            output_expression
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
