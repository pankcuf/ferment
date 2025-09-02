use quote::{format_ident, quote, ToTokens};
use syn::{FnArg, PatType, Receiver, ReturnType, Signature};
use syn::__private::TokenStream2;
use syn::token::{Const, Semi};
use crate::ast::{CommaPunctuated, Depunctuated};
use crate::composer::{AspectPresentable, AttrComposable, ConversionFromComposer, SourceAccessible, SourceComposable, ConversionToComposer, TypeAspect, VarComposer, VTableComposer};
use crate::context::ScopeContext;
use crate::ext::{Accessory, ExpressionComposable, Mangle, Resolve, ToPath, ToType};
use crate::lang::{FromDictionary, RustSpecification, Specification};
use crate::presentable::{ScopeContextPresentable, TypeContext};
use crate::presentation::{ArgPresentation, BindingPresentation, DictionaryExpr, DictionaryName, FFIFullPath, Name};

pub type ExpressionWrapper<SPEC> = fn(<SPEC as Specification>::Expr) -> <SPEC as Specification>::Expr;

impl SourceComposable for VTableComposer<RustSpecification> {
    type Source = ScopeContext;
    type Output = BindingPresentation;

    fn compose(&self, source: &Self::Source) -> Self::Output {
        let target_type = self.present_target_aspect();
        let ffi_aspect = self.present_ffi_aspect();
        let trait_ty = self.raw_target_type_aspect().present(source);
        let full_target_type = source.full_type_for(&target_type);
        let full_type = source.full_type_for(&trait_ty);
        let maybe_item_trait = source.maybe_item_trait(&full_type.to_path());
        if maybe_item_trait.is_none() {
            return BindingPresentation::Empty;
        }
        let mut methods_declarations = CommaPunctuated::new();
        let mut methods_implementations = Depunctuated::new();
        self.vtable_method_composers
            .iter()
            .for_each(|method_composer| {
                let method_composer = method_composer.borrow();
                let method_scope_context = method_composer.source_ref();
                let method_ty_context = method_composer.type_context();
                let sig_context = match &method_ty_context {
                    TypeContext::Fn { sig_context, .. } => sig_context,
                    _ => panic!("Not a function")
                };

                let sig = sig_context.maybe_signature().unwrap();
                let Signature { ident, output, inputs, .. } = sig;
                let name = Name::<RustSpecification>::TraitImplVtableFn(trait_ty.mangle_ident_default(), method_scope_context.scope.to_type().mangle_ident_default());
                let mut args = CommaPunctuated::new();
                let mut args_conversions = CommaPunctuated::new();
                inputs.iter().for_each(|arg| match arg {
                    FnArg::Receiver(Receiver { mutability, reference, attrs, .. }) => {
                        let arg_pres = ArgPresentation::inherited_field(
                            attrs,
                            <RustSpecification as Specification>::Name::dictionary_name(DictionaryName::Self_).mangle_ident_default(),
                            VarComposer::<RustSpecification>::key_ref_in_composer_scope(sig_context.receiver_ty())
                                .compose(&method_scope_context)
                                .to_type()
                        );
                        args.push(arg_pres);
                        let (acc, expr): (TokenStream2, ExpressionWrapper<RustSpecification>) = match (mutability, reference) {
                            (Some(r#mut), _) => (r#mut.to_token_stream(), <RustSpecification as Specification>::Expr::mut_ref),
                            (_, Some(..)) => (Const::default().to_token_stream(), <RustSpecification as Specification>::Expr::r#ref),
                            (..) => (Const::default().to_token_stream(), <RustSpecification as Specification>::Expr::simple_expr),
                        };
                        args_conversions.push(ArgPresentation::attr_tokens(attrs, expr(<RustSpecification as Specification>::Expr::dict_expr(DictionaryExpr::self_as_trait(&full_target_type, acc))).present(source)));
                    },
                    FnArg::Typed(PatType { ty, attrs, pat, .. }) => {
                        let name = Name::<RustSpecification>::pat(pat);
                        args.push(ArgPresentation::inherited_field(attrs, name.mangle_ident_default(), VarComposer::<RustSpecification>::key_ref_in_composer_scope(ty).compose(&method_scope_context).to_type()));
                        args_conversions.push(ArgPresentation::attr_tokens(attrs, ConversionFromComposer::<RustSpecification>::key_in_composer_scope(name, ty).compose(&method_scope_context).present(source)));
                    }
                });

                let (out, presentable_output_conversion) = match &output {
                    ReturnType::Default => (ReturnType::Default, <RustSpecification as Specification>::Expr::simple(Semi::default())),
                    ReturnType::Type(_, ty) => (
                        ReturnType::Type(Default::default(), Box::new(VarComposer::<RustSpecification>::key_ref_in_composer_scope(ty).compose(&method_scope_context).to_type())),
                        ConversionToComposer::<RustSpecification>::key_in_composer_scope(<RustSpecification as Specification>::Name::dictionary_name(DictionaryName::Obj), ty)
                            .compose(&method_scope_context)
                    )
                };
                let output_conversion = presentable_output_conversion.present(&method_scope_context);
                methods_declarations.push(BindingPresentation::StaticVTableInnerFnDeclaration {
                    name: name.to_token_stream(),
                    fn_name: ident.clone()
                });
                methods_implementations.push(BindingPresentation::StaticVTableInnerFn {
                    aspect: (vec![], vec![], None),
                    name: name.to_token_stream(),
                    args,
                    output: out,
                    body: quote! {
                        let obj = <#target_type as #trait_ty>::#ident(#args_conversions);
                        #output_conversion
                    }
                });
            });
        let trait_ident = trait_ty.mangle_ident_default();
        let name = Name::<RustSpecification>::TraitImplVtable(ffi_aspect.mangle_ident_default(), trait_ident);
        let full_trait_path: FFIFullPath<RustSpecification> = trait_ty.resolve(&source);
        let full_trait_type = full_trait_path.to_type();
        let mut fq_trait_vtable = full_trait_type.to_path();
        fq_trait_vtable.segments.last_mut().unwrap().ident = format_ident!("{}_VTable", fq_trait_vtable.segments.last().unwrap().ident);
        let attrs = self.compose_attributes();
        let result = BindingPresentation::StaticVTable {
            attrs: attrs.clone(),
            name: name.to_token_stream(),
            fq_trait_vtable: fq_trait_vtable.to_token_stream(),
            methods_declarations,
            methods_implementations,
            bindings: Depunctuated::from_iter([
                BindingPresentation::ObjAsTrait {
                    aspect: (attrs.clone(), vec![], None),
                    item_var: target_type.joined_const(),
                    trait_type: full_trait_type.to_token_stream(),
                    name: Name::<RustSpecification>::TraitFn(target_type.clone(), full_trait_type.clone()),
                    vtable_name: name.to_token_stream(),
                },
                BindingPresentation::ObjAsTraitDestructor {
                    aspect: (attrs.clone(), vec![], None),
                    item_type: target_type.to_token_stream(),
                    trait_type: full_trait_type.to_token_stream(),
                    name: Name::<RustSpecification>::TraitDestructor(target_type, full_trait_type),
                }
            ])
        };
        result
    }
}
