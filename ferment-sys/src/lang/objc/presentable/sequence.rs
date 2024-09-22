use quote::{quote, ToTokens};
use syn::__private::TokenStream2;
use syn::Path;
use crate::ast::{Assignment, BraceWrapped, Lambda, ParenWrapped};
use crate::context::ScopeContext;
use crate::ext::{Mangle, Terminated, ToPath};
use crate::lang::objc::{ObjCFermentate, ObjCSpecification};
use crate::presentable::{ScopeContextPresentable, PresentableSequence, Expression};
use crate::presentation::{present_struct, DictionaryName, InterfacesMethodExpr};

impl<SPEC> ScopeContextPresentable for PresentableSequence<ObjCFermentate, SPEC>
    where SPEC: ObjCSpecification {
    type Presentation = TokenStream2;

    fn present(&self, source: &ScopeContext) -> Self::Presentation {
        let result = match self {
            PresentableSequence::Empty =>
                quote!(),
            PresentableSequence::RoundBracesFields((aspect, fields)) => {
                //println!("SequenceOutput::{}({}, {:?})", self, aspect, fields);
                let name = aspect.present(source);
                let presentation = ParenWrapped::new(fields.clone()).present(source);
                quote!(#name #presentation)
            },
            PresentableSequence::CurlyBracesFields((aspect, fields)) => {
                //println!("SequenceOutput::{}({}, {:?})", self, aspect, fields);
                let name = aspect.present(source);
                let presentation = BraceWrapped::new(fields.clone()).present(source);
                quote!(#name #presentation)
            },
            PresentableSequence::RoundVariantFields((aspect, fields)) => {
                //println!("SequenceOutput::{}({}, {:?})", self, aspect, fields);
                let attrs = aspect.attrs();
                let path: Path = aspect.present(source).to_path();
                let ident = &path.segments.last().unwrap().ident;
                let presentation = ParenWrapped::new(fields.clone()).present(source);
                quote! {
                    #(#attrs)*
                    #ident #presentation
                }
            }
            PresentableSequence::CurlyVariantFields((aspect, fields)) => {
                //println!("SequenceOutput::{}({}, {:?})", self, aspect, fields);
                let attrs = aspect.attrs();
                let path = aspect.present(source).to_path();
                let ident = &path.segments.last().unwrap().ident;
                let presentation = BraceWrapped::new(fields.clone()).present(source);

                quote! {
                    #(#attrs)*
                    #ident #presentation
                }
            }
            PresentableSequence::Variants(aspect, attrs, fields) => {
                //println!("SequenceOutput::{}({}, {:?})", self, aspect, fields);
                let name = aspect.present(source).mangle_ident_default();
                let presentation = BraceWrapped::new(fields.clone()).present(source);
                attrs.wrap(quote!(#name #presentation))
                // quote! {
                //     #attrs
                //     #name #presentation
                // }
            },
            PresentableSequence::UnnamedStruct((aspect, fields)) => {
                //println!("SequenceOutput::{}({}, {:?})", self, aspect, fields);
                let ffi_type = aspect.present(source);
                present_struct(
                    &ffi_type.to_path().segments.last().unwrap().ident,
                    aspect.attrs(),
                    ParenWrapped::new(fields.clone()).present(source).terminated())
            },
            PresentableSequence::NamedStruct((aspect, fields)) => {
                //println!("SequenceOutput::{}({}, {:?})", self, aspect, fields);
                let ffi_type = aspect.present(source);
                present_struct(
                    &ffi_type.to_path().segments.last().unwrap().ident,
                    aspect.attrs(),
                    BraceWrapped::new(fields.clone()).present(source))
            },
            PresentableSequence::Enum(context) => {
                //println!("SequenceOutput::{}({:?})", self, context);
                let enum_presentation = context.present(source);
                quote! {
                    #[repr(C)]
                    #[derive(Clone)]
                    #[non_exhaustive]
                    pub enum #enum_presentation
                }
            },
            PresentableSequence::TypeAliasFromConversion((_, fields)) => {
                //println!("SequenceOutput::{}({:?})", self, fields);
                fields.present(source).to_token_stream()
                // Depunctuated::from_iter(fields.clone())
                //     .present(source)
                //     .to_token_stream()
            },
            PresentableSequence::NoFields(aspect) => {
                //println!("SequenceOutput::{}({})", self, aspect);
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
            PresentableSequence::NoFieldsConversion(aspect) => {
                // println!("SequenceOutput::{}({})", self, aspect);
                aspect.present(source)
                    .to_token_stream()
            },
            PresentableSequence::EnumUnitFields((aspect, fields)) => {
                //println!("SequenceOutput::{}({}, {:?})", self, aspect, fields);
                Assignment::new(
                    aspect.present(source).to_path().segments.last().unwrap().ident.clone(),
                    fields.present(source))
                    .to_token_stream()
            },
            PresentableSequence::FromRoot(field_context, conversions) => {
                //println!("SequenceOutput::{}({}, {:?})", self, field_context, conversions);
                // let objc_name =
                let conversions = conversions.present(source);
                let field_path = field_context.present(source);
                quote! {

                        id *obj = [[self alloc] init];
                        if (obj) {
                            #conversions
                        }
                        return obj;

                    // DSResult_ok_String_err_Option_u32 *obj = [[self alloc] init];
                    // if (obj) {
                    //     #conversions
                    //     obj.ok = [DSResult_ok_String_err_Option_u32 to_ok:#field_path];
                    //     obj.error = [DSResult_ok_String_err_Option_u32 to_error:#field_path];
                    // }
                    // return obj;
                }
                //
                // quote!(let ffi_ref = #field_path; #conversions)
            }
            PresentableSequence::Boxed(conversions) => {
                //println!("SequenceOutput::{}({})", self, conversions);
                InterfacesMethodExpr::Boxed(conversions.present(source))
                    .to_token_stream()
            }
            PresentableSequence::Lambda(l_value, r_value) => {
                //println!("SequenceOutput::{}({:?}, {:?})", self, l_value, r_value);
                Lambda::new(l_value.present(source), r_value.present(source))
                    .to_token_stream()
            }
            PresentableSequence::DerefFFI => {
                //println!("SequenceOutput::{}({})", self, field_path);
                let field_path = DictionaryName::Ffi;
                quote!(&*#field_path)
            }
            PresentableSequence::Obj => {
                //println!("SequenceOutput::{}", self);
                DictionaryName::Obj.to_token_stream()
            },
            PresentableSequence::UnboxedRoot => {
                //println!("SequenceOutput::{}", self);
                Expression::<ObjCFermentate, SPEC>::unbox_any(DictionaryName::Ffi)
                    .present(source)
            },
            PresentableSequence::StructDropBody(items) => {
                let arg_name = DictionaryName::Self_;
                let field_dtors = items.present(source);
                quote! {
                    if (!#arg_name) return;
                    #field_dtors
                    free(#arg_name);
                }
            },
            PresentableSequence::DropCode(items) => {
                //println!("SequenceOutput::{}({:?})", self, items);
                BraceWrapped::new(items.clone())
                    .present(source)
            }
        };
        // println!("SequenceOutput::{}({})", self, result);
        result
    }
}
