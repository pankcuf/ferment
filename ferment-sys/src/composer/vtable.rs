use std::cell::RefCell;
use std::rc::Rc;
use quote::{format_ident, quote, ToTokens};
use syn::{Attribute, FnArg, PatType, Receiver, ReturnType, Signature, Visibility};
use syn::token::Semi;
use ferment_macro::ComposerBase;
use crate::ast::{CommaPunctuated, Depunctuated};
use crate::composable::{AttrsModel, GenModel, LifetimesModel};
use crate::composer::{BasicComposerOwner, AspectPresentable, BasicComposer, BasicComposerLink, ComposerLink, Linkable, SourceComposable, TypeAspect, VarComposer, FromConversionFullComposer, ToConversionComposer, AttrComposable, VariableComposer, SigComposerLink, SourceAccessible};
use crate::context::{ScopeContext, ScopeContextLink};
use crate::ext::{Mangle, Resolve, ToPath, ToType};
use crate::lang::{FromDictionary, LangFermentable, RustSpecification, Specification};
use crate::presentable::{Expression, ScopeContextPresentable};
use crate::presentation::{ArgPresentation, BindingPresentation, DictionaryExpr, DictionaryName, DocComposer, FFIFullPath, Name, RustFermentate};

#[derive(ComposerBase)]
pub struct VTableComposer<LANG, SPEC>
    where LANG: LangFermentable + 'static,
          SPEC: Specification<LANG> + 'static {
    pub base: BasicComposerLink<LANG, SPEC, Self>,
    pub vtable_method_composers: Vec<SigComposerLink<LANG, SPEC>>,
}

impl<LANG, SPEC> VTableComposer<LANG, SPEC>
where LANG: LangFermentable,
      SPEC: Specification<LANG> {
    pub fn from_trait_path(ty_context: SPEC::TYC, attrs: &Vec<Attribute>, vtable_method_composers: Vec<SigComposerLink<LANG, SPEC>>, context: ScopeContextLink) -> ComposerLink<Self> {
        let root = Rc::new(RefCell::new(Self {
            base: BasicComposer::from(
                DocComposer::new(ty_context.to_token_stream()),
                AttrsModel::from(attrs),
                ty_context,
                GenModel::default(),
                LifetimesModel::default(),
                // LinkedContextComposer::new(default_doc, Self::target_type_aspect),
                context),
            vtable_method_composers
        }));
        {
            let mut composer = root.borrow_mut();
            composer.base.link(&root);
        }
        root

    }
}
impl<SPEC> SourceComposable for VTableComposer<RustFermentate, SPEC>
    where SPEC: RustSpecification {
    type Source = ScopeContext;
    type Output = BindingPresentation;

    fn compose(&self, source: &Self::Source) -> Self::Output {
        let target_type = self.present_target_aspect();
        let ffi_aspect = self.present_ffi_aspect();
        let trait_ty = self.raw_target_type_aspect().present(&source);
        // TODO: External traits
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
                let sig_context = method_ty_context.sig_context();
                let sig = sig_context.maybe_signature().unwrap();
                let Signature { ident, output, inputs, .. } = sig;
                let name = Name::<RustFermentate, SPEC>::TraitImplVtableFn(trait_ty.mangle_ident_default(), method_scope_context.scope.to_type().mangle_ident_default());
                let mut args = CommaPunctuated::new();
                let mut args_conversions = CommaPunctuated::new();
                inputs.iter().for_each(|arg| {
                    match arg {
                        FnArg::Receiver(Receiver { mutability, reference, attrs, .. }) => {
                            args.push(ArgPresentation::field(
                                attrs,
                                Visibility::Inherited,
                                Some(Name::<RustFermentate, SPEC>::dictionary_name(DictionaryName::Self_).mangle_ident_default()),
                                VariableComposer::<RustFermentate, SPEC>::from(sig_context.receiver_ty())
                                    .compose(&method_scope_context)
                                    .to_type()
                            ));
                            args_conversions.push(ArgPresentation::expr(attrs, match (mutability, reference) {
                                (Some(..), _) => SPEC::Expr::AsMutRef(Expression::dict_expr(DictionaryExpr::SelfAsTrait(full_target_type.to_token_stream(), quote!(mut))).into()),
                                (_, Some(..)) => SPEC::Expr::AsRef(Expression::dict_expr(DictionaryExpr::SelfAsTrait(full_target_type.to_token_stream(), quote!(const))).into()),
                                (..) => Expression::dict_expr(DictionaryExpr::SelfAsTrait(full_target_type.to_token_stream(), quote!(const))).into(),
                            }.present(source)));

                        },
                        FnArg::Typed(PatType { ty, attrs, pat, .. }) => {
                            let var = VarComposer::<RustFermentate, SPEC>::key_in_scope(ty, &method_scope_context.scope)
                                .compose(&method_scope_context)
                                .to_type();
                            args.push(ArgPresentation::field(attrs, Visibility::Inherited, Some(Name::<RustFermentate, SPEC>::Pat(*pat.clone()).mangle_ident_default()), var));
                            args_conversions.push(ArgPresentation::expr(attrs, FromConversionFullComposer::<RustFermentate, SPEC>::key_in_scope(Name::Pat(*pat.clone()), ty, &method_scope_context.scope)
                                .compose(&method_scope_context).present(source)));
                        }
                    }
                });

                let (out, presentable_output_conversion) = match &output {
                    ReturnType::Default => (ReturnType::Default, SPEC::Expr::Simple(Semi::default().to_token_stream())),
                    ReturnType::Type(_, ty) => {
                        let var = VarComposer::<RustFermentate, SPEC>::key_in_scope(ty, &method_scope_context.scope)
                            .compose(&method_scope_context)
                            .to_type();
                        (
                            ReturnType::Type(Default::default(), Box::new(var)),
                            ToConversionComposer::<RustFermentate, SPEC>::new(SPEC::Name::dictionary_name(DictionaryName::Obj), *ty.clone(), None)
                                .compose(&method_scope_context)
                        )
                    }
                };
                let output_conversion = presentable_output_conversion.present(&method_scope_context);
                methods_declarations.push(BindingPresentation::StaticVTableInnerFnDeclaration {
                    name: name.to_token_stream(),
                    fn_name: ident.clone()
                });
                methods_implementations.push(BindingPresentation::StaticVTableInnerFn {
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
        let name = Name::<RustFermentate, SPEC>::TraitImplVtable(ffi_aspect.mangle_ident_default(), trait_ident);
        let full_trait_path: FFIFullPath<RustFermentate, SPEC> = trait_ty.resolve(&source);
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
                    attrs: attrs.clone(),
                    name: Name::<RustFermentate, SPEC>::TraitFn(target_type.clone(), full_trait_type.clone()).to_token_stream(),
                    item_type: target_type.clone(),
                    trait_type: full_trait_type.to_token_stream(),
                    vtable_name: name.to_token_stream(),
                },
                BindingPresentation::ObjAsTraitDestructor {
                    attrs,
                    name: Name::<RustFermentate, SPEC>::TraitDestructor(target_type.clone(), full_trait_type.clone()).to_token_stream(),
                    item_type: target_type.to_token_stream(),
                    trait_type: full_trait_type.to_token_stream(),
                    generics: None,
                }
            ])
        };
        result
    }
}
