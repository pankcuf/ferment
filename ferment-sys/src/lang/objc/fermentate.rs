use std::fmt::{Display, Formatter, Write};
use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::__private::TokenStream2;
use crate::ast::{CommaPunctuatedTokens, Depunctuated, SemiPunctuated, SemiPunctuatedTokens};
use crate::composer::SourceFermentable;
use crate::lang::LangFermentable;
use crate::lang::objc::formatter::format_properties;
use crate::lang::objc::ObjCFermentate;
use crate::tree::{CrateTree, ScopeTree};
use super::presentation::Property;

// #[derive(Clone, Debug)]
// pub struct InterfaceImplementation {
//     pub interface: InterfacePresentation,
//     pub implementation: ImplementationPresentation,
// }

#[derive(Clone, Debug)]
pub enum InterfaceImplementation {
    Default {
        objc_name: TokenStream2,
        properties: SemiPunctuated<Property>,
    },
    BindingsDeclaration {

        objc_name: TokenStream2,
        c_name: TokenStream2,

    },
    BindingsImplementation {
        objc_name: TokenStream2,
        c_name: TokenStream2,
        to_conversions: SemiPunctuatedTokens,
        property_names: CommaPunctuatedTokens


    },
    ConversionsDeclaration {
        objc_name: TokenStream2,
        c_name: TokenStream2,


    },

    ConversionsImplementation {
        objc_name: TokenStream2,
        c_name: TokenStream2,
        from_conversions_statements: SemiPunctuated<Property>,
        to_conversions_statements: SemiPunctuatedTokens,
        destroy_conversions_statements: SemiPunctuatedTokens,

    },
}


// impl InterfaceImplementation {
    // pub fn default(
    //     objc_name: TokenStream2,
    //     // c_name: TokenStream2,
    //     properties: SemiPunctuated<Property>,
    //     // properties_inits: SemiPunctuated<Property>
    // ) -> Self {
    //     Self::Default {
    //         objc_name,
    //         properties,
    //     }
    //     // Self {
    //     //     interface: InterfacePresentation::Default {
    //     //         name: objc_name.clone(),
    //     //         c_type: c_name.clone(),
    //     //         properties,
    //     //     },
    //     //     implementation: ImplementationPresentation::Default {
    //     //         objc_name: objc_name.clone(),
    //     //         c_type: c_name.clone(),
    //     //         properties_inits,
    //     //     },
    //     // }
    // }
    // pub fn c(
    //     objc_name: TokenStream2,
    //     c_name: TokenStream2,
    //     property_ctors: SemiPunctuatedTokens,
    //     property_dtors: SemiPunctuatedTokens
    // ) -> Self {
    //     Self {
    //         interface: InterfacePresentation::C {
    //             name: objc_name.clone(),
    //             c_type: c_name.clone()
    //         },
    //         implementation: ImplementationPresentation::C {
    //             objc_name: objc_name.clone(),
    //             c_type: c_name.clone(),
    //             property_ctors,
    //             property_dtors
    //         },
    //     }
    // }
    // pub fn rust(
    //     objc_name: TokenStream2,
    //     c_name: TokenStream2,
    //     property_names: CommaPunctuatedTokens,
    //     property_ctors: SemiPunctuatedTokens
    // ) -> Self {
    //     Self {
    //         interface: InterfacePresentation::Rust {
    //             name: objc_name.clone(),
    //             c_type: c_name.clone()
    //         },
    //         implementation: ImplementationPresentation::Rust {
    //             objc_name: objc_name.clone(),
    //             c_type: c_name.clone(),
    //             c_var: quote!(struct #c_name *),
    //             property_names,
    //             property_ctors,
    //         },
    //     }
    // }
    // pub fn args(
    //     objc_name: TokenStream2,
    //     c_name: TokenStream2,
    //     args: Depunctuated<ArgPresentation>,
    //     prop_implementations: Depunctuated<TokenStream2>
    // ) -> Self {
    //     InterfaceImplementation {
    //         interface: InterfacePresentation::Args {
    //             name: objc_name.clone(),
    //             c_type: c_name.clone(),
    //             args,
    //         },
    //         implementation: ImplementationPresentation::Args {
    //             objc_name: objc_name.clone(),
    //             prop_implementations,
    //         },
    //     }
    // }
// }

impl Display for InterfaceImplementation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            InterfaceImplementation::Default { objc_name, properties } => {
                f.write_str(format!("@interface {objc_name} : NSObject\n").as_str())?;
                for property in properties {
                    f.write_str(format!("{};\n", property.to_token_stream().to_string()).as_str())?;
                }

                // f.write_str(format!("{}", format_properties(properties)).as_str())?;
                f.write_str("@end")
            }
            InterfaceImplementation::BindingsDeclaration { objc_name, c_name } => {
                f.write_str(format!("@interface {objc_name} (Bindings)\n").as_str())?;
                f.write_str(format!("+ (struct {c_name} *)ffi_ctor:(instancetype)obj;\n").as_str())?;
                f.write_str(format!("+ (void)ffi_dtor:(struct {c_name} *)ffi_ref;\n").as_str())?;
                f.write_str("@end")
            }
            InterfaceImplementation::BindingsImplementation { objc_name, c_name, to_conversions, property_names: properties } => {
                f.write_str(format!("@implementation {objc_name} (Bindings)\n").as_str())?;
                f.write_str(format!("+ (struct {c_name} *)ffi_ctor:(instancetype)obj {{\n").as_str())?;
                for to_conversion in to_conversions {
                    f.write_str(format!("\t{};\n", to_conversion).as_str())?;
                }
                f.write_str(format!("\treturn {c_name}_ctor({});\n", properties.to_token_stream().to_string()).as_str())?;
                f.write_str("}\n")?;
                f.write_str(format!("+ (void)ffi_dtor:(struct {c_name} *)ffi_ref {{\n").as_str())?;
                f.write_str(format!("\t{c_name}_destroy(ffi_ref);\n").as_str())?;
                f.write_str("}\n")?;
                f.write_str("@end")
            }
            InterfaceImplementation::ConversionsDeclaration { objc_name, c_name } => {
                f.write_str(format!("@interface {objc_name} (Conversions)\n").as_str())?;
                f.write_str(format!("+ (instancetype)ffi_from:(struct {c_name} *)ffi_ref;\n").as_str())?;
                f.write_str(format!("+ (instancetype _Nullable)ffi_from_opt:(struct {c_name} *)ffi_ref;\n").as_str())?;
                f.write_str(format!("+ (struct {c_name} *)ffi_to:(instancetype)obj;\n").as_str())?;
                f.write_str(format!("+ (void)ffi_destroy:(struct {c_name} *)ffi_ref;\n").as_str())?;
                f.write_str(format!("+ (struct {c_name} *)ffi_to_opt:(instancetype _Nullable)obj;\n").as_str())?;
                f.write_str("@end")
            }
            InterfaceImplementation::ConversionsImplementation { objc_name, c_name, from_conversions_statements, to_conversions_statements, destroy_conversions_statements } => {
                f.write_str(format!("@implementation {objc_name} (Conversions)\n").as_str())?;
                f.write_str(format!("+ (instancetype)ffi_from:(struct {c_name} *)ffi_ref {{\n").as_str())?;
                f.write_str(format!("\t{objc_name} *obj = [[self alloc] init];\n").as_str())?;
                f.write_str("\tif (obj) {\n")?;
                for statement in from_conversions_statements {
                    f.write_str(format!("\t\t{};\n", statement).as_str())?;
                }
                f.write_str("\t}\n")?;
                f.write_str("\treturn obj;\n")?;
                f.write_str("}\n")?;
                f.write_str(format!("+ (instancetype _Nullable)ffi_from_opt:(struct {c_name} *)ffi_ref {{\n").as_str())?;
                f.write_str("\treturn ffi_ref ? [self ffi_from:ffi_ref] : nil;\n")?;
                f.write_str("}\n")?;
                f.write_str(format!("+ (struct {c_name} *)ffi_to:(instancetype)obj {{\n").as_str())?;
                f.write_str(format!("\t{objc_name} *self_ = malloc(sizeof({c_name}));\n").as_str())?;
                for statement in to_conversions_statements {
                    f.write_str(format!("\t{};\n", statement).as_str())?;
                }
                f.write_str("\treturn self_;\n")?;
                f.write_str("}\n")?;
                f.write_str(format!("+ (struct {c_name} *)ffi_to_opt:(instancetype _Nullable)obj {{\n").as_str())?;
                f.write_str("\treturn obj ? [self ffi_to:obj] : nil;\n")?;
                f.write_str("}\n")?;
                f.write_str(format!("+ (void)ffi_destroy:({c_name} *)ffi_ref {{\n").as_str())?;
                f.write_str("\tif (!ffi_ref) return;\n")?;
                for statement in destroy_conversions_statements {
                    f.write_str(format!("\t{};\n", statement).as_str())?;
                }
                f.write_str("\tfree(ffi_ref);\n")?;
                f.write_str("}\n")?;
                f.write_str("@end")

            }
        }
    }
}

impl ToTokens for InterfaceImplementation {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        match self {
            InterfaceImplementation::Default { objc_name, properties } => {
                // @interface DSdash_spv_masternode_processor_crypto_byte_util_UInt768 : NSObject
                // @property (nonatomic, readwrite) DSArr_u8_96 *o_0;
                // @end

                quote! {
                    @interface #objc_name: NSObject
                    #properties
                    @end
                }
            }
            InterfaceImplementation::BindingsDeclaration { objc_name, c_name } => {
                // @interface DSdash_spv_masternode_processor_crypto_byte_util_UInt768 (Bindings)
                // + (struct dash_spv_masternode_processor_crypto_byte_util_UInt768 *)ffi_ctor:(instancetype)obj;
                // + (void)ffi_dtor:(struct dash_spv_masternode_processor_crypto_byte_util_UInt768 *)ffi_ref;
                // @end
                quote! {
                    @interface #objc_name (Bindings)
                    + (struct #c_name *)ffi_ctor:(instancetype)obj;
                    + (void)ffi_dtor:(struct #c_name *)ffi_ref;
                    @end
                }
            }
            InterfaceImplementation::BindingsImplementation { objc_name, c_name, to_conversions, property_names: properties } => {
                let ctor_name = format_ident!("{}_ctor", c_name.to_string());
                let dtor_name = format_ident!("{}_destroy", c_name.to_string());
            // + (struct dash_spv_masternode_processor_crypto_byte_util_UInt768 *)ffi_ctor:(instancetype)obj {
            //     Arr_u8_96 *o_0 = [DSArr_u8_96 ffi_to:obj.o_0];
            //     return dash_spv_masternode_processor_crypto_byte_util_UInt768_ctor(o_0);
            // }

            quote! {
                    @implementation #objc_name (Bindings)
                    + (struct #c_name *)ffi_ctor:(instancetype)obj {
                        #to_conversions;
                        return #ctor_name(#properties);
                        //return dash_spv_masternode_processor_crypto_byte_util_UInt768_ctor([DSArr_u8_96 ffi_to:obj.o_0]);
                    }
                    + (void)ffi_dtor:(struct #c_name *)ffi_ref {
                        #dtor_name(ffi_ref);
                    }
                    @end
                }
            }
            InterfaceImplementation::ConversionsDeclaration { objc_name, c_name } => {
                quote! {
                    @interface #objc_name (Conversions)
                    + (instancetype)ffi_from:(struct #c_name *)ffi_ref;
                    + (instancetype _Nullable)ffi_from_opt:(struct #c_name *)ffi_ref;
                    + (struct #c_name *)ffi_to:(instancetype)obj;
                    + (void)ffi_destroy:(struct #c_name *)ffi_ref;
                    + (struct #c_name *)ffi_to_opt:(instancetype _Nullable)obj;
                @end

                }
            }
            InterfaceImplementation::ConversionsImplementation { objc_name, c_name, from_conversions_statements: from_conversions, to_conversions_statements: to_conversions, destroy_conversions_statements: destroy_conversions } => {

                quote! {
                    @implementation #objc_name (Conversions)

                    + (instancetype)ffi_from:(struct #c_name *)ffi_ref {
                        id *obj = [[self alloc] init];
                        if (obj) {
                            #from_conversions;
                            // obj.o_0 = [DSArr_u8_96 ffi_from:ffi_ref->o_0];
                        }
                        return obj;
                    }
                    + (instancetype _Nullable)ffi_from_opt:(struct #c_name *)ffi_ref {
                        return ffi_ref ? [self ffi_from:ffi_ref] : nil;
                    }

                    + (struct #c_name *)ffi_to:(instancetype)obj {
                        #objc_name *self_ = malloc(sizeof(#c_name));
                        #to_conversions;
                        //self_->o_0 = [DSArr_u8_96 ffi_to:obj.o_0];
                        return self_;
                    }
                    + (struct #c_name *)ffi_to_opt:(instancetype _Nullable)obj {
                        return obj ? [self ffi_to:obj] : nil;
                    }

                    + (void)ffi_destroy:(#c_name *)ffi_ref {
                        if (!ffi_ref) return;
                        #destroy_conversions;
                        // [DSArr_u8_96 ffi_destroy:ffi_ref->o_0];
                        free(ffi_ref);
                    }
                    @end

                }
            }
        }.to_tokens(tokens)
        // self.interface.to_tokens(tokens);
        // self.implementation.to_tokens(tokens);
    }
}

// impl Display for InterfaceImplementation {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         f.write_str(self.to_token_stream().to_string().as_str())
//
//         // f.write_str(self.interface.to_token_stream().to_string().as_str())?;
//         // f.write_str(self.implementation.to_token_stream().to_string().as_str())
//     }
// }


#[derive(Clone, Debug)]
pub enum Fermentate {
    Empty,
    TokenStream(TokenStream2),
    Item {
        header_name: String,
        imports: Depunctuated<TokenStream2>,
        implementations: Depunctuated<InterfaceImplementation>
    },
    ScopeTree(ScopeTree),
    CrateTree(CrateTree),
}
impl LangFermentable for Fermentate {}

impl Default for Fermentate {
    fn default() -> Self {
        Self::Empty
    }
}


impl Display for Fermentate {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => {
                f.write_str("\n")
            },
            Self::Item { header_name, imports, implementations } => {
                f.write_str("#import <Foundation/Foundation.h>\n")?;
                f.write_str(format!("#import \"{}.h\"\n", header_name).as_str())?;
                for import in imports {
                    f.write_str(format!("#import \"{}.h\"\n", import).as_str())?;
                }
                f.write_str("NS_ASSUME_NONNULL_BEGIN\n")?;
                for i in implementations {
                    f.write_str(i.to_string().as_str())?
                }
                f.write_str("\nNS_ASSUME_NONNULL_END\n")
            }
            Self::TokenStream(token_stream) =>
                f.write_str(token_stream.to_string().as_str()),
            Self::ScopeTree(tree) =>
                f.write_str(<ScopeTree as SourceFermentable<ObjCFermentate>>::ferment(tree).to_token_stream().to_string().as_str()),
            Self::CrateTree(tree) =>
                f.write_str(<CrateTree as SourceFermentable<ObjCFermentate>>::ferment(tree).to_token_stream().to_string().as_str()),

        }
    }
}
impl Fermentate {
    pub fn objc_files(&self) -> Vec<String> {
        vec!["objc_wrapper.m".to_string(), "objc_wrapper.h".to_string()]
    }
}

impl ToTokens for Fermentate {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Fermentate::Item {
                header_name: _,
                imports: _,
                implementations: _ } => {
                // quote! {
                //
                // }
            }
            Fermentate::Empty => {},
            Fermentate::TokenStream(token_stream) =>
                token_stream
                    .to_tokens(tokens),
            Fermentate::ScopeTree(tree) =>
                <ScopeTree as SourceFermentable<ObjCFermentate>>::ferment(tree)
                    .to_tokens(tokens),
            Fermentate::CrateTree(tree) =>
                <CrateTree as SourceFermentable<ObjCFermentate>>::ferment(tree)
                    .to_tokens(tokens),
        }
    }
}

// impl From<super::composers::ItemComposer> for Fermentate {
//     fn from(value: super::composers::ItemComposer) -> Self {
//         todo!()
//     }
// }