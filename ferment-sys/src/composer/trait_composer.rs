use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::Debug;
use std::rc::Rc;
use std::vec;
use proc_macro2::Ident;
use quote::ToTokens;
use syn::{Generics, ItemTrait, parse_quote, TraitItem, TraitItemMethod};
use syn::token::Comma;
use ferment_macro::ComposerBase;
use crate::ast::{BraceWrapped, CommaPunctuated};
use crate::composable::{AttrsModel, FieldComposer, FieldTypeKind, FnSignatureContext, GenModel, TraitTypeModel};
use crate::composer::{AspectPresentable, AttrComposable, BasicComposer, BasicComposerOwner, SourceComposable, ComposerLink, constants, DocsComposable, Linkable, SigComposer, SigComposerLink, SourceAccessible, SourceFermentable, BasicComposerLink};
use crate::context::{ScopeChain, ScopeContextLink};
use crate::ext::{Join, Mangle, ToPath, ToType};
use crate::lang::{LangFermentable, RustSpecification, Specification};
use crate::presentable::{Aspect, NameTreeContext, PresentableArgument, ScopeContextPresentable, PresentableSequence, Expression};
use crate::presentation::{DictionaryName, DocPresentation, FFIObjectPresentation, Name, RustFermentate};

#[derive(ComposerBase)]
pub struct TraitComposer<LANG, SPEC>
    where LANG: LangFermentable + 'static,
          SPEC: Specification<LANG, Attr: Debug, Expr=Expression<LANG, SPEC>, Var: ToType> + 'static,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          PresentableArgument<LANG, SPEC>: ScopeContextPresentable {
    pub base: BasicComposerLink<Self, LANG, SPEC>,
    pub methods: Vec<SigComposerLink<LANG, SPEC>>,
    #[allow(unused)]
    pub types: HashMap<Ident, TraitTypeModel>,
}

impl<LANG, SPEC> TraitComposer<LANG, SPEC>
    where LANG: LangFermentable,
          SPEC: Specification<LANG, Attr: Debug, Expr=Expression<LANG, SPEC>, Var: ToType>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          PresentableSequence<LANG, SPEC>: ScopeContextPresentable,
          PresentableArgument<LANG, SPEC>: ScopeContextPresentable,
          Self: AspectPresentable<SPEC::TYC> {
    pub fn from_item_trait(
        item_trait: &ItemTrait,
        ty_context: SPEC::TYC,
        scope: &ScopeChain,
        scope_context: &ScopeContextLink) -> ComposerLink<Self> {
        let ItemTrait { ident, .. } = item_trait;
        let self_ty = ident.to_type();
        // let trait_ident = &item_trait.ident;
        // let fn_name = self.ident.unwrap();
        // let mut full_fn_path = scope.joined(&trait_ident);
        // if self.scope.is_crate_based() {
        //     full_fn_path.replace_first_with(&PathHolder::from(source.scope.crate_ident().to_path()))
        // }
        let source = scope_context.borrow();
        let mut methods = vec![];
        let mut types = HashMap::new();
        item_trait.items
            .iter()
            .for_each(|trait_item| match trait_item {
                TraitItem::Method(trait_item_method) => {
                    let TraitItemMethod { sig, attrs, .. } = trait_item_method;
                    let sig_context = FnSignatureContext::TraitInner(self_ty.clone(), Some(self_ty.clone()), sig.clone());
                    let method_scope_context = Rc::new(RefCell::new(source.joined(trait_item_method)));
                    let ty_context = ty_context.join_fn(
                        scope.joined_path_holder(&sig.ident).0,
                        sig_context,
                        attrs.clone());
                    methods.push(
                        SigComposer::with_context(
                            ty_context,
                            &sig.generics,
                            attrs,
                            &method_scope_context));
                },
                TraitItem::Type(trait_item_type) => {
                    types.insert(trait_item_type.ident.clone(), TraitTypeModel::from_item_type(trait_item_type));
                },
                _ => {}
            });
        Self::new(
            methods,
            types,
            ty_context,
            Some(item_trait.generics.clone()),
            AttrsModel::from(&item_trait.attrs),
            scope_context)
    }

    fn new(
        methods: Vec<SigComposerLink<LANG, SPEC>>,
        types: HashMap<Ident, TraitTypeModel>,
        ty_context: SPEC::TYC,
        generics: Option<Generics>,
        attrs: AttrsModel,
        context: &ScopeContextLink
    ) -> ComposerLink<Self> {
        // let ty_context = Context::Trait { path: self_path, attrs: attrs.cfg_attributes() };
        let root = Rc::new(RefCell::new(Self {
            base: BasicComposer::from(attrs, ty_context, GenModel::new(generics.clone()), constants::composer_doc(), Rc::clone(context)),
            methods,
            types,
        }));
        {
            let mut composer = root.borrow_mut();
            composer.base.link(&root);
        }
        root
    }
}

impl<LANG, SPEC> DocsComposable for TraitComposer<LANG, SPEC>
    where LANG: LangFermentable,
          SPEC: Specification<LANG, Attr: Debug, Expr=Expression<LANG, SPEC>, Var: ToType>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          PresentableArgument<LANG, SPEC>: ScopeContextPresentable {
    fn compose_docs(&self) -> DocPresentation {
        DocPresentation::Direct(self.base.doc.compose(&()))
    }
}

impl<SPEC> SourceFermentable<RustFermentate> for TraitComposer<RustFermentate, SPEC>
    where SPEC: RustSpecification {
    fn ferment(&self) -> RustFermentate {
        // TODO: source.scope or local_scope?
        let attrs = self.compose_attributes();
        let ffi_type = self.present_ffi_aspect();
        let mangled_ty = ffi_type.mangle_ident_default();
        let vtable_name = Name::<RustFermentate, SPEC>::Vtable(mangled_ty.clone());
        RustFermentate::Trait {
            comment: DocPresentation::Empty,
            vtable: FFIObjectPresentation::TraitVTable {
                attrs: attrs.clone(),
                name: vtable_name.to_path(),
                fields: BraceWrapped::<_, Comma>::new(
                    CommaPunctuated::from_iter(
                        self.methods.iter()
                            .map(|composer| composer.borrow().ferment())))
                    .to_token_stream()
            },
            trait_object: FFIObjectPresentation::TraitObject {
                attrs,
                name: Name::<RustFermentate, SPEC>::TraitObj(mangled_ty).to_path(),
                fields: BraceWrapped::new(
                    CommaPunctuated::from_iter([
                        FieldComposer::<RustFermentate, SPEC>::named(
                            Name::Dictionary(DictionaryName::Object),
                            FieldTypeKind::Type(parse_quote!(*const ()))),
                        FieldComposer::<RustFermentate, SPEC>::named(
                            Name::Dictionary(DictionaryName::Vtable),
                            FieldTypeKind::Type(parse_quote!(*const #vtable_name))),
                    ])).present(&self.context().borrow())
                }
            }
    }
}

