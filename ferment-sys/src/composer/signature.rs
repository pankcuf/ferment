use std::cell::RefCell;
use std::rc::Rc;
use quote::ToTokens;
use syn::{parse_quote, Attribute, Field, FieldMutability, Generics, ImplItemFn, ItemFn, Lifetime, Signature, TraitItemFn, Type, TypePtr, Visibility};
use ferment_macro::ComposerBase;
use crate::composable::{AttrsModel, GenModel, LifetimesModel};
use crate::composer::{BasicComposer, BasicComposerLink, BasicComposerOwner, ComposerLink, DocsComposable, Linkable, SourceAccessible, SourceComposable};
use crate::context::{ScopeContext, ScopeContextLink};
use crate::ext::{FFITypeResolve, ItemExtension, Mangle};
use crate::lang::Specification;
use crate::presentation::{DocComposer, DocPresentation};

#[derive(ComposerBase)]
pub struct SigComposer<SPEC>
    where SPEC: Specification + 'static {
    pub base: BasicComposerLink<SPEC, Self>,
}

impl<SPEC> SigComposer<SPEC>
    where SPEC: Specification {

    fn new(
        ty_context: SPEC::TYC,
        generics: Option<Generics>,
        lifetimes: Vec<Lifetime>,
        attrs: AttrsModel,
        context: &ScopeContextLink) -> ComposerLink<Self> {
        let root = Rc::new(RefCell::new(Self {
            base: BasicComposer::from(DocComposer::new(ty_context.to_token_stream()), attrs, ty_context, GenModel::new(generics), LifetimesModel::new(lifetimes), Rc::clone(context)),
        }));
        {
            let mut composer = root.borrow_mut();
            composer.base.link(&root);
        }
        root
    }
    pub fn with_context(
        ty_context: SPEC::TYC,
        generics: &Generics,
        lifetimes: &Vec<Lifetime>,
        attrs: &Vec<Attribute>,
        context: &ScopeContextLink
    ) -> ComposerLink<Self> {
        Self::new(
            ty_context,
            Some(generics.clone()),
            lifetimes.clone(),
            AttrsModel::from(attrs),
            context)
    }
    pub fn from_item_fn(
        item_fn: &ItemFn,
        ty_context: SPEC::TYC,
        context: &ScopeContextLink
    ) -> ComposerLink<Self> {
        let ItemFn { attrs, sig: Signature { generics, ..}, .. } = item_fn;
        Self::with_context(
            ty_context,
            generics,
            &vec![],
            attrs,
            context)
    }
    pub fn from_type_bare_fn(
        ty_context: SPEC::TYC,
        generics: &Generics,
        lifetimes: &Vec<Lifetime>,
        attrs: &Vec<Attribute>,
        context: &ScopeContextLink
    ) -> ComposerLink<Self> {
        Self::with_context(
            ty_context,
            generics,
            lifetimes,
            attrs,
            context
        )
    }

    pub fn from_impl_item_method(
        impl_item_method: &ImplItemFn,
        ty_context: SPEC::TYC,
        context: &ScopeContextLink
    ) -> ComposerLink<Self> {
        let ImplItemFn { sig, .. } = impl_item_method;
        Self::with_context(
            ty_context,
            &sig.generics,
            &vec![],
            sig.maybe_attrs().unwrap_or(&vec![]),
            context
        )
    }
    pub fn from_trait_item_method(
        trait_item_method: &TraitItemFn,
        ty_context: SPEC::TYC,
        context: &ScopeContextLink
    ) -> ComposerLink<Self> {
        let TraitItemFn { sig, attrs, .. } = trait_item_method;
        Self::with_context(ty_context, &sig.generics, &vec![], attrs, context)
    }
}

impl<SPEC> DocsComposable for SigComposer<SPEC>
    where SPEC: Specification {
    fn compose_docs(&self) -> DocPresentation {
        DocPresentation::Direct(self.base.doc.compose(self.context()))
    }
}


pub fn field<SPEC>(name: SPEC::Name , ty: &Type, source: &ScopeContext) -> Field
    where SPEC: Specification {
    Field {
        attrs: vec![],
        vis: Visibility::Inherited,
        mutability: FieldMutability::None,
        ident: Some(name.mangle_ident_default()),
        colon_token: None,
        ty: match ty {
            Type::Ptr(TypePtr { const_token, mutability, .. }) => {
                let ty = ty.full_type(source);
                if const_token.is_some() {
                    parse_quote!(*const #ty)
                } else if mutability.is_some() {
                    parse_quote!(*mut #ty)
                } else {
                    ty
                }
            },
            _ => ty.full_type(source)
        },
    }
}
