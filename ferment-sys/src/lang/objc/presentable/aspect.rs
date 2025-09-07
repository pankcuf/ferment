use quote::{format_ident, quote, ToTokens};
use syn::{Attribute, parse_quote, Type, TypeSlice, ItemFn, Signature, TypePath};
use syn::__private::TokenStream2;
use crate::ast::{DelimiterTrait, Wrapped};
use crate::composable::FnSignatureContext;
use crate::composer::PunctuatedArgKinds;
use crate::context::ScopeContext;
use crate::kind::{GenericTypeKind, MixinKind};
use crate::ext::{Accessory, Join, Mangle, Resolve, ResolveTrait, ToType};
use crate::lang::objc::ObjCSpecification;
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
    pub fn allocate<I, SEP>(&self, fields: Wrapped<PunctuatedArgKinds<ObjCSpecification, SEP>, SEP, I>, source: &ScopeContext) -> TokenStream2
        where I: DelimiterTrait,
              SEP: ToTokens {
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
            Aspect::Target(TypeContext::Enum { ident, prefix, .. } |
                           TypeContext::Struct { ident , prefix, .. }) =>
                format_ident!("{prefix}{}", Resolve::<Type>::resolve(ident, source).mangle_tokens_default().to_string()).to_type(),
            Aspect::Target(TypeContext::EnumVariant { ident, variant_ident, .. }) |
            Aspect::RawTarget(TypeContext::EnumVariant { ident, variant_ident, .. }) =>
                Resolve::<Type>::resolve(ident, source)
                    .joined_ident(variant_ident),
            Aspect::Target(TypeContext::Fn { path, .. }) |
            Aspect::FFI(TypeContext::Fn { path, sig_context: FnSignatureContext::Impl(..), .. }) |
            Aspect::RawTarget(TypeContext::Fn { path, .. } | TypeContext::Trait { path , .. } | TypeContext::Impl { path , .. }) =>
                path.to_type(),
            Aspect::Target(TypeContext::Trait { path , .. } | TypeContext::Impl { path , .. }) =>
                path.resolve(source),
            Aspect::Target(TypeContext::Mixin { mixin_kind: MixinKind::Generic(GenericTypeKind::Slice(Type::Slice(TypeSlice { elem, .. }))), .. }) =>
                parse_quote!(Vec<#elem>),
            Aspect::Target(TypeContext::Mixin { prefix, mixin_kind: MixinKind::Generic(kind), .. }) =>
                format_ident!("{prefix}{}", kind.ty().unwrap().mangle_tokens_default().to_string())
                    .to_type(),
            Aspect::Target(TypeContext::Mixin { mixin_kind: MixinKind::Bounds(model), .. }) |
            Aspect::RawTarget(TypeContext::Mixin { mixin_kind: MixinKind::Bounds(model), .. }) =>
                model.to_type(),
            Aspect::FFI(TypeContext::Mixin { mixin_kind: MixinKind::Generic(kind), .. }) =>
                kind.ty()
                    .cloned()
                    .unwrap()
                    .mangle_ident_default()
                    .to_type(),
            Aspect::FFI(TypeContext::Mixin { mixin_kind: MixinKind::Bounds(model), .. }) =>
                model.mangle_ident_default()
                    .to_type(),
            Aspect::FFI(TypeContext::Enum { ident , .. } | TypeContext::Struct { ident , .. } | TypeContext::Fn { sig_context: FnSignatureContext::ModFn(ItemFn { sig: Signature { ident, .. }, .. }) | FnSignatureContext::Bare(ident, _), .. }) =>
                Resolve::<Type>::resolve(ident, source)
                    .mangle_ident_default()
                    .to_type(),
            Aspect::FFI(TypeContext::Trait { path , .. } | TypeContext::Impl { path , .. }) =>
                Resolve::<Type>::resolve(path, source)
                    .mangle_ident_default()
                    .to_type(),
            Aspect::FFI(TypeContext::EnumVariant { ident, variant_ident, .. }) =>
                Resolve::<Type>::resolve(ident, source)
                    .mangle_ident_default()
                    .to_type()
                    .joined_ident(variant_ident),
            Aspect::FFI(TypeContext::Fn { sig_context: FnSignatureContext::TraitInner(_, self_ty, _), .. }) =>
                Resolve::<Type>::resolve(self_ty, source)
                    .mangle_ident_default()
                    .to_type(),
            Aspect::FFI(TypeContext::Fn { path, sig_context: FnSignatureContext::TraitImpl(_, self_ty, trait_ty), .. }) =>
                Resolve::<Type>::resolve(trait_ty, source)
                    .maybe_trait_ty(source)
                    .map(|full_trait_ty| {
                        let self_ty = Resolve::<Type>::resolve(self_ty, source);
                        let type_path: TypePath = parse_quote!(<#self_ty as #full_trait_ty>);
                        Type::Path(type_path.joined(&path.segments.last().unwrap().ident))
                    }).unwrap_or_else(|| path.to_type()),
            Aspect::FFI(TypeContext::Fn { path, sig_context: FnSignatureContext::TraitAsType(_, self_ty, trait_ty), .. }) => {
                let self_ty = Resolve::<Type>::resolve(self_ty, source);
                let trait_ty = Resolve::<Type>::resolve(trait_ty, source)
                    .maybe_trait_ty(source);
                let fn_name = &path.segments.last().unwrap().ident;
                parse_quote!(<#self_ty as #trait_ty>::#fn_name)
            }
            Aspect::RawTarget(TypeContext::Mixin { mixin_kind: MixinKind::Generic(kind), .. }) =>
                kind.ty()
                    .cloned()
                    .unwrap(),
            Aspect::RawTarget(TypeContext::Enum { ident , .. } | TypeContext::Struct { ident , .. }) =>
                ident.to_type(),
        }
    }
}
