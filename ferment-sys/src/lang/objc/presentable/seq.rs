use proc_macro2::Ident;
use quote::{quote, ToTokens};
use syn::__private::TokenStream2;
use syn::Path;
use crate::ast::{Assignment, BraceWrapped, ParenWrapped};
use crate::context::ScopeContext;
use crate::ext::{Mangle, Terminated, ToPath};
use crate::lang::objc::{ObjCFermentate, ObjCSpecification};
use crate::lang::objc::composers::AttrWrapper;
use crate::presentable::{ScopeContextPresentable, SeqKind};
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

impl<SPEC> ScopeContextPresentable for SeqKind<ObjCFermentate, SPEC>
    where SPEC: ObjCSpecification {
    type Presentation = TokenStream2;

    fn present(&self, source: &ScopeContext) -> Self::Presentation {
        let result = match self {
            SeqKind::Empty |
            SeqKind::FromStub(..) |
            SeqKind::ToStub(..) |
            SeqKind::DropStub(..) |
            SeqKind::StubStruct(..) =>
                quote!(),
            SeqKind::StructFrom(field_context, conversions) => {
                let conversions = conversions.present(source);
                let field_path = field_context.present(source);



                println!("OBJC SEQ StructFrom ({}, {})", field_path, conversions);
                quote! {
                    #conversions
                }
            }
            SeqKind::StructTo(field_context, conversions) => {
                let c_type = field_context.present(source);
                let conversions = conversions.present(source);
                println!("OBJC SEQ StructTo ({}, {})", c_type, conversions);
                quote! {
                    #conversions
                }
            }
            SeqKind::FromUnnamedFields(((aspect, _attrs, _generics, _name_kind), fields)) => {
                let presentation = aspect.allocate(ParenWrapped::new(fields.clone()), source);
                // presentation
                let name = aspect.present(source).mangle_ident_default();
                println!("OBJC SEQ FromUnnamedFields: {}", presentation);
                println!("OBJC SEQ FromUnnamedFields2: {}", quote!(case #name));
                quote! {
                    case #name
                }
            }
            SeqKind::ToUnnamedFields(((aspect, _attrs, _generics, _name_kind), fields)) => {
                let presentation = aspect.allocate(ParenWrapped::new(fields.clone()), source);
                // presentation
                let name = aspect.present(source).mangle_ident_default();
                println!("OBJC SEQ ToUnnamedFields: {}", presentation);
                println!("OBJC SEQ ToUnnamedFields2: {}", quote!(case #name));
                quote! {
                    case #name
                }
            },
            SeqKind::FromNamedFields(((aspect, _attrs, _generics, _name_kind), fields)) => {
                let presentation = aspect.allocate(BraceWrapped::new(fields.clone()), source);
                println!("OBJC SEQ NamedFields: {}", presentation);
                presentation
            },
            SeqKind::ToNamedFields(((aspect, _attrs, _generics, _name_kind), fields)) => {
                let presentation = aspect.allocate(BraceWrapped::new(fields.clone()), source);
                println!("OBJC SEQ NamedFields: {}", presentation);
                presentation
            },
            SeqKind::UnnamedVariantFields(((aspect, _attrs, _generics, _is_round), fields)) => {
                //println!("SequenceOutput::{}({}, {:?})", self, aspect, fields);
                let attrs = aspect.attrs();
                let path: Path = aspect.present(source).to_path();
                let ident = &path.segments.last().unwrap().ident;
                let presentation = ParenWrapped::new(fields.clone()).present(source);
                println!("OBJC SEQ UnnamedVariantFields ({}, {})", ident, presentation);
                SPEC::Attr::from(attrs)
                    .wrap(quote!(#ident #presentation))
            }
            SeqKind::NamedVariantFields(((aspect, _attrs, _generics, _is_round), fields)) => {
                //println!("SequenceOutput::{}({}, {:?})", self, aspect, fields);
                let attrs = aspect.attrs();
                let path = aspect.present(source).to_path();
                let ident = &path.segments.last().unwrap().ident;
                let presentation = BraceWrapped::new(fields.clone()).present(source);
                println!("OBJC SEQ NamedVariantFields ({}, {})", ident, presentation);
                SPEC::Attr::from(attrs)
                    .wrap(quote!(#ident #presentation))
            }
            SeqKind::Variants(aspect, attrs, fields) => {
                let name = aspect.present(source).mangle_ident_default();
                let presentation = BraceWrapped::new(fields.clone()).present(source);
                println!("OBJC SEQ Variants ({}, {})", name, presentation.to_token_stream());
                SPEC::Attr::wrap(attrs, quote!(#name #presentation))
            },
            SeqKind::UnnamedStruct(((aspect, _attrs, _generics, _is_round), fields)) => {
                let ffi_type = aspect.present(source);
                let presentation = ParenWrapped::new(fields.clone()).present(source);
                println!("OBJC SEQ UnnamedStruct ({}, {})", ffi_type.to_token_stream(), presentation);
                present_struct(
                    &ffi_type.to_path().segments.last().unwrap().ident,
                    SPEC::Attr::from(aspect.attrs()),
                    presentation.terminated())
            },
            SeqKind::NamedStruct(((aspect, _attrs, _generics, _is_round), fields)) => {
                let ffi_type = aspect.present(source);
                let presentation = BraceWrapped::new(fields.clone()).present(source);
                println!("OBJC SEQ NamedStruct ({}, {:?})", ffi_type.to_token_stream(), presentation);
                present_struct(
                    &ffi_type.to_path().segments.last().unwrap().ident,
                    SPEC::Attr::from(aspect.attrs()),
                    presentation)
            },
            SeqKind::Enum(context) => {
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
            SeqKind::TypeAliasFromConversion((_, fields)) => {
                let fields = fields.present(source);
                println!("OBJC SEQ TypeAliasFromConversion ({})", fields.to_token_stream());
                fields.to_token_stream()
            },
            SeqKind::Unit(aspect) => {
                let attrs = aspect.attrs();
                let path = aspect.present(source)
                    .to_path();
                println!("OBJC SEQ Unit ({})", path.to_token_stream());

                let last_segment = path.segments
                    .last()
                    .expect("Empty path");
                SPEC::Attr::from(attrs)
                    .wrap(last_segment)
                    .to_token_stream()
            },
            SeqKind::NoFieldsConversion(aspect) => {
                let presentation = aspect.present(source);
                println!("OBJC SEQ NoFieldsConversion ({})", presentation.to_token_stream());
                presentation
                    .to_token_stream()
            },
            SeqKind::EnumUnitFields(((aspect, _attrs, _generics, _is_round), fields)) => {
                let aspect = aspect.present(source).to_path();
                let ffi_name = &aspect.segments.last().unwrap().ident;
                let fields = fields.present(source);
                println!("OBJC SEQ EnumUnitFields ({}, {})", ffi_name, fields.to_token_stream());
                Assignment::new(ffi_name.clone(), fields)
                    .to_token_stream()
            },
            // SeqKind::Boxed(conversions) => {
            //     let conversions = conversions.present(source);
            //     println!("OBJC SEQ Boxed ({})", conversions);
            //     conversions
            // }
            SeqKind::EnumVariantFrom(l_value, r_value) => {
                println!("OBJC SEQ EnumVariantFrom ({}, {})", l_value, r_value);
                let left = l_value.present(source);
                let right = r_value.present(source);
                println!("OBJC SEQ EnumVariantFrom2 ({}, {})", left, right);
                quote! {
                    #left: {
                        #right
                    }
                }
            },
            SeqKind::EnumVariantTo(l_value, r_value) => {
                let left = l_value.present(source);
                let right = r_value.present(source);
                println!("OBJC SEQ EnumVariantTo ({}, {})", left, right);
                quote! {
                    #left: {
                        #right
                    }
                }
            }
            SeqKind::EnumVariantDrop(l_value, r_value) => {
                let left = l_value.present(source);
                let right = r_value.present(source);
                println!("OBJC SEQ EnumVariantDrop ({}, {})", left, right);
                quote! {
                    #left: {
                        #right
                    }
                }
            }
            SeqKind::DerefFFI => {
                println!("OBJC SEQ DerefFFI ({})", DictionaryName::Self_.to_token_stream());
                DictionaryName::Self_.to_token_stream()
            }
            SeqKind::Obj => {
                println!("OBJC SEQ Obj ({})", DictionaryName::Obj.to_token_stream());
                DictionaryName::Obj.to_token_stream()
            },
            // SeqKind::UnboxedRoot => {
            //     let expr = Expression::<ObjCFermentate, SPEC>::destroy_complex_tokens(DictionaryName::Ffi);
            //     let presentation = expr.present(source);
            //     println!("OBJC UnboxedRoot ({})", presentation);
            //     presentation
            // },
            SeqKind::StructDropBody(((tyc, _attrs, _generics, _is_round), items)) => {
                let aspect = tyc.present(source);
                let field_dtors = items.present(source);
                println!("OBJC StructDropBody: {} {}", aspect.to_token_stream(), field_dtors.to_token_stream());
                let arg_name = DictionaryName::FfiRef;
                quote! {
                    if (!#arg_name) return;
                    #field_dtors;
                    free(#arg_name);
                }
            },
            SeqKind::DropCode((_aspect, items)) => {
                let presentation = BraceWrapped::new(items.clone())
                    .present(source);
                println!("OBJC DropCode: {}", presentation);
                presentation
            }
        };
        // println!("SequenceOutput::{}({})", self, result);
        result
    }
}
