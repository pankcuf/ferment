use std::fmt::{Debug, Formatter};
use proc_macro2::{Ident, TokenStream as TokenStream2};
use syn::{BareFnArg, FnArg, Generics, ItemFn, parse_quote, Pat, PatIdent, PatType, Receiver, ReturnType, Signature, Type, TypeBareFn};
use quote::{quote, ToTokens};
use syn::punctuated::Punctuated;
use syn::token::{Comma, RArrow};
use crate::composer::{Composer, Depunctuated};
use crate::composition::CfgAttributes;
use crate::context::ScopeContext;
use crate::conversion::FieldTypeConversion;
use crate::ext::{Conversion, FFIResolveExtended, Mangle, Resolve};
use crate::holder::PathHolder;
use crate::naming::{DictionaryFieldName, Name};
use crate::presentation::context::{FieldTypePresentableContext, OwnedItemPresentableContext};

#[derive(Clone)]
pub enum FnSignatureContext {
    ModFn(ItemFn),
    Impl(Type, Option<Type>, Signature),
    Bare(Ident, TypeBareFn),
    TraitInner(Type, Option<Type>, Signature)
}

impl Debug for FnSignatureContext {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            FnSignatureContext::ModFn(sig) =>
                format!("ModFn({})", sig.to_token_stream()),
            FnSignatureContext::Impl(self_ty, trait_ty, sig) =>
                format!("Impl(self: {}, trait: {}, sig: {}", self_ty.to_token_stream(), trait_ty.to_token_stream(), sig.to_token_stream()),
            FnSignatureContext::TraitInner(self_ty, trait_ty, sig) =>
                format!("TraitInner(self: {}, trait: {}, sig: {}", self_ty.to_token_stream(), trait_ty.to_token_stream(), sig.to_token_stream()),
            FnSignatureContext::Bare(ident, type_bare_fn) =>
                format!("Bare({}, {})", ident.to_token_stream(), type_bare_fn.to_token_stream()),
        }.as_str())
    }
}

impl FnSignatureContext {
    #[allow(unused)]
    pub fn is_trait_fn(&self) -> bool {
        match self {
            FnSignatureContext::Impl(_, Some(_), _) => true,
            _ => false
        }
    }
}
#[derive(Clone, Debug)]
pub struct FnReturnTypeComposer {
    pub presentation: ReturnType,
    pub conversion: FieldTypePresentableContext,
}

// impl<'a> Composer<'a> for FnReturnTypeComposer {
//     type Source = ScopeContext;
//     type Result = FieldTypePresentableContext;
//
//     fn compose(&self, source: &'a Self::Source) -> Self::Result {
//         self.conversion.present(source)
//     }
// }
// impl<'a> Composer<'a> for FnReturnTypeComposer {
//     type Source = ScopeContext;
//     type Result = ReturnType;
//
//     fn compose(&self, source: &'a Self::Source) -> Self::Result {
//         self.presentation.clone()
//     }
// }

#[derive(Clone)]
pub struct FnArgComposer {
    pub name: Option<TokenStream2>,
    pub name_type_original: OwnedItemPresentableContext,
    pub name_type_conversion: FieldTypePresentableContext,
}

impl Debug for FnArgComposer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FnArgComposition")
            .field("name", &format!("{}", self.name.to_token_stream()))
            .field("name_type_original", &format!("{}", self.name_type_original))
            .field("name_type_conversion", &format!("{}", self.name_type_conversion))
            .finish()
    }
}

#[derive(Clone, Debug)]
pub struct FnSignatureComposition {
    pub is_async: bool,
    pub ident: Option<Ident>,
    pub scope: PathHolder,
    pub return_type: FnReturnTypeComposer,
    pub arguments: Depunctuated<FnArgComposer>,
    pub generics: Option<Generics>,
    pub impl_context: FnSignatureContext,
    // pub self_ty: Option<Type>,
}

// impl Composition for FnSignatureComposition {
//     type Context = FnSignatureCompositionContext;
//     type Presentation = BindingPresentation;
//
//     fn present(self, context: Self::Context, source: &ScopeContext) -> Self::Presentation {
//         println!("FnSignatureComposition::present: {:?} --> {:?}", context, self);
//         match context {
//             FnSignatureCompositionContext::FFIObject => {
//                 let arguments = self.to_original_args();
//                 let fn_name = self.ident.unwrap();
//                 let mut full_fn_path = self.scope.joined(&fn_name);
//                 if self.scope.is_crate_based() {
//                     full_fn_path.replace_first_with(&PathHolder::from(source.scope.crate_ident().to_path()))
//                 }
//                 let argument_conversions = self.arguments
//                     .iter()
//                     .map(|arg| OwnedItemPresentableContext::Conversion(arg.name_type_conversion.present(source)))
//                     .collect::<Punctuated<_, _>>();
//                 let name = Name::ModFn(full_fn_path.0.clone());
//                 let aspect_context = Context::Fn {
//                     path: full_fn_path.0,
//                     sig_context: self.impl_context.clone()
//                 };
//                 let aspect = Aspect::FFI(aspect_context);
//                 let fields_presenter = constants::ROUND_BRACES_FIELDS_PRESENTER((aspect, argument_conversions));
//                 let input_conversions = fields_presenter.present(source);
//                 let return_type = self.return_type.presentation;
//                 let output_conversions = self.return_type
//                     .conversion
//                     .present(source);
//                 BindingPresentation::RegularFunction {
//                     is_async: self.is_async,
//                     arguments: arguments.present(source),
//                     name,
//                     input_conversions,
//                     return_type,
//                     output_conversions
//                 }
//             },
//             FnSignatureCompositionContext::FFIObjectCallback => {
//                 let arguments = self.to_original_args().present(source);
//                 let output_expression = self.return_type.presentation;
//                 let name = self.ident.clone().unwrap().to_token_stream();
//                 BindingPresentation::Callback {
//                     name,
//                     arguments,
//                     output_expression
//                 }
//             },
//             FnSignatureCompositionContext::TraitVTableInner => {
//                 let fn_name = self.ident.clone().unwrap();
//                 let presentation = Wrapped::<_, Paren>::new(self.to_original_args().present(source));
//                 let name_and_args = quote!(unsafe extern "C" fn #presentation);
//                 let output_expression = self.return_type.presentation;
//
//                 BindingPresentation::TraitVTableInnerFn {
//                     name: Name::VTableInnerFn(fn_name),
//                     name_and_args,
//                     output_expression
//                 }
//             },
//             // FnSignatureCompositionContext::StaticVTableInner => {
//             //
//             //     BindingPresentation::StaticVTableInnerFn { name, args, output, body }
//             //
//             // },
//             // FnSignatureCompositionContext::StaticVTableInner => {
//             //
//             //     BindingPresentation::StaticVTable {
//             //         name: Name::TraitImplVtable(item_name.clone(), trait_ident.clone()),
//             //         fq_trait_vtable: fq_trait_vtable.to_token_stream(),
//             //         methods_declarations,
//             //         methods_implementations,
//             //     }
//             // },
//             // FnSignatureCompositionContext::TraitVTableImpl => {
//             //
//             // }
//                 // FnSignatureCompositionContext::StaticVTable(trait_decomposition) => {
//             //     let item_full_ty = context.full_type_for(&parse_quote!(#item_name));
//             //     let trait_full_ty = context.full_type_for(&parse_quote!(#trait_ident));
//             //     // let (vtable_methods_implentations, vtable_methods_declarations): (Vec<TraitVTablePresentation>, Vec<TraitVTablePresentation>) = trait_decomposition.methods.into_iter()
//             //     // let (vtable_methods_implentations, method_names, method_signatures): (Vec<TraitVTablePresentation>, Vec<Ident>, Vec<Ident>) = trait_decomposition.methods.into_iter()
//             //     let methods_compositions: Vec<TraitVTableMethodComposition> = trait_decomposition.methods.into_iter()
//             //         .map(|signature_decomposition| {
//             //
//             //             let FnReturnTypeComposition { presentation: output_expression, conversion: output_conversions } = signature_decomposition.return_type;
//             //             let fn_name = signature_decomposition.ident.unwrap();
//             //             let ffi_method_ident = format_ident!("{}_{}", item_name, fn_name);
//             //             // let arguments = signature_decomposition.arguments
//             //             //     .iter()
//             //             //     .map(|arg| arg.name_type_original.clone())
//             //             //     .collect::<Vec<_>>();
//             //             //
//             //             // let name_and_args = ROUND_BRACES_FIELDS_PRESENTER((quote!(unsafe extern "C" fn #ffi_method_ident), arguments)).present(context);
//             //             // let argument_names = IteratorPresentationContext::Round(
//             //             //     signature_decomposition.arguments
//             //             //         .iter()
//             //             //         .map(|arg|
//             //             //             arg.name.map_or(
//             //             //                 OwnedItemPresenterContext::Conversion(quote!(cast_obj)),
//             //             //                 |_| OwnedItemPresenterContext::Conversion(arg.name_type_conversion.clone())))
//             //             //         .collect())
//             //             //     .present(context);
//             //
//             //             TraitVTableMethodComposition {
//             //                 fn_name,
//             //                 ffi_fn_name: ffi_method_ident,
//             //                 signature_composition: signature_decomposition.clone()
//             //             }
//             //             // (TraitVTablePresentation::Method {
//             //             //     fn_name: fn_name.clone(),
//             //             //     sig_name: ffi_method_ident.clone(),
//             //             //     argument_names,
//             //             //     name_and_args,
//             //             //     output_expression,
//             //             //     item_type: item_full_ty.clone(),
//             //             //     trait_type: trait_full_ty.clone(),
//             //             //     output_conversions,
//             //             // }, fn_name, ffi_method_ident)
//             //         }).collect();
//             //
//             //     // FFIObjectPresentation::StaticVTable {
//             //     //     name: f,
//             //     //     methods_names: vec![],
//             //     //     methods_signatures: vec![],
//             //     //     fq_trait_vtable: Default::default(),
//             //     //     methods_implementations: vec![],
//             //     //     methods_declarations: vec![],
//             //     // }
//             //     // FFIObjectPresentation::StaticVTable {
//             //     //     name: Name::TraitImplVtable(item_name.clone(), trait_ident.clone()),
//             //     //     fq_trait_vtable: if is_defined_in_same_scope { quote!(#trait_vtable_ident) } else { quote!(#trait_scope::#trait_vtable_ident) },
//             //     //     methods_implementations: vtable_methods_implentations,
//             //     //     methods_declarations: vtable_methods_declarations,
//             //     // }
//             //     FFIObjectPresentation::StaticVTable {
//             //         name: Name::TraitImplVtable(item_name.clone(), trait_ident.clone()),
//             //         fq_trait_vtable: if is_defined_in_same_scope { quote!(#trait_vtable_ident) } else { quote!(#trait_scope::#trait_vtable_ident) },
//             //         // methods_implementations: vtable_methods_implentations,
//             //         // methods_declarations: vtable_methods_declarations,
//             //         methods_compositions,
//             //     }
//             // }
//         }
//     }
// }

impl<'a> Composer<'a> for ReturnType {
    type Source = (bool, &'a ScopeContext);
    type Result = FnReturnTypeComposer;

    fn compose(&self, source: &Self::Source) -> Self::Result {
        let (bare, source) = source;
        match (bare, self) {
            (false, ReturnType::Default) => FnReturnTypeComposer {
                presentation: ReturnType::Default,
                conversion: FieldTypePresentableContext::LineTermination
            },
            (false, ReturnType::Type(_, ty)) => FnReturnTypeComposer {
                presentation: ReturnType::Type(RArrow::default(), Box::new(ty.ffi_full_dictionary_type_presenter(source))),
                conversion: ty.conversion_to(FieldTypePresentableContext::Obj)
            },
            (true, ReturnType::Type(token, field_type)) => FnReturnTypeComposer {
                presentation: ReturnType::Type(token.clone(), Box::new(field_type.ffi_full_dictionary_type_presenter(source))),
                conversion: FieldTypePresentableContext::Empty
            },
            (true, ReturnType::Default) => FnReturnTypeComposer {
                presentation: ReturnType::Default,
                conversion: FieldTypePresentableContext::Empty,
            }
        }
    }
}


// impl FnSignatureComposition {
//     pub fn to_original_args(&self) -> Punctuated<OwnedItemPresentableContext, Comma> {
//         Punctuated::from_iter(self.arguments.iter().map(|arg| arg.name_type_original.clone()))
//     }
//     pub fn from_signature(impl_context: &FnSignatureContext, sig: &Signature, scope: &PathHolder, source: &ScopeContext) -> Self {
//         let Signature { output, ident, inputs, generics, .. } = sig;
//         let return_type = output.compose(&(false, source));
//         let ident = Some(ident.clone());
//         let arguments = inputs
//             .iter()
//             .map(|arg| match arg {
//                 FnArg::Receiver(Receiver { mutability, reference, .. }) => {
//                     let (mangled_ident, name_type_conversion) = match impl_context {
//                         FnSignatureContext::Impl(self_ty, Some(trait_ty), sig) => (
//                             trait_ty.resolve(source).mangle_ident_default(),
//                             FieldTypePresentableContext::SelfAsTrait(self_ty.resolve(source).to_token_stream())
//                         ),
//                         FnSignatureContext::Impl(self_ty, None, sig) => (
//                             self_ty.resolve(source).mangle_ident_default(),
//                             FieldTypePresentableContext::From(FieldTypePresentableContext::Self_.into())
//                         ),
//                         _ => panic!("[ERROR] Receiver in args ({:?})", impl_context),
//                     };
//                     let access = mutability.as_ref().map_or(quote!(const), ToTokens::to_token_stream);
//                     let name_type_original = OwnedItemPresentableContext::Named(
//                         FieldTypeConversion::Named(
//                             Name::Dictionary(DictionaryFieldName::Self_),
//                             parse_quote!(* #access #mangled_ident)),
//                         false);
//                     let name_type_conversion = if reference.is_some() {
//                         FieldTypePresentableContext::AsRef(name_type_conversion.into())
//                     } else {
//                         name_type_conversion
//                     };
//                     FnArgComposition {
//                         name: None,
//                         name_type_original,
//                         name_type_conversion
//                     }
//                 },
//                 FnArg::Typed(pat_ty) =>
//                     pat_ty.compose(source),
//             })
//             .collect();
//
//         FnSignatureComposition {
//             is_async: sig.asyncness.is_some(),
//             ident,
//             scope: scope.clone(),
//             return_type,
//             arguments,
//             generics: Some(generics.clone()),
//             impl_context: impl_context.clone()
//         }
//     }
//
//     pub fn from_bare_fn(bare_fn: &TypeBareFn, ident: &Ident, scope: &PathHolder, source: &ScopeContext) -> Self {
//         let TypeBareFn { inputs, output, .. } = bare_fn;
//         let arguments = inputs.compose(source);
//         let return_type = output.compose(&(true, source));
//         FnSignatureComposition {
//             is_async: false,
//             ident: Some(ident.clone()),
//             scope: scope.clone(),
//             arguments,
//             return_type,
//             impl_context: FnSignatureContext::Bare(ident.clone(), bare_fn.clone()),
//             generics: None
//         }
//     }
// }

// fn handle_bare_fn_return_type(output: &ReturnType, context: &ScopeContext) -> FnReturnTypeComposition {
//     FnReturnTypeComposition {
//         presentation: if let ReturnType::Type(token, field_type) = output {
//             ReturnType::Type(token.clone(), Box::new(field_type.ffi_full_dictionary_type_presenter(context)))
//         } else {
//             ReturnType::Default
//         },
//         conversion: FieldTypePresentableContext::Empty
//     }
// }
impl<'a> Composer<'a> for Punctuated<BareFnArg, Comma> {
    type Source = ScopeContext;
    type Result = Depunctuated<FnArgComposer>;
    fn compose(&self, source: &Self::Source) -> Self::Result {
        self.iter()
            .map(|bare_fn_arg| bare_fn_arg.compose(source))
            .collect()
    }
}

// impl<'a> Composer<'a> for Punctuated<FnArg, Comma> {
//     type Source = ScopeContext;
//     type Result = Depunctuated<FnArgComposition>;
//     fn compose(&self, source: &Self::Source) -> Self::Result {
//         self.iter()
//             .map(|fn_arg| bare_fn_arg.compose(source))
//             .collect()
//     }
// }

impl<'a> Composer<'a> for BareFnArg {
    type Source = ScopeContext;
    type Result = FnArgComposer;
    fn compose(&self, source: &Self::Source) -> Self::Result {
        let BareFnArg { ty, attrs, name, .. } = self;
        let name = name.clone().map(|(ident, _)| ident);
        // println!("BareFnArg::compose: {}", ty.to_token_stream());
        FnArgComposer {
            name: name.clone().map(|g| g.to_token_stream()),
            name_type_original: OwnedItemPresentableContext::Named(
                FieldTypeConversion::Named(
                    Name::Optional(name),
                    ty.ffi_full_dictionary_type_presenter(source),
                    attrs.cfg_attributes()),
                false),
            name_type_conversion: FieldTypePresentableContext::Empty
        }
    }
}

impl<'a> Composer<'a> for PatType {
    type Source = (&'a FnSignatureContext, &'a ScopeContext);
    type Result = FnArgComposer;
    fn compose(&self, source: &Self::Source) -> Self::Result {
        let (ctx, source) = source;
        let PatType { ty, attrs, pat, .. } = self;
        // TODO: handle mut/const with pat
        // println!("PatType::compose: {}", ty.to_token_stream());
        let name_type_original = OwnedItemPresentableContext::Named(
            FieldTypeConversion::Named(
                Name::Pat(*pat.clone()),
                ty.ffi_full_dictionary_type_presenter(source),
                attrs.cfg_attributes()),
            false);
        let name_type_conversion = match &**pat {
            Pat::Ident(PatIdent { ident, .. }) => {
                //println!("Compose PatType: {} --> {}", ty.to_token_stream(), ty.resolve(source).to_token_stream());
                let full_ty = ty.resolve(source);
                let conversion = full_ty.conversion_from(FieldTypePresentableContext::Simple(quote!(#ident)));
                match &**ty {
                    Type::Reference(..) =>
                        FieldTypePresentableContext::AsRef(conversion.into()),
                    _ => conversion

                }
            },
            _ =>
                panic!("error: Arg conversion not supported: {}", quote!(#ty)),
        };
        // println!("PatType: {}", name_type_conversion);
        FnArgComposer {
            name: Some(pat.to_token_stream()),
            name_type_original,
            name_type_conversion
        }
    }
}

impl<'a> Composer<'a> for FnArg {
    type Source = (&'a FnSignatureContext, &'a ScopeContext);
    type Result = FnArgComposer;

    fn compose(&self, source: &'a Self::Source) -> Self::Result {
        match self {
            FnArg::Receiver(receiver) =>
                receiver.compose(source),
            FnArg::Typed(pat_ty) =>
                pat_ty.compose(source),
        }
    }
}

impl<'a> Composer<'a> for Receiver {
    type Source = (&'a FnSignatureContext, &'a ScopeContext);
    type Result = FnArgComposer;

    fn compose(&self, source: &'a Self::Source) -> Self::Result {
        let (ctx, source) = source;
        let Receiver { mutability, reference, attrs, .. } = self;
        match ctx {
            FnSignatureContext::ModFn(_) => panic!("receiver in mod fn"),
            FnSignatureContext::Bare(_, _) => panic!("receiver in bare fn"),
            FnSignatureContext::Impl(self_ty, maybe_trait_ty, _) |
            FnSignatureContext::TraitInner(self_ty, maybe_trait_ty, _) => {
                let (mangled_ident, name_type_conversion) = match maybe_trait_ty {
                    Some(trait_ty) => (
                        trait_ty.resolve(&source).mangle_ident_default(),
                        FieldTypePresentableContext::SelfAsTrait(self_ty.resolve(&source).to_token_stream())
                    ),
                    None => (
                        self_ty.resolve(&source).mangle_ident_default(),
                        FieldTypePresentableContext::From(FieldTypePresentableContext::Self_.into())
                    )
                };
                let access = mutability.as_ref().map_or(quote!(const), ToTokens::to_token_stream);
                let name_type_original = OwnedItemPresentableContext::Named(
                    FieldTypeConversion::Named(
                        Name::Dictionary(DictionaryFieldName::Self_),
                        parse_quote!(* #access #mangled_ident),
                        attrs.cfg_attributes()),
                    false);
                let name_type_conversion = if reference.is_some() {
                    FieldTypePresentableContext::AsRef(name_type_conversion.into())
                } else {
                    name_type_conversion
                };
                FnArgComposer {
                    name: None,
                    name_type_original,
                    name_type_conversion
                }
            },
        }
    }
}


