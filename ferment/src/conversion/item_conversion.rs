use std::cell::RefCell;
use std::fmt::Formatter;
use std::rc::Rc;
use syn::{Attribute, Expr, Field, Fields, FieldsNamed, FieldsUnnamed, Ident, ImplItem, ImplItemConst, ImplItemMethod, ImplItemType, Item, ItemEnum, ItemFn, ItemImpl, ItemMod, ItemStruct, ItemTrait, ItemType, ItemUse, Meta, NestedMeta, parse_quote, Path, Signature, TraitBound, Type, TypeParamBound, TypePath, TypePtr, TypeReference, TypeTraitObject, UseGlob, UseGroup, UseName, UsePath, UseRename, UseTree, Variant};
use quote::{quote, ToTokens};
use syn::__private::TokenStream2;
use syn::punctuated::Punctuated;
use crate::composer::{ConversionsComposer, IParentComposer, ItemComposer, ParentComposer, VariantComposer};
use crate::composer::constants::{BYPASS_FIELD_CONTEXT, enum_variant_composer_ctor_named, enum_variant_composer_ctor_unit, enum_variant_composer_ctor_unnamed, struct_composer_ctor_named, struct_composer_ctor_unnamed};
use crate::composer::enum_composer::EnumComposer;
use crate::composition::{AttrsComposition, Composition, context::FnSignatureCompositionContext, FnSignatureComposition, TraitDecompositionPart2};
use crate::composition::context::TraitDecompositionPart2Context;
use crate::context::{Scope, ScopeChain, ScopeContext};
use crate::conversion::{FieldTypeConversion, MacroType};
use crate::ext::Pop;
use crate::interface::{CURLY_BRACES_FIELDS_PRESENTER, ROOT_DESTROY_CONTEXT_COMPOSER, ROUND_BRACES_FIELDS_PRESENTER};
use crate::naming::Name;
use crate::presentation::{DocPresentation, Expansion, FFIObjectPresentation};
use crate::presentation::context::{FieldTypePresentableContext, OwnedItemPresentableContext, OwnerIteratorPresentationContext};
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
                // println!("make_expansion: fn: {}", item.sig.ident.to_token_stream());
                let signature = FnSignatureComposition::from_signature(&item.sig, None, scope.self_path_holder().popped(), &scope_context.borrow());
                // let signature = FnSignatureComposition::from_signature(&item.sig, scope.popped(), &scope_context.borrow());
                Expansion::Function {
                    comment: DocPresentation::Safety(Name::Optional(signature.ident.clone())),
                    binding: signature.present(FnSignatureCompositionContext::FFIObject, &scope_context.borrow()),
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



fn enum_expansion(item_enum: &ItemEnum, item_scope: &ScopeChain, context: &ParentComposer<ScopeContext>) -> Expansion {
    let ItemEnum { ident: target_name, variants, generics, .. } = item_enum;
    let source = context.borrow();
    let target_full_type = source.full_type_for(&parse_quote!(#target_name));
    let (composers, presenters) = variants.iter().map(|Variant { attrs, ident: variant_name, fields, discriminant, .. }| {
        let (variant_presenter, fields_context): (VariantComposer, Punctuated<OwnedItemPresentableContext, _>) = match discriminant {
            Some((_, Expr::Lit(lit, ..))) => (
                |local_context| OwnerIteratorPresentationContext::EnumUnitFields(local_context),
                Punctuated::from_iter([OwnedItemPresentableContext::Conversion(quote!(#lit))])),
            None => match fields {
                Fields::Unit => (
                    |(name, _)|
                        OwnerIteratorPresentationContext::NoFields(name),
                    Punctuated::new()),
                Fields::Unnamed(FieldsUnnamed { unnamed, .. }) => (
                    |local_context|
                        OwnerIteratorPresentationContext::RoundBracesFields(local_context),
                    unnamed
                        .iter()
                        .map(|field|
                            OwnedItemPresentableContext::DefaultFieldType(field.ty.clone()))
                        .collect(),
                ),
                Fields::Named(FieldsNamed { named, .. }) => (
                    |local_context|
                        OwnerIteratorPresentationContext::CurlyBracesFields(local_context),
                    named
                        .iter()
                        .map(|Field { ident, ty, .. }|
                            OwnedItemPresentableContext::Named(
                                FieldTypeConversion::Named(Name::Optional(ident.clone()), ty.clone()), false))
                        .collect(),
                ),
            },
            _ => panic!("Error variant discriminant"),
        };
        let composer = match fields {
            Fields::Unit =>
                ItemComposer::enum_variant_composer(
                    Name::EnumVariant(parse_quote!(#target_name), variant_name.clone()),
                    // Name::EnumFFIVariant(target_name.clone(), variant_name.clone()),
                    Name::EnumVariant(target_full_type.clone(), variant_name.clone()),
                    AttrsComposition::new(attrs.clone(), variant_name.clone(), item_scope.clone()),
                    context,
                    |(name, _)|
                        OwnerIteratorPresentationContext::NoFields(name),
                    |(name, _)|
                        OwnerIteratorPresentationContext::NoFields(name),
                    BYPASS_FIELD_CONTEXT,
                    ROOT_DESTROY_CONTEXT_COMPOSER,
                    |_| FieldTypePresentableContext::Scope,
                    enum_variant_composer_ctor_unit(),
                    ConversionsComposer::Empty
                ),
            Fields::Unnamed(fields) =>
                ItemComposer::enum_variant_composer(
                    Name::EnumVariant(parse_quote!(#target_name), variant_name.clone()),
                    // Name::EnumFFIVariant(target_name.clone(), variant_name.clone()),
                    Name::EnumVariant(target_full_type.clone(), variant_name.clone()),
                    AttrsComposition::new(attrs.clone(), variant_name.clone(), item_scope.clone()),
                    context,
                    ROUND_BRACES_FIELDS_PRESENTER,
                    ROUND_BRACES_FIELDS_PRESENTER,
                    BYPASS_FIELD_CONTEXT,
                    ROOT_DESTROY_CONTEXT_COMPOSER,
                    BYPASS_FIELD_CONTEXT,
                    enum_variant_composer_ctor_unnamed(),
                    ConversionsComposer::UnnamedEnumVariant(fields)
                ),
            Fields::Named(fields) =>
                ItemComposer::enum_variant_composer(
                    Name::EnumVariant(parse_quote!(#target_name), variant_name.clone()),
                    // Name::EnumFFIVariant(target_name.clone(), variant_name.clone()),
                    Name::EnumVariant(target_full_type.clone(), variant_name.clone()),
                    AttrsComposition::new(attrs.clone(), variant_name.clone(), item_scope.clone()),
                    context,
                    CURLY_BRACES_FIELDS_PRESENTER,
                    CURLY_BRACES_FIELDS_PRESENTER,
                    |(field_path, field_context)|
                        FieldTypePresentableContext::Named((field_path.clone(), Box::new(field_context.clone()))),
                    ROOT_DESTROY_CONTEXT_COMPOSER,
                    BYPASS_FIELD_CONTEXT,
                    enum_variant_composer_ctor_named(),
                    ConversionsComposer::NamedStruct(fields)
                )
        };
       (composer, (variant_presenter, Name::Variant(variant_name.clone()), fields_context))
    }).unzip();
    EnumComposer::new(
        Name::Variant(target_name.clone()),
        generics.clone(),
        AttrsComposition::new(item_enum.attrs.clone(), target_name.clone(), item_scope.clone()),
        composers,
        presenters,
        context
    ).borrow()
        .expand()
}

fn struct_expansion(item_struct: &ItemStruct, scope: &ScopeChain, scope_context: &ParentComposer<ScopeContext>) -> Expansion {
    // let scope = scope.self_scope();
    // println!("struct_expansion: {}", item_struct.ident);
    let ItemStruct { attrs, fields: ref f, ident: target_name, generics, .. } = item_struct;
    let source = scope_context.borrow();
    // let traits = item_traits_expansions(&attrs_composition, &ctx);
    // let attrs_composer = AttrsComposer::new(attrs_composition);
    // let traits = attrs_composer.compose(&ctx);
    let full_ty = source.full_type_for(&parse_quote!(#target_name));
    match f {
        Fields::Unnamed(ref fields) =>
            ItemComposer::struct_composer(
                Name::Variant(target_name.clone()),
                Name::Type(full_ty),
                Some(generics.clone()),
                AttrsComposition::new(attrs.clone(), target_name.clone(), scope.clone()),
                scope_context,
                |local_context|
                    OwnerIteratorPresentationContext::UnnamedStruct(local_context),
                |field_type|
                    OwnedItemPresentableContext::DefaultFieldType(field_type.ty().clone()),
                ROUND_BRACES_FIELDS_PRESENTER,
                BYPASS_FIELD_CONTEXT,
                struct_composer_ctor_unnamed(),
                ConversionsComposer::UnnamedStruct(fields)
            ),
        Fields::Named(ref fields) =>
            ItemComposer::struct_composer(
                Name::Variant(target_name.clone()),
                Name::Type(full_ty),
                Some(generics.clone()),
                AttrsComposition::new(attrs.clone(), target_name.clone(), scope.clone()),
                scope_context,
                |local_context|
                    OwnerIteratorPresentationContext::NamedStruct(local_context),
                |field_type|
                    OwnedItemPresentableContext::Named(field_type.clone(), true),
                CURLY_BRACES_FIELDS_PRESENTER,
                |(field_path, field_context)|
                    FieldTypePresentableContext::Named((field_path.clone(), Box::new(field_context.clone()))),
                struct_composer_ctor_named(),
                ConversionsComposer::NamedStruct(fields)
            ),
        Fields::Unit => panic!("Fields::Unit is not supported yet"),
    }.borrow()
        .expand()
}


fn trait_expansion(item_trait: &ItemTrait, scope: &ScopeChain, context: &ParentComposer<ScopeContext>) -> Expansion {
    let ItemTrait { ident, items, .. } = item_trait;
    let source = context.borrow();
    let trait_decomposition = TraitDecompositionPart2::from_trait_items(items, Some(parse_quote!(#ident)), scope.self_path_holder(), &source);
    let fields = trait_decomposition.present(TraitDecompositionPart2Context::VTableInnerFunctions, &source);
    let vtable_name = Name::Vtable(ident.clone());
    // println!("trait_expansion: {}: {}", trait_obj_name, vtable_name);
    Expansion::Trait {
        comment: DocPresentation::Empty,
        vtable: FFIObjectPresentation::TraitVTable {
            name: vtable_name.clone(),
            fields
        },
        trait_object: FFIObjectPresentation::TraitObject {
            name: Name::TraitObj(ident.clone()),
            vtable_name
        }
    }
}

fn type_expansion(item_type: &ItemType, scope: &ScopeChain, context: &Rc<RefCell<ScopeContext>>) -> Expansion {
    let ItemType { ident, ty, attrs, generics, .. } = item_type;
    let source = context.borrow();
    match &**ty {
        Type::BareFn(bare_fn) =>
            Expansion::Callback {
                comment: DocPresentation::Default(Name::Variant(ident.clone())),
                binding: FnSignatureComposition::from_bare_fn(bare_fn, ident, scope.self_path_holder().clone(), &source)
                    .present(FnSignatureCompositionContext::FFIObjectCallback, &source),
            },
        _ => {
            let full_ty = source.full_type_for(&parse_quote!(#ident));
            ItemComposer::type_alias_composer(
                Name::Variant(ident.clone()),
                Name::Type(full_ty),
                Some(generics.clone()),
                AttrsComposition { attrs: attrs.clone(), ident: ident.clone(), scope: scope.clone() },
                context,
                ConversionsComposer::TypeAlias(ty))
                .borrow()
                .expand()
        }
    }
}

fn impl_expansion(item_impl: &ItemImpl, scope: &ScopeChain, scope_context: &Rc<RefCell<ScopeContext>>) -> Expansion {
    let ItemImpl { generics: _, trait_, self_ty, items, ..  } = item_impl;
    let impl_item_compositions = items.iter().filter_map(|impl_item| {
        match impl_item {
            ImplItem::Method(ImplItemMethod { sig, .. }) => {
                let signature = FnSignatureComposition::from_signature(sig, Some(*self_ty.clone()), scope.self_path_holder().clone(), &scope_context.borrow());
                Some(signature.present(FnSignatureCompositionContext::FFIObject, &scope_context.borrow()))
            },
            ImplItem::Type(ImplItemType { .. }) => None,
            ImplItem::Const(ImplItemConst { .. }) => None,
            _ => None,
        }
    }).collect();
    // let source = scope_context.borrow();
    match trait_ {
        None => {

            // println!("impl_expansion.1: self_ty: {}", self_ty.to_token_stream());
            // println!("impl_expansion.1: items: {}", quote!(#(#items)*));

            // NEED:
            // pub unsafe extern "C" fn get_balance(obj: *const ()) -> u64 {
            //     let obj = crate::identity::identity::Identity::get_balance(
            //         ferment_interfaces::FFIConversion::ffi_from_const(obj as *const _),
            //     );
            //     obj
            // }

            // CURRENT:
            // #[no_mangle]
            // pub unsafe extern "C" fn get_balance(obj: *const Identity) -> u64 {
            //     let obj = crate::identity::identity::Identity::get_balance(
            //         &ferment_interfaces::FFIConversion::ffi_from_const(obj),
            //     );
            //     obj
            // }

        },
        Some((_, _path, _)) => {
            // let trait_type = parse_quote!(#path);
            // let trait_full_type = source.full_type_for(&trait_type);

            // let gtx = source.context.read().unwrap();
            // let trait_scope = gtx.actual_scope_for_type(&trait_type, scope);

            // println!("impl_expansion.2: trait_scope: {}", trait_scope.to_token_stream());

            // let (trait_composition, trait_scope) = ctx.find_item_trait_in_scope(path);

            // ctx.item_trait_with_ident_for()
            // let item_full_type = source.full_type_for(self_ty);

            // let trait_item = ctx.item_trait_with_ident_for()
            //
            // println!("impl_expansion.2: trait_full_type: {}", trait_full_type.to_token_stream());
            // println!("impl_expansion.2: item_ty: {}", item_full_type.to_token_stream());
            // // println!("impl_expansion.2: trait_composition: {:?}", trait_composition);
            // // println!("impl_expansion.2: trait_scope: {:?}", trait_scope);
            // println!("impl_expansion.2: items: {}", quote!(#(#items)*));
            // println!("impl_expansion.2: trait: {}", quote!(#path));
        }
    }
    Expansion::Impl { comment: DocPresentation::Empty, items: impl_item_compositions }
}
// V1:
// let cast_obj = &(*(obj as *const crate::transport::transport_request::Status));
// let obj = <crate::transport::transport_request::Status as crate::transport::transport_request::SomeOtherTrait>::some_other_method(cast_obj);
// obj

// V2:
// impl Identity {
//     pub unsafe fn create_basic_identity(
//         id: *mut [u8; 32],
//         _platform_version: *const crate::fermented::types::nested::PlatformVersion)
//         -> *mut crate::fermented::generics::Result_ok_crate_identity_identity_Identity_err_crate_nested_ProtocolError {
//         let result = crate::identity::identity::Identity::create_basic_identity(
//             *id,
//             &ferment_interfaces::FFIConversion::ffi_from_const(_platform_version)
//         );
//         ferment_interfaces::FFIConversion::ffi_to(result)
//     }
//     pub unsafe fn create_basic_identity_v0(id: *mut [u8; 32]) -> *mut Self {
//         let result = crate::identity::identity::Identity::create_basic_identity_v0(*id);
//         ferment_interfaces::FFIConversion::ffi_to(result)
//     }
//
//     pub unsafe fn get_balance(&self) -> u64 {
//         let cast_obj = ferment_interfaces::FFIConversion::ffi_from_const(self);
//         let result = crate::identity::identity::Identity::get_balance(&ferment_interfaces::FFIConversion::ffi_from_const(&cast_obj));
//         result
//     }
//
// }
