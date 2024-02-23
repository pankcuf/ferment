use quote::{quote, ToTokens};
use syn::__private::TokenStream2;
use crate::context::ScopeContext;
use crate::interface::{ffi_from_conversion, ffi_to_conversion, interface, package, package_boxed_expression, package_unbox_any_expression, package_unbox_any_expression_terminated};
use crate::presentation::context::OwnerIteratorPresentationContext;
use crate::presentation::ScopeContextPresentable;

#[derive(Clone, Debug)]
pub enum FieldTypePresentationContext {
    Empty,
    Simple(TokenStream2),
    To(TokenStream2),
    ToOpt(TokenStream2),
    UnwrapOr(TokenStream2, TokenStream2),
    ToVec(OwnerIteratorPresentationContext),
    ToVecPtr,
    // Empty,
    // Callback {}
    LineTermination,
    UnboxAny(TokenStream2),
    UnboxAnyTerminated(TokenStream2),
    IsNull(TokenStream2),
    DestroyConversion(TokenStream2, TokenStream2),
    FromRawParts(TokenStream2),
    From(TokenStream2),
    FromOffsetMap,
    FromOpt(TokenStream2),
    FromArray(TokenStream2),
    AsRef(TokenStream2),
    AsMutRef(TokenStream2),
    IfThenSome(TokenStream2, TokenStream2),
    Named((TokenStream2, Box<FieldTypePresentationContext>)),
    Scope,
    Terminated(Box<FieldTypePresentationContext>),
}

impl ScopeContextPresentable for FieldTypePresentationContext {
    type Presentation = TokenStream2;

    fn present(&self, context: &ScopeContext) -> TokenStream2 {
        match self {
            FieldTypePresentationContext::Simple(field_path) =>
                field_path.to_token_stream(),
            FieldTypePresentationContext::To(field_path) =>
                ffi_to_conversion(field_path.to_token_stream()),
            FieldTypePresentationContext::UnwrapOr(field_path, default) => {
                quote!(#field_path.unwrap_or(#default))
            },
            FieldTypePresentationContext::ToVec(presentation_context) => {
                presentation_context.present(context)
            }
            FieldTypePresentationContext::ToOpt(field_path) => {
                let package = package();
                let interface = interface();
                quote!(#package::#interface::ffi_to_opt(#field_path))
            },
            FieldTypePresentationContext::ToVecPtr => {
                let expr = package_boxed_expression(quote!(o));
                let package = package();
                quote!(#package::boxed_vec(obj.map(|o| #expr).collect()))
            }
            FieldTypePresentationContext::LineTermination => quote!(;),
            FieldTypePresentationContext::Empty => quote!(),
            FieldTypePresentationContext::UnboxAny(field_path) =>
                package_unbox_any_expression(field_path.to_token_stream()),
            FieldTypePresentationContext::UnboxAnyTerminated(field_path) =>
                package_unbox_any_expression_terminated(field_path.to_token_stream()),
            FieldTypePresentationContext::IsNull(field_path) => {
                let conversion = package_unbox_any_expression_terminated(field_path.clone());
                quote!(if !#field_path.is_null() { #conversion })
            }
            FieldTypePresentationContext::DestroyConversion(field_path, path) => {
                let package = package();
                let interface = interface();
                quote!(<std::os::raw::c_char as #package::#interface<#path>>::destroy(#field_path))
            },

            FieldTypePresentationContext::FromRawParts(field_type) => {
                quote!(std::slice::from_raw_parts(values as *const #field_type, count).to_vec())
            },
            FieldTypePresentationContext::From(field_path) => {
                ffi_from_conversion(field_path.to_token_stream())
            },
            FieldTypePresentationContext::FromOpt(field_path) => {
                let package = package();
                let interface = interface();
                quote!(#package::#interface::ffi_from_opt(#field_path))
            },
            FieldTypePresentationContext::FromOffsetMap => {
                let ffi_from_conversion =
                    ffi_from_conversion(quote!(*values.add(i)));
                quote!((0..count).map(|i| #ffi_from_conversion).collect())
            },
            FieldTypePresentationContext::FromArray(field_path) => {
                quote!(*#field_path)
            },
            FieldTypePresentationContext::AsRef(field_path) => {
                quote!(&#field_path)
            },
            FieldTypePresentationContext::AsMutRef(field_path) => {
                quote!(&mut #field_path)
            },
            FieldTypePresentationContext::IfThenSome(field_path, condition) => {
                // quote!((#field_path > 0).then_some(#field_path))
                quote!((#field_path #condition).then_some(#field_path))
            }
            FieldTypePresentationContext::Named((l_value, r_value)) => {
                let ty = r_value.present(context);
                quote!(#l_value: #ty)
            }
            FieldTypePresentationContext::Scope => {
                quote!({})
            }
            FieldTypePresentationContext::Terminated(presentation_context) => {
                let ty = presentation_context.present(context);
                quote!(#ty;)
            }
        }
    }
}

