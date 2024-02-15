use std::cell::RefCell;
use std::fmt::Formatter;
use std::rc::Rc;
use syn::{Attribute, Expr, Field, Fields, FieldsNamed, FieldsUnnamed, Ident, ImplItem, ImplItemConst, ImplItemMethod, ImplItemType, Item, ItemEnum, ItemFn, ItemImpl, ItemMod, ItemStruct, ItemTrait, ItemType, ItemUse, Meta, NestedMeta, parse_quote, Path, Signature, TraitBound, Type, TypeParamBound, TypePath, TypePtr, TypeReference, TypeTraitObject, UseGlob, UseGroup, UseName, UsePath, UseRename, UseTree, Variant};
use quote::{format_ident, quote, ToTokens};
use syn::__private::TokenStream2;
use crate::composer::{AttrsComposer, Composer, ConversionsComposer, ItemComposer};
use crate::composition::{AttrsComposition, Composition, context::FnSignatureCompositionContext, FnSignatureComposition, TraitDecompositionPart2};
use crate::composition::context::TraitDecompositionPart2Context;
use crate::context::{Scope, ScopeChain, ScopeContext};
use crate::conversion::{FieldTypeConversion, MacroType};
use crate::interface::{CURLY_BRACES_FIELDS_PRESENTER, EMPTY_DESTROY_PRESENTER, NAMED_CONVERSION_PRESENTER, package_unboxed_root, ROOT_DESTROY_CONTEXT_PRESENTER, ROUND_BRACES_FIELDS_PRESENTER, SIMPLE_CONVERSION_PRESENTER, SIMPLE_PRESENTER, SIMPLE_TERMINATED_PRESENTER};
use crate::naming::Name;
use crate::presentation::{BindingPresentation, DropInterfacePresentation, FromConversionPresentation, ScopeContextPresentable, ToConversionPresentation};
use crate::presentation::context::{IteratorPresentationContext, OwnedItemPresenterContext, OwnerIteratorPresentationContext};
use crate::presentation::conversion_interface_presentation::ConversionInterfacePresentation;
use crate::presentation::doc_presentation::DocPresentation;
use crate::presentation::expansion::Expansion;
use crate::presentation::ffi_object_presentation::FFIObjectPresentation;
use crate::tree::ScopeTreeExportID;


#[derive(Clone)]
pub enum ItemConversion {
    Mod(ItemMod, ScopeChain),
    Struct(ItemStruct, ScopeChain),
    Enum(ItemEnum, ScopeChain),
    Type(ItemType, ScopeChain),
    Fn(ItemFn, ScopeChain),
    Use(ItemUse, ScopeChain),
    Trait(ItemTrait, ScopeChain),
    Impl(ItemImpl, ScopeChain)
}

impl std::fmt::Debug for ItemConversion {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}: {}", self.name(), self.ident()))
    }
}

impl std::fmt::Display for ItemConversion {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}

impl ToTokens for ItemConversion {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        match self {
            ItemConversion::Mod(item, ..) => item.to_tokens(tokens),
            ItemConversion::Struct(item, ..) => item.to_tokens(tokens),
            ItemConversion::Enum(item, ..) => item.to_tokens(tokens),
            ItemConversion::Type(item, ..) => item.to_tokens(tokens),
            ItemConversion::Fn(item, ..) => item.to_tokens(tokens),
            ItemConversion::Trait(item, ..) => item.to_tokens(tokens),
            ItemConversion::Impl(item, ..) => item.to_tokens(tokens),
            ItemConversion::Use(item, ..) => item.to_tokens(tokens),
        }
    }
}

impl<'a> TryFrom<(&'a Item, &'a ScopeChain)> for ItemConversion {
    type Error = String;
    fn try_from(value: (&'a Item, &'a ScopeChain)) -> Result<Self, Self::Error> {
        match value.0 {
            Item::Mod(item) => Ok(Self::Mod(item.clone(), value.1.clone())),
            Item::Struct(item) => Ok(Self::Struct(item.clone(), value.1.clone())),
            Item::Enum(item) => Ok(Self::Enum(item.clone(), value.1.clone())),
            Item::Type(item) => Ok(Self::Type(item.clone(), value.1.clone())),
            Item::Fn(item) => Ok(Self::Fn(item.clone(), value.1.clone())),
            Item::Trait(item) => Ok(Self::Trait(item.clone(), value.1.clone())),
            Item::Impl(item) => Ok(Self::Impl(item.clone(), value.1.clone())),
            item => Err(format!("Error: {}", item.to_token_stream()))
        }
    }
}

impl<'a> TryFrom<(&'a Item, ScopeChain)> for ItemConversion {
    type Error = String;
    fn try_from(value: (&'a Item, ScopeChain)) -> Result<Self, Self::Error> {
        match value.0 {
            Item::Mod(item) => Ok(Self::Mod(item.clone(), value.1)),
            Item::Struct(item) => Ok(Self::Struct(item.clone(), value.1)),
            Item::Enum(item) => Ok(Self::Enum(item.clone(), value.1)),
            Item::Type(item) => Ok(Self::Type(item.clone(), value.1)),
            Item::Fn(item) => Ok(Self::Fn(item.clone(), value.1)),
            Item::Trait(item) => Ok(Self::Trait(item.clone(), value.1)),
            Item::Impl(item) => Ok(Self::Impl(item.clone(), value.1)),
            item => Err(format!("Error: {}", item.to_token_stream()))
        }
    }
}

impl<'a> From<&'a ItemConversion> for Item {
    fn from(value: &'a ItemConversion) -> Self {
        match value {
            ItemConversion::Mod(item, _) => parse_quote!(#item),
            ItemConversion::Struct(item, _) =>  parse_quote!(#item),
            ItemConversion::Enum(item, _) =>  parse_quote!(#item),
            ItemConversion::Type(item, _) =>  parse_quote!(#item),
            ItemConversion::Fn(item, _) =>  parse_quote!(#item),
            ItemConversion::Use(item, _) =>  parse_quote!(#item),
            ItemConversion::Trait(item, _) =>  parse_quote!(#item),
            ItemConversion::Impl(item, _) => parse_quote!(#item),
        }
    }
}

fn path_ident_ref<'a>(path: &'a Path) -> Option<&'a Ident> {
    path.segments.last().map(|last_segment| &last_segment.ident)
}
fn path_ident(path: &Path) -> Option<Ident> {
    path.segments.last().map(|last_segment| last_segment.ident.clone())
}
pub fn type_ident(ty: &Type) -> Option<Ident> {
    match ty {
        Type::Path(TypePath { path, .. }) =>
            path_ident(path),
        Type::Reference(TypeReference { elem, .. }) |
        Type::Ptr(TypePtr { elem, .. }) =>
            type_ident(elem),
        Type::TraitObject(TypeTraitObject { bounds, .. }) => {
            bounds.iter().find_map(|b| match b {
                TypeParamBound::Trait(TraitBound { path, ..}) => path_ident(path),
                _ => None
            })
        }
        _ => panic!("DDDDD")
    }
}
pub fn type_ident_ref<'a>(ty: &'a Type) -> Option<&'a Ident> {
    match ty {
        Type::Path(TypePath { path, .. }) =>
            path_ident_ref(path),
        Type::Reference(TypeReference { elem, .. }) |
        Type::Ptr(TypePtr { elem, .. }) =>
            type_ident_ref(elem),
        Type::TraitObject(TypeTraitObject { bounds, .. }) => {
            bounds.iter().find_map(|b| match b {
                TypeParamBound::Trait(TraitBound { path, ..}) => path_ident_ref(path),
                _ => None
            })
        }
        _ => panic!("DDDDD")
    }
}

impl ItemConversion {
    pub const fn name(&self) -> &str {
        match self {
            Self::Mod(..) => "mod",
            Self::Struct(..) => "struct",
            Self::Enum(..) => "enum",
            Self::Type(..) => "type",
            Self::Fn(..) => "fn",
            Self::Use(..) => "use",
            Self::Trait(..) => "trait",
            Self::Impl(..) => "impl",
        }
    }

    pub fn scope_chain(&self) -> &ScopeChain {
        match self {
            ItemConversion::Mod(_, scope) => scope,
            ItemConversion::Struct(_, scope) => scope,
            ItemConversion::Enum(_, scope) => scope,
            ItemConversion::Type(_, scope) => scope,
            ItemConversion::Fn(_, scope) => scope,
            ItemConversion::Trait(_, scope) => scope,
            ItemConversion::Use(_, scope) => scope,
            ItemConversion::Impl(_, scope) => scope,
        }
    }

    pub fn scope(&self) -> &Scope {
        match self {
            ItemConversion::Mod(_, scope) => scope.self_scope(),
            ItemConversion::Struct(_, scope) => scope.self_scope(),
            ItemConversion::Enum(_, scope) => scope.self_scope(),
            ItemConversion::Type(_, scope) => scope.self_scope(),
            ItemConversion::Fn(_, scope) => scope.self_scope(),
            ItemConversion::Trait(_, scope) => scope.self_scope(),
            ItemConversion::Use(_, scope) => scope.self_scope(),
            ItemConversion::Impl(_, scope) => scope.self_scope(),
        }
    }

    pub fn attrs(&self) -> &Vec<Attribute> {
        match self {
            ItemConversion::Mod(item, _) => &item.attrs,
            ItemConversion::Struct(item, _) => &item.attrs,
            ItemConversion::Enum(item, _) => &item.attrs,
            ItemConversion::Type(item, _) => &item.attrs,
            ItemConversion::Fn(item, _) => &item.attrs,
            ItemConversion::Trait(item, _) => &item.attrs,
            ItemConversion::Use(item, _) => &item.attrs,
            ItemConversion::Impl(item, _) => &item.attrs,
        }
    }

    pub fn fold_use(tree: &UseTree) -> Vec<&Ident> {
        match tree {
            UseTree::Path(UsePath { ident, .. }) => vec![ident],
            UseTree::Name(UseName { ident }) => vec![ident],
            UseTree::Rename(UseRename { rename, .. }) => vec![rename],
            UseTree::Glob(UseGlob { .. }) => vec![],
            UseTree::Group(UseGroup { items , .. }) =>
                items.iter().flat_map(Self::fold_use).collect()
        }
    }


    pub fn ident(&self) -> ScopeTreeExportID {
        match self {
            ItemConversion::Mod(ItemMod { ident, .. }, ..) |
            ItemConversion::Struct(ItemStruct { ident, .. }, ..) |
            ItemConversion::Enum(ItemEnum { ident, .. }, ..) |
            ItemConversion::Type(ItemType { ident, .. }, ..) |
            ItemConversion::Fn(ItemFn { sig: Signature { ident, .. }, .. }, ..) |
            ItemConversion::Trait(ItemTrait { ident, .. }, ..) => ScopeTreeExportID::Ident(ident.clone()),
            ItemConversion::Use(ItemUse { tree, .. }, ..) =>
                ScopeTreeExportID::Ident(Self::fold_use(tree).first().cloned().unwrap().clone()),
            ItemConversion::Impl(ItemImpl { self_ty, trait_, .. }, ..) => ScopeTreeExportID::Impl(*self_ty.clone(), trait_.clone().map(|(_, path, _)| path)),
                // type_ident(self_ty).unwrap(),
        }
    }

    pub fn is_labeled_with_macro_type(path: &Path, macro_type: &str) -> bool {
        path.segments
            .iter()
            .any(|segment| macro_type == segment.ident.to_string().as_str())
    }


    pub fn is_labeled_for_register(path: &Path) -> bool {
        Self::is_labeled_with_macro_type(path, "register")
    }

    pub fn is_owner_labeled_with_trait_implementation(path: &Path) -> bool {
        Self::is_labeled_with_macro_type(path, "export")
    }

    // pub fn has_export_macro_attribute(&self) -> bool {
    //     self.attrs().iter().filter(|Attribute { path, .. }| Self::is_labeled_for_export(path)).count() > 0
    // }

    pub fn macro_type(&self) -> Option<MacroType> {
        self.attrs()
            .iter()
            .find_map(|attr| {
                let path = &attr.path;
                let mut arguments = Vec::<Path>::new();
                if let Ok(Meta::List(meta_list)) = attr.parse_meta() {
                    meta_list.nested.iter().for_each(|meta| {
                        if let NestedMeta::Meta(Meta::Path(path)) = meta {
                            arguments.push(path.clone());
                        }
                    });
                }
                match path.segments.last().unwrap().ident.to_string().as_str() {
                    "export" =>
                        Some(MacroType::Export),
                    "register" => {
                        let first_path = arguments.first().unwrap();
                        Some(MacroType::Register(parse_quote!(#first_path)))
                    },
                    _ =>
                        None
                }
            })
    }

    pub fn has_register_macro_attribute(&self) -> bool {
        self.attrs().iter().filter(|Attribute { path, .. }| Self::is_labeled_for_register(path)).count() > 0
    }

    pub fn make_expansion<'a>(&self, scope_context: &Rc<RefCell<ScopeContext>>) -> Expansion {
        match self {
            ItemConversion::Mod(..) => Expansion::Empty,
            ItemConversion::Struct(item, scope) =>
                struct_expansion(item, scope, scope_context),
            ItemConversion::Enum(item, scope) =>
                enum_expansion(item, scope, scope_context),
            ItemConversion::Type(item, scope) =>
                type_expansion(item, scope, scope_context),
            ItemConversion::Fn(item, scope) => {
                println!("make_expansion: fn: {}", item.sig.ident.to_token_stream());
                let signature = FnSignatureComposition::from_signature(&item.sig, scope.self_path_holder().popped(), &scope_context.borrow());
                // let signature = FnSignatureComposition::from_signature(&item.sig, scope.popped(), &scope_context.borrow());
                Expansion::Function {
                    comment: DocPresentation::Safety(signature.ident.to_token_stream()),
                    ffi_presentation: signature.present(FnSignatureCompositionContext::FFIObject, &scope_context.borrow()),
                }
            },
            ItemConversion::Trait(item, scope) =>
                trait_expansion(item, scope, scope_context),
            ItemConversion::Impl(item, scope) =>
                impl_expansion(item, scope, scope_context),
            ItemConversion::Use(_item, _scope) =>
                Expansion::Use { comment: DocPresentation::Empty },
        }
    }
}



fn enum_expansion(item_enum: &ItemEnum, item_scope: &ScopeChain, context: &Rc<RefCell<ScopeContext>>) -> Expansion {
    // let /*item_scope*/ = item_scope.self_scope();
    let ItemEnum { ident: target_name, variants, generics, .. } = item_enum;
    let variants_count = variants.len();
    let ctx = context.borrow();
    let attrs_composition = AttrsComposition::new(
        item_enum.attrs.clone(),
        target_name.clone(),
        item_scope.clone(),
    );

    let mut conversions_to_ffi = Vec::<TokenStream2>::with_capacity(variants_count);
    let mut conversions_from_ffi = Vec::<TokenStream2>::with_capacity(variants_count);
    let mut variants_fields = Vec::<OwnedItemPresenterContext>::with_capacity(variants_count);
    let mut variants_constructors = Vec::<BindingPresentation>::with_capacity(variants_count);
    let mut destroy_fields = Vec::<TokenStream2>::new();
    let mut drop_fields = Vec::<OwnedItemPresenterContext>::new();
    variants.iter().for_each(|Variant { attrs, ident: variant_name, fields, discriminant, .. }| {
        let full_ty = ctx.full_type_for(&parse_quote!(#target_name));
        let target_variant_path = quote!(#full_ty::#variant_name);
        let ffi_variant_path = quote!(#target_name::#variant_name);
        let variant_mangled_path = format_ident!("{}_{}", target_name, variant_name);
        let (variant_presenter, fields_context): (fn((TokenStream2, Vec<OwnedItemPresenterContext>)) -> OwnerIteratorPresentationContext, Vec<OwnedItemPresenterContext>) = match discriminant {
            Some((_, Expr::Lit(lit, ..))) => (
                |(name, fields): (TokenStream2, Vec<OwnedItemPresenterContext>)|
                    OwnerIteratorPresentationContext::EnumUnitFields(name, fields),
                vec![OwnedItemPresenterContext::Conversion(quote!(#lit))]),
            None => match fields {
                Fields::Unit => (
                    |(name, _)|
                        OwnerIteratorPresentationContext::NoFields(name),
                    vec![]),
                Fields::Unnamed(FieldsUnnamed { unnamed, .. }) => (
                    |(name, fields)|
                        OwnerIteratorPresentationContext::EnumUnamedVariant(name, fields),
                    unnamed
                        .iter()
                        .map(|field| OwnedItemPresenterContext::Conversion(ctx.ffi_full_dictionary_field_type_presenter(&field.ty).to_token_stream()))
                        .collect(),
                ),
                Fields::Named(FieldsNamed { named, .. }) => (
                    |(name, fields)|
                        OwnerIteratorPresentationContext::EnumNamedVariant(name, fields),
                    named
                        .iter()
                        .map(|Field { ident, ty: field_type, .. }|
                                 OwnedItemPresenterContext::Named(FieldTypeConversion::Named(Name::Optional(ident.clone()), ctx.ffi_full_dictionary_field_type_presenter(field_type)), false))


                            // OwnedItemPresenterContext::Conversion(NAMED_CONVERSION_PRESENTER(
                            //     ident.clone().unwrap().to_token_stream(),
                            //     ctx.ffi_full_dictionary_field_type_presenter(field_type))))
                        .collect(),
                ),
            },
            _ => panic!("Error variant discriminant"),
        };
        let attrs_composition = AttrsComposition::new(
            attrs.clone(),
            variant_name.clone(),
            item_scope.clone(),
        );
        let composer = match fields {
            Fields::Unit =>
                ItemComposer::enum_variant_default_composer(
                    parse_quote!(#ffi_variant_path),
                    parse_quote!(#target_variant_path),
                    attrs_composition,
                    context,
                    |(name, _)|
                        OwnerIteratorPresentationContext::NoFields(name),
                    |(name, _)|
                        OwnerIteratorPresentationContext::NoFields(name),
                    SIMPLE_CONVERSION_PRESENTER,
                    ROOT_DESTROY_CONTEXT_PRESENTER,
                    EMPTY_DESTROY_PRESENTER,
                    |_| IteratorPresentationContext::Empty,
                    |field_type|
                        OwnedItemPresenterContext::Named(field_type, false),
                    |field_type|
                        OwnedItemPresenterContext::DefaultField(field_type),
                    ConversionsComposer::Empty
                ),
            Fields::Unnamed(fields) =>
                ItemComposer::enum_variant_default_composer(
                    parse_quote!(#ffi_variant_path),
                    parse_quote!(#target_variant_path),
                    attrs_composition,
                    context,
                    ROUND_BRACES_FIELDS_PRESENTER,
                    ROUND_BRACES_FIELDS_PRESENTER,
                    SIMPLE_CONVERSION_PRESENTER,
                    ROOT_DESTROY_CONTEXT_PRESENTER,
                    SIMPLE_TERMINATED_PRESENTER,
                    |fields| {
                        if fields.is_empty() {
                            IteratorPresentationContext::Empty
                        } else {
                            IteratorPresentationContext::Round(fields)
                        }
                    },
                    |field_type|
                        OwnedItemPresenterContext::Named(field_type, false),
                    |field_type|
                        OwnedItemPresenterContext::DefaultField(field_type),
                    ConversionsComposer::UnnamedEnumVariant(fields)
                ),
            Fields::Named(fields) =>
                ItemComposer::enum_variant_default_composer(
                    parse_quote!(#ffi_variant_path),
                    parse_quote!(#target_variant_path),
                    attrs_composition,
                    context,
                    CURLY_BRACES_FIELDS_PRESENTER,
                    CURLY_BRACES_FIELDS_PRESENTER,
                    NAMED_CONVERSION_PRESENTER,
                    ROOT_DESTROY_CONTEXT_PRESENTER,
                    SIMPLE_PRESENTER,
                    |fields| {
                        if fields.is_empty() {
                            IteratorPresentationContext::Empty
                        } else {
                            IteratorPresentationContext::Curly(fields)
                        }
                    },
                    |field_type|
                        OwnedItemPresenterContext::Named(field_type, false),
                    |field_type|
                        OwnedItemPresenterContext::DefaultField(field_type),
                    ConversionsComposer::NamedStruct(fields)
                )
        };
        let composer_owned = composer.borrow();
        variants_fields.push(OwnedItemPresenterContext::Conversion(variant_presenter((quote!(#variant_name), fields_context)).present(&ctx)));
        conversions_from_ffi.push(composer_owned.compose_from());
        conversions_to_ffi.push(composer_owned.compose_to());
        destroy_fields.push(composer_owned.compose_destroy());
        drop_fields.push(OwnedItemPresenterContext::Conversion(composer_owned.compose_drop()));
        variants_constructors.push(BindingPresentation::EnumVariantConstructor {
            ffi_ident: target_name.to_token_stream(),
            ffi_variant_ident: variant_mangled_path,
            ffi_variant_path: ffi_variant_path.to_token_stream(),
            ctor_arguments: composer_owned.ffi_conversions_composer.bindings_composer.compose_arguments(),
            body_presentation: composer_owned.ffi_conversions_composer.bindings_composer.present_field_names(&ctx),
            context: Rc::clone(context)
        })
    });
    let comment = DocPresentation::Default(quote!(#target_name));
    let ffi_presentation =
        FFIObjectPresentation::Full(OwnerIteratorPresentationContext::Enum(quote!(#target_name), variants_fields).present(&ctx));
    let ctx = context.borrow();
    let target_full_type = ctx.full_type_for(&parse_quote!(#target_name));
    let conversion_presentation = ConversionInterfacePresentation::Interface {
        ffi_type: parse_quote!(#target_name),
        target_type: quote!(#target_full_type),
        from_presentation: FromConversionPresentation::Enum(conversions_from_ffi),
        to_presentation: ToConversionPresentation::Enum(conversions_to_ffi),
        destroy_presentation: package_unboxed_root(),
        generics: Some(generics.clone()),
    };
    let drop = DropInterfacePresentation::Full(
        quote!(#target_name),
        IteratorPresentationContext::EnumDestroy(drop_fields).present(&ctx),
    );

    variants_constructors.push(BindingPresentation::Destructor {
        ffi_name: quote!(#target_name),
        destructor_ident: Name::Destructor(target_name.clone())
    });

    let attrs_composer = AttrsComposer::new(attrs_composition);
    Expansion::Full {
        comment,
        ffi_presentation,
        conversion: conversion_presentation,
        drop,
        bindings: variants_constructors,
        traits: attrs_composer.compose(&ctx)
    }
}

fn struct_expansion(item_struct: &ItemStruct, scope: &ScopeChain, scope_context: &Rc<RefCell<ScopeContext>>) -> Expansion {
    // let scope = scope.self_scope();
    // println!("struct_expansion: {}", item_struct.ident);
    let ItemStruct { attrs, fields: ref f, ident: target_name, .. } = item_struct;
    let ctx = scope_context.borrow();
    let attrs_composition = AttrsComposition::new(attrs.clone(), target_name.clone(), scope.clone());
    // let traits = item_traits_expansions(&attrs_composition, &ctx);
    // let attrs_composer = AttrsComposer::new(attrs_composition);
    // let traits = attrs_composer.compose(&ctx);
    let full_ty = ctx.full_type_for(&parse_quote!(#target_name));

    let composer = match f {
        Fields::Unnamed(ref fields) =>
            ItemComposer::struct_composer(
                parse_quote!(#target_name),
                parse_quote!(#full_ty),
                attrs_composition,
                scope_context,
                |(name, fields)|
                    OwnerIteratorPresentationContext::UnnamedStruct(name, fields),
                |field_type|
                    OwnedItemPresenterContext::DefaultFieldType(field_type),
                ROUND_BRACES_FIELDS_PRESENTER,
                SIMPLE_CONVERSION_PRESENTER,
                |fields|
                    IteratorPresentationContext::Round(fields),
                |field_type|
                    OwnedItemPresenterContext::BindingArg(field_type),
                |field_type|
                    OwnedItemPresenterContext::BindingField(field_type),
                ConversionsComposer::UnnamedStruct(fields)
            ),
        Fields::Named(ref fields) =>
            ItemComposer::struct_composer(
                parse_quote!(#target_name),
                parse_quote!(#full_ty),
                attrs_composition,
                scope_context,
                |(name, fields)|
                    OwnerIteratorPresentationContext::NamedStruct(name, fields),
                |field_type|
                    OwnedItemPresenterContext::Named(field_type, true),
                CURLY_BRACES_FIELDS_PRESENTER,
                NAMED_CONVERSION_PRESENTER,
                |fields|
                    IteratorPresentationContext::Curly(fields),
                |field_type|
                    OwnedItemPresenterContext::Named(field_type, false),
                |field_type|
                    OwnedItemPresenterContext::DefaultField(field_type),
                ConversionsComposer::NamedStruct(fields)
            ),
        Fields::Unit => panic!("Fields::Unit is not supported yet"),
    };
    let composer_owned = composer.borrow();
    composer_owned.make_expansion()
}


fn trait_expansion(item_trait: &ItemTrait, scope: &ScopeChain, context: &Rc<RefCell<ScopeContext>>) -> Expansion {
    let ItemTrait { ident, items, .. } = item_trait;
    let context = context.borrow();
    let trait_decomposition = TraitDecompositionPart2::from_trait_items(items, scope.self_path_holder(), &context);
    let fields = trait_decomposition.present(TraitDecompositionPart2Context::VTableInnerFunctions, &context);
    let vtable_name = Name::Vtable(ident.clone());
    // println!("trait_expansion: {}: {}", trait_obj_name, vtable_name);
    Expansion::Trait {
        comment: DocPresentation::Empty,
        vtable: FFIObjectPresentation::TraitVTable {
            name: vtable_name.to_token_stream(),
            fields
        },
        trait_object: FFIObjectPresentation::TraitObject {
            name: Name::TraitObj(ident.clone()),
            vtable_name: vtable_name.to_token_stream()
        }
    }
}

fn type_expansion(item_type: &ItemType, scope: &ScopeChain, context: &Rc<RefCell<ScopeContext>>) -> Expansion {
    // let scope = scope.self_scope();
    let ItemType { ident, ty, attrs, .. } = item_type;
    let ctx = context.borrow();
    match &**ty {
        Type::BareFn(bare_fn) =>
            Expansion::Callback {
                comment: DocPresentation::Default(quote!(#ident)),
                ffi_presentation: FnSignatureComposition::from_bare_fn(bare_fn, ident, scope.self_path_holder().clone(), &ctx)
                    .present(FnSignatureCompositionContext::FFIObjectCallback, &ctx),
            },
        _ => {
            let full_ty = ctx.full_type_for(&parse_quote!(#ident));
            ItemComposer::type_alias_composer(
                parse_quote!(#ident),
                parse_quote!(#full_ty),
                AttrsComposition {
                    attrs: attrs.clone(),
                    ident: ident.clone(),
                    scope: scope.clone(),
                },
                context,
                ConversionsComposer::TypeAlias(ty))
                .borrow()
                .make_expansion()
        }
    }
}

fn impl_expansion(item_impl: &ItemImpl, scope: &ScopeChain, scope_context: &Rc<RefCell<ScopeContext>>) -> Expansion {
    // let cast_obj = &(*(obj as *const crate::transport::transport_request::Status));
    // let obj = <crate::transport::transport_request::Status as crate::transport::transport_request::SomeOtherTrait>::some_other_method(cast_obj);
    // obj
    let ItemImpl { generics: _, trait_, self_ty, items, ..  } = item_impl;
    let impl_item_compositions = items.iter().filter_map(|impl_item| {
        match impl_item {
            ImplItem::Method(ImplItemMethod { sig, .. }) => {
                let signature = FnSignatureComposition::from_signature(sig, scope.self_path_holder().clone(), &scope_context.borrow());
                Some(signature.present(FnSignatureCompositionContext::FFIObject, &scope_context.borrow()))
            },
            ImplItem::Type(ImplItemType { .. }) => None,
            ImplItem::Const(ImplItemConst { .. }) => None,
            _ => None,
        }
    }).collect::<Vec<_>>();
    let ctx = scope_context.borrow();
    match trait_ {
        None => {

            println!("impl_expansion.1: self_ty: {}", self_ty.to_token_stream());
            println!("impl_expansion.1: items: {}", quote!(#(#items)*));
        },
        Some((_, path, _)) => {
            let trait_type = parse_quote!(#path);
            let trait_full_type = ctx.full_type_for(&trait_type);

            let gtx = ctx.context.read().unwrap();
            let trait_scope = gtx.actual_scope_for_type(&trait_type, scope);

            println!("impl_expansion.2: trait_scope: {}", trait_scope.to_token_stream());

            // let (trait_composition, trait_scope) = ctx.find_item_trait_in_scope(path);

            // ctx.item_trait_with_ident_for()
            let item_full_type = ctx.full_type_for(self_ty);

            // let trait_item = ctx.item_trait_with_ident_for()

            println!("impl_expansion.2: trait_full_type: {}", trait_full_type.to_token_stream());
            println!("impl_expansion.2: item_ty: {}", item_full_type.to_token_stream());
            // println!("impl_expansion.2: trait_composition: {:?}", trait_composition);
            // println!("impl_expansion.2: trait_scope: {:?}", trait_scope);
            println!("impl_expansion.2: items: {}", quote!(#(#items)*));
            println!("impl_expansion.2: trait: {}", quote!(#path));
        }
    }
    Expansion::Impl { comment: DocPresentation::Empty, items: impl_item_compositions }
}