use quote::{format_ident, quote, ToTokens};
use syn::__private::TokenStream2;
use crate::ast::{CommaPunctuatedTokens, Depunctuated, SemiPunctuated, SemiPunctuatedTokens};
use crate::presentation::Name;
use super::Property;
use super::super::CategoryKind;

pub struct Implementation {
    pub name: TokenStream2,
    pub category: Option<CategoryKind>,
    pub body: Depunctuated<TokenStream2>
}

impl Implementation {
    pub fn def(name: TokenStream2, body: Depunctuated<TokenStream2>) -> Self {
        Self { name, category: None, body }
    }
    pub fn c_ext(name: TokenStream2, body: Depunctuated<TokenStream2>) -> Self {
        Self { name, category: Some(CategoryKind::C), body }
    }
    pub fn rust_ext(name: TokenStream2, body: Depunctuated<TokenStream2>) -> Self {
        Self { name, category: Some(CategoryKind::Rust), body }
    }
    pub fn args_ext(name: TokenStream2, body: Depunctuated<TokenStream2>) -> Self {
        Self { name, category: Some(CategoryKind::Args), body }
    }
}
impl ToTokens for Implementation {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let Self { name, category, body } = self;
        let category = category.as_ref().map(|c| quote!((#c))).unwrap_or_default();
        let stream = quote! {
            @implementation #name #category
            #body
            @end
        };
        stream.to_tokens(tokens)
    }
}

#[derive(Clone, Debug)]
pub enum ImplementationPresentation {
    Default {
        objc_name: Name,
        c_type: TokenStream2,
        properties_inits: SemiPunctuated<Property>
    },
    C {
        objc_name: Name,
        c_type: TokenStream2,
        property_ctors: SemiPunctuatedTokens,
        property_dtors: SemiPunctuatedTokens,
    },
    Rust {
        objc_name: Name,
        c_type: TokenStream2,
        c_var: TokenStream2,
        property_names: CommaPunctuatedTokens,
        property_ctors: SemiPunctuatedTokens,
    },
    Args {
        objc_name: Name,
        prop_implementations: Depunctuated<TokenStream2>
    },
}

impl ToTokens for ImplementationPresentation {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        match self {
            // @implementation DSArr_u8_32
            // + (instancetype)initWith:(struct Arr_u8_32 *)self_ {
            //     self = [[DSArr_u8_32 alloc] init];
            //     if (self) {
            //         self.values = [DSArr_u8_32 to_values:self_];
            //     }
            //     return self;
            // }
            // @end
            ImplementationPresentation::Default { objc_name, c_type, properties_inits: properties } => {
                Implementation::def(objc_name.to_token_stream(), Depunctuated::from_iter([
                    quote! {
                        + (instancetype)initWith:(#c_type)self_ {
                            self = [[#objc_name alloc] init];
                            if (self) {
                                #properties
                            }
                            return self;
                        }
                    }
                ]))
            },
            // @implementation DSArr_u8_32 (C)
            // - (struct Arr_u8_32 *)c_ctor {
            //     struct Arr_u8_32 *self_ = malloc(sizeof(struct Arr_u8_32));
            //     self_->values = [self from_values];
            //     self_->count = self.values.count;
            //     return self_;
            // }
            // + (void)c_dtor:(struct Arr_u8_32 *)self_ {
            //     if (!self_)
            //         return;
            //     if (self_->count > 0) {
            //         free(self_->values);
            //     }
            //     free(self_);
            // }
            // @end

            ImplementationPresentation::C { objc_name, c_type, property_ctors, property_dtors } => {
                Implementation::c_ext(objc_name.to_token_stream(), Depunctuated::from_iter([
                    quote! {
                        - (#c_type)c_ctor {
                            #c_type self_ = malloc(sizeof(#c_type));
                            #property_ctors
                            return self_;
                        }
                    },
                    quote! {
                        + (void)c_dtor:(#c_type)self_ {
                            if (!self_)
                                return;
                            #property_dtors
                            free(self_);
                        }
                    }
                ]))
            },
            // @implementation DSArr_u8_32 (Rust)
            // - (struct Arr_u8_32 *)rust_ctor {
            //     uint8_t *values = [self from_values];
            //     uintptr_t count = self.values.count;
            //     return Arr_u8_32_ctor(count, values);
            // }
            // + (void)rust_dtor:(struct Arr_u8_32 *)self_ {
            //     Arr_u8_32_destroy(self_);
            // }
            // @end
            ImplementationPresentation::Rust {
                objc_name,
                c_type,
                c_var,
                property_names,
                property_ctors
            } => {
                let ctor = format_ident!("{}_ctor", c_type.to_string());
                let dtor = format_ident!("{}_destroy", c_type.to_string());
                Implementation::rust_ext(
                    objc_name.to_token_stream(),
                    Depunctuated::from_iter([
                        quote! {
                            - (#c_var)rust_ctor {
                                #property_ctors
                                return #ctor(#property_names);
                            }
                        },
                        quote! {
                            + (void)rust_dtor:(#c_var)self_ {
                                #dtor(self_);
                            }
                        }
                    ])
                )
            },
            // @implementation DSArr_u8_32 (Args)
            // + (NSArray<NSNumber *> *)to_values:(struct Arr_u8_32 *)self_ {
            //     uintptr_t count = self_->count;
            //     NSMutableArray<NSNumber *> *values = [NSMutableArray arrayWithCapacity:count];
            //     for (NSUInteger i = 0; i < count; i++) {
            //         [values addObject:[NSNumber numberWithUnsignedInt:self_->values[i]]];
            //     }
            //     return values;
            // }
            // - (uint8_t *)from_values {
            //      uintptr_t count = self.values.count;
            //      uint8_t *values = malloc(count * sizeof(uint8_t));
            //      for (NSUInteger i = 0; i < count; i++) {
            //          values[i] = self.values[i].unsignedIntValue;
            //      }
            //      return values;
            // }
            // @end
            ImplementationPresentation::Args { objc_name, prop_implementations } => {
                Implementation::args_ext(objc_name.to_token_stream(), prop_implementations.clone())
            }
        }.to_tokens(tokens)
    }
}
