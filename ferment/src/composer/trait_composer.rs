use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::vec;
use proc_macro2::Ident;
use quote::ToTokens;
use syn::{Attribute, Generics, ItemTrait, parse_quote, Path, TraitItem, TraitItemMethod};
use syn::token::Comma;
use crate::ast::{BraceWrapped, CommaPunctuated, Depunctuated};
use crate::composable::{AttrsModel, CfgAttributes, FieldComposer, FieldTypeKind, FnSignatureContext, GenModel, TraitTypeModel};
use crate::composer::{AttrComposable, BasicComposer, BasicComposerOwner, Composer, constants, DocsComposable, Linkable, NameComposable, ComposerLink, SigComposer, SigComposerLink, SourceAccessible, SourceFermentable2, TraitComposerLink, GenericsComposable, NameContext};
use crate::context::{ScopeChain, ScopeContext};
use crate::ext::{Join, Mangle, ToPath, ToType};
use crate::lang::{LangAttrSpecification, LangGenSpecification};
use crate::presentable::{Context, OwnedItemPresentableContext, ScopeContextPresentable, SequenceOutput};
use crate::presentation::{DictionaryName, DocPresentation, RustFermentate, FFIObjectPresentation, Name};

// #[derive(BasicComposerOwner)]
pub struct TraitComposer<LANG, SPEC, Gen>
    where LANG: Clone + 'static,
          SPEC: LangAttrSpecification<LANG> + 'static,
          Gen: LangGenSpecification<LANG> + 'static {
    pub base: BasicComposer<TraitComposerLink<LANG, SPEC, Gen>, LANG, SPEC, Gen>,
    pub methods: Vec<SigComposerLink<LANG, SPEC, Gen>>,
    #[allow(unused)]
    pub types: HashMap<Ident, TraitTypeModel>,
}

impl<LANG, SPEC, Gen> BasicComposerOwner<Context, LANG, SPEC, Gen> for TraitComposer<LANG, SPEC, Gen>
    where LANG: Clone,
          SPEC: LangAttrSpecification<LANG>,
          Gen: LangGenSpecification<LANG> {
    fn base(&self) -> &BasicComposer<ComposerLink<Self>, LANG, SPEC, Gen> {
        &self.base
    }
}

impl<LANG, SPEC, Gen> TraitComposer<LANG, SPEC, Gen>
    where LANG: Clone,
          SPEC: LangAttrSpecification<LANG>,
          Gen: LangGenSpecification<LANG>,
          SequenceOutput<LANG, SPEC>: ScopeContextPresentable,
          OwnedItemPresentableContext<LANG, SPEC>: ScopeContextPresentable {
    pub fn from_item_trait(
        item_trait: &ItemTrait,
        scope: &ScopeChain,
        scope_context: &ComposerLink<ScopeContext>) -> ComposerLink<Self> {
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
                    let composer = SigComposer::with_context(
                        scope.joined_path_holder(&sig.ident).0,
                        sig_context,
                        &sig.generics,
                        attrs,
                        &method_scope_context
                    );
                    methods.push(composer);
                    // methods.push(FnSignatureComposition::from_signature(&sig_context, sig, scope, &source));
                },
                TraitItem::Type(trait_item_type) => {
                    types.insert(trait_item_type.ident.clone(), TraitTypeModel::from_item_type(trait_item_type));
                },
                // TraitItem::Const(TraitItemConst { attrs, const_token, ident, colon_token, ty, default, semi_token }) => {
                //
                // },
                _ => {}
            });
        // let self_ty = item_trait.ident.to_type();
        // let mangled_ty = self_ty.resolve(&source).mangle_ident_default();

        Self::new(
            methods,
            types,
            item_trait.ident.to_path(),
            Some(item_trait.generics.clone()),
            AttrsModel::from(&item_trait.attrs),
            scope_context)
    }

    fn new(
        methods: Vec<SigComposerLink<LANG, SPEC, Gen>>,
        types: HashMap<Ident, TraitTypeModel>,
        self_path: Path,
        generics: Option<Generics>,
        attrs: AttrsModel,
        context: &ComposerLink<ScopeContext>
    ) -> ComposerLink<Self> {
        let ty_context = Context::Trait { path: self_path, attrs: attrs.cfg_attributes() };
        let root = Rc::new(RefCell::new(Self {
            base: BasicComposer::<ComposerLink<Self>, LANG, SPEC, Gen>::from(attrs, ty_context, GenModel::new(generics.clone()), constants::composer_doc(), Rc::clone(context)),
            methods,
            types,
        }));
        {
            let mut composer = root.borrow_mut();
            composer.base.link(&root);
        }
        root
    }
    // pub fn with_context(
    //     path: Path,
    //     target_name: &Ident,
    //     sig_context: FnSignatureContext,
    //     generics: &Generics,
    //     attrs: &Vec<Attribute>,
    //     scope: &ScopeChain,
    //     context: &ParentComposer<ScopeContext>
    // ) -> TraitParentComposer {
    //     Self::new(
    //         path,
    //         sig_context,
    //         Some(generics.clone()),
    //         AttrsModel::from(attrs, target_name, scope),
    //         constants::composer_doc_default(),
    //         context)
    // }
}

impl<LANG, SPEC, Gen> DocsComposable for TraitComposer<LANG, SPEC, Gen>
    where LANG: Clone,
          SPEC: LangAttrSpecification<LANG>,
          Gen: LangGenSpecification<LANG> {
    fn compose_docs(&self) -> DocPresentation {
        DocPresentation::Direct(self.base.doc.compose(&()))
    }
}
impl<LANG, SPEC, Gen> AttrComposable<SPEC> for TraitComposer<LANG, SPEC, Gen>
    where LANG: Clone + 'static,
          SPEC: LangAttrSpecification<LANG> + 'static,
          Gen: LangGenSpecification<LANG> + 'static,
          SequenceOutput<LANG, SPEC>: ScopeContextPresentable,
          OwnedItemPresentableContext<LANG, SPEC>: ScopeContextPresentable {
    fn compose_attributes(&self) -> SPEC {
        self.base().compose_attributes()
    }
}
impl<LANG, SPEC, Gen> GenericsComposable<Gen> for TraitComposer<LANG, SPEC, Gen>
    where LANG: Clone + 'static,
          SPEC: LangAttrSpecification<LANG> + 'static,
          Gen: LangGenSpecification<LANG> + 'static,
          SequenceOutput<LANG, SPEC>: ScopeContextPresentable,
          OwnedItemPresentableContext<LANG, SPEC>: ScopeContextPresentable {
    fn compose_generics(&self) -> Gen {
        self.base().compose_generics()
    }
}
impl<LANG, SPEC, Gen> SourceAccessible for TraitComposer<LANG, SPEC, Gen>
    where LANG: Clone,
          SPEC: LangAttrSpecification<LANG>,
          Gen: LangGenSpecification<LANG>,
          SequenceOutput<LANG, SPEC>: ScopeContextPresentable,
          OwnedItemPresentableContext<LANG, SPEC>: ScopeContextPresentable {
    fn context(&self) -> &ComposerLink<ScopeContext> {
        self.base().context()
    }
}

impl<LANG, SPEC, Gen> NameContext<Context> for TraitComposer<LANG, SPEC, Gen>
    where LANG: Clone,
          SPEC: LangAttrSpecification<LANG>,
          Gen: LangGenSpecification<LANG>,
          SequenceOutput<LANG, SPEC>: ScopeContextPresentable,
          OwnedItemPresentableContext<LANG, SPEC>: ScopeContextPresentable {
    fn name_context_ref(&self) -> &Context {
        self.base().name_context_ref()
    }
}



impl SourceFermentable2<RustFermentate> for TraitComposer<RustFermentate, Vec<Attribute>, Option<Generics>> {
    fn ferment(&self) -> Depunctuated<RustFermentate> {
        let source = self.context().borrow();
        // TODO: source.scope or local_scope?
        // let self_ty = item_trait.ident.to_type();
        // let mangled_ty = self_ty.resolve(&source).mangle_ident_default();
        // let trait_decomposition = TraitDecompositionPart2::from_item_trait(item_trait, self_ty, scope.self_path_holder_ref(), context);
        // let attrs = <Self as BasicComposable<TraitParentComposer, Context, Vec<Attribute>, Option<Generics>>>::compose_attributes(self);
        let attrs = self.compose_attributes();
        // let ffi_type = <Self as NameContext<Context>>::ffi_name_aspect(self).present(&source);
        let ffi_type = self.compose_ffi_name();


        let mangled_ty = ffi_type.mangle_ident_default();
        let vtable_name = Name::Vtable(mangled_ty.clone());
        let mut rust_fields = CommaPunctuated::<RustFermentate>::new();
        self.methods
            .iter()
            .for_each(|sig_composer| {
                let fermentate = sig_composer.borrow().ferment();
                rust_fields.extend(fermentate);
                // fermentate
            });
        Depunctuated::from_iter([
            // Fermentate::Rust(
                RustFermentate::Trait {
                    comment: DocPresentation::Empty,
                    vtable: FFIObjectPresentation::TraitVTable {
                        attrs: attrs.clone(),
                        name: vtable_name.clone(),
                        fields: BraceWrapped::<_, Comma>::new(rust_fields).to_token_stream()
                    },
                    trait_object: FFIObjectPresentation::TraitObject {
                        attrs,
                        name: Name::TraitObj(mangled_ty),
                        fields: BraceWrapped::new(CommaPunctuated::from_iter([
                            FieldComposer::<RustFermentate, Vec<Attribute>>::named(
                                Name::Dictionary(DictionaryName::Object),
                                FieldTypeKind::Type(parse_quote!(*const ()))),
                            FieldComposer::<RustFermentate, Vec<Attribute>>::named(
                                Name::Dictionary(DictionaryName::Vtable),
                                FieldTypeKind::Type(parse_quote!(*const #vtable_name))),
                        ])).present(&source)
                    }
                }
            // )
        ])
    }
}
