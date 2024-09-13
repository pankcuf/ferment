use proc_macro2::Ident;
use syn::{Attribute, Fields, Generics, ItemEnum, ItemFn, ItemImpl, ItemStruct, ItemTrait, ItemType, Type};
use syn::punctuated::Punctuated;
use syn::token::{Brace, Paren};
use crate::ast::Depunctuated;
use crate::composable::AttrsModel;
use crate::composer::{AttrComposable, EnumComposer, EnumComposerLink, FFIAspect, ImplComposer, ImplComposerLink, ItemComposer, ItemComposerLink, OpaqueItemComposer, OpaqueItemComposerLink, ComposerLink, SigComposer, SigComposerLink, SourceFermentable2, TraitComposer, TraitComposerLink, Composer};
use crate::context::{ScopeChain, ScopeContext};
use crate::lang::{LangAttrSpecification, LangGenSpecification};
use crate::presentable::{BindingPresentableContext, OwnedItemPresentableContext, ScopeContextPresentable, SequenceOutput};
use crate::presentation::RustFermentate;

#[allow(unused)]
pub enum ItemComposerWrapper<LANG, SPEC, Gen>
    where LANG: Clone + 'static,
          SPEC: LangAttrSpecification<LANG> + 'static,
          Gen: LangGenSpecification<LANG> + 'static,
          SequenceOutput<LANG, SPEC>: ScopeContextPresentable,
          OwnedItemPresentableContext<LANG, SPEC>: ScopeContextPresentable {
    Enum(EnumComposerLink<LANG, SPEC, Gen>),
    EnumVariantNamed(ItemComposerLink<Brace, LANG, SPEC, Gen>),
    EnumVariantUnnamed(ItemComposerLink<Paren, LANG, SPEC, Gen>),
    EnumVariantUnit(ItemComposerLink<Brace, LANG, SPEC, Gen>),
    StructNamed(ItemComposerLink<Brace, LANG, SPEC, Gen>),
    StructUnnamed(ItemComposerLink<Paren, LANG, SPEC, Gen>),
    OpaqueStructNamed(OpaqueItemComposerLink<Brace, LANG, SPEC, Gen>),
    OpaqueStructUnnamed(OpaqueItemComposerLink<Paren, LANG, SPEC, Gen>),
    Sig(SigComposerLink<LANG, SPEC, Gen>),
    TypeAlias(ItemComposerLink<Paren, LANG, SPEC, Gen>),
    Trait(TraitComposerLink<LANG, SPEC, Gen>),
    Impl(ImplComposerLink<LANG, SPEC, Gen>),
}

impl<LANG, SPEC, Gen> ItemComposerWrapper<LANG, SPEC, Gen>
    where LANG: Clone,
          SPEC: LangAttrSpecification<LANG>,
          Gen: LangGenSpecification<LANG>,
          SequenceOutput<LANG, SPEC>: ScopeContextPresentable,
          OwnedItemPresentableContext<LANG, SPEC>: ScopeContextPresentable,
          // Self: SourceFermentable2<LANG>
{

    pub fn r#trait(item_trait: &ItemTrait, scope: &ScopeChain, context: &ComposerLink<ScopeContext>) -> Self {
        ItemComposerWrapper::Trait(TraitComposer::from_item_trait(item_trait, scope, context))
    }
    pub fn r#impl(item_impl: &ItemImpl, scope: &ScopeChain, context: &ComposerLink<ScopeContext>) -> Self {
        ItemComposerWrapper::Impl(ImplComposer::from_item_impl(item_impl, scope, context))
    }

    pub fn r#fn(item_fn: &ItemFn, scope: &ScopeChain, context: &ComposerLink<ScopeContext>) -> Self {
        ItemComposerWrapper::Sig(SigComposer::from_item_fn(item_fn, scope, context))
    }

    pub fn r#enum(item_enum: &ItemEnum, context: &ComposerLink<ScopeContext>) -> Self {
        ItemComposerWrapper::<LANG, SPEC, Gen>::Enum(EnumComposer::<LANG, SPEC, Gen>::new(item_enum, context))
    }
    pub fn r#type(item_type: &ItemType, scope: &ScopeChain, context: &ComposerLink<ScopeContext>) -> Self {
        let ItemType { ident: target_name, ty, attrs, generics, .. } = item_type;
        match &**ty {
            Type::BareFn(type_bare_fn) =>
                ItemComposerWrapper::Sig(SigComposer::from_type_bare_fn(type_bare_fn, target_name, generics, attrs, scope, context)),
            _ =>
                ItemComposerWrapper::TypeAlias(ItemComposer::<Paren, LANG, SPEC, Gen>::type_alias_composer(target_name, ty, generics, attrs, context))
        }
    }
    pub fn variant(fields: &Fields, target_name: &Ident, variant_name: &Ident, attrs: &Vec<Attribute>, context: &ComposerLink<ScopeContext>) -> Self {
        let attrs = AttrsModel::from(attrs);
        match fields {
            Fields::Unit =>
                ItemComposerWrapper::EnumVariantUnit(ItemComposer::enum_variant_composer_unit(target_name, variant_name, attrs, &Punctuated::new(), context)),
            Fields::Unnamed(fields) =>
                ItemComposerWrapper::EnumVariantUnnamed(ItemComposer::enum_variant_composer_unnamed(target_name, variant_name, attrs, &fields.unnamed, context)),
            Fields::Named(fields) =>
                ItemComposerWrapper::EnumVariantUnit(ItemComposer::enum_variant_composer_named(target_name, variant_name, attrs, &fields.named, context)),
        }
    }

    pub fn r#struct(item_struct: &ItemStruct, context: &ComposerLink<ScopeContext>) -> Self {
        let ItemStruct { attrs, fields: ref f, ident: target_name, generics, .. } = item_struct;
        match f {
            Fields::Unnamed(ref fields) =>
                ItemComposerWrapper::StructUnnamed(ItemComposer::<Paren, LANG, SPEC, Gen>::struct_composer_unnamed(target_name, attrs, generics, &fields.unnamed, context)),
            Fields::Named(ref fields) =>
                ItemComposerWrapper::StructNamed(ItemComposer::<Brace, LANG, SPEC, Gen>::struct_composer_named(target_name, attrs, generics, &fields.named, context)),
            Fields::Unit =>
                ItemComposerWrapper::StructNamed(ItemComposer::<Brace, LANG, SPEC, Gen>::struct_composer_named(target_name, attrs, generics, &Punctuated::new(), context)),
        }
    }
    pub fn opaque_struct(item_struct: &ItemStruct, context: &ComposerLink<ScopeContext>) -> Self {
        let ItemStruct { attrs, fields: ref f, ident: target_name, generics, .. } = item_struct;
        match f {
            Fields::Unnamed(ref fields) =>
                ItemComposerWrapper::OpaqueStructUnnamed(OpaqueItemComposer::<Paren, LANG, SPEC, Gen>::struct_composer_unnamed(target_name, attrs, generics, &fields.unnamed, context)),
            Fields::Named(ref fields) =>
                ItemComposerWrapper::OpaqueStructNamed(OpaqueItemComposer::<Brace, LANG, SPEC, Gen>::struct_composer_named(target_name, attrs, generics, &fields.named, context)),
            Fields::Unit =>
                ItemComposerWrapper::OpaqueStructNamed(OpaqueItemComposer::<Brace, LANG, SPEC, Gen>::struct_composer_named(target_name, attrs, generics, &Punctuated::new(), context))
        }
    }

    pub fn compose_aspect(&self, aspect: FFIAspect) -> SequenceOutput<LANG, SPEC> {
        match self {
            ItemComposerWrapper::EnumVariantNamed(composer) =>
                composer.borrow().compose_aspect(aspect),
            ItemComposerWrapper::EnumVariantUnnamed(composer) =>
                composer.borrow().compose_aspect(aspect),
            ItemComposerWrapper::EnumVariantUnit(composer) =>
                composer.borrow().compose_aspect(aspect),
            ItemComposerWrapper::StructNamed(composer) =>
                composer.borrow().compose_aspect(aspect),
            ItemComposerWrapper::StructUnnamed(composer) =>
                composer.borrow().compose_aspect(aspect),
            ItemComposerWrapper::TypeAlias(composer) =>
                composer.borrow().compose_aspect(aspect),
            _ => SequenceOutput::Empty
        }
    }
    pub fn compose_ctor(&self, _source: &ScopeContext) -> Option<BindingPresentableContext<LANG, SPEC, Gen>> {
        match self {
            ItemComposerWrapper::EnumVariantNamed(composer) =>
                Some(composer.borrow().bindings_composer.ctor_composer.compose(&())),
            ItemComposerWrapper::EnumVariantUnnamed(composer) =>
                Some(composer.borrow().bindings_composer.ctor_composer.compose(&())),
            ItemComposerWrapper::EnumVariantUnit(composer) =>
                Some(composer.borrow().bindings_composer.ctor_composer.compose(&())),
            ItemComposerWrapper::StructNamed(composer) =>
                Some(composer.borrow().bindings_composer.ctor_composer.compose(&())),
            ItemComposerWrapper::StructUnnamed(composer) =>
                Some(composer.borrow().bindings_composer.ctor_composer.compose(&())),
            ItemComposerWrapper::OpaqueStructUnnamed(composer) =>
                Some(composer.borrow().bindings_composer.ctor_composer.compose(&())),
            ItemComposerWrapper::OpaqueStructNamed(composer) =>
                Some(composer.borrow().bindings_composer.ctor_composer.compose(&())),
            _ => None,
        }
    }
    // pub fn compose_attributes(&self) -> Vec<Attribute> {
    //     match self {
    //         ItemComposerWrapper::Enum(composer) =>
    //             <EnumComposer<Paren, LANG, SPEC> as AttrComposable<Vec<Attribute>>>::compose_attributes(&composer.borrow()),
    //             // composer.borrow().compose_attributes(),
    //             // EnumComposer::compose_attributes(&composer.borrow()),
    //         ItemComposerWrapper::EnumVariantNamed(composer) |
    //         ItemComposerWrapper::EnumVariantUnit(composer) |
    //         ItemComposerWrapper::StructNamed(composer) =>
    //             <ItemComposer<Brace, LANG, SPEC> as AttrComposable<Vec<Attribute>>>::compose_attributes(&composer.borrow()),
    //             // <ItemComposer<Brace> as BasicComposable<ItemParentComposer<Brace>, Context, Vec<Attribute>, Option<Generics>>>::compose_attributes(&composer.borrow()),
    //         ItemComposerWrapper::EnumVariantUnnamed(composer) |
    //         ItemComposerWrapper::StructUnnamed(composer) =>
    //             <ItemComposer<Paren, LANG, SPEC> as AttrComposable<Vec<Attribute>>>::compose_attributes(&composer.borrow()),
    //         ItemComposerWrapper::OpaqueStructUnnamed(composer) =>
    //             <OpaqueItemComposer<Paren, LANG, SPEC> as AttrComposable<Vec<Attribute>>>::compose_attributes(&composer.borrow()),
    //         ItemComposerWrapper::OpaqueStructNamed(composer) =>
    //             <OpaqueItemComposer<Brace, LANG, SPEC> as AttrComposable<Vec<Attribute>>>::compose_attributes(&composer.borrow()),
    //         ItemComposerWrapper::Sig(composer) =>
    //             composer.borrow().compose_attributes(),
    //         ItemComposerWrapper::TypeAlias(composer) =>
    //             composer.borrow().compose_attributes(),
    //         ItemComposerWrapper::Trait(composer) =>
    //             composer.borrow().compose_attributes(),
    //         ItemComposerWrapper::Impl(composer) =>
    //             composer.borrow().compose_attributes(),
    //     }
    // }
    // pub fn ferment(&self) -> Depunctuated<LANG> {
    //     match self {
    //         ItemComposerWrapper::Enum(composer) => composer.borrow().ferment(),
    //         ItemComposerWrapper::EnumVariantNamed(composer) => composer.borrow().ferment(),
    //         ItemComposerWrapper::EnumVariantUnnamed(composer) => composer.borrow().ferment(),
    //         ItemComposerWrapper::EnumVariantUnit(composer) => composer.borrow().ferment(),
    //         ItemComposerWrapper::StructNamed(composer) => composer.borrow().ferment(),
    //         ItemComposerWrapper::StructUnnamed(composer) => composer.borrow().ferment(),
    //         ItemComposerWrapper::OpaqueStructUnnamed(composer) => composer.borrow().ferment(),
    //         ItemComposerWrapper::OpaqueStructNamed(composer) => composer.borrow().ferment(),
    //         ItemComposerWrapper::Sig(composer) => composer.borrow().ferment(),
    //         ItemComposerWrapper::TypeAlias(composer) => composer.borrow().ferment(),
    //         ItemComposerWrapper::Trait(composer) => composer.borrow().ferment(),
    //         ItemComposerWrapper::Impl(composer) => composer.borrow().ferment(),
    //     }
    // }
}

impl AttrComposable<Vec<Attribute>> for ItemComposerWrapper<RustFermentate, Vec<Attribute>, Option<Generics>> {
    fn compose_attributes(&self) -> Vec<Attribute> {
        match self {
            ItemComposerWrapper::Enum(composer) =>
                composer.borrow().compose_attributes(),
                // <EnumComposer<Paren, LANG, SPEC> as AttrComposable<Vec<Attribute>>>::compose_attributes(&composer.borrow()),
            // composer.borrow().compose_attributes(),
            // EnumComposer::compose_attributes(&composer.borrow()),
            ItemComposerWrapper::EnumVariantNamed(composer) |
            ItemComposerWrapper::EnumVariantUnit(composer) |
            ItemComposerWrapper::StructNamed(composer) =>
                composer.borrow().compose_attributes(),
                // <ItemComposer<Brace, LANG, SPEC> as AttrComposable<Vec<Attribute>>>::compose_attributes(&composer.borrow()),
            // <ItemComposer<Brace> as BasicComposable<ItemParentComposer<Brace>, Context, Vec<Attribute>, Option<Generics>>>::compose_attributes(&composer.borrow()),
            ItemComposerWrapper::EnumVariantUnnamed(composer) |
            ItemComposerWrapper::StructUnnamed(composer) =>
                composer.borrow().compose_attributes(),
                // <ItemComposer<Paren, LANG, SPEC> as AttrComposable<Vec<Attribute>>>::compose_attributes(&composer.borrow()),
            ItemComposerWrapper::OpaqueStructUnnamed(composer) =>
                composer.borrow().compose_attributes(),
            // <OpaqueItemComposer<Paren, LANG, SPEC> as AttrComposable<Vec<Attribute>>>::compose_attributes(&composer.borrow()),
            ItemComposerWrapper::OpaqueStructNamed(composer) =>
                composer.borrow().compose_attributes(),
                // <OpaqueItemComposer<Brace, LANG, SPEC> as AttrComposable<Vec<Attribute>>>::compose_attributes(&composer.borrow()),
            ItemComposerWrapper::Sig(composer) =>
                composer.borrow().compose_attributes(),
            ItemComposerWrapper::TypeAlias(composer) =>
                composer.borrow().compose_attributes(),
            ItemComposerWrapper::Trait(composer) =>
                composer.borrow().compose_attributes(),
            ItemComposerWrapper::Impl(composer) =>
                composer.borrow().compose_attributes(),
        }    }
}

// impl<I, LANG, SPEC> BindingComposable<I, LANG, SPEC> for ItemComposerWrapper<LANG, SPEC>
//     where I: DelimiterTrait + ?Sized,
//           LANG: Clone, SPEC: LangAttrSpecification<LANG>,
//           SequenceOutput<LANG, SPEC>: ScopeContextPresentable,
//           OwnedItemPresentableContext<LANG, SPEC>: ScopeContextPresentable, {
//     fn compose_bindings(&self) -> Depunctuated<BindingPresentableContext<I, LANG, SPEC>> {
//         match self {
//             ItemComposerWrapper::Enum(composer) =>
//                 composer.borrow().compose_bindings(),
//             // <EnumComposer<Paren, LANG, SPEC> as AttrComposable<Vec<Attribute>>>::compose_attributes(&composer.borrow()),
//             // composer.borrow().compose_attributes(),
//             // EnumComposer::compose_attributes(&composer.borrow()),
//             ItemComposerWrapper::EnumVariantNamed(composer) |
//             ItemComposerWrapper::EnumVariantUnit(composer) |
//             ItemComposerWrapper::StructNamed(composer) =>
//                 composer.borrow().compose_bindings(),
//             // <ItemComposer<Brace, LANG, SPEC> as AttrComposable<Vec<Attribute>>>::compose_attributes(&composer.borrow()),
//             // <ItemComposer<Brace> as BasicComposable<ItemParentComposer<Brace>, Context, Vec<Attribute>, Option<Generics>>>::compose_attributes(&composer.borrow()),
//             ItemComposerWrapper::EnumVariantUnnamed(composer) |
//             ItemComposerWrapper::StructUnnamed(composer) =>
//                 composer.borrow().compose_bindings(),
//             // <ItemComposer<Paren, LANG, SPEC> as AttrComposable<Vec<Attribute>>>::compose_attributes(&composer.borrow()),
//             ItemComposerWrapper::OpaqueStructUnnamed(composer) =>
//                 composer.borrow().compose_bindings(),
//             // <OpaqueItemComposer<Paren, LANG, SPEC> as AttrComposable<Vec<Attribute>>>::compose_attributes(&composer.borrow()),
//             ItemComposerWrapper::OpaqueStructNamed(composer) =>
//                 composer.borrow().compose_bindings(),
//             // <OpaqueItemComposer<Brace, LANG, SPEC> as AttrComposable<Vec<Attribute>>>::compose_attributes(&composer.borrow()),
//             ItemComposerWrapper::Sig(composer) =>
//                 composer.borrow().compose_bindings(),
//             ItemComposerWrapper::TypeAlias(composer) =>
//                 composer.borrow().compose_bindings(),
//             ItemComposerWrapper::Trait(composer) =>
//                 composer.borrow().compose_bindings(),
//             ItemComposerWrapper::Impl(composer) =>
//                 composer.borrow().compose_bindings(),
//         }
//     }
//
// }

impl ItemComposerWrapper<RustFermentate, Vec<Attribute>, Option<Generics>> {
    pub fn ferment(&self) -> Depunctuated<RustFermentate> {
        match self {
            ItemComposerWrapper::Enum(composer) => composer.borrow().ferment(),
            ItemComposerWrapper::EnumVariantNamed(composer) => composer.borrow().ferment(),
            ItemComposerWrapper::EnumVariantUnnamed(composer) => composer.borrow().ferment(),
            ItemComposerWrapper::EnumVariantUnit(composer) => composer.borrow().ferment(),
            ItemComposerWrapper::StructNamed(composer) => composer.borrow().ferment(),
            ItemComposerWrapper::StructUnnamed(composer) => composer.borrow().ferment(),
            ItemComposerWrapper::OpaqueStructUnnamed(composer) => composer.borrow().ferment(),
            ItemComposerWrapper::OpaqueStructNamed(composer) => composer.borrow().ferment(),
            ItemComposerWrapper::Sig(composer) => composer.borrow().ferment(),
            ItemComposerWrapper::TypeAlias(composer) => composer.borrow().ferment(),
            ItemComposerWrapper::Trait(composer) => composer.borrow().ferment(),
            ItemComposerWrapper::Impl(composer) => composer.borrow().ferment(),
        }
    }

}




// impl<LANG, SPEC> ToTokens for ItemComposerWrapper<LANG, SPEC>
//     where LANG: Clone + ToTokens,
//           SPEC: LangAttrSpecification<LANG>,
//           SequenceOutput<LANG, SPEC>: ScopeContextPresentable,
//           OwnedItemPresentableContext<LANG, SPEC>: ScopeContextPresentable {
//     fn to_tokens(&self, tokens: &mut TokenStream) {
//         self.ferment()
//             .to_tokens(tokens);
//     }
// }