use quote::{quote, ToTokens};
use syn::__private::TokenStream2;
use crate::context::ScopeContext;
use crate::conversion::FieldTypeConversion;
use crate::interface::{ffi_from_conversion, ffi_to_conversion, interface, obj, package, package_boxed_expression, package_unbox_any_expression, package_unbox_any_expression_terminated};
use crate::presentation::context::OwnerIteratorPresentationContext;
use crate::presentation::ScopeContextPresentable;

#[derive(Clone, Debug)]
pub enum FieldTypePresentableContext {
    Empty,
    Simple(TokenStream2),
    Add(Box<FieldTypePresentableContext>, TokenStream2),
    To(Box<FieldTypePresentableContext>),
    ToOpt(Box<FieldTypePresentableContext>),
    UnwrapOr(Box<FieldTypePresentableContext>, TokenStream2),
    ToVec(OwnerIteratorPresentationContext),
    ToVecPtr,
    Obj,
    ObjFieldName(TokenStream2),
    FieldTypeConversionName(FieldTypeConversion),
    LineTermination,
    Boxed(Box<FieldTypePresentableContext>),
    UnboxAny(Box<FieldTypePresentableContext>),
    UnboxAnyTerminated(Box<FieldTypePresentableContext>),
    IsNull(Box<FieldTypePresentableContext>),
    DestroyConversion(Box<FieldTypePresentableContext>, TokenStream2),
    FromRawParts(TokenStream2),
    From(Box<FieldTypePresentableContext>),
    FromOffsetMap,
    FromOpt(Box<FieldTypePresentableContext>),
    AsRef(TokenStream2),
    AsMutRef(TokenStream2),
    IfThenSome(Box<FieldTypePresentableContext>, TokenStream2),
    Named((TokenStream2, Box<FieldTypePresentableContext>)),
    Scope,
    Deref(TokenStream2),
    DerefContext(Box<FieldTypePresentableContext>),
    FfiRefWithFieldName(Box<FieldTypePresentableContext>),
    Match(Box<FieldTypePresentableContext>),
}

impl ScopeContextPresentable for FieldTypePresentableContext {
    type Presentation = TokenStream2;

    fn present(&self, source: &ScopeContext) -> TokenStream2 {
        match self {
            FieldTypePresentableContext::Simple(field_path) =>
                field_path.to_token_stream(),
            FieldTypePresentableContext::To(presentation_context) => {
                let field_path = presentation_context.present(source);
                ffi_to_conversion(field_path)
            },
            FieldTypePresentableContext::Add(presentation_context, index) => {
                let field_path = presentation_context.present(source);
                quote!(#field_path.add(#index))
            },
            FieldTypePresentableContext::UnwrapOr(presentation_context, default) => {
                let field_path = presentation_context.present(source);
                quote!(#field_path.unwrap_or(#default))
            },
            FieldTypePresentableContext::ToVec(presentation_context) => {
                presentation_context.present(source)
            }
            FieldTypePresentableContext::ToOpt(presentation_context) => {
                let package = package();
                let interface = interface();
                let field_path = presentation_context.present(source);
                quote!(#package::#interface::ffi_to_opt(#field_path))
            },
            FieldTypePresentableContext::ToVecPtr => {
                let expr = package_boxed_expression(quote!(o));
                let package = package();
                quote!(#package::boxed_vec(obj.map(|o| #expr).collect()))
            },
            FieldTypePresentableContext::LineTermination => quote!(;),
            FieldTypePresentableContext::Empty => quote!(),
            FieldTypePresentableContext::Boxed(presentation_context) => {
                package_boxed_expression(presentation_context.present(source))
            },
            FieldTypePresentableContext::UnboxAny(presentation_context) => {
                package_unbox_any_expression(presentation_context.present(source))
            }
            FieldTypePresentableContext::UnboxAnyTerminated(presentation_context) => {
                let field_path = presentation_context.present(source);
                package_unbox_any_expression_terminated(field_path)
            },
            FieldTypePresentableContext::IsNull(presentation_context) => {
                let field_path = presentation_context.present(source);
                let conversion = package_unbox_any_expression_terminated(field_path.clone());
                quote!(if !#field_path.is_null() { #conversion })
            },
            FieldTypePresentableContext::DestroyConversion(presentation_context, path) => {
                let package = package();
                let interface = interface();
                let field_path = presentation_context.present(source);
                quote!(<std::os::raw::c_char as #package::#interface<#path>>::destroy(#field_path))
            },

            FieldTypePresentableContext::FromRawParts(field_type) => {
                quote!(std::slice::from_raw_parts(values as *const #field_type, count).to_vec())
            },
            FieldTypePresentableContext::From(presentation_context) => {
                let field_path = presentation_context.present(source);
                ffi_from_conversion(field_path)
            },
            FieldTypePresentableContext::FromOpt(presentation_context) => {
                let package = package();
                let interface = interface();
                let field_path = presentation_context.present(source);
                quote!(#package::#interface::ffi_from_opt(#field_path))
            },
            FieldTypePresentableContext::FromOffsetMap => {
                let ffi_from_conversion =
                    ffi_from_conversion(quote!(*values.add(i)));
                quote!((0..count).map(|i| #ffi_from_conversion).collect())
            },
            FieldTypePresentableContext::AsRef(field_path) => {
                quote!(&#field_path)
            },
            FieldTypePresentableContext::AsMutRef(field_path) => {
                quote!(&mut #field_path)
            },
            FieldTypePresentableContext::IfThenSome(presentation_context, condition) => {
                let field_path = presentation_context.present(source);
                quote!((#field_path #condition).then(|| #field_path))
            }
            FieldTypePresentableContext::Named((l_value, presentation_context)) => {
                let ty = presentation_context.present(source);
                quote!(#l_value: #ty)
            }
            FieldTypePresentableContext::Scope => {
                quote!({})
            }
            FieldTypePresentableContext::FfiRefWithFieldName(presentation_context) => {
                let field_name = presentation_context.present(source);
                quote!(ffi_ref.#field_name)
            }
            FieldTypePresentableContext::Deref(field_name) => {
                quote!(*#field_name)
            }
            FieldTypePresentableContext::DerefContext(presentation_context) => {
                FieldTypePresentableContext::Deref(presentation_context.present(source)).present(source)
            }
            FieldTypePresentableContext::Obj => obj(),
            FieldTypePresentableContext::ObjFieldName(field_name) => {
                quote!(obj.#field_name)
            }
            FieldTypePresentableContext::FieldTypeConversionName(field_type) => {
                field_type.name()
            }
            FieldTypePresentableContext::Match(presentation_context) => {
                let field_path = presentation_context.present(source);
                quote!(match #field_path)
            }
        }
    }
}
