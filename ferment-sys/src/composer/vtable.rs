use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::Rc;
use quote::{format_ident, quote, ToTokens};
use syn::{Attribute, FnArg, PatType, Receiver, ReturnType, Signature, Type, Visibility};
use syn::token::Semi;
use ferment_macro::ComposerBase;
use crate::ast::{CommaPunctuated, Depunctuated};
use crate::composable::{AttrsModel, FieldComposer, FnSignatureContext, GenModel, TraitVTableComposer};
use crate::composer::{BasicComposerOwner, constants, AspectPresentable, BasicComposer, BasicComposerLink, ComposerLink, Linkable, SourceAccessible, SourceComposable, TypeAspect, VarComposer, from_trait_receiver_expr_composer, from_receiver_expr_composer, FromConversionFullComposer, ToConversionComposer, AttrComposable};
use crate::context::{ScopeContext, ScopeContextLink};
use crate::ext::{Mangle, Resolve, ToPath, ToType};
use crate::lang::{LangFermentable, RustSpecification, Specification};
use crate::presentable::{Aspect, Expression, PresentableArgument, ScopeContextPresentable};
use crate::presentation::{BindingPresentation, DictionaryName, FFIFullPath, Name, RustFermentate};

#[derive(ComposerBase)]
pub struct VTableComposer<LANG, SPEC>
    where LANG: LangFermentable + 'static,
          SPEC: Specification<LANG, Attr: Debug, Expr = Expression<LANG, SPEC>, Var: ToType> + 'static,
          SPEC::Expr: ScopeContextPresentable,
          Expression<LANG, SPEC>: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          PresentableArgument<LANG, SPEC>: ScopeContextPresentable {
    pub base: BasicComposerLink<Self, LANG, SPEC>,
}

impl<LANG, SPEC> VTableComposer<LANG, SPEC>
where LANG: LangFermentable,
      SPEC: Specification<LANG, Expr = Expression<LANG, SPEC>>,
      Expression<LANG, SPEC>: ScopeContextPresentable,
      Aspect<SPEC::TYC>: ScopeContextPresentable,
      PresentableArgument<LANG, SPEC>: ScopeContextPresentable {
    pub fn from_trait_path(ty_context: SPEC::TYC, attrs: &Vec<Attribute>, context: &ScopeContextLink) -> ComposerLink<Self> {
        let root = Rc::new(RefCell::new(Self {
            base: BasicComposer::from(
                AttrsModel::from(attrs),
                ty_context,
                GenModel::default(),
                constants::composer_doc(),
                Rc::clone(context)),
        }));
        {
            let mut composer = root.borrow_mut();
            composer.base.link(&root);
        }
        root

    }
}
impl<SPEC> SourceComposable for VTableComposer<RustFermentate, SPEC> where SPEC: RustSpecification {
    type Source = ScopeContext;
    type Output = BindingPresentation;

    fn compose(&self, source: &Self::Source) -> Self::Output {
        let target_type = self.present_target_aspect();
        let ffi_aspect = self.present_ffi_aspect();
        let trait_ty = self.raw_target_type_aspect().present(&source);
        // TODO: External traits
        let full_type = source.full_type_for(&trait_ty);
        let item_trait = source.maybe_item_trait(&full_type.to_path()).unwrap();
        let trait_vtable_composer = TraitVTableComposer::<RustFermentate, SPEC>::from_item_trait(&item_trait, self.type_context(), target_type.clone(), self.context());
        let mut methods_declarations = CommaPunctuated::new();
        let mut methods_implementations = Depunctuated::new();
        trait_vtable_composer
            .method_composers
            .iter()
            .for_each(|sig_composer| {
                let composer = sig_composer.borrow();
                let sig_source = composer.context().borrow();
                let sig_type_context = composer.type_context();
                let signature_context = sig_type_context.sig_context();
                let Signature { ident, output, inputs, .. } = signature_context.maybe_signature().unwrap();
                let name = Name::<RustFermentate, SPEC>::TraitImplVtableFn(trait_ty.mangle_ident_default(), sig_source.scope.to_type().mangle_ident_default());
                let compose_var = |ty: &Type| VarComposer::<RustFermentate, SPEC>::key_in_scope(ty, &sig_source.scope).compose(&sig_source).to_type();
                let (output, presentable_output_conversion) = match &output {
                    ReturnType::Default => (ReturnType::Default, SPEC::Expr::Simple(Semi::default().to_token_stream())),
                    ReturnType::Type(_, ty) => (
                        ReturnType::Type(Default::default(), Box::new(compose_var(ty))),
                        ToConversionComposer::<RustFermentate, SPEC>::new(Name::Dictionary(DictionaryName::Obj), *ty.clone(), None).compose(&sig_source)
                    )
                };
                let fn_arg_composer = |arg: &FnArg| PresentableArgument::<RustFermentate, SPEC>::Named(match arg {
                    FnArg::Receiver(Receiver { mutability: _, reference: _, attrs, .. }) =>
                        FieldComposer::self_typed(compose_var(signature_context.receiver_ty()), attrs),
                    FnArg::Typed(PatType { ty, attrs, pat, .. }) =>
                        FieldComposer::typed(Name::Pat(*pat.clone()), &compose_var(ty), true, attrs)
                }, Visibility::Inherited);
                let fn_arg_conversion_composer = |arg: &FnArg| PresentableArgument::<RustFermentate, SPEC>::AttrExpression(
                    match arg {
                        FnArg::Receiver(Receiver { mutability, reference, .. }) => {
                            let expr = match (mutability, reference) {
                                (Some(..), _) => |expr: SPEC::Expr| SPEC::Expr::AsMutRef(expr.into()),
                                (_, Some(..)) => |expr: SPEC::Expr| SPEC::Expr::AsRef(expr.into()),
                                (..) => |expr: SPEC::Expr| expr.into(),
                            };
                            match signature_context {
                                FnSignatureContext::Impl(self_ty, maybe_trait_ty, _) |
                                FnSignatureContext::TraitInner(self_ty, maybe_trait_ty, _) => expr(match maybe_trait_ty {
                                    Some(..) => from_trait_receiver_expr_composer::<RustFermentate, SPEC>(self_ty, if mutability.is_some() { quote!(mut) } else { quote!(const) }, &sig_source),
                                    None => from_receiver_expr_composer::<RustFermentate, SPEC>(self_ty, &sig_source)
                                }),
                                _ => panic!("Receiver in regular fn")
                            }
                        },
                        FnArg::Typed(PatType { ty, pat, .. }) =>
                            FromConversionFullComposer::<RustFermentate, SPEC>::key_in_scope(Name::Pat(*pat.clone()), ty, &sig_source.scope)
                                .compose(&sig_source),
                    }, SPEC::Attr::default());

                let presentable_args = CommaPunctuated::from_iter(inputs.iter().map(fn_arg_composer));
                let presentable_args_conversions = CommaPunctuated::from_iter(inputs.iter().map(fn_arg_conversion_composer));
                let args_conversions = presentable_args_conversions.present(&sig_source);
                let output_conversion = presentable_output_conversion.present(&sig_source);
                methods_declarations.push(BindingPresentation::StaticVTableInnerFnDeclaration {
                    name: name.to_token_stream(),
                    fn_name: ident.clone()
                });
                methods_implementations.push(BindingPresentation::StaticVTableInnerFn {
                    name: name.to_token_stream(),
                    args: presentable_args.present(&sig_source),
                    output,
                    body: quote! {
                        let obj = <#target_type as #trait_ty>::#ident(#args_conversions);
                        #output_conversion
                    }
                });
            });

        let trait_ident = trait_ty.mangle_ident_default();
        let name = Name::<RustFermentate, SPEC>::TraitImplVtable(ffi_aspect.mangle_ident_default(), trait_ident);
        let full_trait_path: FFIFullPath<RustFermentate, SPEC> = trait_ty.resolve(&source);
        let mut fq_trait_vtable = full_trait_path.to_type().to_path();
        fq_trait_vtable.segments.last_mut().unwrap().ident = format_ident!("{}_VTable", fq_trait_vtable.segments.last().unwrap().ident);
        BindingPresentation::StaticVTable {
            attrs: self.compose_attributes(),
            name: name.to_token_stream(),
            fq_trait_vtable: fq_trait_vtable.to_token_stream(),
            methods_declarations,
            methods_implementations,
        }
    }
}
