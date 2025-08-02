use quote::{quote, ToTokens};
use syn::__private::TokenStream2;
use syn::Path;
use crate::ast::{Assignment, BraceWrapped, Lambda, ParenWrapped};
use crate::context::ScopeContext;
use crate::ext::{LifetimeProcessor, Mangle, ToPath};
use crate::lang::RustSpecification;
use crate::presentable::{ScopeContextPresentable, SeqKind};
use crate::presentation::{present_struct, DictionaryName, InterfacesMethodExpr};

impl ScopeContextPresentable for SeqKind<RustSpecification> {
    type Presentation = TokenStream2;

    fn present(&self, source: &ScopeContext) -> Self::Presentation {
        let result = match self {
            SeqKind::Empty |
            SeqKind::FromStub(..) |
            SeqKind::ToStub(..) |
            SeqKind::DropStub(..) |
            SeqKind::StubStruct(..) =>
                quote!(),
            SeqKind::FromUnnamedFields(((aspect, ..), fields)) |
            SeqKind::ToUnnamedFields(((aspect, ..), fields)) => {
                let name = aspect.present(source);
                let presentation = fields.present(source);
                quote!(#name ( #presentation ) )
            },
            SeqKind::TraitImplFnCall(self_ty, trait_ty, fn_ident, fields) => {
                let presentation = fields.present(source);
                quote!(<#self_ty as #trait_ty>::#fn_ident(#presentation))
            },
            SeqKind::FromNamedFields(((aspect, ..), fields)) |
            SeqKind::ToNamedFields(((aspect, ..), fields)) => {
                let name = aspect.present(source);
                let cleaned_name = name.lifetimes_cleaned();
                let presentation = fields.present(source);
                quote!(#cleaned_name { #presentation })
            },
            SeqKind::TypeAliasFromConversion((_, fields)) => {
                fields.present(source)
                    .to_token_stream()
            },
            SeqKind::UnnamedVariantFields(((aspect, ..), fields)) => {
                let attrs = aspect.attrs();
                let path: Path = aspect.present(source).to_path();
                let ident = &path.segments.last().unwrap().ident;
                let presentation = ParenWrapped::new(fields.clone()).present(source);
                quote! {
                    #(#attrs)*
                    #ident #presentation
                }
            }
            SeqKind::NamedVariantFields(((aspect, ..), fields)) => {
                let attrs = aspect.attrs();
                let path = aspect.present(source).to_path();
                let ident = &path.segments.last().unwrap().ident;
                let presentation = BraceWrapped::new(fields.clone()).present(source);
                quote! {
                    #(#attrs)*
                    #ident #presentation
                }
            }
            SeqKind::Variants(aspect, attrs, fields) => {
                let name = aspect.present(source).mangle_ident_default();
                let presentation = BraceWrapped::new(fields.clone()).present(source);
                quote! {
                    #(#attrs)*
                    #name #presentation
                }
            },
            SeqKind::UnnamedStruct(((aspect, ..), fields)) => {
                let ffi_type = aspect.present(source);
                let fields = fields.present(source);
                present_struct(
                    &ffi_type.to_path().segments.last().unwrap().ident,
                    aspect.attrs(),
                    quote!((#fields);))
            },
            SeqKind::NamedStruct(((aspect, ..), fields)) => {
                let ffi_type = aspect.present(source);
                let fields = fields.present(source);
                present_struct(
                    &ffi_type.to_path().segments.last().unwrap().ident,
                    aspect.attrs(),
                    quote!({#fields}))
            },
            SeqKind::Enum(context) => {
                let enum_presentation = context.present(source);
                quote! {
                    #[repr(C)]
                    #[derive(Clone)]
                    #[non_exhaustive]
                    pub enum #enum_presentation
                }
            },
            SeqKind::Unit(aspect) => {
                let attrs = aspect.attrs();
                let path = aspect.present(source)
                    .to_path();

                let last_segment = path.segments
                    .last()
                    .expect("Empty path");

                quote! {
                    #(#attrs)*
                    #last_segment
                }
            },
            SeqKind::NoFieldsConversion(aspect) => {
                aspect.present(source)
                    .to_token_stream()
            },
            SeqKind::EnumUnitFields(((aspect, ..), fields)) => {
                Assignment::new(
                    aspect.present(source).to_path().segments.last().unwrap().ident.clone(),
                    fields.present(source))
                    .to_token_stream()
            },
            SeqKind::StructFrom(field_context, conversions) => {
                let conversions = conversions.present(source);
                let field_path = field_context.present(source);
                quote!(let ffi_ref = #field_path; #conversions)
            }
            SeqKind::StructTo(_field_context, conversions) => {
                InterfacesMethodExpr::Boxed(conversions.present(source))
                    .to_token_stream()
            }
            SeqKind::EnumVariantFrom(l_value, r_value) |
            SeqKind::EnumVariantTo(l_value, r_value) |
            SeqKind::EnumVariantDrop(l_value, r_value) => {
                Lambda::new(l_value.present(source), r_value.present(source))
                    .to_token_stream()
            }
            SeqKind::DerefFFI => {
                let field_path = DictionaryName::Ffi;
                quote!(&*#field_path)
            }
            SeqKind::Obj => {
                DictionaryName::Obj.to_token_stream()
            },
            SeqKind::StructDropBody((_, items)) => {
                let destructors = items.present(source);
                quote! {
                    let ffi_ref = self;
                    #destructors
                }
            },
            SeqKind::DropCode((_, items)) => {
                let destructors = items.present(source);
                quote!({ #destructors })
            }
        };
        result
    }
}