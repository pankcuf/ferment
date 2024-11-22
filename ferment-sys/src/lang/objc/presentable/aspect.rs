use quote::{format_ident, quote, ToTokens};
use syn::{Attribute, parse_quote, Type, TypeSlice};
use syn::__private::TokenStream2;
use crate::ast::{DelimiterTrait, Wrapped};
use crate::composable::{FnSignatureContext, TypeModeled};
use crate::composer::PunctuatedArgKinds;
use crate::context::ScopeContext;
use crate::conversion::{GenericTypeKind, MixinKind};
use crate::ext::{AsType, Mangle, Resolve, ResolveTrait, ToType};
use crate::lang::objc::{ObjCFermentate, ObjCSpecification};
use crate::lang::objc::presentable::ty_context::TypeContext;
use crate::presentable::{Aspect, ScopeContextPresentable};
use crate::presentation::DictionaryName;

impl Aspect<TypeContext> {

    pub fn alloc_field_name(&self) -> TokenStream2 {
        match self {
            Aspect::Target(_) => DictionaryName::Obj.to_token_stream(),
            Aspect::FFI(_) => DictionaryName::FfiRef.to_token_stream(),
            Aspect::RawTarget(_) => DictionaryName::Obj.to_token_stream(),
        }
    }
    pub fn attrs(&self) -> &Vec<Attribute> {
        match self {
            Aspect::Target(context) => context.attrs(),
            Aspect::FFI(context) => context.attrs(),
            Aspect::RawTarget(context) => context.attrs(),
        }
    }
    pub fn allocate<I, SEP, SPEC>(&self, fields: Wrapped<PunctuatedArgKinds<ObjCFermentate, SPEC, SEP>, SEP, I>, source: &ScopeContext) -> TokenStream2
        where I: DelimiterTrait,
              SEP: ToTokens,
              SPEC: ObjCSpecification {
        let name = self.alloc_field_name();
        let aspect_presentation = self.present(source);
        match self {
            Aspect::Target(_context) => {

                let field_allocators = fields.content.iter().map(|f| {
                    let arg_presentation = f.present(source);
                    quote!(#name.#arg_presentation)
                });
                quote! {
                    #aspect_presentation *#name = [[self alloc] init];
                    #(#field_allocators;)*
                    return #name;
                }
            }
            Aspect::FFI(_context) | Aspect::RawTarget(_context) => {
                let field_allocators = fields.content.iter().map(|f| {
                    let arg_presentation = f.present(source);
                    quote!(#name->#arg_presentation;)
                });

                quote! {
                    struct #aspect_presentation *#name = malloc(sizeof(struct #aspect_presentation));
                    #(#field_allocators)*
                    return #name;
                }
            }
        }
    }
}

impl ScopeContextPresentable for Aspect<TypeContext> {
    type Presentation = Type;

    fn present(&self, source: &ScopeContext) -> Self::Presentation {
        match self {
            Aspect::Target(context) => {
                match context {
                    TypeContext::Enum { ident, prefix, .. } |
                    TypeContext::Struct { ident , prefix, .. } =>
                        {
                            let ty: Type = ident.to_type().resolve(source);
                            format_ident!("{}{}", prefix.to_string(), ty.mangle_tokens_default().to_string()).to_type()
                        },
                    TypeContext::EnumVariant { ident, variant_ident, .. } => {
                        let full_ty = Resolve::<Type>::resolve(&ident.to_type(), source);
                        parse_quote!(#full_ty::#variant_ident)
                    },
                    TypeContext::Fn { path, .. } => {
                        path.to_type()
                    }
                    TypeContext::Trait { path , ..} |
                    TypeContext::Impl { path , ..} =>
                        path.to_type().resolve(source),
                    TypeContext::Mixin { mixin_kind: MixinKind::Generic(GenericTypeKind::Slice(ty)), ..} => {
                        let type_slice: TypeSlice = parse_quote!(#ty);
                        let elem_type = &type_slice.elem;
                        parse_quote!(Vec<#elem_type>)
                    }
                    TypeContext::Mixin { prefix, mixin_kind: MixinKind::Generic(kind), ..} => {
                        let objc_name = kind.ty().unwrap().mangle_tokens_default();
                        format_ident!("{}{}", prefix.to_string(), objc_name.to_string())
                            .to_type()
                    },
                    TypeContext::Mixin { mixin_kind: MixinKind::Bounds(model), ..} =>
                        model.as_type().clone()
                    // model.type_model_ref().ty.clone(),
                }
            },
            Aspect::FFI(context) => {
                match context {
                    TypeContext::Mixin { mixin_kind: MixinKind::Generic(kind), ..} =>
                        kind.ty().cloned().unwrap().mangle_ident_default().to_type(),
                    TypeContext::Mixin { mixin_kind: MixinKind::Bounds(model), ..} =>
                        model.mangle_ident_default().to_type(),
                    TypeContext::Enum { ident , .. } |
                    TypeContext::Struct { ident , .. } => {
                        Resolve::<Type>::resolve(&ident.to_type(), source)
                            .mangle_ident_default()
                            .to_type()
                    }
                    TypeContext::Trait { path , .. } =>
                        Resolve::<Type>::resolve(&path.to_type(), source)
                            .mangle_ident_default()
                            .to_type(),
                    TypeContext::Impl { path , .. } =>
                        Resolve::<Type>::resolve(&path.to_type(), source)
                            .mangle_ident_default()
                            .to_type(),
                    TypeContext::EnumVariant { ident, variant_ident, .. } => {
                        let mangled_ty = Resolve::<Type>::resolve(&ident.to_type(), source).mangle_ident_default();
                        parse_quote!(#mangled_ty::#variant_ident)
                    },
                    TypeContext::Fn { path, sig_context, .. } => {
                        match sig_context {
                            FnSignatureContext::ModFn(item_fn) => {
                                Resolve::<Type>::resolve(&item_fn.sig.ident.to_type(), source)
                                    .mangle_ident_default()
                                    .to_type()
                            }
                            FnSignatureContext::TraitInner(self_ty, _trait_ty, _sig) => {
                                Resolve::<Type>::resolve(self_ty, source)
                                    .mangle_ident_default()
                                    .to_type()
                            },
                            FnSignatureContext::Impl(self_ty, trait_ty, _sig) => {
                                let self_ty = Resolve::<Type>::resolve(self_ty, source);
                                let trait_ty = trait_ty.as_ref()
                                    .and_then(|trait_ty|
                                        Resolve::<Type>::resolve(trait_ty, source)
                                            .maybe_trait_ty(source));

                                match trait_ty {
                                    Some(trait_ty) => {
                                        let fn_name = &path.segments.last().unwrap().ident;
                                        parse_quote!(<#self_ty as #trait_ty>::#fn_name)
                                    }
                                    None => path.to_type()
                                }
                            }
                            FnSignatureContext::Bare(ident, _type_bare_fn) => {
                                Resolve::<Type>::resolve(&ident.to_type(), source)
                                    .mangle_ident_default()
                                    .to_type()
                            }
                        }
                    }
                }
            },
            Aspect::RawTarget(context) => {
                match context {
                    TypeContext::Mixin { mixin_kind: MixinKind::Generic(kind), ..} =>
                        kind.ty().cloned().unwrap(),
                    TypeContext::Mixin { mixin_kind: MixinKind::Bounds(model), ..} =>
                        model.type_model_ref().ty.clone(),
                    TypeContext::Enum { ident , .. } |
                    TypeContext::Struct { ident , .. } =>
                        ident.to_type(),
                    TypeContext::EnumVariant { ident, variant_ident, .. } => {
                        let full_ty = Resolve::<Type>::resolve(&ident.to_type(), source);
                        parse_quote!(#full_ty::#variant_ident)
                    },
                    TypeContext::Fn { path, .. } |
                    TypeContext::Trait { path , .. } |
                    TypeContext::Impl { path , .. } => path.to_type()
                }
            }
        }
    }
}
