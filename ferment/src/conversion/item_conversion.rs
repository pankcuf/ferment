use std::fmt::Formatter;
use syn::{Expr, Field, Fields, FieldsNamed, FieldsUnnamed, Ident, ImplItem, ImplItemConst, ImplItemMethod, ImplItemType, Item, ItemEnum, ItemFn, ItemImpl, ItemMod, ItemStruct, ItemTrait, ItemType, parse_quote, Path, Signature, TraitBound, Type, TypeArray, TypeParamBound, TypePath, TypePtr, TypeReference, TypeTraitObject, Variant};
use quote::{quote, ToTokens};
use syn::__private::TokenStream2;
use syn::punctuated::Punctuated;
use syn::token::Comma;
use crate::composer::{ItemComposer, ParentComposer, VariantComposer};
use crate::composer::composable::SourceExpandable;
use crate::composer::enum_composer::EnumComposer;
use crate::composer::signature::SigComposer;
use crate::composer::trait_composer::TraitComposer;
use crate::composition::{AttrsComposition, CfgAttributes, FnSignatureContext};
use crate::context::{ScopeChain, ScopeContext};
use crate::conversion::FieldTypeConversion;
use crate::ext::{CrateExtension, ToPath, ToType};
use crate::helper::ItemExtension;
use crate::holder::PathHolder;
use crate::naming::Name;
use crate::presentation::{DocPresentation, Expansion};
use crate::presentation::context::{OwnedItemPresentableContext, OwnerIteratorPresentationContext};
use crate::presentation::context::name::{Aspect, Context};
use crate::tree::ScopeTreeExportID;


#[derive(Clone)]
pub enum ItemConversion {
    Mod(ItemMod, ScopeChain),
    Struct(ItemStruct, ScopeChain),
    Enum(ItemEnum, ScopeChain),
    Type(ItemType, ScopeChain),
    Fn(ItemFn, ScopeChain),
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

fn path_ident_ref(path: &Path) -> Option<&Ident> {
    path.segments.last().map(|last_segment| &last_segment.ident)
}
// fn path_ident(path: &Path) -> Option<Ident> {
//     path.segments.last().map(|last_segment| last_segment.ident.clone())
// }
// pub fn type_ident(ty: &Type) -> Option<Ident> {
//     match ty {
//         Type::Path(TypePath { path, .. }) =>
//             path_ident(path),
//         Type::Reference(TypeReference { elem, .. }) |
//         Type::Ptr(TypePtr { elem, .. }) =>
//             type_ident(elem),
//         Type::TraitObject(TypeTraitObject { bounds, .. }) => {
//             bounds.iter().find_map(|b| match b {
//                 TypeParamBound::Trait(TraitBound { path, ..}) => path_ident(path),
//                 _ => None
//             })
//         },
//         Type::Array(TypeArray { elem, .. }) =>
//             type_ident(elem),
//         _ => None,
//     }
// }
pub fn type_ident_ref(ty: &Type) -> Option<&Ident> {
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
        },
        Type::Array(TypeArray { elem, .. }) => type_ident_ref(elem),

        // _ => panic!("No ident ref for {}", ty.to_token_stream())
        _ => None
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
            Self::Trait(..) => "trait",
            Self::Impl(..) => "impl",
        }
    }

    // pub fn attrs(&self) -> &Vec<Attribute> {
    //     match self {
    //         ItemConversion::Mod(item, _) => &item.attrs,
    //         ItemConversion::Struct(item, _) => &item.attrs,
    //         ItemConversion::Enum(item, _) => &item.attrs,
    //         ItemConversion::Type(item, _) => &item.attrs,
    //         ItemConversion::Fn(item, _) => &item.attrs,
    //         ItemConversion::Trait(item, _) => &item.attrs,
    //         ItemConversion::Impl(item, _) => &item.attrs,
    //     }
    // }

    // pub fn fold_use(tree: &UseTree) -> Vec<&Ident> {
    //     match tree {
    //         UseTree::Path(UsePath { ident, .. }) => vec![ident],
    //         UseTree::Name(UseName { ident }) => vec![ident],
    //         UseTree::Rename(UseRename { rename, .. }) => vec![rename],
    //         UseTree::Glob(UseGlob { .. }) => vec![],
    //         UseTree::Group(UseGroup { items , .. }) =>
    //             items.iter().flat_map(Self::fold_use).collect()
    //     }
    // }


    pub fn ident(&self) -> ScopeTreeExportID {
        match self {
            ItemConversion::Mod(ItemMod { ident, .. }, ..) |
            ItemConversion::Struct(ItemStruct { ident, .. }, ..) |
            ItemConversion::Enum(ItemEnum { ident, .. }, ..) |
            ItemConversion::Type(ItemType { ident, .. }, ..) |
            ItemConversion::Fn(ItemFn { sig: Signature { ident, .. }, .. }, ..) |
            ItemConversion::Trait(ItemTrait { ident, .. }, ..) =>
                ScopeTreeExportID::Ident(ident.clone()),
            ItemConversion::Impl(ItemImpl { self_ty, trait_, .. }, ..) =>
                ScopeTreeExportID::Impl(*self_ty.clone(), trait_.clone().map(|(_, path, _)| path)),
        }
    }

    pub fn make_expansion(&self, scope_context: &ParentComposer<ScopeContext>) -> Expansion {
        match self {
            ItemConversion::Mod(..) => Expansion::Empty,
            ItemConversion::Struct(item, scope) =>
                struct_expansion(item, scope, scope_context),
            ItemConversion::Enum(item, scope) =>
                enum_expansion(item, scope, scope_context),
            ItemConversion::Type(item, scope) =>
                type_expansion(item, scope, scope_context),
            ItemConversion::Fn(item, scope) =>
                fn_expansion(item, scope, scope_context),
            ItemConversion::Trait(item, scope) =>
                trait_expansion(item, scope, scope_context),
            ItemConversion::Impl(item, scope) =>
                impl_expansion(item, scope, scope_context),
        }
    }
}

fn enum_expansion(item_enum: &ItemEnum, item_scope: &ScopeChain, context: &ParentComposer<ScopeContext>) -> Expansion {
    let ItemEnum { attrs, ident: target_name, variants, generics, .. } = item_enum;
    // println!("enum_expansion: {:?}", item_enum);
    EnumComposer::new(
        target_name,
        generics,
        attrs,
        item_scope,
        context,
        variants.iter()
            .map(|Variant { attrs, ident: variant_name, fields, discriminant, .. }| {
                let (variant_composer, fields_context): (VariantComposer, Punctuated<OwnedItemPresentableContext, Comma>) = match discriminant {
                    Some((_, Expr::Lit(lit, ..))) => (
                        |local_context| OwnerIteratorPresentationContext::EnumUnitFields(local_context.clone()),
                        Punctuated::from_iter([OwnedItemPresentableContext::Conversion(quote!(#lit), attrs.cfg_attributes().to_token_stream())])),
                    None => match fields {
                        Fields::Unit => (
                            |(aspect, _)| OwnerIteratorPresentationContext::NoFields(aspect.clone()),
                            Punctuated::new()
                        ),
                        Fields::Unnamed(FieldsUnnamed { unnamed, .. }) => (
                            |local_context| OwnerIteratorPresentationContext::RoundVariantFields(local_context.clone()),
                            unnamed
                                .iter()
                                .map(|field_type| OwnedItemPresentableContext::DefaultFieldType(field_type.ty.clone(), field_type.attrs.cfg_attributes().to_token_stream()))
                                .collect(),
                        ),
                        Fields::Named(FieldsNamed { named, .. }) => (
                            |local_context| OwnerIteratorPresentationContext::CurlyVariantFields(local_context.clone()),
                            named
                                .iter()
                                .map(|Field { ident, attrs, ty, .. }|
                                    OwnedItemPresentableContext::Named(
                                        FieldTypeConversion::Named(Name::Optional(ident.clone()), ty.clone(), attrs.cfg_attributes()), false))
                                .collect(),
                        ),
                    },
                    _ => panic!("Error variant discriminant"),
                };
                let name_context = Context::EnumVariant {
                    ident: target_name.clone(),
                    variant_ident: variant_name.clone(),
                    attrs: attrs.cfg_attributes()
                };
                let aspect = Aspect::FFI(Context::EnumVariant { ident: target_name.clone(), variant_ident: variant_name.clone(), attrs: attrs.cfg_attributes() });
                let attrs = AttrsComposition::from(attrs, variant_name, item_scope);
                let composer = match fields {
                    Fields::Unit =>
                        ItemComposer::enum_variant_composer_unit(name_context, attrs, &Punctuated::new(), context),
                    Fields::Unnamed(fields) =>
                        ItemComposer::enum_variant_composer_unnamed(name_context, attrs, &fields.unnamed, context),
                    Fields::Named(fields) =>
                        ItemComposer::enum_variant_composer_named(name_context, attrs, &fields.named, context)
                };
                (composer, (variant_composer, (aspect, fields_context)))
            }).unzip())
        .borrow()
        .expand()
}


fn struct_expansion(item_struct: &ItemStruct, scope: &ScopeChain, scope_context: &ParentComposer<ScopeContext>) -> Expansion {
    let ItemStruct { attrs, fields: ref f, ident: target_name, generics, .. } = item_struct;
    // println!("struct_expansion: {}: [{} --- {}]", item_struct.ident, scope.crate_scope(), scope.self_path_holder());
    // println!("struct_expansion: [{}] --- [{}]", scope, scope_context.borrow().scope);
    match f {
        Fields::Unnamed(ref fields) =>
            ItemComposer::struct_composer_unnamed(target_name, attrs, generics, &fields.unnamed, scope, scope_context),
        Fields::Named(ref fields) =>
            ItemComposer::struct_composer_named(target_name, attrs, generics, &fields.named, scope, scope_context),
        Fields::Unit =>
            ItemComposer::struct_composer_named(target_name, attrs, generics, &Punctuated::new(), scope, scope_context),
        // panic!("Fields::Unit is not supported yet"),
    }.borrow().expand()
}

fn type_expansion(item_type: &ItemType, scope: &ScopeChain, context: &ParentComposer<ScopeContext>) -> Expansion {
    let source = context.borrow();
    let ItemType { ident: target_name, ty, attrs, generics, .. } = item_type;
    // println!("type_expansion: [{}] --- [{}]", scope, source.scope);

    match &**ty {
        Type::BareFn(type_bare_fn) => {
            let full_fn_path = scope.joined_path_holder(target_name);
            SigComposer::with_context(
                full_fn_path.0,
                target_name,
                FnSignatureContext::Bare(target_name.clone(), type_bare_fn.clone()),
                generics,
                attrs,
                scope,
                context)
                .borrow()
                .expand()
            // let mut full_fn_path = scope.joined(target_name);
            // if scope.is_crate_based() {
            //     full_fn_path.replace_first_with(&PathHolder::from(source.scope.crate_ident().to_path()))
            // }
            // Expansion::Function {
            //     comment: DocPresentation::Default(Name::Ident(target_name.clone())),
            //     binding: FnSignatureComposition::from_bare_fn(type_bare_fn, target_name, local_scope.self_path_holder_ref(), &source)
            //         .present(FnSignatureCompositionContext::FFIObjectCallback, &source),
            // }
        },
        _ =>
            ItemComposer::type_alias_composer(target_name, ty, generics, attrs, scope, context)
                .borrow()
                .expand()
    }
}
fn trait_expansion(item_trait: &ItemTrait, scope: &ScopeChain, context: &ParentComposer<ScopeContext>) -> Expansion {
    let self_ty = item_trait.ident.to_type();
    let source = context.borrow();
    // println!("trait_expansion: [{}] --- [{}]", scope, source.scope);
    TraitComposer::from_item_trait(item_trait, self_ty, scope, context)
        .borrow()
        .expand()
    //
    // let mangled_ty = self_ty.resolve(&source).mangle_ident_default();
    // let trait_decomposition = TraitDecompositionPart2::from_item_trait(item_trait, self_ty, scope.self_path_holder_ref(), context);
    // let vtable_name = Name::Vtable(mangled_ty.clone());
    // Expansion::Trait {
    //     comment: DocPresentation::Empty,
    //     vtable: FFIObjectPresentation::TraitVTable {
    //         name: vtable_name.clone(),
    //         fields: trait_decomposition.present(TraitDecompositionPart2Context::VTableInnerFunctions, &source)
    //     },
    //     trait_object: FFIObjectPresentation::TraitObject {
    //         name: Name::TraitObj(mangled_ty),
    //         vtable_name
    //     }
    // }
}

fn fn_expansion(item: &ItemFn, scope: &ScopeChain, context: &ParentComposer<ScopeContext>) -> Expansion {
    let ItemFn { attrs, sig, .. } = item;
    let source = context.borrow();
    // println!("fn_expansion: [{}] --- [{}]", scope, source.scope);
    SigComposer::with_context(
        scope.self_path().crate_named(&source.scope.crate_ident().to_path()),
        &sig.ident,
        FnSignatureContext::ModFn(item.clone()),
        &sig.generics,
        attrs,
        scope,
        context)
        .borrow()
        .expand()


    // let signature = FnSignatureComposition::from_signature(&FnSignatureContext::ModFn(item.clone()), sig, &scope.parent_path_holder(), &source);
    // Expansion::Function {
    //     comment: DocPresentation::Safety(Name::Optional(signature.ident.clone())),
    //     binding: signature.present(FnSignatureCompositionContext::FFIObject, &source),
    // }

}

fn impl_expansion(item_impl: &ItemImpl, scope: &ScopeChain, scope_context: &ParentComposer<ScopeContext>) -> Expansion {
    // println!("impl_expansion: {} {}", item_impl.trait_.as_ref().map_or(format!(""), |(_, p, _)| format!("{} for", p.to_token_stream())), item_impl.self_ty.to_token_stream());
    let ItemImpl { generics: _, trait_, self_ty, items, ..  } = item_impl;
    let source = scope_context.borrow();
    // println!("impl_expansion: [{}] --- [{}]", scope, source.scope);
    let mut full_fn_path = scope.self_path_holder();

    // let mut full_fn_path = self_scope.joined(ident);
    if full_fn_path.is_crate_based() {
        full_fn_path.replace_first_with(&PathHolder::from(scope.crate_ident().to_path()));
    }

    let impl_item_compositions = items.iter().filter_map(|impl_item| {
        // <ferment_example::chain::common::chain_type::DevnetType as ferment_example::chain::common::chain_type::IHaveChainSettings>
        match impl_item {
            ImplItem::Method(ImplItemMethod { sig,  .. }) => {
                let sig_context = FnSignatureContext::Impl(*self_ty.clone(), match trait_ {
                    None => None,
                    Some((_, path, _)) => Some(parse_quote!(#path))
                }, sig.clone());
                Some(SigComposer::with_context(
                    scope.joined_path_holder(&sig.ident).0,
                    &sig.ident,
                    sig_context,
                    &sig.generics,
                    sig.maybe_attrs().unwrap_or(&vec![]),
                    scope,
                    scope_context
                ).borrow().expand())
                // Some(FnSignatureComposition::from_signature(&impl_context, sig, scope.self_path_holder_ref(), &source)
                //     .present(FnSignatureCompositionContext::FFIObject, &source))
            },
            ImplItem::Type(ImplItemType { .. }) => None,
            ImplItem::Const(ImplItemConst { .. }) => None,
            _ => None,
        }
    }).collect();
    // match trait_ {
    //     None => {
    //
    //         // println!("impl_expansion.1: self_ty: {}", self_ty.to_token_stream());
    //         // println!("impl_expansion.1: items: {}", quote!(#(#items)*));
    //
    //         // NEED:
    //         // pub unsafe extern "C" fn get_balance(obj: *const ()) -> u64 {
    //         //     let obj = crate::identity::identity::Identity::get_balance(
    //         //         ferment_interfaces::FFIConversion::ffi_from_const(obj as *const _),
    //         //     );
    //         //     obj
    //         // }
    //
    //         // CURRENT:
    //         // #[no_mangle]
    //         // pub unsafe extern "C" fn get_balance(obj: *const Identity) -> u64 {
    //         //     let obj = crate::identity::identity::Identity::get_balance(
    //         //         &ferment_interfaces::FFIConversion::ffi_from_const(obj),
    //         //     );
    //         //     obj
    //         // }
    //
    //     },
    //     Some((_, _path, _)) => {
    //         // let trait_type = parse_quote!(#path);
    //         // let trait_full_type = source.full_type_for(&trait_type);
    //
    //         // let gtx = source.context.read().unwrap();
    //         // let trait_scope = gtx.actual_scope_for_type(&trait_type, scope);
    //
    //         // println!("impl_expansion.2: trait_scope: {}", trait_scope.to_token_stream());
    //
    //         // let (trait_composition, trait_scope) = ctx.find_item_trait_in_scope(path);
    //
    //         // ctx.item_trait_with_ident_for()
    //         // let item_full_type = source.full_type_for(self_ty);
    //
    //         // let trait_item = ctx.item_trait_with_ident_for()
    //         //
    //         // println!("impl_expansion.2: trait_full_type: {}", trait_full_type.to_token_stream());
    //         // println!("impl_expansion.2: item_ty: {}", item_full_type.to_token_stream());
    //         // // println!("impl_expansion.2: trait_composition: {:?}", trait_composition);
    //         // // println!("impl_expansion.2: trait_scope: {:?}", trait_scope);
    //         // println!("impl_expansion.2: items: {}", quote!(#(#items)*));
    //         // println!("impl_expansion.2: trait: {}", quote!(#path));
    //     }
    // }
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
