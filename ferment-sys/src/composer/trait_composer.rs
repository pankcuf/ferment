use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::vec;
use proc_macro2::Ident;
use quote::ToTokens;
use syn::{Generics, ItemTrait, parse_quote, TraitItem, TraitItemMethod, Lifetime};
use syn::token::Comma;
use ferment_macro::ComposerBase;
use crate::ast::{BraceWrapped, CommaPunctuated};
use crate::composable::{AttrsModel, FieldComposer, FieldTypeKind, FnSignatureContext, GenModel, LifetimesModel, TraitTypeModel};
use crate::composer::{AspectPresentable, AttrComposable, BasicComposer, BasicComposerOwner, SourceComposable, ComposerLink, DocsComposable, Linkable, SigComposer, SigComposerLink, SourceAccessible, SourceFermentable, BasicComposerLink};
use crate::context::{ScopeChain, ScopeContextLink};
use crate::ext::{Join, Mangle, ToPath, ToType};
use crate::lang::{FromDictionary, LangFermentable, RustSpecification, Specification};
use crate::presentable::{NameTreeContext, ScopeContextPresentable};
use crate::presentation::{DictionaryName, DocComposer, DocPresentation, FFIObjectPresentation, Name, RustFermentate};

#[derive(ComposerBase)]
pub struct TraitComposer<LANG, SPEC>
    where LANG: LangFermentable + 'static,
          SPEC: Specification<LANG> + 'static {
    pub base: BasicComposerLink<LANG, SPEC, Self>,
    pub methods: Vec<SigComposerLink<LANG, SPEC>>,
    #[allow(unused)]
    pub types: HashMap<Ident, TraitTypeModel>,
}

impl<LANG, SPEC> TraitComposer<LANG, SPEC>
    where LANG: LangFermentable,
          SPEC: Specification<LANG> {
    pub fn from_item_trait(
        item_trait: &ItemTrait,
        ty_context: SPEC::TYC,
        scope: &ScopeChain,
        scope_context: &ScopeContextLink) -> ComposerLink<Self> {
        let ItemTrait { attrs, generics, ident, items,  .. } = item_trait;
        let self_ty = ident.to_type();
        let source = scope_context.borrow();
        let mut methods = vec![];
        let mut types = HashMap::new();
        items
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
                    methods.push(SigComposer::from_trait_item_method(trait_item_method, ty_context, &method_scope_context));
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
            Some(generics.clone()),
            vec![],
            AttrsModel::from(attrs),
            scope_context)
    }

    fn new(
        methods: Vec<SigComposerLink<LANG, SPEC>>,
        types: HashMap<Ident, TraitTypeModel>,
        ty_context: SPEC::TYC,
        generics: Option<Generics>,
        lifetimes: Vec<Lifetime>,
        attrs: AttrsModel,
        context: &ScopeContextLink
    ) -> ComposerLink<Self> {
        let root = Rc::new(RefCell::new(Self {
            base: BasicComposer::from(DocComposer::new(ty_context.to_token_stream()), attrs, ty_context, GenModel::new(generics.clone()), LifetimesModel::new(lifetimes), Rc::clone(context)),
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
          SPEC: Specification<LANG> {
    fn compose_docs(&self) -> DocPresentation {
        DocPresentation::Direct(self.base.doc.compose(self.context()))
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
                            SPEC::Name::dictionary_name(DictionaryName::Object),
                            FieldTypeKind::Type(parse_quote!(*const ()))),
                        FieldComposer::<RustFermentate, SPEC>::named(
                            SPEC::Name::dictionary_name(DictionaryName::Vtable),
                            FieldTypeKind::Type(parse_quote!(*const #vtable_name))),
                    ])).present(&self.context().borrow())
                }
            }
    }
}

