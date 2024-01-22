use proc_macro2::{Ident, TokenStream as TokenStream2};
use syn::{BareFnArg, FnArg, Pat, PatIdent, PatType, Receiver, ReturnType, Signature, Type, TypeBareFn, TypePath};
use quote::{quote, ToTokens};
use syn::punctuated::Punctuated;
use syn::token::Comma;
use crate::context::ScopeContext;
use crate::helper::{ffi_fn_name, from_array, from_path, from_slice, to_path};
use crate::holder::PathHolder;
use crate::interface::{NAMED_CONVERSION_PRESENTER, ROUND_BRACES_FIELDS_PRESENTER, SIMPLE_PAIR_PRESENTER};
use crate::presentation::context::OwnedItemPresenterContext;
use crate::presentation::ffi_object_presentation::FFIObjectPresentation;
use crate::presentation::ScopeContextPresentable;

#[derive(Clone, Debug)]
pub struct FnReturnTypeDecomposition {
    pub presentation: TokenStream2,
    pub conversion: TokenStream2
}

#[derive(Clone, Debug)]
pub struct FnArgDecomposition {
    pub name: Option<TokenStream2>,
    pub name_type_original: TokenStream2,
    pub name_type_conversion: TokenStream2,
}

#[derive(Clone, Debug)]
pub struct FnSignatureDecomposition {
    pub is_async: bool,
    pub ident: Option<Ident>,
    pub scope: PathHolder,
    pub return_type: FnReturnTypeDecomposition,
    pub arguments: Vec<FnArgDecomposition>,
}

impl FnSignatureDecomposition {
    pub fn from_signature(sig: &Signature, scope: PathHolder, context: &ScopeContext) -> Self {
        let Signature { output, ident, inputs, .. } = sig;
        // TODO: make a path
        let return_type = handle_fn_return_type(output, context);
        let ident = Some(ident.clone());
        let arguments = handle_fn_args(inputs, context);
        let is_async = sig.asyncness.is_some();
        FnSignatureDecomposition { is_async, ident, scope, return_type, arguments }
    }

    pub fn from_bare_fn(bare_fn: &TypeBareFn, ident: &Ident, scope: PathHolder, context: &ScopeContext) -> Self {
        let TypeBareFn { inputs, output, .. } = bare_fn;
        let arguments = handle_bare_fn_args(inputs, context);
        let return_type = handle_bare_fn_return_type(output, context);
        FnSignatureDecomposition {
            is_async: false,
            ident: Some(ident.clone()),
            scope,
            arguments,
            return_type
        }
    }

    pub fn present_callback(self) -> FFIObjectPresentation {
        let arguments = self.arguments.iter().map(|arg| arg.name_type_original.clone()).collect();
        let output_expression = self.return_type.presentation;
        FFIObjectPresentation::Callback {
            name: self.ident.clone().unwrap().to_token_stream(),
            arguments,
            output_expression
        }
    }

    pub fn present_fn(self, context: &ScopeContext) -> FFIObjectPresentation {
        let arguments = self.arguments.iter().map(|arg| arg.name_type_original.clone()).collect();
        let fn_name = self.ident.unwrap();
        let full_fn_path = self.scope.joined(&fn_name);
        let argument_conversions = self.arguments.iter().map(|arg| OwnedItemPresenterContext::Conversion(arg.name_type_conversion.clone())).collect::<Vec<_>>();
        let name = ffi_fn_name(&fn_name).to_token_stream();
        let input_conversions = ROUND_BRACES_FIELDS_PRESENTER((quote!(#full_fn_path), argument_conversions)).present(context);
        let output_expression = self.return_type.presentation;
        let output_conversions = self.return_type.conversion;
        if self.is_async {
            FFIObjectPresentation::AsyncFunction { name, arguments, input_conversions, output_expression, output_conversions }
        } else {
            FFIObjectPresentation::Function { name, arguments, input_conversions, output_expression, output_conversions }
        }
    }

    pub fn present_trait_vtable_inner_fn(self, context: &ScopeContext) -> TokenStream2 {
        let arguments = self.arguments.iter().map(|arg| OwnedItemPresenterContext::Conversion(arg.name_type_original.clone())).collect::<Vec<_>>();
        // println!("present_trait_vtable_inner_fn: {}", quote!(#(#arguments),*));
        let fn_name = self.ident.unwrap();
        let name_and_args = ROUND_BRACES_FIELDS_PRESENTER((quote!(unsafe extern "C" fn), arguments)).present(context);
        let output_expression = self.return_type.presentation;
        quote!(pub #fn_name: #name_and_args -> #output_expression)
    }
}

fn handle_fn_return_type(output: &ReturnType, context: &ScopeContext) -> FnReturnTypeDecomposition {
    match output {
        ReturnType::Default => FnReturnTypeDecomposition { presentation: quote!(()), conversion: quote!(;) },
        ReturnType::Type(_, field_type) => {
            let presentation = context.ffi_full_dictionary_field_type_presenter(field_type);
            let conversion = match &**field_type {
                Type::Path(TypePath { path, .. }) => to_path(quote!(obj), path, context),
                _ => panic!("error: output conversion: {}", quote!(#field_type)),
            };
            FnReturnTypeDecomposition { presentation, conversion }
        },
    }
}
fn handle_arg_type(ty: &Type, pat: &Pat, context: &ScopeContext) -> TokenStream2 {
    match (ty, pat) {
        (Type::Path(TypePath { path, .. }), Pat::Ident(PatIdent { ident, .. })) =>
            from_path(quote!(#ident), path),
        (Type::Reference(type_reference), pat) => {
            let arg_type = handle_arg_type(&type_reference.elem, pat, context);
            if let Some(_mutable) = type_reference.mutability {
                quote!(&mut #arg_type)
            } else {
                quote!(&#arg_type)
            }
        },
        (Type::Array(type_array), Pat::Ident(PatIdent { ident, .. })) => {
            // let arg_type = handle_arg_type(&type_array.elem, pat, context);
            // let len = &type_array.len;
            from_array(quote!(#ident), type_array)
        },
        (Type::Slice(type_slice), Pat::Ident(PatIdent { ident, .. })) => {
            from_slice(quote!(#ident), type_slice)
        },
        (Type::TraitObject(type_trait_object), Pat::Ident(PatIdent { ident, .. })) => {

            quote!(&#type_trait_object)
        },
        // (Type::Ptr(TypePtr { star_token, const_token, mutability, elem }), Pat::Ident(PatIdent { ident, .. })) =>
        _ => panic!("error: Arg conversion not supported: {}", quote!(#ty)),
    }
}

fn handle_bare_fn_return_type(output: &ReturnType, context: &ScopeContext) -> FnReturnTypeDecomposition {
    match output {
        ReturnType::Default => FnReturnTypeDecomposition { presentation: quote!(), conversion: quote!() },
        ReturnType::Type(token, field_type) => {
            let pres = context.ffi_full_dictionary_field_type_presenter(field_type);
            let presentation = SIMPLE_PAIR_PRESENTER(quote!(#token), pres);
            FnReturnTypeDecomposition { presentation, conversion: quote!() }
        }
    }
}

fn handle_fn_args(inputs: &Punctuated<FnArg, Comma>, context: &ScopeContext) -> Vec<FnArgDecomposition> {
    // TODO: replace Fn arguments with crate::fermented::generics::#ident or #import
    inputs
        .iter()
        .map(|arg| match arg {
            FnArg::Receiver(Receiver { mutability, .. }) => FnArgDecomposition {
                name: None,
                name_type_original: match mutability {
                    Some(..) => quote!(obj: *mut ()),
                    _ => quote!(obj: *const ())
                },
                name_type_conversion: quote!()
            },
            FnArg::Typed(PatType { ty, pat, .. }) => {
                let pres = context.ffi_full_dictionary_field_type_presenter(ty);
                let name_type_original = NAMED_CONVERSION_PRESENTER(pat.to_token_stream(), quote!(#pres));
                let name_type_conversion = handle_arg_type(ty, pat, context);
                FnArgDecomposition {
                    name: Some(pat.to_token_stream()),
                    name_type_original,
                    name_type_conversion
                }
            },
        })
        .collect()
}

fn handle_bare_fn_args(inputs: &Punctuated<BareFnArg, Comma>, context: &ScopeContext) -> Vec<FnArgDecomposition> {
    inputs
        .iter()
        .map(|BareFnArg { ty, name, .. }| {
            let name = name.clone().map(|(ident, _)| ident.to_token_stream());
            let pres = context.ffi_full_dictionary_field_type_presenter(ty);
            FnArgDecomposition {
                name: name.clone(),
                name_type_original: NAMED_CONVERSION_PRESENTER(name.unwrap(), pres),
                name_type_conversion: quote!()
            }
        })
        .collect::<Vec<_>>()
}
