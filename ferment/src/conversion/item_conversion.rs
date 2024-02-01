use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::fmt::Formatter;
use std::rc::Rc;
use syn::{Attribute, Expr, Field, Fields, FieldsNamed, FieldsUnnamed, FnArg, Ident, Item, ItemEnum, ItemFn, ItemImpl, ItemMod, ItemStruct, ItemTrait, ItemType, ItemUse, Meta, NestedMeta, parse_quote, Path, PathSegment, PatType, ReturnType, Signature, TraitBound, TraitItem, TraitItemConst, TraitItemMethod, TraitItemType, Type, TypeParamBound, TypePath, TypePtr, TypeReference, TypeTraitObject, UseGlob, UseGroup, UseName, UsePath, UseRename, UseTree, Variant};
use quote::{format_ident, quote, ToTokens};
use syn::__private::TokenStream2;
use crate::composer::{AttrsComposer, Composer, ConversionsComposer, ItemComposer};
use crate::composition::{AttrsComposition, collect_generic_types_in_type, Composition, context::FnSignatureCompositionContext, FnSignatureComposition, GenericConversion, ImportComposition, TraitDecompositionPart2};
use crate::composition::context::TraitDecompositionPart2Context;
use crate::context::{Scope, ScopeChain, ScopeContext};
use crate::conversion::{Conversion, FieldTypeConversion, ImportConversion, MacroAttributes, MacroType, ObjectConversion};
use crate::formatter::{format_token_stream, format_type_holders};
use crate::helper::{ffi_destructor_name, ffi_trait_obj_name, ffi_vtable_name};
use crate::holder::{PathHolder, TypeHolder};
use crate::interface::{CURLY_BRACES_FIELDS_PRESENTER, EMPTY_DESTROY_PRESENTER, NAMED_CONVERSION_PRESENTER, package_unboxed_root, ROOT_DESTROY_CONTEXT_PRESENTER, ROUND_BRACES_FIELDS_PRESENTER, SIMPLE_CONVERSION_PRESENTER, SIMPLE_PRESENTER, SIMPLE_TERMINATED_PRESENTER};
use crate::presentation::{BindingPresentation, DropInterfacePresentation, FromConversionPresentation, ScopeContextPresentable, ToConversionPresentation};
use crate::presentation::context::{OwnedItemPresenterContext, IteratorPresentationContext, OwnerIteratorPresentationContext};
use crate::presentation::conversion_interface_presentation::ConversionInterfacePresentation;
use crate::presentation::doc_presentation::DocPresentation;
use crate::presentation::expansion::Expansion;
use crate::presentation::ffi_object_presentation::FFIObjectPresentation;


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
        f.write_fmt(format_args!("{}: {}", self.name(), self.ident().to_token_stream()))
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

// impl<'a> TryFrom<&'a Item> for ItemConversion {
//     type Error = String;
//     fn try_from(value: &'a Item) -> Result<Self, Self::Error> {
//         match value {
//             Item::Mod(item) => Ok(Self::Mod(item.clone(), EMPTY)),
//             Item::Struct(item) => Ok(Self::Struct(item.clone(), EMPTY)),
//             Item::Enum(item) => Ok(Self::Enum(item.clone(), EMPTY)),
//             Item::Type(item) => Ok(Self::Type(item.clone(), EMPTY)),
//             Item::Fn(item) => Ok(Self::Fn(item.clone(), EMPTY)),
//             Item::Trait(item) => Ok(Self::Trait(item.clone(), EMPTY)),
//             Item::Impl(item) => Ok(Self::Impl(item.clone(), EMPTY)),
//             item => Err(format!("Error: {}", item.to_token_stream()))
//         }
//     }
// }

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

// impl<'a> TryFrom<(Item, &'a PathHolder)> for ItemConversion {
//     type Error = String;
//     fn try_from(value: (Item, &'a PathHolder)) -> Result<Self, Self::Error> {
//         match value.0 {
//             Item::Mod(item) => Ok(Self::Mod(item, value.1.clone())),
//             Item::Struct(item) => Ok(Self::Struct(item, value.1.clone())),
//             Item::Enum(item) => Ok(Self::Enum(item, value.1.clone())),
//             Item::Type(item) => Ok(Self::Type(item, value.1.clone())),
//             Item::Fn(item) => Ok(Self::Fn(item, value.1.clone())),
//             Item::Trait(item) => Ok(Self::Trait(item, value.1.clone())),
//             Item::Impl(item) => Ok(Self::Impl(item, value.1.clone())),
//             item => Err(format!("Error: {}", item.to_token_stream()))
//         }
//     }
// }

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


    pub fn ident(&self) -> Ident {
        match self {
            ItemConversion::Mod(ItemMod { ident, .. }, ..) => ident.clone(),
            ItemConversion::Struct(ItemStruct { ident, .. }, ..) => ident.clone(),
            ItemConversion::Enum(ItemEnum { ident, .. }, ..) => ident.clone(),
            ItemConversion::Type(ItemType { ident, .. }, ..) => ident.clone(),
            ItemConversion::Fn(ItemFn { sig: Signature { ident, .. }, .. }, ..) => ident.clone(),
            ItemConversion::Trait(ItemTrait { ident, .. }, ..) => ident.clone(),
            ItemConversion::Impl(ItemImpl { self_ty, .. }, ..) => type_ident(self_ty).unwrap(),
            ItemConversion::Use(ItemUse { tree, .. }, ..) =>
                Self::fold_use(tree).first().cloned().unwrap().clone(),
        }
    }

    pub fn is_labeled_with_macro_type(path: &Path, macro_type: &str) -> bool {
        path.segments
            .iter()
            .any(|segment| macro_type == segment.ident.to_string().as_str())
    }

    pub fn is_labeled_for_export(path: &Path) -> bool {
        Self::is_labeled_with_macro_type(path, "export")
    }

    pub fn is_labeled_for_register(path: &Path) -> bool {
        Self::is_labeled_with_macro_type(path, "register")
    }

    pub fn is_owner_labeled_with_trait_implementation(path: &Path) -> bool {
        Self::is_labeled_with_macro_type(path, "export")
    }

    pub fn has_export_macro_attribute(&self) -> bool {
        self.attrs().iter().filter(|Attribute { path, .. }| Self::is_labeled_for_export(path)).count() > 0
    }

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

    pub fn handle_attributes_with_handler<F: FnMut(MacroAttributes)>(&self, attrs: &[Attribute], mut handler: F) {
        attrs.iter()
            .for_each(|attr|
                if Self::is_labeled_for_export(&attr.path) || Self::is_owner_labeled_with_trait_implementation(&attr.path) {
                    let mut arguments = Vec::<Path>::new();
                    if let Ok(Meta::List(meta_list)) = attr.parse_meta() {
                        meta_list.nested.iter().for_each(|meta| {
                            if let NestedMeta::Meta(Meta::Path(path)) = meta {
                                arguments.push(path.clone());
                            }
                        });
                    }
                    handler(MacroAttributes {
                        path: attr.path.clone(),
                        arguments
                    })
                }
            )
    }

    pub fn collect_compositions(&self) -> Vec<TypeHolder> {
        let mut type_and_paths: Vec<TypeHolder> = Vec::new();
        let mut cache_type = |ty: &Type|
            type_and_paths.push(TypeHolder(ty.clone()));
        let mut cache_fields = |fields: &Fields, _attrs: &MacroAttributes| match fields {
            Fields::Unnamed(FieldsUnnamed { unnamed: fields, .. }) |
            Fields::Named(FieldsNamed { named: fields, .. }) =>
                fields.iter().for_each(|field| cache_type(&field.ty)),
            Fields::Unit => {}
        };
        match self {
            Self::Mod(ItemMod { content: Some((_, items)), .. }, scope) =>
                items.iter()
                    .flat_map(|m| Self::try_from((m, scope.clone())))
                    .for_each(|conversion|
                        type_and_paths.extend(conversion.collect_compositions())),
            Self::Struct(item_struct, ..) =>
                self.handle_attributes_with_handler(&item_struct.attrs, |attrs|
                    cache_fields(&item_struct.fields, &attrs)),
            Self::Enum(item_enum, ..) =>
                self.handle_attributes_with_handler(&item_enum.attrs, |attrs|
                    item_enum.variants.iter().for_each(|Variant { fields, .. }|
                        cache_fields(fields, &attrs))),
            Self::Type(ItemType { attrs, ty, .. }, ..) =>
                self.handle_attributes_with_handler(attrs, |_attrs|
                    cache_type(ty)),
            Self::Fn(item_fn, ..) =>
                self.handle_attributes_with_handler(&item_fn.attrs, |_attrs| {
                    item_fn.sig.inputs.iter().for_each(|arg|
                        if let FnArg::Typed(PatType { ty, .. }) = arg {
                            cache_type(ty);
                        });
                    if let ReturnType::Type(_, ty) = &item_fn.sig.output {
                        cache_type(ty);
                    }
                }),
            Self::Trait(item_trait, ..) => self.handle_attributes_with_handler(&item_trait.attrs, |_attrs| {
                item_trait.items.iter().for_each(|trait_item| match trait_item {
                    TraitItem::Type(TraitItemType { default: Some((_, ty)), .. }) =>
                        cache_type(ty),
                    TraitItem::Method(TraitItemMethod { sig, .. }) => {
                        sig.inputs.iter().for_each(|arg|
                            if let FnArg::Typed(PatType { ty, .. }) = arg {
                                cache_type(ty);
                            });
                        if let ReturnType::Type(_, ty) = &sig.output {
                            cache_type(ty);
                        }
                    },
                    TraitItem::Const(TraitItemConst { ty, .. }) =>
                        cache_type(ty),
                    _ => {}
                });
            }),
            _ => {}
        }

        type_and_paths
    }

    fn import_pair(path: &Path, imports: &HashMap<PathHolder, Path>) -> (ImportConversion, PathHolder) {
        let original_or_external_pair = |value| {
            println!("import_pair:::value: {}", format_token_stream(value));
            let scope = PathHolder::from(value);
            (if scope.has_belong_to_current_crate() { ImportConversion::Original } else { ImportConversion::External }, scope)
        };
        let path_scope= PathHolder::from(path);
        println!("import_pair: {}", format_token_stream(path));
        match path.get_ident() {
            Some(ident) => match ident.to_string().as_str() {
                // accessible without specifying scope
                "i8" | "u8" | "i16" | "u16" | "i32" | "u32" | "i64" | "u64" | "i128" | "u128" | "isize"
                | "usize" | "bool" | "str" | "String" | "Vec" | "Option" | "Box" =>
                    (ImportConversion::None, parse_quote!(#ident)),
                // they are defined in the same scope, so it should be imported sometimes outside this scope (export-only)
                _ =>
                    imports.get(&path_scope)
                        .map_or((ImportConversion::Inner, parse_quote!(#ident)), original_or_external_pair)
            },
            // partial chunk
            None => {
                imports.get(&path_scope)
                    .map_or(match path.segments.last().unwrap().ident.to_string().as_str() {
                    "Vec" | "Option" | "Box" => (ImportConversion::None, path_scope),
                    _ => (ImportConversion::ExternalChunk, path_scope),
                }, original_or_external_pair)
            }
        }
    }

    fn cache_type_in(container: &mut HashMap<ImportConversion, HashSet<ImportComposition>>, ty: &Type, scope: &ScopeChain, imports: &HashMap<PathHolder, Path>) {
        // Types which are used as a part of types (for generics and composite types)
        // let type_conversion = TypeHolder::from(ty);
        // let involved = <TypePathHolder as Conversion>::nested_items(ty, &VisitorContext::Unknown);
        let involved = <TypeHolder as Conversion>::nested_items(ty, scope);
        involved.iter()
            .for_each(|ty| {
                let path: Path = parse_quote!(#ty);
                if let Some(PathSegment { ident, .. }) = path.segments.last() {
                    let (import_type, scope) = Self::import_pair(&path, imports);
                    container
                        .entry(import_type)
                        .or_default()
                        .insert(ImportComposition::from((ident, &scope)));
                }
            });
    }

    fn cache_fields_in(container: &mut HashMap<ImportConversion, HashSet<ImportComposition>>, fields: &Fields, scope: &ScopeChain, imports: &HashMap<PathHolder, Path>) {
        match fields {
            Fields::Unnamed(FieldsUnnamed { unnamed: fields, .. }) |
            Fields::Named(FieldsNamed { named: fields, .. }) =>
                fields.iter()
                    .for_each(|field| Self::cache_type_in(container, &field.ty, scope, imports)),
            Fields::Unit => {}
        }
    }

    pub fn get_used_imports(&self, imports: &HashMap<PathHolder, Path>) -> HashMap<ImportConversion, HashSet<ImportComposition>> {
        self.classify_imports(imports)
            .into_iter()
            .filter_map(|(import_type, used_imports)|
                import_type.get_imports_for(used_imports))
            .collect()
    }

    pub fn classify_imports(&self, imports: &HashMap<PathHolder, Path>) -> HashMap<ImportConversion, HashSet<ImportComposition>> {
        let mut container = HashMap::new();
        match self {
            ItemConversion::Mod(ItemMod { content: Some((_, items)), .. }, scope) =>
                items.iter()
                    .flat_map(|item| Self::try_from((item, scope.clone())))
                    .for_each(|conversion|
                        container.extend(conversion.classify_imports(imports))),
            ItemConversion::Struct(item_struct, scope) =>
                self.handle_attributes_with_handler(&item_struct.attrs, |_path|
                    Self::cache_fields_in(&mut container, &item_struct.fields, scope, imports)),
            ItemConversion::Enum(item_enum, scope) =>
                self.handle_attributes_with_handler(&item_enum.attrs, |_path| item_enum.variants.iter().for_each(|Variant { fields, .. }|
                    Self::cache_fields_in(&mut container, fields, scope, imports))),
            ItemConversion::Type(ItemType { attrs, ty, .. }, scope) =>
                self.handle_attributes_with_handler(attrs, |_path|
                    Self::cache_type_in(&mut container, ty, scope, imports)),
            ItemConversion::Fn(item_fn, scope) =>
                self.handle_attributes_with_handler(&item_fn.attrs, |_path| {
                    item_fn.sig.inputs.iter().for_each(|arg| {
                        if let FnArg::Typed(PatType { ty, .. }) = arg {
                            Self::cache_type_in(&mut container, ty, scope, imports)
                        }
                    });
                    if let ReturnType::Type(_, ty) = &item_fn.sig.output {
                        Self::cache_type_in(&mut container, ty, scope, imports)
                    };
                }),
            ItemConversion::Trait(item_trait, scope) =>
                self.handle_attributes_with_handler(&item_trait.attrs, |_path| {
                    item_trait.items.iter().for_each(|trait_item| match trait_item {
                        TraitItem::Method(TraitItemMethod { sig, .. }) => {
                            sig.inputs.iter().for_each(|arg| {
                                if let FnArg::Typed(PatType { ty, .. }) = arg {
                                    Self::cache_type_in(&mut container, ty, scope, imports)
                                }
                            });
                            if let ReturnType::Type(_, ty) = &sig.output {
                                Self::cache_type_in(&mut container, ty, scope, imports)
                            };
                        },
                        TraitItem::Type(TraitItemType { default: Some((_, ty)), .. }) =>
                            Self::cache_type_in(&mut container, ty, scope, imports),
                        _ => {}
                    });
                }),
            _ => {}
        }
        container
    }

    pub(crate) fn find_generics(&self) -> HashSet<TypeHolder> {
        let compositions = self.collect_compositions();
        // collect all types with generics and ensure their uniqueness
        // since we don't want to implement interface multiple times for same object
        let mut generics: HashSet<TypeHolder> = HashSet::new();
        compositions
            .iter()
            .for_each(|TypeHolder(field_type)|
                collect_generic_types_in_type(field_type, &mut generics));
        if !generics.is_empty() {
            println!("GENERICS in {}: {}", self.ident(), format_type_holders(&generics));
        }
        generics
    }

    pub fn find_generics_fq(&self, scope_types: &HashMap<TypeHolder, ObjectConversion>) -> HashSet<GenericConversion> {
        self.find_generics()
            .iter()
            .filter_map(|holder| scope_types.get(holder))
            .map(GenericConversion::from)
            .collect()
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
                let signature = FnSignatureComposition::from_signature(&item.sig, scope.self_scope().self_scope.popped(), &scope_context.borrow());
                // let signature = FnSignatureComposition::from_signature(&item.sig, scope.popped(), &scope_context.borrow());
                Expansion::Function {
                    comment: DocPresentation::Safety(signature.ident.to_token_stream()),
                    ffi_presentation: signature.present(FnSignatureCompositionContext::FFIObject, &scope_context.borrow()),
                }
            },
            ItemConversion::Trait(item, scope) =>
                trait_expansion(item, scope, scope_context),
            ItemConversion::Impl(_item, _scope) =>
                Expansion::Impl { comment: DocPresentation::Empty },
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
                                 OwnedItemPresenterContext::Named(FieldTypeConversion::Named(quote!(#ident), ctx.ffi_full_dictionary_field_type_presenter(field_type)), false))


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
            ctor_arguments: composer_owned.ffi_conversions_composer.bindings_composer.compose_arguments(|field_type|
                OwnedItemPresenterContext::Named(field_type, false)),
            body_presentation: composer_owned.ffi_conversions_composer.bindings_composer.compose_field_names(&ctx, |field_type|
                OwnedItemPresenterContext::DefaultField(field_type)),
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
        destructor_ident: ffi_destructor_name(target_name).to_token_stream()
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
    let trait_decomposition = TraitDecompositionPart2::from_trait_items(items, &scope.self_scope().self_scope, &context);
    let fields = trait_decomposition.present(TraitDecompositionPart2Context::VTableInnerFunctions, &context);
    let trait_obj_name = ffi_trait_obj_name(ident);
    let vtable_name = ffi_vtable_name(ident);
    println!("trait_expansion: {}: {}", trait_obj_name, vtable_name);
    Expansion::Trait {
        comment: DocPresentation::Empty,
        vtable: FFIObjectPresentation::TraitVTable {
            name: vtable_name.to_token_stream(),
            fields
        },
        trait_object: FFIObjectPresentation::TraitObject {
            name: trait_obj_name.to_token_stream(),
            vtable_name: vtable_name.to_token_stream()
        }
    }
}

fn type_expansion(item_type: &ItemType, scope: &ScopeChain, context: &Rc<RefCell<ScopeContext>>) -> Expansion {
    // let scope = scope.self_scope();
    let ItemType { ident, ty, attrs, .. } = item_type;
    let ctx = context.borrow();
    match &**ty {
        Type::BareFn(bare_fn) => {
            let decomposition = FnSignatureComposition::from_bare_fn(bare_fn, ident, scope.self_scope().self_scope.clone(), &ctx);
            Expansion::Callback {
                comment: DocPresentation::Default(quote!(#ident)),
                ffi_presentation: decomposition.present(FnSignatureCompositionContext::FFIObjectCallback, &ctx),
            }
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
