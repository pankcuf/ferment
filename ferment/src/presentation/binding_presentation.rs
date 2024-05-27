use proc_macro2::{Ident, TokenStream as TokenStream2};
use quote::{quote, ToTokens};
use syn::{ReturnType, Type};
use syn::punctuated::Punctuated;
use syn::token::RArrow;
use crate::composer::{CommaPunctuated, ConstructorPresentableContext, Depunctuated};
use crate::conversion::{FieldTypeConversion, FieldTypeConversionKind};
use crate::ext::{Accessory, Mangle, Pop, Terminated, ToPath, ToType};
use crate::naming::{DictionaryName, InterfacesMethodExpr, Name};

#[derive(Clone, Debug)]
#[allow(unused)]
pub enum BindingPresentation {
    Empty,
    Constructor {
        context: ConstructorPresentableContext,
        ctor_arguments: CommaPunctuated<TokenStream2>,
        body_presentation: TokenStream2,
    },
    Destructor {
        attrs: TokenStream2,
        name: Name,
        ffi_name: Type,
    },
    Getter {
        attrs: TokenStream2,
        name: Name,
        field_name: TokenStream2,
        obj_type: Type,
        field_type: Type
    },
    Setter {
        attrs: TokenStream2,
        name: Name,
        field_name: TokenStream2,
        obj_type: Type,
        field_type: Type
    },
    ObjAsTrait {
        attrs: TokenStream2,
        name: Name,
        item_type: Type,
        trait_type: TokenStream2,
        vtable_name: Name,
    },
    ObjAsTraitDestructor {
        attrs: TokenStream2,
        name: Name,
        item_type: TokenStream2,
        trait_type: TokenStream2,
    },
    RegularFunction {
        attrs: TokenStream2,
        name: Name,
        is_async: bool,
        arguments: CommaPunctuated<TokenStream2>,
        input_conversions: TokenStream2,
        return_type: ReturnType,
        output_conversions: TokenStream2,
    },
    Callback {
        name: TokenStream2,
        arguments: CommaPunctuated<TokenStream2>,
        output_expression: ReturnType,
    },
    TraitVTableInnerFn {
        name: Name,
        name_and_args: TokenStream2,
        output_expression: ReturnType,
    },
    StaticVTableInnerFnDeclaration {
        name: Name,
        fn_name: Ident
    },
    StaticVTableInnerFn {
        name: Name,
        args: CommaPunctuated<TokenStream2>,
        output: ReturnType,
        body: TokenStream2,
    },
    StaticVTable {
        name: Name,
        methods_declarations: CommaPunctuated<BindingPresentation>,
        methods_implementations: Depunctuated<BindingPresentation>,
        fq_trait_vtable: TokenStream2,
    },
}

fn present_pub_function<T: ToTokens>(
    attrs: TokenStream2,
    name: TokenStream2,
    args: CommaPunctuated<T>,
    output: ReturnType,
    body: TokenStream2) -> TokenStream2 {
    present_function(attrs, quote!(pub), name, args, output, body)
}
pub fn present_function<T: ToTokens>(
    attrs: TokenStream2,
    acc: TokenStream2,
    name: TokenStream2,
    args: CommaPunctuated<T>,
    output: ReturnType,
    body: TokenStream2) -> TokenStream2 {
    quote! {
       #attrs
       #[no_mangle]
       #acc unsafe extern "C" fn #name(#args) #output {
            #body
        }
    }
}

impl ToTokens for BindingPresentation {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        match self {
            Self::Empty =>
                quote!(),
            Self::Constructor { context, ctor_arguments, body_presentation} => {
                match context {
                    ConstructorPresentableContext::EnumVariant(ffi_type, attrs) => {
                        let variant_path = ffi_type.to_path();
                        present_pub_function(
                            attrs.to_token_stream(),
                            Name::Constructor(ffi_type.clone()).mangle_ident_default().to_token_stream(),
                            ctor_arguments.clone(),
                            ReturnType::Type(RArrow::default(), variant_path.popped().to_token_stream().joined_mut().to_type().into()),
                            InterfacesMethodExpr::Boxed(quote!(#variant_path #body_presentation)).to_token_stream())
                    }
                    ConstructorPresentableContext::Default(ffi_type, attrs) => {
                        present_pub_function(
                            attrs.to_token_stream(),
                            Name::Constructor(ffi_type.clone()).mangle_ident_default().to_token_stream(),
                            ctor_arguments.clone(),
                            ReturnType::Type(RArrow::default(), ffi_type.joined_mut().into()),
                            InterfacesMethodExpr::Boxed(quote!(#ffi_type #body_presentation)).to_token_stream())
                    }
                }
            },
            Self::Destructor { name, ffi_name, attrs } => {
                present_pub_function(
                    attrs.to_token_stream(),
                    name.mangle_ident_default().to_token_stream(),
                    Punctuated::from_iter([
                        FieldTypeConversion::Named(Name::Dictionary(DictionaryName::Ffi), FieldTypeConversionKind::Type(ffi_name.joined_mut()), Depunctuated::new())
                    ]),
                    ReturnType::Default,
                    InterfacesMethodExpr::UnboxAny(DictionaryName::Ffi.to_token_stream()).to_token_stream().terminated()
                )
            },
            Self::ObjAsTrait { name, item_type, trait_type, vtable_name, attrs } => {
                let fields = CommaPunctuated::from_iter([
                    FieldTypeConversion::named(Name::Dictionary(DictionaryName::Object), FieldTypeConversionKind::Conversion(quote!(obj as *const ()))),
                    FieldTypeConversion::named(Name::Dictionary(DictionaryName::Vtable), FieldTypeConversionKind::Conversion(quote!(&#vtable_name))),
                ]);
                present_pub_function(
                    attrs.to_token_stream(),
                    name.mangle_ident_default().to_token_stream(),
                    Punctuated::from_iter([
                        FieldTypeConversion::named(Name::Dictionary(DictionaryName::Obj), FieldTypeConversionKind::Type(item_type.joined_const()))
                    ]),
                    ReturnType::Type(RArrow::default(), trait_type.to_type().into()),
                    quote!(#trait_type { #fields })
                )
            },
            BindingPresentation::ObjAsTraitDestructor { name, item_type, trait_type, attrs } => {
                present_pub_function(
                    attrs.to_token_stream(),
                    name.mangle_ident_default().to_token_stream(),
                    Punctuated::from_iter([FieldTypeConversion::named(Name::Dictionary(DictionaryName::Obj), FieldTypeConversionKind::Conversion(trait_type.to_token_stream()))]),
                    ReturnType::Default,
                    InterfacesMethodExpr::UnboxAny(quote!(obj.object as *mut #item_type)).to_token_stream().terminated()
                )
            },
            BindingPresentation::Getter { name, field_name, obj_type, field_type, attrs } => {
                present_pub_function(
                    attrs.to_token_stream(),
                    name.mangle_ident_default().to_token_stream(),
                    Punctuated::from_iter([
                        FieldTypeConversion::named(Name::Dictionary(DictionaryName::Obj), FieldTypeConversionKind::Type(obj_type.joined_const()))]),
                    ReturnType::Type(RArrow::default(), field_type.clone().into()),
                    quote!((*obj).#field_name)
                )
            },
            BindingPresentation::Setter { name, field_name, obj_type, field_type, attrs } => {
                present_pub_function(
                    attrs.to_token_stream(),
                    name.mangle_ident_default().to_token_stream(),
                    CommaPunctuated::from_iter([
                        FieldTypeConversion::named(Name::Dictionary(DictionaryName::Obj), FieldTypeConversionKind::Type(obj_type.joined_mut())),
                        FieldTypeConversion::named(Name::Dictionary(DictionaryName::Value), FieldTypeConversionKind::Type(field_type.clone())),
                    ]),
                    ReturnType::Default,
                    quote!((*obj).#field_name = value;))
            },
            BindingPresentation::RegularFunction { attrs, is_async, name, arguments, input_conversions, return_type, output_conversions } => {
                if *is_async {
                    let mut args = Punctuated::from_iter([quote!(runtime: *mut std::os::raw::c_void)]);
                    args.extend(arguments.clone());
                    present_pub_function(
                        attrs.to_token_stream(),
                        name.mangle_ident_default().to_token_stream(),
                        args,
                        return_type.clone(),
                        quote! {
                            let rt = unsafe { &*(runtime as *mut tokio::runtime::Runtime) };
                            let obj = rt.block_on(async { #input_conversions .await });
                            #output_conversions
                        }
                    )
                } else {
                    present_pub_function(
                        attrs.to_token_stream(),
                        name.mangle_ident_default().to_token_stream(),
                        arguments.clone(),
                        return_type.clone(),
                        quote!(let obj = #input_conversions; #output_conversions)
                    )
                }
            },
            BindingPresentation::Callback { name, arguments, output_expression: return_type } =>
                quote!(pub type #name = unsafe extern "C" fn(#arguments) #return_type;),
            BindingPresentation::StaticVTable { name, fq_trait_vtable, methods_declarations, methods_implementations } => {
                quote! {
                    static #name: #fq_trait_vtable = {
                        #methods_implementations
                        #fq_trait_vtable {
                            #methods_declarations
                        }
                    };
                }
            },
            BindingPresentation::TraitVTableInnerFn { name, name_and_args, output_expression } => {
                quote!(pub #name: #name_and_args #output_expression)
            }
            BindingPresentation::StaticVTableInnerFn { name, args, output, body } => {
                present_function(
                    quote!(),
                    quote!(),
                    name.to_token_stream(),
                    args.clone(),
                    output.clone(),
                    body.clone()
                )
            },
            BindingPresentation::StaticVTableInnerFnDeclaration { name, fn_name } =>
                quote!(#fn_name: #name),

        }.to_tokens(tokens)
     }
}
