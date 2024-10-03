use proc_macro2::Ident;
use quote::{quote, ToTokens};
use syn::__private::TokenStream2;
use syn::Path;
use crate::ast::{Assignment, BraceWrapped, Lambda, ParenWrapped};
use crate::context::ScopeContext;
use crate::ext::{Mangle, Terminated, ToPath};
use crate::lang::objc::{ObjCFermentate, ObjCSpecification};
use crate::lang::objc::composers::AttrWrapper;
use crate::presentable::{ScopeContextPresentable, PresentableSequence, Expression};
use crate::presentation::DictionaryName;

/*
@interface DSdash_spv_masternode_processor_crypto_byte_util_UInt768 : NSObject
@property (nonatomic, readwrite) DSArr_u8_96 *o_0;
+ (instancetype)ffi_from:(struct dash_spv_masternode_processor_crypto_byte_util_UInt768 *)obj;
+ (struct dash_spv_masternode_processor_crypto_byte_util_UInt768 *)ffi_to:(instancetype)self_;
+ (void)ffi_destroy:(dash_spv_masternode_processor_crypto_byte_util_UInt768 *)obj;
+ (struct dash_spv_masternode_processor_crypto_byte_util_UInt768 *)ffi_ctor:(instancetype)self_;
+ (void)ffi_dtor:(struct dash_spv_masternode_processor_crypto_byte_util_UInt768 *)obj;
@end

@implementation DSdash_spv_masternode_processor_crypto_byte_util_UInt768
+ (instancetype)ffi_from:(struct dash_spv_masternode_processor_crypto_byte_util_UInt768 *)obj {
    id *self_ = [[self alloc] init];
    self_.o_0 = [DSArr_u8_96 ffi_from:obj->o_0];
    return self_;
}
+ (struct dash_spv_masternode_processor_crypto_byte_util_UInt768 *)ffi_to:(instancetype)self_ {
    dash_spv_masternode_processor_crypto_byte_util_UInt768 *obj = malloc(sizeof(dash_spv_masternode_processor_crypto_byte_util_UInt768));
    obj->o_0 = [DSArr_u8_96 ffi_to:self_.o_0];
    return obj;
}
+ (void)ffi_destroy:(dash_spv_masternode_processor_crypto_byte_util_UInt768 *)obj {
    if (!obj) return;
    [DSArr_u8_96 ffi_destroy:obj->o_0];
    free(obj);
}
+ (struct dash_spv_masternode_processor_crypto_byte_util_UInt768 *)ffi_ctor:(instancetype)self_ {
    return dash_spv_masternode_processor_crypto_byte_util_UInt768_ctor([DSArr_u8_96 ffi_to:self.o_0]);
}
+ (void)ffi_dtor:(struct dash_spv_masternode_processor_crypto_byte_util_UInt768 *)obj {
    dash_spv_masternode_processor_crypto_byte_util_UInt768_destroy(obj);
}
@end
*/

pub fn present_struct<T: ToTokens>(
    ident: &Ident,
    attrs: AttrWrapper,
    implementation: T
) -> TokenStream2 {
    attrs.wrap(quote! {
        @interface #ident : NSObject
        #implementation
        @end
    })
}

impl<SPEC> ScopeContextPresentable for PresentableSequence<ObjCFermentate, SPEC>
    where SPEC: ObjCSpecification {
    type Presentation = TokenStream2;

    fn present(&self, source: &ScopeContext) -> Self::Presentation {
        let result = match self {
            PresentableSequence::Empty =>
                quote!(),
            PresentableSequence::RoundBracesFields(((aspect, _generics), fields)) => {
                //println!("SequenceOutput::{}({}, {:?})", self, aspect, fields);
                let name = aspect.present(source);
                let presentation = ParenWrapped::new(fields.clone()).present(source);
                println!("OBJC SEQ RoundBracesFields ({}, {:?})", name.to_token_stream(), presentation.to_token_stream());
                quote!(#name #presentation)
            },
            PresentableSequence::CurlyBracesFields(((aspect, _generics), fields)) => {
                let name = aspect.present(source);
                let presentation = BraceWrapped::new(fields.clone()).present(source);
                println!("OBJC SEQ CurlyBracesFields ({}, {})", name.to_token_stream(), presentation);
                quote!(#name #presentation)
            },
            PresentableSequence::RoundVariantFields(((aspect, _generics), fields)) => {
                //println!("SequenceOutput::{}({}, {:?})", self, aspect, fields);
                let attrs = aspect.attrs();
                let path: Path = aspect.present(source).to_path();
                let ident = &path.segments.last().unwrap().ident;
                let presentation = ParenWrapped::new(fields.clone()).present(source);
                println!("OBJC SEQ RoundVariantFields ({}, {})", ident, presentation);
                SPEC::Attr::from(attrs)
                    .wrap(quote!(#ident #presentation))
            }
            PresentableSequence::CurlyVariantFields(((aspect, _generics), fields)) => {
                //println!("SequenceOutput::{}({}, {:?})", self, aspect, fields);
                let attrs = aspect.attrs();
                let path = aspect.present(source).to_path();
                let ident = &path.segments.last().unwrap().ident;
                let presentation = BraceWrapped::new(fields.clone()).present(source);
                println!("OBJC SEQ CurlyVariantFields ({}, {})", ident, presentation);
                SPEC::Attr::from(attrs)
                    .wrap(quote!(#ident #presentation))
            }
            PresentableSequence::Variants(aspect, attrs, fields) => {
                println!("OBJC SEQ Variants ({}, {:?})", aspect, fields);
                let name = aspect.present(source).mangle_ident_default();
                let presentation = BraceWrapped::new(fields.clone()).present(source);
                SPEC::Attr::wrap(attrs, quote!(#name #presentation))
            },
            PresentableSequence::UnnamedStruct(((aspect, _generics), fields)) => {
                println!("OBJC SEQ UnnamedStruct ({}, {:?})", aspect, fields);
                let ffi_type = aspect.present(source);
                present_struct(
                    &ffi_type.to_path().segments.last().unwrap().ident,
                    SPEC::Attr::from(aspect.attrs()),
                    ParenWrapped::new(fields.clone()).present(source).terminated())
            },
            PresentableSequence::NamedStruct(((aspect, _generics), fields)) => {
                println!("OBJC SEQ NamedStruct ({}, {:?})", aspect, fields);
                let ffi_type = aspect.present(source);
                present_struct(
                    &ffi_type.to_path().segments.last().unwrap().ident,
                    SPEC::Attr::from(aspect.attrs()),
                    BraceWrapped::new(fields.clone()).present(source))
            },
            PresentableSequence::Enum(context) => {
                println!("OBJC SEQ Enum ({:?})", context);
                quote!()
                //println!("SequenceOutput::{}({:?})", self, context);
                // let enum_presentation = context.present(source);
                // quote! {
                //     #[repr(C)]
                //     #[derive(Clone)]
                //     #[non_exhaustive]
                //     pub enum #enum_presentation
                // }
            },
            PresentableSequence::TypeAliasFromConversion((_, fields)) => {
                let fields = fields.present(source);
                println!("OBJC SEQ TypeAliasFromConversion ({})", fields.to_token_stream());
                fields.to_token_stream()
                // Depunctuated::from_iter(fields.clone())
                //     .present(source)
                //     .to_token_stream()
            },
            PresentableSequence::NoFields(aspect) => {
                println!("OBJC SEQ NoFields ({:?})", aspect);
                let attrs = aspect.attrs();
                let path = aspect.present(source)
                    .to_path();

                let last_segment = path.segments
                    .last()
                    .expect("Empty path");
                SPEC::Attr::from(attrs).wrap(last_segment).to_token_stream()
            },
            PresentableSequence::NoFieldsConversion(aspect) => {
                println!("OBJC SEQ NoFieldsConversion ({:?})", aspect);
                aspect.present(source)
                    .to_token_stream()
            },
            PresentableSequence::EnumUnitFields(((aspect, _generics), fields)) => {
                let aspect = aspect.present(source).to_path();
                let ffi_name = &aspect.segments.last().unwrap().ident;
                let fields = fields.present(source);
                println!("OBJC SEQ EnumUnitFields ({}, {:?})", ffi_name, fields);
                Assignment::new(ffi_name.clone(), fields)
                    .to_token_stream()
            },
            PresentableSequence::FromRoot(field_context, conversions) => {
                let conversions = conversions.present(source);
                let field_path = field_context.present(source);
                println!("OBJC SEQ FromRoot ({}, {})", field_path, conversions);

            // + (instancetype)ffi_from:(struct dash_spv_masternode_processor_crypto_byte_util_UInt768 *)obj {
            //     id *obj = [[self alloc] init];
            //     obj.o_0 = [DSArr_u8_96 ffi_from:obj->o_0];
            //     return obj;
            // }
                quote! {

                    #field_path *obj = [[#field_path alloc] init];
                    #conversions
                    //obj.time = [DSTuple_std_time_Duration_std_time_Duration ffi_from:self_->o_0];
                    return obj;
                }
            }
            PresentableSequence::ToRoot(field_context, conversions) => {
                let c_type = field_context.present(source);
                let conversions = conversions.present(source);
            // + (struct dash_spv_masternode_processor_crypto_byte_util_UInt768 *)ffi_to:(instancetype)self_ {
            //     dash_spv_masternode_processor_crypto_byte_util_UInt768 *obj = malloc(sizeof(dash_spv_masternode_processor_crypto_byte_util_UInt768));
            //     obj->o_0 = [DSArr_u8_96 ffi_to:self_.o_0];
            //     return obj;
            // }
                println!("OBJC SEQ ToRoot ({}, {})", c_type, conversions);

                quote! {
                    #c_type *obj = malloc(sizeof(#c_type))
                    #conversions
                    return obj;
                }
            }
            PresentableSequence::Boxed(conversions) => {
                let conversions = conversions.present(source);
                println!("OBJC SEQ Boxed ({})", conversions);
                conversions
                //println!("SequenceOutput::{}({})", self, conversions);
                // InterfacesMethodExpr::Boxed(conversions.present(source))
                //     .to_token_stream()
            }
            PresentableSequence::Lambda(l_value, r_value) => {
                let left = l_value.present(source);
                let right = r_value.present(source);
                println!("OBJC SEQ Lambda ({}, {})", left, right);
                Lambda::new(left, right)
                    .to_token_stream()
            }
            PresentableSequence::DerefFFI => {
                println!("OBJC SEQ DerefFFI ({})", DictionaryName::Self_.to_token_stream());
                DictionaryName::Self_.to_token_stream()
                // let field_path = DictionaryName::Ffi;
                // quote!(&*#field_path)
            }
            PresentableSequence::Obj => {
                println!("OBJC SEQ Obj ({})", DictionaryName::Obj.to_token_stream());
                //println!("SequenceOutput::{}", self);
                DictionaryName::Obj.to_token_stream()
            },
            PresentableSequence::UnboxedRoot => {
                let expr = Expression::<ObjCFermentate, SPEC>::destroy_complex_tokens(DictionaryName::Ffi);
                let presentation = expr.present(source);
                println!("OBJC UnboxedRoot ({})", presentation);
                presentation
            },
            PresentableSequence::StructDropBody(((tyc, _generics), items)) => {
                println!("OBJC StructDropBody: {} {:?}", tyc.present(source).to_token_stream(), items);
                let arg_name = DictionaryName::Obj;
                let field_dtors = items.present(source);

            // + (void)ffi_destroy:(dash_spv_masternode_processor_crypto_byte_util_UInt768 *)obj {
            //     if (!obj) return;
            //     [DSArr_u8_96 ffi_destroy:obj->o_0];
            //     free(obj);
            // }

                quote! {
                    if (!#arg_name) return;
                    #field_dtors
                    free(#arg_name);
                }
            },
            PresentableSequence::DropCode((_aspect, items)) => {
                println!("OBJC DropCode: {:?}", items);
                BraceWrapped::new(items.clone())
                    .present(source)
            }
        };
        // println!("SequenceOutput::{}({})", self, result);
        result
    }
}
