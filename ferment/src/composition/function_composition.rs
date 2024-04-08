use std::fmt::{Debug, Formatter};
use proc_macro2::{Ident, TokenStream as TokenStream2};
use syn::{BareFnArg, FnArg, Generics, parse_quote, Pat, Path, PathSegment, PatIdent, PatType, Receiver, ReturnType, Signature, Type, TypeBareFn};
use quote::{quote, ToTokens};
use syn::punctuated::Punctuated;
use syn::token::{Comma, Paren, RArrow};
use crate::composer::constants;
use crate::composition::Composition;
use crate::composition::context::FnSignatureCompositionContext;
use crate::context::ScopeContext;
use crate::conversion::FieldTypeConversion;
use crate::ext::{Conversion, CrateExtension, FFIResolveExtended, Mangle, ResolveTrait};
use crate::holder::PathHolder;
use crate::naming::{DictionaryFieldName, Name};
use crate::presentation::context::{FieldTypePresentableContext, OwnedItemPresentableContext};
use crate::presentation::BindingPresentation;
use crate::presentation::context::name::{Aspect, Context};
use crate::presentation::ScopeContextPresentable;
use crate::wrapped::Wrapped;

#[derive(Clone)]
pub enum FnSignatureContext {
    ModFn,
    Impl(Type, Option<Type>)
}

impl Debug for FnSignatureContext {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            FnSignatureContext::ModFn =>
                format!("ModFn"),
            FnSignatureContext::Impl(self_ty, trait_ty) =>
                format!("Impl(self: {}, trait: {}", self_ty.to_token_stream(), trait_ty.to_token_stream())
        }.as_str())
    }
}

impl FnSignatureContext {
    pub fn is_trait_fn(&self) -> bool {
        match self {
            FnSignatureContext::Impl(_, Some(_)) => true,
            _ => false
        }
    }
}
#[derive(Clone, Debug)]
pub struct FnReturnTypeComposition {
    pub presentation: ReturnType,
    pub conversion: FieldTypePresentableContext,
}

#[derive(Clone)]
pub struct FnArgComposition {
    pub name: Option<TokenStream2>,
    pub name_type_original: OwnedItemPresentableContext,
    pub name_type_conversion: FieldTypePresentableContext,
}

impl Debug for FnArgComposition {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FnArgComposition")
            .field("name", &format!("{}", self.name.to_token_stream()))
            .field("name_type_original", &format!("{}", self.name_type_original))
            .field("name_type_conversion", &format!("{:?}", self.name_type_conversion))
            .finish()
    }
}

#[derive(Clone, Debug)]
pub struct FnSignatureComposition {
    pub is_async: bool,
    pub ident: Option<Ident>,
    pub scope: PathHolder,
    pub return_type: FnReturnTypeComposition,
    pub arguments: Vec<FnArgComposition>,
    pub generics: Option<Generics>,
    pub impl_context: FnSignatureContext,
    // pub self_ty: Option<Type>,
}

impl Composition for FnSignatureComposition {
    type Context = FnSignatureCompositionContext;
    type Presentation = BindingPresentation;

    fn present(self, composition_context: Self::Context, source: &ScopeContext) -> Self::Presentation {
        match composition_context {
            FnSignatureCompositionContext::FFIObject => {
                let arguments = self.arguments
                    .iter()
                    .map(|arg| {

                        // if self.impl_context.is_trait_fn() {
                        //     println!("FnSignatureCompositionContext::FFIObject::present (original): {}", arg.name_type_original.present(source));
                        // }
                        arg.name_type_original.clone()
                    })
                    .collect::<Punctuated<_, Comma>>();

                let fn_name = self.ident.unwrap();
                let mut full_fn_path = self.scope.joined(&fn_name);
                if self.scope.is_crate_based() {
                    let crate_chunk = Path::from(PathSegment::from(source.scope.crate_ident().clone()));
                    full_fn_path.0 = full_fn_path.0.replaced_first_with_ident(&crate_chunk);
                }
                let argument_conversions = self.arguments
                    .iter()
                    .map(|arg| {
                        // if self.impl_context.is_trait_fn() {
                        //     println!("FnSignatureCompositionContext::FFIObject::present (input conversion): {}", arg.name_type_conversion.present(source));
                        // }
                        OwnedItemPresentableContext::Conversion(arg.name_type_conversion.present(source))
                    })
                    .collect::<Punctuated<_, _>>();
                let name = Name::ModFn(full_fn_path.0.clone());
                let aspect_context = match self.impl_context {
                    FnSignatureContext::ModFn =>
                        Context::Fn {
                            path: full_fn_path.0,
                            self_ty: None,
                            trait_ty: None
                        },
                    FnSignatureContext::Impl(self_ty, trait_ty) =>
                        Context::Fn {
                            path: full_fn_path.0,
                            self_ty: Some(source.full_type_for(&self_ty)),
                            trait_ty : trait_ty.as_ref()
                                .and_then(|trait_ty|
                                    source.full_type_for(trait_ty)
                                        .trait_ty(source)
                                        .and_then(|full_trait_ty|
                                            full_trait_ty.to_ty()))
                        },
                };
                let aspect = Aspect::FFI(aspect_context);
                let input_conversions = constants::ROUND_BRACES_FIELDS_PRESENTER((aspect, argument_conversions)).present(source);
                let return_type = self.return_type.presentation;
                let output_conversions = self.return_type.conversion.present(source);
                // if self.impl_context.is_trait_fn() {
                //     println!("FnSignatureCompositionContext::FFIObject::present (input_conversions): {}", input_conversions);
                // }
                BindingPresentation::RegularFunction {
                    is_async: self.is_async,
                    name,
                    arguments: arguments.present(source),
                    input_conversions,
                    return_type,
                    output_conversions
                }
            },
            FnSignatureCompositionContext::FFIObjectCallback => {
                let arguments = self.arguments
                    .iter()
                    .map(|arg| arg.name_type_original.present(source))
                    .collect::<Punctuated<_, _>>();
                let output_expression = self.return_type.presentation;
                let name = self.ident.clone().unwrap().to_token_stream();
                // println!("FnSignatureCompositionContext::FFIObjectCallback: {}: {}: {}", name, arguments.to_token_stream(), output_expression.to_token_stream());
                BindingPresentation::Callback {
                    name,
                    arguments,
                    output_expression
                }
            },
            FnSignatureCompositionContext::TraitVTableInner => {
                // println!("FnSignatureCompositionContext::TraitVTableInner: {:#?}", self.arguments);
                let arguments = self.arguments.iter()
                    .map(|arg| {
                        // println!("FnSignatureCompositionContext::TraitVTableInner::ARg: {}", arg.name_type_original);
                        arg.name_type_original.clone()
                    })
                    .collect::<Punctuated<_, Comma>>();
                let fn_name = self.ident.unwrap();
                let presentation = Wrapped::<_, Paren>::new(arguments.present(source));
                // println!("FnSignatureCompositionContext::TraitVTableInner::presentation: {}", presentation.to_token_stream());
                let name_and_args = quote!(unsafe extern "C" fn #presentation);
                let output_expression = self.return_type.presentation;

                BindingPresentation::TraitVTableInnerFn {
                    name: Name::VTableInnerFn(fn_name),
                    name_and_args,
                    output_expression
                }
            }
            // FnSignatureCompositionContext::StaticVTable(trait_decomposition) => {
            //     let item_full_ty = context.full_type_for(&parse_quote!(#item_name));
            //     let trait_full_ty = context.full_type_for(&parse_quote!(#trait_ident));
            //     // let (vtable_methods_implentations, vtable_methods_declarations): (Vec<TraitVTablePresentation>, Vec<TraitVTablePresentation>) = trait_decomposition.methods.into_iter()
            //     // let (vtable_methods_implentations, method_names, method_signatures): (Vec<TraitVTablePresentation>, Vec<Ident>, Vec<Ident>) = trait_decomposition.methods.into_iter()
            //     let methods_compositions: Vec<TraitVTableMethodComposition> = trait_decomposition.methods.into_iter()
            //         .map(|signature_decomposition| {
            //
            //             let FnReturnTypeComposition { presentation: output_expression, conversion: output_conversions } = signature_decomposition.return_type;
            //             let fn_name = signature_decomposition.ident.unwrap();
            //             let ffi_method_ident = format_ident!("{}_{}", item_name, fn_name);
            //             // let arguments = signature_decomposition.arguments
            //             //     .iter()
            //             //     .map(|arg| arg.name_type_original.clone())
            //             //     .collect::<Vec<_>>();
            //             //
            //             // let name_and_args = ROUND_BRACES_FIELDS_PRESENTER((quote!(unsafe extern "C" fn #ffi_method_ident), arguments)).present(context);
            //             // let argument_names = IteratorPresentationContext::Round(
            //             //     signature_decomposition.arguments
            //             //         .iter()
            //             //         .map(|arg|
            //             //             arg.name.map_or(
            //             //                 OwnedItemPresenterContext::Conversion(quote!(cast_obj)),
            //             //                 |_| OwnedItemPresenterContext::Conversion(arg.name_type_conversion.clone())))
            //             //         .collect())
            //             //     .present(context);
            //
            //             TraitVTableMethodComposition {
            //                 fn_name,
            //                 ffi_fn_name: ffi_method_ident,
            //                 signature_composition: signature_decomposition.clone()
            //             }
            //             // (TraitVTablePresentation::Method {
            //             //     fn_name: fn_name.clone(),
            //             //     sig_name: ffi_method_ident.clone(),
            //             //     argument_names,
            //             //     name_and_args,
            //             //     output_expression,
            //             //     item_type: item_full_ty.clone(),
            //             //     trait_type: trait_full_ty.clone(),
            //             //     output_conversions,
            //             // }, fn_name, ffi_method_ident)
            //         }).collect();
            //
            //     // FFIObjectPresentation::StaticVTable {
            //     //     name: f,
            //     //     methods_names: vec![],
            //     //     methods_signatures: vec![],
            //     //     fq_trait_vtable: Default::default(),
            //     //     methods_implementations: vec![],
            //     //     methods_declarations: vec![],
            //     // }
            //     // FFIObjectPresentation::StaticVTable {
            //     //     name: Name::TraitImplVtable(item_name.clone(), trait_ident.clone()),
            //     //     fq_trait_vtable: if is_defined_in_same_scope { quote!(#trait_vtable_ident) } else { quote!(#trait_scope::#trait_vtable_ident) },
            //     //     methods_implementations: vtable_methods_implentations,
            //     //     methods_declarations: vtable_methods_declarations,
            //     // }
            //     FFIObjectPresentation::StaticVTable {
            //         name: Name::TraitImplVtable(item_name.clone(), trait_ident.clone()),
            //         fq_trait_vtable: if is_defined_in_same_scope { quote!(#trait_vtable_ident) } else { quote!(#trait_scope::#trait_vtable_ident) },
            //         // methods_implementations: vtable_methods_implentations,
            //         // methods_declarations: vtable_methods_declarations,
            //         methods_compositions,
            //     }
            // }
        }
    }
}


impl FnSignatureComposition {
    pub fn from_signature(impl_context: &FnSignatureContext, sig: &Signature, scope: &PathHolder, source: &ScopeContext) -> Self {
        let Signature { output, ident, inputs, generics, .. } = sig;
        let return_type = handle_fn_return_type(output, source);
        let ident = Some(ident.clone());
        let arguments = handle_fn_args(inputs, &impl_context, source);
        if impl_context.is_trait_fn() {
            println!("FnSignatureComposition::from_signature: [{}]: {:?} \n{:#?}", ident.to_token_stream(), impl_context, arguments);
        }
        let is_async = sig.asyncness.is_some();
        FnSignatureComposition { is_async, ident, scope: scope.clone(), return_type, arguments, generics: Some(generics.clone()), impl_context: impl_context.clone() }
    }

    pub fn from_bare_fn(bare_fn: &TypeBareFn, ident: &Ident, scope: PathHolder, context: &ScopeContext) -> Self {
        let TypeBareFn { inputs, output, .. } = bare_fn;
        let arguments = handle_bare_fn_args(inputs, context);
        let return_type = handle_bare_fn_return_type(output, context);
        FnSignatureComposition {
            is_async: false,
            ident: Some(ident.clone()),
            scope,
            arguments,
            return_type,
            impl_context: FnSignatureContext::ModFn,
            generics: None
        }
    }
}

fn handle_fn_return_type(output: &ReturnType, source: &ScopeContext) -> FnReturnTypeComposition {
    match output {
        ReturnType::Default => FnReturnTypeComposition {
            presentation: ReturnType::Default,
            conversion: FieldTypePresentableContext::LineTermination
        },
        ReturnType::Type(_, field_type) => FnReturnTypeComposition {
            presentation: ReturnType::Type(RArrow::default(), Box::new(field_type.ffi_full_dictionary_type_presenter(source))),
            conversion: field_type.conversion_to(FieldTypePresentableContext::Obj)
        },
    }
}
// fn handle_arg_type(ty: &Type, pat: &Pat, source: &ScopeContext) -> TokenStream2 {
//
//     match pat {
//         Pat::Ident(PatIdent { ident, .. }) => ty.conversion_from(FieldTypePresentableContext::Simple(quote!(#ident))),
//         _ => panic!("error: Arg conversion not supported: {}", quote!(#ty)),
//     }.present(source)
//
//     // ty.conversion_from()
//     // match (ty, pat) {
//     //     (Type::Path(type_path), Pat::Ident(PatIdent { ident, .. })) =>
//     //         type_path.conversion_from(FieldTypePresentableContext::Simple(quote!(#ident))),
//     //     (Type::Array(type_array), Pat::Ident(PatIdent { ident, .. })) =>
//     //         type_array.conversion_from(FieldTypePresentableContext::Simple(quote!(#ident))),
//     //     (Type::Slice(type_slice), Pat::Ident(PatIdent { ident, .. })) =>
//     //         type_slice.conversion_from(FieldTypePresentableContext::Simple(quote!(#ident))),
//     //     (Type::Tuple(type_tuple), Pat::Ident(PatIdent { ident, .. })) =>
//     //         type_tuple.conversion_from(FieldTypePresentableContext::Simple(quote!(#ident))),
//     //     (Type::TraitObject(type_trait_object), Pat::Ident(PatIdent { ident: _, .. })) =>
//     //         type_trait_object.conversion_from(FieldTypePresentableContext::Simple(type_trait_object.to_token_stream())),
//     //     (Type::Reference(type_reference), Pat::Ident(PatIdent { ident, .. })) =>
//     //         type_reference.conversion_from(FieldTypePresentableContext::Simple(quote!(#ident))),
//     //     // (Type::Ptr(TypePtr { star_token, const_token, mutability, elem }), Pat::Ident(PatIdent { ident, .. })) =>
//     //     _ => panic!("error: Arg conversion not supported: {}", quote!(#ty)),
//     // }.present(source)
// }

fn handle_bare_fn_return_type(output: &ReturnType, context: &ScopeContext) -> FnReturnTypeComposition {
    FnReturnTypeComposition {
        presentation: if let ReturnType::Type(token, field_type) = output {
            ReturnType::Type(token.clone(), Box::new(field_type.ffi_full_dictionary_type_presenter(context)))
        } else {
            ReturnType::Default
        },
        conversion: FieldTypePresentableContext::Empty
    }
}

fn handle_fn_args(inputs: &Punctuated<FnArg, Comma>, impl_context: &FnSignatureContext, source: &ScopeContext) -> Vec<FnArgComposition> {
    // TODO: replace Fn arguments with crate::fermented::generics::#ident or #import
    inputs
        .iter()
        .map(|arg| match arg {
            FnArg::Receiver(Receiver { mutability, reference, .. }) => {
                // println!("FnArg::Receiver: {}", ty.to_token_stream());
                let (full_ty, name_type_conversion) = match impl_context {
                    FnSignatureContext::Impl(self_ty, trait_ty) =>
                        ({
                             // let full_self_ty = source.full_type_for(self_ty).mangle_ident_default();
                             // let full_self_ty = source.full_type_for(self_ty);
                             match trait_ty {
                                 None =>
                                     source.full_type_for(self_ty)
                                         .mangle_ident_default(),
                                 Some(trait_ty) => {
                                     source.full_type_for(trait_ty)
                                         .mangle_ident_default()
                                     // quote!(<#full_self_ty as #trait_path>)
                                 }
                             }.to_token_stream()
                            // full_self_ty.mangle_ident_default().to_token_stream()
                         },
                            //&*((*self_).object as *const ferment_example::chain::common::chain_type::DevnetType)
                            //                            let self_ = (*self_).object as *const ferment_example::chain::common::chain_type::DevnetType;
                         match trait_ty {
                                None =>
                                    FieldTypePresentableContext::From(FieldTypePresentableContext::Self_.into()),
                                Some(_trait_ty) =>
                                    FieldTypePresentableContext::SelfAsTrait(source.full_type_for(self_ty).to_token_stream())
                                    // FieldTypePresentableContext::FromConst(FieldTypePresentableContext::SelfAsTrait(source.full_type_for(self_ty).ffi_external_path_converted(source).unwrap().to_token_stream()).into())
                            }
                         ),
                    FnSignatureContext::ModFn => panic!("[ERROR] Receiver in ModFn")
                };
                let access = mutability.as_ref().map_or(quote!(const), ToTokens::to_token_stream);
                let name_type_original = OwnedItemPresentableContext::Named(
                    FieldTypeConversion::Named(
                        Name::Dictionary(DictionaryFieldName::Self_), parse_quote!(* #access #full_ty)),
                    false);
                let name_type_conversion = if reference.is_some() {
                    FieldTypePresentableContext::AsRef(name_type_conversion.into())
                } else {
                    name_type_conversion
                };
                FnArgComposition {
                    name: None,
                    name_type_original,
                    name_type_conversion
                }
            },
            FnArg::Typed(PatType { ty, pat, .. }) => {
                // TODO: handle mut/const with pat
                // println!("FnArg::Typed: {}", ty.to_token_stream());
                let full_ty = source.full_type_for(ty);
                let full_type = full_ty.ffi_full_dictionary_type_presenter(source);
                let name_type_original = OwnedItemPresentableContext::Named(
                    FieldTypeConversion::Named(Name::Pat(*pat.clone()), full_type),
                    false);
                let name_type_conversion = match &**pat {
                    Pat::Ident(PatIdent { ident, .. }) => {
                        ty.conversion_from(FieldTypePresentableContext::Simple(quote!(#ident)))
                    },
                    _ => panic!("error: Arg conversion not supported: {}", quote!(#ty)),
                };
                FnArgComposition {
                    name: Some(pat.to_token_stream()),
                    name_type_original,
                    name_type_conversion
                }
            },
        })
        .collect()
}

fn handle_bare_fn_args(inputs: &Punctuated<BareFnArg, Comma>, context: &ScopeContext) -> Vec<FnArgComposition> {
    inputs
        .iter()
        .map(|BareFnArg { ty, name, .. }| {
            let name = name.clone().map(|(ident, _)| ident);
            FnArgComposition {
                name: name.clone().map(|g| g.to_token_stream()),
                name_type_original: OwnedItemPresentableContext::Named(
                    FieldTypeConversion::Named(
                        Name::Optional(name),
                        ty.ffi_full_dictionary_type_presenter(context)),
                    false),
                name_type_conversion: FieldTypePresentableContext::Empty
            }
        })
        .collect::<Vec<_>>()
}
