use proc_macro2::{Ident, TokenStream as TokenStream2};
use quote::{quote, ToTokens};
use syn::{Generics, PathArguments, ReturnType, Type};
use syn::punctuated::Punctuated;
use syn::token::RArrow;
use crate::ast::{CommaPunctuated, CommaPunctuatedTokens, Depunctuated};
use crate::composable::{FieldComposer, FieldTypeConversionKind};
use crate::composer::CommaPunctuatedArgs;
use crate::ext::{Accessory, Mangle, Pop, Terminated, ToPath, ToType};
use crate::presentable::ConstructorPresentableContext;
use crate::presentation::{ArgPresentation, create_callback, DictionaryName, Expansion, InterfacePresentation, InterfacesMethodExpr, Name};

#[derive(Clone, Debug)]
#[allow(unused)]
pub enum BindingPresentation {
    Empty,
    Constructor {
        context: ConstructorPresentableContext,
        ctor_arguments: CommaPunctuatedArgs,
        body_presentation: TokenStream2,
    },
    Destructor {
        attrs: Depunctuated<Expansion>,
        ty: Type,
        generics: Option<Generics>,
    },
    Getter {
        attrs: Depunctuated<Expansion>,
        name: Name,
        field_name: TokenStream2,
        obj_type: Type,
        field_type: Type,
        generics: Option<Generics>,
    },
    Setter {
        attrs: Depunctuated<Expansion>,
        name: Name,
        field_name: TokenStream2,
        obj_type: Type,
        field_type: Type,
        generics: Option<Generics>,
    },
    GetterOpaque {
        attrs: Depunctuated<Expansion>,
        name: Name,
        field_name: TokenStream2,
        obj_type: Type,
        field_type: Type,
        generics: Option<Generics>,
    },
    SetterOpaque {
        attrs: Depunctuated<Expansion>,
        name: Name,
        field_name: TokenStream2,
        obj_type: Type,
        field_type: Type,
        generics: Option<Generics>,
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
        generics: Option<Generics>,
    },
    RegularFunction {
        attrs: Depunctuated<Expansion>,
        name: Name,
        is_async: bool,
        arguments: CommaPunctuatedArgs,
        input_conversions: TokenStream2,
        return_type: ReturnType,
        output_conversions: TokenStream2,
        generics: Option<Generics>,
    },
    // Callback {
    //     name: TokenStream2,
    //     arguments: CommaPunctuated<TokenStream2>,
    //     output_expression: ReturnType,
    // },
    Callback {
        name: Ident,
        attrs: Depunctuated<Expansion>,
        ffi_args: CommaPunctuatedTokens,
        result: ReturnType,
        conversion: InterfacePresentation,
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
        args: CommaPunctuatedArgs,
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
    generics: Option<Generics>,
    body: TokenStream2) -> TokenStream2 {
    present_function(attrs, quote!(pub), name, args, output, generics, body)
}
pub fn present_function<T: ToTokens>(
    attrs: TokenStream2,
    acc: TokenStream2,
    name: TokenStream2,
    args: CommaPunctuated<T>,
    output: ReturnType,
    generics: Option<Generics>,
    body: TokenStream2) -> TokenStream2 {
    match generics {
        None => quote! {
           #attrs
           #[no_mangle]
           #acc unsafe extern "C" fn #name(#args) #output {
                #body
            }
        },
        Some(Generics { params, where_clause, .. }) => quote! {
           #attrs
           #[no_mangle]
           #acc unsafe extern "C" fn #name<#params>(#args) #output #where_clause {
                #body
            }
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
                    ConstructorPresentableContext::EnumVariant((ffi_type, attrs, generics)) => {
                        let variant_path = ffi_type.to_path();
                        present_pub_function(
                            attrs.to_token_stream(),
                            Name::Constructor(ffi_type.clone()).mangle_tokens_default(),
                            ctor_arguments.clone(),
                            ReturnType::Type(RArrow::default(), variant_path.popped().to_token_stream().joined_mut().to_type().into()),
                            generics.clone(),
                            InterfacesMethodExpr::Boxed(quote!(#variant_path #body_presentation)).to_token_stream())
                    }
                    ConstructorPresentableContext::Default((ffi_type, attrs, generics)) => {
                        let mut ffi_path = ffi_type.to_path();
                        ffi_path.segments.last_mut().unwrap().arguments = PathArguments::None;
                        present_pub_function(
                            attrs.to_token_stream(),
                            Name::Constructor(ffi_type.clone()).mangle_tokens_default(),
                            ctor_arguments.clone(),
                            ReturnType::Type(RArrow::default(), ffi_type.joined_mut().into()),
                            generics.clone(),
                            InterfacesMethodExpr::Boxed(quote!(#ffi_path #body_presentation)).to_token_stream())
                    }
                }
            },
            Self::Destructor { ty, attrs, generics } => {
                let name = Name::Destructor(ty.clone());
                present_pub_function(
                    attrs.to_token_stream(),
                    name.mangle_tokens_default(),
                    Punctuated::from_iter([
                        FieldComposer::named(Name::Dictionary(DictionaryName::Ffi), FieldTypeConversionKind::Type(ty.joined_mut()))
                    ]),
                    ReturnType::Default,
                    generics.clone(),
                    InterfacesMethodExpr::UnboxAny(DictionaryName::Ffi.to_token_stream()).to_token_stream().terminated()
                )
            },
            Self::ObjAsTrait { name, item_type, trait_type, vtable_name, attrs } => {
                let fields = CommaPunctuated::from_iter([
                    FieldComposer::named(Name::Dictionary(DictionaryName::Object), FieldTypeConversionKind::Conversion(quote!(obj as *const ()))),
                    FieldComposer::named(Name::Dictionary(DictionaryName::Vtable), FieldTypeConversionKind::Conversion(quote!(&#vtable_name))),
                ]);
                present_pub_function(
                    attrs.to_token_stream(),
                    name.mangle_tokens_default(),
                    Punctuated::from_iter([
                        FieldComposer::named(Name::Dictionary(DictionaryName::Obj), FieldTypeConversionKind::Type(item_type.joined_const()))
                    ]),
                    ReturnType::Type(RArrow::default(), trait_type.to_type().into()),
                    None,
                    quote!(#trait_type { #fields })
                )
            },
            BindingPresentation::ObjAsTraitDestructor { name, item_type, trait_type, attrs, generics } => {
                present_pub_function(
                    attrs.to_token_stream(),
                    name.mangle_tokens_default(),
                    Punctuated::from_iter([FieldComposer::named(Name::Dictionary(DictionaryName::Obj), FieldTypeConversionKind::Conversion(trait_type.to_token_stream()))]),
                    ReturnType::Default,
                    generics.clone(),
                    InterfacesMethodExpr::UnboxAny(quote!(obj.object as *mut #item_type)).to_token_stream().terminated()
                )
            },
            BindingPresentation::Getter { name, field_name, obj_type, field_type, attrs, generics } => {
                present_pub_function(
                    attrs.to_token_stream(),
                    name.mangle_tokens_default(),
                    Punctuated::from_iter([
                        FieldComposer::named(Name::Dictionary(DictionaryName::Obj), FieldTypeConversionKind::Type(obj_type.joined_const()))]),
                    ReturnType::Type(RArrow::default(), field_type.clone().into()),
                    generics.clone(),
                    quote!((*obj).#field_name)
                )
            },
            BindingPresentation::Setter { name, field_name, obj_type, field_type, attrs, generics } => {
                // println!("BindingPresentation::Setter: {}\n\t{}\n\t{}\n\t{}\n\t{}", name.mangle_ident_default(), field_name, obj_type.to_token_stream(), field_type.to_token_stream(), attrs);
                present_pub_function(
                    attrs.to_token_stream(),
                    name.mangle_tokens_default(),
                    CommaPunctuated::from_iter([
                        FieldComposer::named(Name::Dictionary(DictionaryName::Obj), FieldTypeConversionKind::Type(obj_type.joined_mut())),
                        FieldComposer::named(Name::Dictionary(DictionaryName::Value), FieldTypeConversionKind::Type(field_type.clone())),
                    ]),
                    ReturnType::Default,
                    generics.clone(),
                    quote!((*obj).#field_name = value;))
            },
            BindingPresentation::GetterOpaque { name, field_name, obj_type, field_type, attrs, generics } => {
                present_pub_function(
                    attrs.to_token_stream(),
                    name.mangle_tokens_default(),
                    Punctuated::from_iter([
                        FieldComposer::named(Name::Dictionary(DictionaryName::Obj), FieldTypeConversionKind::Type(obj_type.joined_const()))]),
                    ReturnType::Type(RArrow::default(), field_type.clone().into()),
                    generics.clone(),
                    quote!((*obj).#field_name)
                )
            },
            BindingPresentation::SetterOpaque { name, field_name, obj_type, field_type, attrs, generics } => {
                // println!("BindingPresentation::SetterOpaque: {}\n\t{}\n\t{}\n\t{}\n\t{}", name.mangle_ident_default(), field_name, obj_type.to_token_stream(), field_type.to_token_stream(), attrs.to_token_stream());
                present_pub_function(
                    attrs.to_token_stream(),
                    name.mangle_tokens_default(),
                    CommaPunctuated::from_iter([
                        FieldComposer::named(Name::Dictionary(DictionaryName::Obj), FieldTypeConversionKind::Type(obj_type.joined_mut())),
                        FieldComposer::named(Name::Dictionary(DictionaryName::Value), FieldTypeConversionKind::Type(field_type.clone())),
                    ]),
                    ReturnType::Default,
                    generics.clone(),
                    quote!((*obj).#field_name = value;))
            },
            BindingPresentation::RegularFunction { attrs, is_async, name, arguments, input_conversions, return_type, output_conversions, generics } => {
                if *is_async {
                    let mut args = Punctuated::from_iter([ArgPresentation::NamedType { attrs: quote!(), name: quote!(runtime), var: quote!(*mut std::os::raw::c_void) }]);
                    args.extend(arguments.clone());
                    present_pub_function(
                        attrs.to_token_stream(),
                        name.mangle_tokens_default(),
                        args,
                        return_type.clone(),
                        generics.clone(),
                        quote! {
                            let rt = unsafe { &*(runtime as *mut tokio::runtime::Runtime) };
                            let obj = rt.block_on(async { #input_conversions .await });
                            #output_conversions
                        }
                    )
                } else {
                    present_pub_function(
                        attrs.to_token_stream(),
                        name.mangle_tokens_default(),
                        arguments.clone(),
                        return_type.clone(),
                        generics.clone(),
                        quote!(let obj = #input_conversions; #output_conversions)
                    )
                }
            },
            // BindingPresentation::Callback { name, arguments, output_expression: return_type } =>
            //     quote!(pub type #name = unsafe extern "C" fn(#arguments) #return_type;),
            BindingPresentation::Callback { name, attrs, ffi_args, result, conversion } => {
                let definition = create_callback(name, attrs.to_token_stream(), ffi_args.to_token_stream(), result.clone());
                quote! {
                    #definition
                    #conversion
                }
            }

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
                    None,
                    body.clone()
                )
            },
            BindingPresentation::StaticVTableInnerFnDeclaration { name, fn_name } =>
                quote!(#fn_name: #name),

        }.to_tokens(tokens)
     }
}
