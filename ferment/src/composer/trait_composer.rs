use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use proc_macro2::Ident;
use syn::{Generics, ItemTrait, parse_quote, Path, TraitItem, TraitItemMethod, Type};
use crate::composer::{BraceWrapped, CommaPunctuated, constants, Depunctuated, ParentComposer, SigParentComposer, TraitParentComposer, TypeContextComposer};
use crate::composer::basic::BasicComposer;
use crate::composer::composable::{BasicComposable, SourceExpandable, NameContext, SourceAccessible};
use crate::composer::r#abstract::{Composer, ParentLinker};
use crate::composer::signature::SigComposer;
use crate::composition::{AttrsComposition, CfgAttributes, FnSignatureContext, TraitTypeDecomposition};
use crate::context::{ScopeChain, ScopeContext};
use crate::conversion::{FieldTypeConversion, FieldTypeConversionKind};
use crate::ext::{Mangle, ToPath, ToType};
use crate::naming::{DictionaryName, Name};
use crate::presentation::{DocPresentation, Expansion, FFIObjectPresentation, ScopeContextPresentable};
use crate::presentation::context::name::Context;

pub struct TraitComposer {
    pub base: BasicComposer<TraitParentComposer>,
    pub methods: Vec<SigParentComposer>,
    pub types: HashMap<Ident, TraitTypeDecomposition>,
}

impl TraitComposer {
    pub fn from_item_trait(
        item_trait: &ItemTrait,
        self_ty: Type,
        scope: &ScopeChain,
        context: &ParentComposer<ScopeContext>) -> TraitParentComposer {
        let trait_ident = &item_trait.ident;
        // let fn_name = self.ident.unwrap();
        // let mut full_fn_path = scope.joined(&trait_ident);
        // if self.scope.is_crate_based() {
        //     full_fn_path.replace_first_with(&PathHolder::from(source.scope.crate_ident().to_path()))
        // }
        let source = context.borrow();
        let mut methods = vec![];
        let mut types = HashMap::new();
        item_trait.items
            .iter()
            .for_each(|trait_item| match trait_item {
                TraitItem::Method(TraitItemMethod { sig, attrs, .. } ) => {
                    let sig_context = FnSignatureContext::TraitInner(self_ty.clone(), Some(trait_ident.to_type()), sig.clone());

                    let composer = SigComposer::with_context(
                        scope.joined_path_holder(&sig.ident).0,
                        &sig.ident,
                        sig_context,
                        &sig.generics,
                        attrs,
                        &source.scope,
                        context
                    );
                    methods.push(composer);
                    // methods.push(FnSignatureComposition::from_signature(&sig_context, sig, scope, &source));
                },
                TraitItem::Type(trait_item_type) => {
                    types.insert(trait_item_type.ident.clone(), TraitTypeDecomposition::from_item_type(trait_item_type));
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
            AttrsComposition::from(&item_trait.attrs, &item_trait.ident, scope),
            context)
    }

    fn new(
        methods: Vec<SigParentComposer>,
        types: HashMap<Ident, TraitTypeDecomposition>,
        self_path: Path,
        generics: Option<Generics>,
        attrs: AttrsComposition,
        context: &ParentComposer<ScopeContext>
    ) -> TraitParentComposer {
        let ty_context = Context::Trait { path: self_path, attrs: attrs.cfg_attributes_expanded() };
        let root = Rc::new(RefCell::new(Self {
            base: BasicComposer::from(attrs, ty_context, generics, constants::composer_doc(), Rc::clone(context)),
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
    //         AttrsComposition::from(attrs, target_name, scope),
    //         constants::composer_doc_default(),
    //         context)
    // }
}

impl NameContext for TraitComposer {
    fn name_context_ref(&self) -> &Context {
        self.base.name_context_ref()
    }
}

impl BasicComposable<TraitParentComposer> for TraitComposer {
    fn compose_attributes(&self) -> Depunctuated<Expansion> {
        self.base.compose_attributes()
    }
    fn compose_generics(&self) -> Option<Generics> {
        self.base.generics.compose(self.context())
    }
    fn compose_docs(&self) -> DocPresentation {
        DocPresentation::Direct(self.base.doc.compose(&()))
    }
}

impl SourceAccessible for TraitComposer {
    fn context(&self) -> &ParentComposer<ScopeContext> {
        self.base.context()
    }
}

impl SourceExpandable for TraitComposer {
    fn expand(&self) -> Expansion {
        let source = self.context().borrow();
        // TODO: source.scope or local_scope?
        // let self_ty = item_trait.ident.to_type();
        // let mangled_ty = self_ty.resolve(&source).mangle_ident_default();
        // let trait_decomposition = TraitDecompositionPart2::from_item_trait(item_trait, self_ty, scope.self_path_holder_ref(), context);
        let ffi_type = self.ffi_name_aspect().present(&source);
        let mangled_ty = ffi_type.mangle_ident_default();
        let vtable_name = Name::Vtable(mangled_ty.clone());
        let fields: CommaPunctuated<Expansion> = self.methods
            .iter()
            .map(|sig_composer| sig_composer.borrow().expand())
            .collect();
        let attrs = self.compose_attributes();
        Expansion::Trait {
            comment: DocPresentation::Empty,
            vtable: FFIObjectPresentation::TraitVTable {
                attrs: attrs.clone(),
                name: vtable_name.clone(),
                fields: BraceWrapped::new(fields).present(&source)
            },
            trait_object: FFIObjectPresentation::TraitObject {
                attrs,
                name: Name::TraitObj(mangled_ty),
                fields: BraceWrapped::new(CommaPunctuated::from_iter([
                    FieldTypeConversion::named(
                        Name::Dictionary(DictionaryName::Object),
                        FieldTypeConversionKind::Type(parse_quote!(*const ()))),
                    FieldTypeConversion::named(
                        Name::Dictionary(DictionaryName::Vtable),
                        FieldTypeConversionKind::Type(parse_quote!(*const #vtable_name))),
                ])).present(&source)
            }
        }
    }
}
