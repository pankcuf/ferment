use std::fmt::{Display, Formatter};
use std::ops::Add;
use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::__private::TokenStream2;
use crate::ast::{CommaPunctuatedTokens, Depunctuated, SemiPunctuated};
use crate::composer::SourceFermentable;
use crate::lang::LangFermentable;
use crate::lang::objc::ObjCFermentate;
use crate::lang::objc::presentable::ArgPresentation;
use crate::tree::{CrateTree, ScopeTree};

#[derive(Clone, Debug)]
pub enum InterfaceImplementation {
    Default {
        objc_name: TokenStream2,
        properties: SemiPunctuated<ArgPresentation>,
    },
    BindingsDeclaration {
        objc_name: TokenStream2,
        c_name: TokenStream2,

    },
    BindingsImplementation {
        objc_name: TokenStream2,
        c_name: TokenStream2,
        to_conversions: CommaPunctuatedTokens,
        property_names: CommaPunctuatedTokens
    },
    ConversionsDeclaration {
        objc_name: TokenStream2,
        c_name: TokenStream2,
    },

    ConversionsImplementation {
        objc_name: TokenStream2,
        c_name: TokenStream2,
        from_conversions_statements: TokenStream2,
        to_conversions_statements: TokenStream2,
        destroy_body: TokenStream2,
    },
    MacroCall(TokenStream2)
}

impl Display for InterfaceImplementation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            InterfaceImplementation::Default { objc_name, properties } => {
                f.write_str(format!("@interface {objc_name} : NSObject\n").as_str())?;
                for property in properties {
                    f.write_str(format!("{};\n", property.to_token_stream().to_string()).as_str())?;
                }
                f.write_str("@end")
            }
            InterfaceImplementation::BindingsDeclaration { objc_name, c_name } => {
                f.write_str(format!("@interface {objc_name} (Bindings_{c_name})\n").as_str())?;
                f.write_str(format!("+ (struct {c_name} *)ffi_ctor:({objc_name}*)obj;\n").as_str())?;
                f.write_str(format!("+ (void)ffi_dtor:(struct {c_name} *)ffi_ref;\n").as_str())?;
                f.write_str("@end")
            }
            InterfaceImplementation::BindingsImplementation { objc_name, c_name, to_conversions, property_names: properties } => {
                f.write_str(format!("@implementation {objc_name} (Bindings_{c_name})\n").as_str())?;
                f.write_str(format!("+ (struct {c_name} *)ffi_ctor:({objc_name} *)obj {{\n").as_str())?;
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
                f.write_str(format!("@interface {objc_name} (Conversions_{c_name})\n").as_str())?;
                f.write_str(format!("+ ({objc_name} *)ffi_from:(struct {c_name} *)ffi_ref;\n").as_str())?;
                f.write_str(format!("+ ({objc_name} * _Nullable)ffi_from_opt:(struct {c_name} *)ffi_ref;\n").as_str())?;
                f.write_str(format!("+ (struct {c_name} *)ffi_to:({objc_name} *)obj;\n").as_str())?;
                f.write_str(format!("+ (struct {c_name} *)ffi_to_opt:({objc_name} * _Nullable)obj;\n").as_str())?;
                f.write_str(format!("+ (void)ffi_destroy:(struct {c_name} *)ffi_ref;\n").as_str())?;
                f.write_str("@end")
            }
            InterfaceImplementation::ConversionsImplementation { objc_name, c_name, from_conversions_statements, to_conversions_statements, destroy_body } => {
                f.write_str(format!("@implementation {objc_name} (Conversions_{c_name})\n").as_str())?;
                f.write_str(format!("+ ({objc_name} *)ffi_from:(struct {c_name} *)ffi_ref {{\n").as_str())?;
                if !from_conversions_statements.is_empty() {
                    f.write_str(format!("\t{}\n", from_conversions_statements.to_string()).as_str())?;
                }
                f.write_str("}\n")?;
                f.write_str(format!("+ ({objc_name} * _Nullable)ffi_from_opt:(struct {c_name} *)ffi_ref {{\n").as_str())?;
                f.write_str("\treturn ffi_ref ? [self ffi_from:ffi_ref] : nil;\n")?;
                f.write_str("}\n")?;
                f.write_str(format!("+ (struct {c_name} *)ffi_to:({objc_name} *)obj {{\n").as_str())?;
                if !to_conversions_statements.is_empty() {
                    f.write_str(format!("\t{}\n", to_conversions_statements.to_string()).as_str())?;
                }
                f.write_str("}\n")?;
                f.write_str(format!("+ (struct {c_name} *)ffi_to_opt:({objc_name} * _Nullable)obj {{\n").as_str())?;
                f.write_str("\treturn obj ? [self ffi_to:obj] : nil;\n")?;
                f.write_str("}\n")?;
                f.write_str(format!("+ (void)ffi_destroy:(struct {c_name} *)ffi_ref {{\n").as_str())?;
                f.write_str(format!("\t{}\n", destroy_body.to_string()).as_str())?;
                f.write_str("}\n")?;
                f.write_str("@end")

            }
            InterfaceImplementation::MacroCall(macro_call) =>
                f.write_str(macro_call.to_string().as_str())
        }
    }
}

impl ToTokens for InterfaceImplementation {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        match self {
            InterfaceImplementation::Default { objc_name, properties } => {
                quote! {
                    @interface #objc_name: NSObject
                    #properties;
                    @end
                }
            }
            InterfaceImplementation::BindingsDeclaration { objc_name, c_name } => {
                let interface_name = format_ident!("Bindings_{}", c_name.to_string());
                quote! {
                    @interface #objc_name (#interface_name)
                    + (struct #c_name *)ffi_ctor:(#objc_name *)obj;
                    + (void)ffi_dtor:(struct #c_name *)ffi_ref;
                    @end
                }
            }
            InterfaceImplementation::BindingsImplementation { objc_name, c_name, to_conversions, property_names: _ } => {
                let ctor_name = format_ident!("{}_ctor", c_name.to_string());
                let dtor_name = format_ident!("{}_destroy", c_name.to_string());
                let interface_name = format_ident!("Bindings_{}", c_name.to_string());
                quote! {
                    @implementation #objc_name (#interface_name)
                    + (struct #c_name *)ffi_ctor:(#objc_name *)obj {
                        // #to_conversions;
                        return #ctor_name(#to_conversions);
                    }
                    + (void)ffi_dtor:(struct #c_name *)ffi_ref {
                        #dtor_name(ffi_ref);
                    }
                    @end
                }
            }
            InterfaceImplementation::ConversionsDeclaration { objc_name, c_name } => {
                let interface_name = format_ident!("Conversions_{}", c_name.to_string());
                quote! {
                    @interface #objc_name (#interface_name)
                    + (#objc_name *)ffi_from:(struct #c_name *)ffi_ref;
                    + (#objc_name * _Nullable)ffi_from_opt:(struct #c_name *)ffi_ref;
                    + (struct #c_name *)ffi_to:(#objc_name *)obj;
                    + (struct #c_name *)ffi_to_opt:(#objc_name * _Nullable)obj;
                    + (void)ffi_destroy:(struct #c_name *)ffi_ref;
                @end

                }
            }
            InterfaceImplementation::ConversionsImplementation { objc_name, c_name, from_conversions_statements: from_conversions, to_conversions_statements: to_conversions, destroy_body } => {
                let interface_name = format_ident!("Conversions_{}", c_name.to_string());
                quote! {
                    @implementation #objc_name (#interface_name)
                    + (#objc_name *)ffi_from:(struct #c_name *)ffi_ref {
                        #from_conversions
                    }
                    + (#objc_name * _Nullable)ffi_from_opt:(struct #c_name *)ffi_ref {
                        return ffi_ref ? [self ffi_from:ffi_ref] : nil;
                    }

                    + (struct #c_name *)ffi_to:(#objc_name *)obj {
                        #to_conversions
                    }
                    + (struct #c_name *)ffi_to_opt:(#objc_name * _Nullable)obj {
                        return obj ? [self ffi_to:obj] : nil;
                    }

                    + (void)ffi_destroy:(struct #c_name *)ffi_ref {
                        #destroy_body
                    }
                    @end
                }
            },
            InterfaceImplementation::MacroCall(macro_call) => {
                quote! {
                    #macro_call
                }
            }
        }.to_tokens(tokens)
    }
}


#[derive(Clone, Debug)]
pub enum Fermentate {
    Empty,
    TokenStream(TokenStream2),
    Item {
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

fn add_fermented_string(acc: String, i: &InterfaceImplementation) -> String {
    acc.add(format!("{}\n", i.to_string()).as_str())
}

impl Display for Fermentate {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => {
                f.write_str("\n")
            },
            Self::Item { implementations } => {
                // f.write_str("#import <Foundation/Foundation.h>\n")?;
                // f.write_str(format!("#import \"{}.h\"\n", header_name).as_str())?;
                //
                // for import in imports {
                //     f.write_str(format!("#import \"{}.h\"\n", import).as_str())?;
                // }
                // f.write_str("NS_ASSUME_NONNULL_BEGIN\n")?;
                f.write_str(implementations.iter().fold(String::new(), add_fermented_string).as_str())
                // f.write_str("\nNS_ASSUME_NONNULL_END\n")
            }
            Self::TokenStream(token_stream) =>
                f.write_str(token_stream.to_string().as_str()),
            Self::ScopeTree(tree) =>
                f.write_str(<ScopeTree as SourceFermentable<ObjCFermentate>>::ferment(tree).to_token_stream().to_string().as_str()),
            Self::CrateTree(tree) => {
                f.write_str(<CrateTree as SourceFermentable<ObjCFermentate>>::ferment(tree).to_token_stream().to_string().as_str())
            },

        }
    }
}
impl ToTokens for Fermentate {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Fermentate::Item { implementations } =>
                implementations.to_tokens(tokens),
            Fermentate::TokenStream(token_stream) =>
                token_stream.to_tokens(tokens),
            Fermentate::ScopeTree(tree) =>
                <ScopeTree as SourceFermentable<ObjCFermentate>>::ferment(tree).to_tokens(tokens),
            Fermentate::CrateTree(tree) =>
                <CrateTree as SourceFermentable<ObjCFermentate>>::ferment(tree).to_tokens(tokens),
            _ => {}
        }
    }
}
