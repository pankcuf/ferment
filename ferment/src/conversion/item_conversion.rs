use std::collections::{HashMap, HashSet};
use std::fmt::Formatter;
use syn::{Attribute, Expr, Field, Fields, FieldsNamed, FieldsUnnamed, FnArg, GenericParam, Ident, ImplItem, ImplItemConst, ImplItemMethod, ImplItemType, Item, ItemEnum, ItemFn, ItemImpl, ItemMod, ItemStruct, ItemTrait, ItemType, ItemUse, Meta, NestedMeta, parse_quote, Path, PathSegment, PatType, ReturnType, Signature, TraitBound, TraitItem, TraitItemConst, TraitItemMethod, TraitItemType, Type, TypeParam, TypeParamBound, TypePath, TypePtr, TypeReference, TypeTraitObject, UseGlob, UseGroup, UseName, UsePath, UseRename, UseTree, Variant};
use quote::{format_ident, quote, ToTokens};
use syn::__private::TokenStream2;
use crate::composer::{ConversionsComposer, ItemComposer};
use crate::composition::{FnReturnTypeDecomposition, FnSignatureDecomposition, GenericConversion, ImportComposition, TraitDecompositionPart1, TraitDecompositionPart2, TypeComposition};
use crate::context::{ScopeContext, TraitCompositionPart1, VisitorContext};
use crate::conversion::{Conversion, GenericPathConversion, ImportConversion, PathConversion, TypeConversion};
use crate::conversion::macro_conversion::{MacroAttributes, MacroType};
use crate::formatter::format_token_stream;
use crate::helper::{ffi_destructor_name, ffi_trait_obj_name, ffi_vtable_name, path_arguments_to_types};
use crate::holder::{EMPTY, PathHolder, TypeHolder};
use crate::interface::{CURLY_BRACES_FIELDS_PRESENTER, CURLY_ITER_PRESENTER, DEFAULT_DICT_FIELD_TYPE_PRESENTER, EMPTY_DESTROY_PRESENTER, EMPTY_ITERATOR_PRESENTER, ENUM_DESTROY_PRESENTER, ENUM_NAMED_VARIANT_PRESENTER, ENUM_PRESENTER, ENUM_UNIT_FIELDS_PRESENTER, ENUM_UNNAMED_VARIANT_PRESENTER, FFI_FULL_DICTIONARY_FIELD_TYPE_PRESENTER, NAMED_CONVERSION_PRESENTER, NAMED_DICT_FIELD_TYPE_PRESENTER, NAMED_STRUCT_PRESENTER, NO_FIELDS_PRESENTER, package_unboxed_root, ROOT_DESTROY_CONTEXT_PRESENTER, ROUND_BRACES_FIELDS_PRESENTER, ROUND_ITER_PRESENTER, SIMPLE_CONVERSION_PRESENTER, SIMPLE_PRESENTER, SIMPLE_TERMINATED_PRESENTER, UNNAMED_STRUCT_PRESENTER};
use crate::presentation::{BindingPresentation, ConversionInterfacePresentation, DocPresentation, DropInterfacePresentation, Expansion, FFIObjectPresentation, FromConversionPresentation, ToConversionPresentation, TraitVTablePresentation};
use crate::visitor::Visitor;


#[derive(Clone)]
pub enum ItemConversion {
    Mod(ItemMod, PathHolder),
    Struct(ItemStruct, PathHolder),
    Enum(ItemEnum, PathHolder),
    Type(ItemType, PathHolder),
    Fn(ItemFn, PathHolder),
    Use(ItemUse, PathHolder),
    Trait(ItemTrait, PathHolder),
    Impl(ItemImpl, PathHolder)
}

impl ItemConversion {
    pub(crate) fn is_impl(&self) -> bool {
        if let Self::Impl(..) = self {
            true
        } else {
            false
        }
    }
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

impl<'a> TryFrom<&'a Item> for ItemConversion {
    type Error = String;
    fn try_from(value: &'a Item) -> Result<Self, Self::Error> {
        match value {
            Item::Mod(item) => Ok(Self::Mod(item.clone(), EMPTY)),
            Item::Struct(item) => Ok(Self::Struct(item.clone(), EMPTY)),
            Item::Enum(item) => Ok(Self::Enum(item.clone(), EMPTY)),
            Item::Type(item) => Ok(Self::Type(item.clone(), EMPTY)),
            Item::Fn(item) => Ok(Self::Fn(item.clone(), EMPTY)),
            Item::Trait(item) => Ok(Self::Trait(item.clone(), EMPTY)),
            Item::Impl(item) => Ok(Self::Impl(item.clone(), EMPTY)),
            item => Err(format!("Error: {}", item.to_token_stream()))
        }
    }
}

impl<'a> TryFrom<(&'a Item, &'a PathHolder)> for ItemConversion {
    type Error = String;
    fn try_from(value: (&'a Item, &'a PathHolder)) -> Result<Self, Self::Error> {
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

impl<'a> TryFrom<(Item, &'a PathHolder)> for ItemConversion {
    type Error = String;
    fn try_from(value: (Item, &'a PathHolder)) -> Result<Self, Self::Error> {
        match value.0 {
            Item::Mod(item) => Ok(Self::Mod(item, value.1.clone())),
            Item::Struct(item) => Ok(Self::Struct(item, value.1.clone())),
            Item::Enum(item) => Ok(Self::Enum(item, value.1.clone())),
            Item::Type(item) => Ok(Self::Type(item, value.1.clone())),
            Item::Fn(item) => Ok(Self::Fn(item, value.1.clone())),
            Item::Trait(item) => Ok(Self::Trait(item, value.1.clone())),
            Item::Impl(item) => Ok(Self::Impl(item, value.1.clone())),
            item => Err(format!("Error: {}", item.to_token_stream()))
        }
    }
}

impl<'a> TryFrom<(&'a Item, PathHolder)> for ItemConversion {
    type Error = String;
    fn try_from(value: (&'a Item, PathHolder)) -> Result<Self, Self::Error> {
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

pub fn type_ident(ty: &Type) -> Ident {
    match ty {
        Type::Path(TypePath { path, .. }) =>
            path.segments.last().unwrap().ident.clone(),
        Type::Reference(TypeReference { elem, .. }) |
        Type::Ptr(TypePtr { elem, .. }) =>
            type_ident(elem),
        Type::TraitObject(TypeTraitObject { bounds, .. }) => {
            bounds.iter().find_map(|b| match b {
                TypeParamBound::Trait(TraitBound { path, ..}) => Some(path.segments.last().unwrap().ident.clone()),
                _ => None
            }).unwrap()
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

    pub fn scope(&self) -> &PathHolder {
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

    fn fold_use(tree: &UseTree) -> Vec<&Ident> {
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
            ItemConversion::Impl(ItemImpl { self_ty, .. }, ..) => type_ident(self_ty),
            ItemConversion::Use(ItemUse { tree, .. }, ..) =>
                Self::fold_use(tree).first().cloned().unwrap().clone(),
        }
    }

    // pub fn path(&self) -> &Type {
    //     match self {
    //         ItemConversion::Mod(ItemMod { ident, .. }, ..) => ident,
    //         ItemConversion::Struct(ItemStruct { ident, .. }, ..) => ident,
    //         ItemConversion::Enum(ItemEnum { ident, .. }, ..) => ident,
    //         ItemConversion::Type(ItemType { ident, .. }, ..) => ident,
    //         ItemConversion::Fn(ItemFn { sig: Signature { ident, .. }, .. }, ..) => ident,
    //         ItemConversion::Trait(ItemTrait { ident, .. }, ..) => ident,
    //         ItemConversion::Impl(ItemImpl { self_ty, .. }, ..) => {
    //             let path: Path = parse_quote!(#self_ty);
    //             &path.segments.last().unwrap().ident
    //         },
    //         ItemConversion::Use(ItemUse { tree, .. }, ..) => Self::fold_use(tree).first().unwrap(),
    //     }
    // }

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

    pub fn collect_all_items(&self) -> Vec<ItemConversion> {
        match self {
            Self::Mod(ItemMod { content: Some((_, items)), .. }, scope) => {
                let mut all_labeled_items: Vec<ItemConversion> = Vec::new();
                items.iter().for_each(|item| match Self::try_from(item) {
                    Ok(ItemConversion::Mod(item_mod, _)) =>
                        all_labeled_items.extend(ItemConversion::Mod(item_mod.clone(), scope.clone()).collect_all_items()),
                    Ok(conversion) =>
                        all_labeled_items.push(conversion),
                    _ => {}
                });
                all_labeled_items
            },
            _ => vec![],
        }
    }

    pub fn collect_compositions(&self) -> Vec<TypeHolder> {
        let mut type_and_paths: Vec<TypeHolder> = Vec::new();
        let mut cache_type = |ty: &Type|
            type_and_paths.push(TypeHolder(ty.clone()));
        let mut cache_fields = |fields: &Fields, attrs: &MacroAttributes| match fields {
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
                self.handle_attributes_with_handler(attrs, |attrs|
                    cache_type(ty)),
            Self::Fn(item_fn, ..) =>
                self.handle_attributes_with_handler(&item_fn.attrs, |attrs| {
                    item_fn.sig.inputs.iter().for_each(|arg|
                        if let FnArg::Typed(PatType { ty, .. }) = arg {
                            cache_type(ty);
                        });
                    if let ReturnType::Type(_, ty) = &item_fn.sig.output {
                        cache_type(ty);
                    }
                }),
            Self::Trait(item_trait, ..) => self.handle_attributes_with_handler(&item_trait.attrs, |attrs| {
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

    // pub fn collect_compositions(&self) -> Vec<TypeAndPathHolder> {
    //     let mut type_and_paths: Vec<TypeAndPathHolder> = Vec::new();
    //     let mut cache_type = |ty: &Type, path: &Path|
    //         type_and_paths.push(TypeAndPathHolder(ty.clone(), path.clone()));
    //     let mut cache_fields = |fields: &Fields, attrs: &MacroAttributes| match fields {
    //         Fields::Unnamed(FieldsUnnamed { unnamed: fields, .. }) |
    //         Fields::Named(FieldsNamed { named: fields, .. }) =>
    //             fields.iter().for_each(|field| cache_type(&field.ty, &attrs.path)),
    //         Fields::Unit => {}
    //     };
    //     match self {
    //         Self::Mod(ItemMod { content: Some((_, items)), .. }, scope) =>
    //             items.iter()
    //                 .flat_map(|m| Self::try_from((m, scope.clone())))
    //                 .for_each(|conversion|
    //                     type_and_paths.extend(conversion.collect_compositions())),
    //         Self::Struct(item_struct, ..) =>
    //             self.handle_attributes_with_handler(&item_struct.attrs, |attrs|
    //                 cache_fields(&item_struct.fields, &attrs)),
    //         Self::Enum(item_enum, ..) =>
    //             self.handle_attributes_with_handler(&item_enum.attrs, |attrs|
    //                 item_enum.variants.iter().for_each(|Variant { fields, .. }|
    //                     cache_fields(fields, &attrs))),
    //         Self::Type(ItemType { attrs, ty, .. }, ..) =>
    //             self.handle_attributes_with_handler(attrs, |attrs|
    //                 cache_type(ty, &attrs.path)),
    //         Self::Fn(item_fn, ..) =>
    //             self.handle_attributes_with_handler(&item_fn.attrs, |attrs| {
    //                 item_fn.sig.inputs.iter().for_each(|arg|
    //                     if let FnArg::Typed(PatType { ty, .. }) = arg {
    //                         cache_type(ty, &attrs.path);
    //                     });
    //                 if let ReturnType::Type(_, ty) = &item_fn.sig.output {
    //                     cache_type(ty, &attrs.path);
    //                 }
    //             }),
    //         Self::Trait(item_trait, ..) => self.handle_attributes_with_handler(&item_trait.attrs, |attrs| {
    //             item_trait.items.iter().for_each(|trait_item| match trait_item {
    //                 TraitItem::Type(TraitItemType { default: Some((_, ty)), .. }) =>
    //                     cache_type(ty, &attrs.path),
    //                 TraitItem::Method(TraitItemMethod { sig, .. }) => {
    //                     sig.inputs.iter().for_each(|arg|
    //                         if let FnArg::Typed(PatType { ty, .. }) = arg {
    //                             cache_type(ty, &attrs.path);
    //                         });
    //                     if let ReturnType::Type(_, ty) = &sig.output {
    //                         cache_type(ty, &attrs.path);
    //                     }
    //                 },
    //                 TraitItem::Const(TraitItemConst { ty, .. }) =>
    //                     cache_type(ty, &attrs.path),
    //                 _ => {}
    //             });
    //         }),
    //         _ => {}
    //     }
    //
    //     type_and_paths
    // }

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
                    (ImportConversion::None, PathHolder::new(parse_quote!(#ident))),
                // they are defined in the same scope, so it should be imported sometimes outside this scope (export-only)
                _ =>
                    imports.get(&path_scope)
                        .map_or((ImportConversion::Inner, PathHolder::new(parse_quote!(#ident))), original_or_external_pair)
            },
            // partial chunk
            None => {
                imports.get(&path_scope)
                    .map_or(match path.segments.last().unwrap().ident.to_string().as_str() {
                    "Vec" | "Option" | "Box" => (ImportConversion::None, path_scope),
                    _ => (ImportConversion::ExternalChunk, path_scope),
                }, original_or_external_pair)

                // match path.segments.last() {
                //     Some(PathSegment { ident, .. }) => {
                //         imports.get(&path_scope)
                //             .map_or(match ident.to_string().as_str() {
                //                 "Vec" | "Option" => (ImportType::None, Scope::new(parse_quote!(#ident))),
                //                 _ => (ImportType::ExternalChunk, Scope::from(path)),
                //             }, original_or_external_pair)
                //     },
                //     _ => (ImportType::None, EMPTY),
                // }
            }
        }
    }

    fn cache_type_in(container: &mut HashMap<ImportConversion, HashSet<ImportComposition>>, ty: &Type, imports: &HashMap<PathHolder, Path>) {
        // Types which are used as a part of types (for generics and composite types)
        // let type_conversion = TypeHolder::from(ty);
        // let involved = <TypePathHolder as Conversion>::nested_items(ty, &VisitorContext::Unknown);
        let involved = <TypeHolder as Conversion>::nested_items(ty, &VisitorContext::Unknown);
        involved.iter()
            .for_each(|ty| {
                let path: Path = parse_quote!(#ty);
                println!("cache_type_in: {}", format_token_stream(&path));
                if let Some(PathSegment { ident, .. }) = path.segments.last() {
                    let (import_type, scope) = Self::import_pair(&path, imports);
                    container
                        .entry(import_type)
                        .or_default()
                        .insert(ImportComposition::from((ident, &scope)));
                }
            });
        // TypeHolder::from(ty)
        //     .get_all_type_paths_involved()
        //     .iter()
        //     .for_each(|type_path| {
        //         let path = &type_path.0.path;
        //         println!("cache_type_in: {}", format_token_stream(path));
        //         if let Some(PathSegment { ident, .. }) = path.segments.last() {
        //             let (import_type, scope) = Self::import_pair(path, imports);
        //             container
        //                 .entry(import_type)
        //                 .or_default()
        //                 .insert(ImportComposition::from((ident, &scope)));
        //         }
        // });
    }

    fn cache_fields_in(container: &mut HashMap<ImportConversion, HashSet<ImportComposition>>, fields: &Fields, imports: &HashMap<PathHolder, Path>) {
        match fields {
            Fields::Unnamed(FieldsUnnamed { unnamed: fields, .. }) |
            Fields::Named(FieldsNamed { named: fields, .. }) =>
                fields.iter()
                    .for_each(|field| Self::cache_type_in(container, &field.ty, imports)),
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
            Self::Mod(ItemMod { content: Some((_, items)), .. }, scope) =>
                items.iter()
                    .flat_map(|item| Self::try_from((item, scope.clone())))
                    .for_each(|conversion|
                        container.extend(conversion.classify_imports(imports))),
            Self::Struct(item_struct, ..) =>
                self.handle_attributes_with_handler(&item_struct.attrs, |_path|
                    Self::cache_fields_in(&mut container, &item_struct.fields, imports)),
            Self::Enum(item_enum, ..) =>
                self.handle_attributes_with_handler(&item_enum.attrs, |_path| item_enum.variants.iter().for_each(|Variant { fields, .. }|
                    Self::cache_fields_in(&mut container, fields, imports))),
            Self::Type(ItemType { attrs, ty, .. }, ..) =>
                self.handle_attributes_with_handler(attrs, |_path|
                    Self::cache_type_in(&mut container, ty, imports)),
            Self::Fn(item_fn, ..) =>
                self.handle_attributes_with_handler(&item_fn.attrs, |_path| {
                    item_fn.sig.inputs.iter().for_each(|arg| {
                        if let FnArg::Typed(PatType { ty, .. }) = arg {
                            Self::cache_type_in(&mut container, ty, imports)
                        }
                    });
                    if let ReturnType::Type(_, ty) = &item_fn.sig.output {
                        Self::cache_type_in(&mut container, ty, imports)
                    };
                }),
            Self::Trait(item_trait, ..) =>
                self.handle_attributes_with_handler(&item_trait.attrs, |_path| {
                    item_trait.items.iter().for_each(|trait_item| match trait_item {
                        TraitItem::Method(TraitItemMethod { sig, .. }) => {
                            sig.inputs.iter().for_each(|arg| {
                                if let FnArg::Typed(PatType { ty, .. }) = arg {
                                    Self::cache_type_in(&mut container, ty, imports)
                                }
                            });
                            if let ReturnType::Type(_, ty) = &sig.output {
                                Self::cache_type_in(&mut container, ty, imports)
                            };
                        },
                        TraitItem::Type(TraitItemType { default: Some((_, ty)), .. }) =>
                            Self::cache_type_in(&mut container, ty, imports),
                        _ => {}
                    });
                }),
            _ => {}
        }
        container
    }
    fn collect_generic_types_in_type(field_type: &Type, generics: &mut HashSet<TypeHolder>) {
        println!("collect_generic_types_in_type: {}", format_token_stream(field_type));
        match field_type {
            Type::Reference(TypeReference { mutability: _, elem, .. }) =>
                Self::collect_generic_types_in_type(elem, generics),
            Type::Path(TypePath { path, .. }) => {
                match PathConversion::from(path) {
                    PathConversion::Complex(path) => {
                        println!("collect_generic_types_in_type: typepath complex: {}", format_token_stream(&path));
                        if let Some(last_segment) = path.segments.last() {
                            if last_segment.ident.to_string().as_str() == "Option" {
                                Self::collect_generic_types_in_type(path_arguments_to_types(&last_segment.arguments)[0], generics);
                            }
                        }
                    },
                    PathConversion::Generic(GenericPathConversion::Result(path)) |
                    PathConversion::Generic(GenericPathConversion::Vec(path)) |
                    PathConversion::Generic(GenericPathConversion::Map(path)) => {
                        println!("collect_generic_types_in_type: typepath generic: {}", format_token_stream(&path));
                        path_arguments_to_types(&path.segments.last().unwrap().arguments)
                            .iter()
                            .for_each(|field_type|
                                add_generic_type(field_type, generics));
                        generics.insert(TypeHolder(field_type.clone()));
                    },
                    _ => {}
                }
            },
            _ => {}
        }
    }
    // fn collect_generic_types_in_type(field_type: &Type, generics: &mut HashSet<TypeAndPathHolder>) {
    //     println!("collect_generic_types_in_type: {}", format_token_stream(field_type));
    //     match field_type {
    //         Type::Reference(TypeReference { mutability: _, elem, .. }) =>
    //             Self::collect_generic_types_in_type(elem, generics),
    //         Type::Path(TypePath { path, .. }) => {
    //             match PathConversion::from(path) {
    //                 PathConversion::Complex(path) => {
    //                     println!("collect_generic_types_in_type: typepath complex: {}", format_token_stream(&path));
    //                     if let Some(last_segment) = path.segments.last() {
    //                         if last_segment.ident.to_string().as_str() == "Option" {
    //                             Self::collect_generic_types_in_type(path_arguments_to_types(&last_segment.arguments)[0], generics);
    //                         }
    //                     }
    //                 },
    //                 PathConversion::Generic(GenericPathConversion::Result(path)) |
    //                 PathConversion::Generic(GenericPathConversion::Vec(path)) |
    //                 PathConversion::Generic(GenericPathConversion::Map(path)) => {
    //                     println!("collect_generic_types_in_type: typepath generic: {}", format_token_stream(&path));
    //                     path_arguments_to_types(&path.segments.last().unwrap().arguments)
    //                         .iter()
    //                         .for_each(|field_type|
    //                             add_generic_type(field_type, generics));
    //                     generics.insert(TypeAndPathHolder(field_type.clone(), path.clone()));
    //                 },
    //                 _ => {}
    //             }
    //         },
    //         _ => {}
    //     }
    // }

    fn find_generic_types_in_compositions(compositions: &[TypeHolder]) -> HashSet<TypeHolder> {
        // collect all types with generics and ensure their uniqueness
        // since we don't want to implement interface multiple times for same object
        let mut generics: HashSet<TypeHolder> = HashSet::new();
        compositions
            .iter()
            .for_each(|TypeHolder(field_type)|
                Self::collect_generic_types_in_type(field_type, &mut generics));
        generics
    }
    pub(crate) fn find_generics(&self) -> HashSet<TypeHolder> {
        Self::find_generic_types_in_compositions(&self.collect_compositions())
    }

    pub fn find_generics_fq(&self, scope_types: &HashMap<TypeHolder, TypeConversion>) -> HashSet<GenericConversion> {
        self.find_generics()
            .iter()
            .filter_map(|holder| scope_types.get(holder))
            .map(GenericConversion::from)
            .collect()
    }
    // fn find_generic_types_in_compositions(compositions: &[TypeAndPathHolder]) -> HashSet<TypeAndPathHolder> {
    //     // collect all types with generics and ensure their uniqueness
    //     // since we don't want to implement interface multiple times for same object
    //     let mut generics: HashSet<TypeAndPathHolder> = HashSet::new();
    //     compositions
    //         .iter()
    //         .for_each(|TypeAndPathHolder(field_type, .. )|
    //             Self::collect_generic_types_in_type(field_type, &mut generics));
    //     generics
    // }

    // pub(crate) fn find_generics(&self) -> HashSet<TypeAndPathHolder> {
    //     Self::find_generic_types_in_compositions(&self.collect_compositions())
    // }
    //
    // pub fn find_generics_fq(&self, scope_types: &HashMap<TypeHolder, TypeConversion>) -> HashSet<GenericConversion> {
    //     self.find_generics()
    //         .iter()
    //         .filter_map(|TypeAndPathHolder { 0: value, .. }| scope_types.get(&TypeHolder::from(value)))
    //         .map(GenericConversion::from)
    //         .collect()
    // }

    fn add_full_qualified_signature(visitor: &mut Visitor, sig: &Signature, scope: &PathHolder, self_scope: &PathHolder, visitor_context: &VisitorContext) {
        if let ReturnType::Type(_arrow_token, ty) = &sig.output {
            visitor.add_full_qualified_type_match(scope, self_scope, ty, visitor_context)
        }
        sig.inputs.iter().for_each(|arg| if let FnArg::Typed(PatType { ty, .. }) = arg {
            visitor.add_full_qualified_type_match(scope, self_scope, ty, visitor_context);
        });
    }

    #[allow(unused)]
    fn add_full_qualified_impl(visitor: &mut Visitor, item_impl: &ItemImpl, scope: &PathHolder, self_scope: &PathHolder) {
        // let trait_path = item_impl.trait_.clone().map(|(_, path, _)| path);
        // let visitor_context = trait_path.map_or(VisitorContext::Object, |_| VisitorContext::Trait(None));
        // return;
        let visitor_context = VisitorContext::Object;
        item_impl.items.iter().for_each(|impl_item| {
            match impl_item {
                ImplItem::Const(ImplItemConst { ty, .. }) => {
                    visitor.add_full_qualified_type_match(scope,  &self_scope, ty, &visitor_context);
                },
                ImplItem::Method(ImplItemMethod { sig, .. }) => {
                    Self::add_full_qualified_signature(visitor, sig, scope, self_scope, &visitor_context)
                },
                ImplItem::Type(ImplItemType { ty, .. }) => {
                    visitor.add_full_qualified_type_match(scope,  &self_scope, ty, &visitor_context);
                },
                _ => {}
            }
        });

    }
    fn add_full_qualified_trait(visitor: &mut Visitor, item_trait: &ItemTrait, scope: &PathHolder) {
        let ident = &item_trait.ident;
        let self_scope = scope.joined(ident);
        visitor.add_full_qualified_trait_match(&self_scope, item_trait);
        let de_trait = TraitDecompositionPart1::from_trait_items(ident, &item_trait.items);
        let de_trait_context = VisitorContext::Trait(Some(de_trait.clone()));
        let mut generics: HashMap<PathHolder, Vec<Path>> = HashMap::new();
        item_trait.generics.params.iter().for_each(|generic_param| {
            match generic_param {
                GenericParam::Type(TypeParam { ident: generic_ident, bounds, .. }) => {
                    let mut de_bounds: Vec<Path> =  vec![];
                    bounds.iter().for_each(|bound| {
                        match bound {
                            TypeParamBound::Trait(TraitBound { path, .. }) => {
                                let ty = parse_quote!(#path);
                                println!("add_full_qualified_trait: (generic trait): {}: {}", format_token_stream(generic_ident), format_token_stream(&ty));
                                de_bounds.push(path.clone());
                                visitor.add_full_qualified_type_match(scope, &self_scope, &ty, &de_trait_context);

                            },
                            TypeParamBound::Lifetime(_lifetime) => {}
                        }
                    });
                    generics.insert(parse_quote!(#generic_ident), de_bounds);
                },
                GenericParam::Lifetime(_lifetime) => {},
                GenericParam::Const(const_param) => {
                    visitor.add_full_qualified_type_match(scope, &self_scope, &const_param.ty, &de_trait_context);
                },
            }
        });
        visitor.add_full_qualified_generic_match(&self_scope, generics);

        item_trait.items.iter().for_each(|trait_item|
            match trait_item {
                TraitItem::Method(TraitItemMethod { sig, .. }) => {
                    Self::add_full_qualified_signature(visitor, sig, scope, &self_scope, &de_trait_context)
                },
                TraitItem::Type(TraitItemType { ident: type_ident, bounds, ..}) => {
                    let local_ty = parse_quote!(Self::#type_ident);
                    visitor.add_full_qualified_type_match(scope, &self_scope, &local_ty, &de_trait_context);
                    println!("add_full_qualified_trait (type): {}: {}", ident, type_ident);
                    // TODO: whether we need to preserve scope or use separate scope + trait ident?
                    // Especially when using Self::  It'll break some logics
                    bounds.iter().for_each(|bound| match bound {
                        TypeParamBound::Trait(TraitBound { path, ..}) => {
                            let ty = parse_quote!(#path);
                            visitor.add_full_qualified_type_match(scope, &self_scope, &ty, &de_trait_context);
                        },
                        _ => {},
                    });
                },
                TraitItem::Const(TraitItemConst { ty, .. }) => {
                    // TransportRequest::Client
                    visitor.add_full_qualified_type_match(scope, &self_scope, ty, &de_trait_context);
                },
                _ => {}
            });
    }

    fn add_full_qualified_trait_type_from_macro(visitor: &mut Visitor, item_trait_attrs: &[Attribute], scope: &PathHolder, ident: &Ident) {
        let trait_names = extract_trait_names(item_trait_attrs);
        let self_scope = scope.joined(ident);
        if !trait_names.is_empty() {
            println!("add_full_qualified_trait_type_from_macro [{}]: {}", format_token_stream(&self_scope), quote!(#(#trait_names), *));
        }
        trait_names.iter().for_each(|trait_name|
            visitor.add_full_qualified_type_match(&scope, &self_scope,&parse_quote!(#trait_name), &VisitorContext::Object));
        let mut lock = visitor.context.write().unwrap();
        lock.used_traits_dictionary
            .entry(self_scope)
            .or_default()
            .extend(trait_names.iter().map(|trait_name| PathHolder::from(trait_name)));
    }

    fn add_full_qualified_type_from_enum(visitor: &mut Visitor, item_enum: &ItemEnum, scope: &PathHolder) {
        let ident = &item_enum.ident;
        let self_scope = scope.joined(ident);
        item_enum.variants.iter().for_each(|Variant { fields, .. }|
            fields.iter().for_each(|Field { ty, .. }|
                visitor.add_full_qualified_type_match(scope, &self_scope, ty, &VisitorContext::Object)));
    }

    fn add_full_qualified_type_from_struct(visitor: &mut Visitor, item_struct: &ItemStruct, scope: &PathHolder) {
        let ident = &item_struct.ident;
        let self_scope = scope.joined(ident);
        item_struct.fields.iter().for_each(|Field { ty, .. }|
            visitor.add_full_qualified_type_match(scope,  &self_scope, ty, &VisitorContext::Object));
    }

    fn add_itself_conversion(visitor: &mut Visitor, scope: &PathHolder, ident: &Ident, ty: TypeConversion) {
        let mut lock = visitor.context.write().unwrap();
        // println!("add_itself_conversion: {}: {}: {}", format_token_stream(scope), format_token_stream(ident), ty);
        lock.scope_types_mut(scope)
            .insert(TypeHolder::new(parse_quote!(#ident)), ty);
    }

    pub fn add_full_qualified_conversion(self, visitor: &mut Visitor) -> ItemConversion {
        //let mut self_is_trait = false;
        match self {
            Self::Struct(item_struct, scope) => {
                let ident = &item_struct.ident;
                let self_scope = scope.joined(ident);
                let type_compo = if item_struct.generics.params.is_empty() {
                    TypeComposition::Single(self_scope.to_type())
                } else {
                    TypeComposition::Composite(self_scope.to_type())
                };
                Self::add_itself_conversion(visitor, &self_scope, ident, TypeConversion::Object(type_compo));
                Self::add_full_qualified_trait_type_from_macro(visitor, &item_struct.attrs, &scope, ident);
                Self::add_full_qualified_type_from_struct(visitor, &item_struct, &scope);
                Self::Struct(item_struct, scope)
            },
            Self::Enum(item_enum, scope) => {
                let ident = &item_enum.ident;
                let self_scope = scope.joined(ident);
                let type_compo = if item_enum.generics.params.is_empty() {
                    TypeComposition::Single(self_scope.to_type())
                } else {
                    TypeComposition::Composite(self_scope.to_type())
                };
                Self::add_itself_conversion(visitor, &self_scope, ident, TypeConversion::Object(type_compo));
                Self::add_full_qualified_trait_type_from_macro(visitor, &item_enum.attrs, &scope, ident);
                Self::add_full_qualified_type_from_enum(visitor, &item_enum, &scope);
                Self::Enum(item_enum, scope)
            },
            Self::Type(item_type, scope) => {
                let ident = &item_type.ident;
                let self_scope = scope.joined(ident);
                let type_compo = if item_type.generics.params.is_empty() {
                    TypeComposition::Single(self_scope.to_type())
                } else {
                    TypeComposition::Composite(self_scope.to_type())
                };
                Self::add_itself_conversion(visitor, &self_scope, ident, TypeConversion::Object(type_compo));
                visitor.add_full_qualified_type_match(&scope, &self_scope, &item_type.ty, &VisitorContext::Object);
                Self::Type(item_type, scope)
            },
            Self::Fn(item_fn, scope) => {
                let ident = &item_fn.sig.ident;
                let self_scope = scope.joined(ident);
                let type_compo = if item_fn.sig.generics.params.is_empty() {
                    TypeComposition::Single(self_scope.to_type())
                } else {
                    TypeComposition::Composite(self_scope.to_type())
                };
                Self::add_itself_conversion(visitor, &self_scope, ident, TypeConversion::Object(type_compo));
                Self::add_full_qualified_signature(visitor, &item_fn.sig, &scope, &self_scope, &VisitorContext::Object);
                Self::Fn(item_fn, scope)
            },
            Self::Trait(item_trait, scope) => {
                let ident = &item_trait.ident;
                let self_scope = scope.joined(ident);
                println!("===> add_full_qualified_conversion:: Trait {}",format_token_stream(&self_scope));
                let type_compo = if item_trait.generics.params.is_empty() {
                    TypeComposition::Single(self_scope.to_type())
                } else {
                    TypeComposition::Composite(self_scope.to_type())
                };

                Self::add_itself_conversion(visitor, &self_scope, ident, TypeConversion::Trait(type_compo, TraitDecompositionPart1::from_trait_items(ident, &item_trait.items)));
                Self::add_full_qualified_trait(visitor, &item_trait, &scope);
                Self::Trait(item_trait, scope)
            },
            Self::Impl(item_impl, scope) => {
                // let self_ty = &*item_impl.self_ty;
                // let ident = type_ident(self_ty);
                // let self_scope = scope.joined(&ident);
                //Self::add_full_qualified_impl(visitor, &item_impl, &scope, &self_scope);
                Self::Impl(item_impl, scope)
            },
            Self::Use(item_use, scope) =>
                Self::Use(item_use, scope),
            Self::Mod(item_mod, scope) => {
                let ident = item_mod.ident.clone();
                let inner_scope = scope.joined(&ident);
                match &item_mod.content {
                    None => {},
                    Some((_, items)) => {
                        items.clone().into_iter().for_each(|item| match item {
                            Item::Use(node) =>
                                visitor.fold_import_tree(&inner_scope, &node.tree, vec![]),
                            Item::Trait(item_trait) => {
                                let ident = &item_trait.ident;
                                let self_scope = inner_scope.joined(ident);
                                println!("===> add_full_qualified_conversion (inner):: Trait {}", format_token_stream(&self_scope));
                                let type_compo = if item_trait.generics.params.is_empty() {
                                    TypeComposition::Single(self_scope.to_type())
                                } else {
                                    TypeComposition::Composite(self_scope.to_type())
                                };
                                Self::add_itself_conversion(visitor, &self_scope, ident, TypeConversion::Trait(type_compo, TraitDecompositionPart1::from_trait_items(ident, &item_trait.items)));
                                Self::add_full_qualified_trait(visitor, &item_trait, &inner_scope)
                            },
                            Item::Fn(ItemFn { sig, .. }) => {
                                let ident = &sig.ident;
                                let self_scope = inner_scope.joined(ident);
                                let type_compo = if sig.generics.params.is_empty() {
                                    TypeComposition::Single(self_scope.to_type())
                                } else {
                                    TypeComposition::Composite(self_scope.to_type())
                                };
                                Self::add_itself_conversion(visitor, &self_scope, ident, TypeConversion::Object(type_compo));
                                Self::add_full_qualified_signature(visitor, &sig, &inner_scope, &inner_scope, &VisitorContext::Object)
                            },
                            Item::Struct(item_struct) => {
                                let ident = &item_struct.ident;
                                let self_scope = inner_scope.joined(ident);
                                let type_compo = if item_struct.generics.params.is_empty() {
                                    TypeComposition::Single(self_scope.to_type())
                                } else {
                                    TypeComposition::Composite(self_scope.to_type())
                                };
                                Self::add_itself_conversion(visitor, &self_scope, ident, TypeConversion::Object(type_compo));
                                Self::add_full_qualified_trait_type_from_macro(visitor, &item_struct.attrs, &inner_scope, ident);
                                Self::add_full_qualified_type_from_struct(visitor, &item_struct, &inner_scope);
                            },
                            Item::Enum(item_enum) => {
                                let ident = &item_enum.ident;
                                let self_scope = inner_scope.joined(ident);
                                let type_compo = if item_enum.generics.params.is_empty() {
                                    TypeComposition::Single(self_scope.to_type())
                                } else {
                                    TypeComposition::Composite(self_scope.to_type())
                                };
                                Self::add_itself_conversion(visitor, &self_scope, ident, TypeConversion::Object(type_compo));
                                Self::add_full_qualified_trait_type_from_macro(visitor, &item_enum.attrs, &inner_scope, ident);
                                Self::add_full_qualified_type_from_enum(visitor, &item_enum, &inner_scope);
                            },
                            Item::Type(item_type) => {
                                let ident = &item_type.ident;
                                let self_scope = scope.joined(ident);
                                let type_compo = if item_type.generics.params.is_empty() {
                                    TypeComposition::Single(self_scope.to_type())
                                } else {
                                    TypeComposition::Composite(self_scope.to_type())
                                };
                                Self::add_itself_conversion(visitor, &self_scope, ident, TypeConversion::Object(type_compo));
                                visitor.add_full_qualified_type_match(&inner_scope, &self_scope, &item_type.ty, &VisitorContext::Object)
                            },
                            // Item::Impl(item_impl) => {
                            //     let self_ty = &item_impl.self_ty;
                            //     let path = parse_quote!(#self_ty);
                            //     let self_scope = scope.joined_path(path);
                            //     Self::add_full_qualified_impl(visitor, &item_impl, &inner_scope, &self_scope);
                            // },
                            _ => {}
                        })
                    }
                }
                Self::Mod(item_mod, scope)
            },
        }
    }

    pub fn make_expansion(&self, item_context: ScopeContext) -> Expansion {
        match self {
            ItemConversion::Mod(..) => Expansion::Empty,
            ItemConversion::Struct(item, scope) =>
                struct_expansion(item, scope, item_context),
            ItemConversion::Enum(item, scope) =>
                enum_expansion(item, scope, item_context),
            ItemConversion::Type(item, scope) =>
                type_expansion(item, scope, item_context),
            ItemConversion::Fn(item, scope) => {
                println!("make_expansion: method: {}", scope.popped());
                let signature = FnSignatureDecomposition::from_signature(&item.sig, scope.popped(), &item_context);
                Expansion::Function {
                    comment: DocPresentation::Safety(signature.ident.to_token_stream()),
                    ffi_presentation: signature.present_fn(),
                }
            },
            ItemConversion::Trait(item, scope) =>
                trait_expansion(item, scope, &item_context),
            ItemConversion::Impl(_item, _scope) =>
                Expansion::Impl { comment: DocPresentation::Empty },
            ItemConversion::Use(_item, _scope) =>
                Expansion::Use { comment: DocPresentation::Empty },
        }
    }
}


fn extract_trait_names(attrs: &[Attribute]) -> Vec<Path> {
    let mut paths = Vec::<Path>::new();
    attrs.iter().for_each(|attr| {
        if attr.path.segments
            .iter()
            .any(|segment| segment.ident == format_ident!("export")) {
            if let Ok(Meta::List(meta_list)) = attr.parse_meta() {
                meta_list.nested.iter().for_each(|meta| {
                    if let NestedMeta::Meta(Meta::Path(path)) = meta {
                        paths.push(path.clone());
                    }
                });
            }
        }
    });
    paths
}


fn enum_expansion(item_enum: &ItemEnum, item_scope: &PathHolder, context: ScopeContext) -> Expansion {
    let ItemEnum { ident: target_name, variants, .. } = item_enum;
    let variants_count = variants.len();
    // let trait_context = context.clone();
    // trait_context.scope =
    let traits = item_traits_expansions((target_name, item_scope), &item_enum.attrs, &context);
    let mut conversions_to_ffi = Vec::<TokenStream2>::with_capacity(variants_count);
    let mut conversions_from_ffi = Vec::<TokenStream2>::with_capacity(variants_count);
    let mut variants_fields = Vec::<TokenStream2>::with_capacity(variants_count);
    let mut variants_constructors = Vec::<BindingPresentation>::with_capacity(variants_count);
    let mut destroy_fields = Vec::<TokenStream2>::new();
    let mut drop_fields = Vec::<TokenStream2>::new();
    variants.iter().for_each(|Variant { ident: variant_name, fields, discriminant, .. }| {
        let full_ty = context.full_type_for(&parse_quote!(#target_name));
        let target_variant_path = quote!(#full_ty::#variant_name);
        let ffi_variant_path = quote!(#target_name::#variant_name);
        let variant_mangled_path = format_ident!("{}_{}", target_name, variant_name);
        let (variant_presenter, fields_context) = match discriminant {
            Some((_, Expr::Lit(lit, ..))) => (ENUM_UNIT_FIELDS_PRESENTER, vec![quote!(#lit)]),
            None => match fields {
                Fields::Unit => (NO_FIELDS_PRESENTER, vec![]),
                Fields::Unnamed(FieldsUnnamed { unnamed, .. }) => (
                    ENUM_UNNAMED_VARIANT_PRESENTER,
                    unnamed
                        .iter()
                        .map(|field|
                            FFI_FULL_DICTIONARY_FIELD_TYPE_PRESENTER(&field.ty, &context))
                        .collect(),
                ),
                Fields::Named(FieldsNamed { named, .. }) => (
                    ENUM_NAMED_VARIANT_PRESENTER,
                    named
                        .iter()
                        .map(|Field { ident, ty: field_type, .. }|
                            NAMED_CONVERSION_PRESENTER(
                                ident.clone().unwrap().to_token_stream(),
                                FFI_FULL_DICTIONARY_FIELD_TYPE_PRESENTER(field_type, &context)))
                        .collect(),
                ),
            },
            _ => panic!("Error variant discriminant"),
        };
        let composer = match fields {
            Fields::Unit =>
                ItemComposer::enum_variant_default_composer(
                    parse_quote!(#ffi_variant_path),
                    parse_quote!(#target_variant_path),
                    context.clone(),
                    NO_FIELDS_PRESENTER,
                    NO_FIELDS_PRESENTER,
                    SIMPLE_CONVERSION_PRESENTER,
                    ROOT_DESTROY_CONTEXT_PRESENTER,
                    EMPTY_DESTROY_PRESENTER,
                    EMPTY_ITERATOR_PRESENTER,
                    ConversionsComposer::Empty
                ),
            Fields::Unnamed(fields) =>
                ItemComposer::enum_variant_default_composer(
                    parse_quote!(#ffi_variant_path),
                    parse_quote!(#target_variant_path),
                    context.clone(),
                    ROUND_BRACES_FIELDS_PRESENTER,
                    ROUND_BRACES_FIELDS_PRESENTER,
                    SIMPLE_CONVERSION_PRESENTER,
                    ROOT_DESTROY_CONTEXT_PRESENTER,
                    SIMPLE_TERMINATED_PRESENTER,
                    |fields| {
                        if fields.is_empty() {
                            quote!()
                        } else {
                            ROUND_ITER_PRESENTER(fields)
                        }
                    },
                    ConversionsComposer::UnnamedEnumVariant(fields)
                ),
            Fields::Named(fields) =>
                ItemComposer::enum_variant_default_composer(
                    parse_quote!(#ffi_variant_path),
                    parse_quote!(#target_variant_path),
                    context.clone(),
                    CURLY_BRACES_FIELDS_PRESENTER,
                    CURLY_BRACES_FIELDS_PRESENTER,
                    NAMED_CONVERSION_PRESENTER,
                    ROOT_DESTROY_CONTEXT_PRESENTER,
                    SIMPLE_PRESENTER,
                    |fields| {
                        if fields.is_empty() {
                            quote!()
                        } else {
                            CURLY_ITER_PRESENTER(fields)
                        }
                    },
                    ConversionsComposer::NamedStruct(fields)
                )
        };
        let composer_owned = composer.borrow();
        variants_fields.push(variant_presenter((quote!(#variant_name), fields_context)));
        conversions_from_ffi.push(composer_owned.compose_from());
        conversions_to_ffi.push(composer_owned.compose_to());
        destroy_fields.push(composer_owned.compose_destroy());
        drop_fields.push(composer_owned.compose_drop());
        variants_constructors.push(BindingPresentation::EnumVariantConstructor {
            ffi_ident: target_name.to_token_stream(),
            ffi_variant_ident: variant_mangled_path,
            ffi_variant_path: ffi_variant_path.to_token_stream(),
            ctor_arguments: composer_owned.ffi_conversions_composer.bindings_composer.compose_arguments(&context),
            body_presentation: composer_owned.ffi_conversions_composer.bindings_composer.compose_field_names(),
        })
    });
    let comment = DocPresentation::Default(quote!(#target_name));
    let ffi_presentation =
        FFIObjectPresentation::Full(ENUM_PRESENTER((quote!(#target_name), variants_fields)));
    let target_full_type = context.full_type_for(&parse_quote!(#target_name));
    let conversion = ConversionInterfacePresentation::Interface {
        ffi_type: parse_quote!(#target_name),
        target_type: quote!(#target_full_type),
        from_presentation: FromConversionPresentation::Enum(conversions_from_ffi),
        to_presentation: ToConversionPresentation::Enum(conversions_to_ffi),
        destroy_presentation: package_unboxed_root(),
        generics: vec![]
    };
    let drop = DropInterfacePresentation::Full(
        quote!(#target_name),
        ENUM_DESTROY_PRESENTER(drop_fields),
    );

    variants_constructors.push(BindingPresentation::Destructor {
        ffi_name: quote!(#target_name),
        destructor_ident: ffi_destructor_name(target_name).to_token_stream()
    });

    Expansion::Full {
        comment,
        ffi_presentation,
        conversion,
        drop,
        bindings: variants_constructors,
        traits
    }
}

fn implement_trait_for_item(item_trait: (&ItemTrait, &PathHolder), item: (&Ident, &PathHolder), context: &ScopeContext) -> TraitVTablePresentation {
    let (item_trait, trait_scope) = item_trait;
    let (item_name, item_scope) = item;
    let trait_decomposition = TraitDecompositionPart2::from_trait_items(&item_trait.items, &EMPTY, context);
    let trait_ident = &item_trait.ident;
    let item_full_ty = context.full_type_for(&parse_quote!(#item_name));
    let trait_full_ty = context.full_type_for(&parse_quote!(#trait_ident));

    let (vtable_methods_implentations, vtable_methods_declarations): (Vec<TokenStream2>, Vec<TokenStream2>) = trait_decomposition.methods.into_iter()
        .map(|signature_decomposition| {
            let FnReturnTypeDecomposition { presentation: output_expression, conversion: output_conversions } = signature_decomposition.return_type;
            let fn_name = signature_decomposition.ident.unwrap();
            let ffi_method_ident = format_ident!("{}_{}", item_name, fn_name);
            let arguments = signature_decomposition.arguments.iter().map(|arg| arg.name_type_original.clone()).collect();
            let mut argument_conversions = vec![quote!(cast_obj)];
            argument_conversions.extend(signature_decomposition.arguments.iter().filter(|arg| arg.name.is_some()).map(|arg| arg.name_type_conversion.clone()));
            let name_and_args = ROUND_BRACES_FIELDS_PRESENTER((quote!(unsafe extern "C" fn #ffi_method_ident), arguments));
            let argument_names = ROUND_ITER_PRESENTER(argument_conversions);
                (quote!(#name_and_args -> #output_expression {
                let cast_obj = &(*(obj as *const #item_full_ty));
                let obj = <#item_full_ty as #trait_full_ty>::#fn_name #argument_names;
                #output_conversions
            }), quote!(#fn_name: #ffi_method_ident))
    }).unzip();
    let trait_vtable_ident = ffi_vtable_name(trait_ident);
    let trait_object_ident = ffi_trait_obj_name(trait_ident);
    let trait_implementor_vtable_ident = format_ident!("{}_{}", item_name, trait_vtable_ident);
    let item_module = item_scope.popped();
    let (fq_trait_vtable, fq_trait_object) = if item_module.eq(&trait_scope.popped()) {
        (quote!(#trait_vtable_ident), quote!(#trait_object_ident))
    } else {
        (quote!(#trait_scope::#trait_vtable_ident), quote!(#trait_scope::#trait_object_ident))
    };
    let vtable = quote! {
        #[allow(non_snake_case, non_upper_case_globals)]
        static #trait_implementor_vtable_ident: #fq_trait_vtable = {
            #(#vtable_methods_implentations)*
            #fq_trait_vtable {
                #(#vtable_methods_declarations,)*
            }
        };
    };
    let binding_ident = format_ident!("{}_as_{}", item_name, trait_object_ident);
    let destructor_binding_ident = ffi_destructor_name(&binding_ident);
    let export = quote! {
        /// # Safety
        #[allow(non_snake_case)]
        #[no_mangle]
        pub extern "C" fn #binding_ident(obj: *const #item_full_ty) -> #fq_trait_object {
            #fq_trait_object {
                object: obj as *const (),
                vtable: &#trait_implementor_vtable_ident,
            }
        }
    };
    let destructor = quote! {
        /// # Safety
        #[allow(non_snake_case)]
        #[no_mangle]
        pub unsafe extern "C" fn #destructor_binding_ident(obj: #fq_trait_object) {
            ferment_interfaces::unbox_any(obj.object as *mut #item_full_ty);
        }
    };
    TraitVTablePresentation::Full { vtable, export, destructor }
}

pub fn trait_items_from_attributes(attrs: &[Attribute], context: &ScopeContext) -> Vec<(TraitCompositionPart1, PathHolder)> {
    // println!("trait_items_from_attributes: [{}]", context.scope);
    let attr_traits = extract_trait_names(attrs);
    attr_traits.iter()
        .map(|trait_name| context.find_item_trait_scope_pair(trait_name))
        .collect()
}

fn item_traits_expansions(item: (&Ident, &PathHolder), attrs: &[Attribute], context: &ScopeContext) -> Vec<TraitVTablePresentation> {
    let mut trait_types = trait_items_from_attributes(attrs, context);
    trait_types.iter_mut()
        .map(|(composition, trait_scope)| {
            // TODO: move to full
            composition.implementors.push(TypeConversion::Object(TypeComposition::Unknown(context.scope.to_type())));
            implement_trait_for_item((&composition.item, trait_scope), item, context)
        })
        .collect()
}

fn struct_expansion(item_struct: &ItemStruct, _scope: &PathHolder, item_context: ScopeContext) -> Expansion {
    // println!("struct_expansion: {}", item_struct.ident);
    let ItemStruct { fields: ref f, ident: target_name, .. } = item_struct;
    let traits = item_traits_expansions((target_name, _scope), &item_struct.attrs, &item_context);
    let full_ty = item_context.full_type_for(&parse_quote!(#target_name));
    let composer = match f {
        Fields::Unnamed(ref fields) =>
            ItemComposer::struct_composer(
                parse_quote!(#target_name),
                parse_quote!(#full_ty),
                item_context,
                UNNAMED_STRUCT_PRESENTER,
                DEFAULT_DICT_FIELD_TYPE_PRESENTER,
                ROUND_BRACES_FIELDS_PRESENTER,
                SIMPLE_CONVERSION_PRESENTER,
                ROUND_ITER_PRESENTER,
                ConversionsComposer::UnnamedStruct(fields)
            ),
        Fields::Named(ref fields) =>
            ItemComposer::struct_composer(
                parse_quote!(#target_name),
                parse_quote!(#full_ty),
                item_context,
                NAMED_STRUCT_PRESENTER,
                NAMED_DICT_FIELD_TYPE_PRESENTER,
                CURLY_BRACES_FIELDS_PRESENTER,
                NAMED_CONVERSION_PRESENTER,
                CURLY_ITER_PRESENTER,
                ConversionsComposer::NamedStruct(fields)
            ),
        Fields::Unit => panic!("Fields::Unit is not supported yet"),
    };
    let composer_owned = composer.borrow();
    composer_owned.make_expansion(traits)
}


fn trait_expansion(item_trait: &ItemTrait, scope: &PathHolder, context: &ScopeContext) -> Expansion {
    let trait_decomposition = TraitDecompositionPart2::from_trait_items(&item_trait.items, scope, context);
    let fields = trait_decomposition.present_trait_vtable_inner_functions();

    let trait_name = &item_trait.ident;
    let vtable_name = ffi_vtable_name(trait_name).to_token_stream();
    Expansion::Trait {
        comment: DocPresentation::Empty,
        vtable: FFIObjectPresentation::TraitVTable {
            name: vtable_name.clone(),
            fields
        },
        trait_object: FFIObjectPresentation::TraitObject {
            name: ffi_trait_obj_name(trait_name).to_token_stream(),
            vtable_name
        }
    }
}

fn type_expansion(item_type: &ItemType, scope: &PathHolder, context: ScopeContext) -> Expansion {
    let ItemType { ident, ty, .. } = item_type;
    match &**ty {
        Type::BareFn(bare_fn) => {
            let decomposition = FnSignatureDecomposition::from_bare_fn(bare_fn, ident, scope.clone(), &context);
            Expansion::Callback {
                comment: DocPresentation::Default(quote!(#ident)),
                ffi_presentation: decomposition.present_callback(),
            }
        },
        _ => {
            let traits = item_traits_expansions((ident, scope), &item_type.attrs, &context);
            let full_ty = context.full_type_for(&parse_quote!(#ident));
            ItemComposer::type_alias_composer(
                parse_quote!(#ident),
                parse_quote!(#full_ty),
                context,
                ConversionsComposer::TypeAlias(ty))
                .borrow()
                .make_expansion(traits)
        }
    }
}

pub fn add_generic_type(field_type: &Type, generics: &mut HashSet<TypeHolder>) {
    if let Type::Path(TypePath { path, .. }) = field_type {
        if let PathConversion::Generic(generic_path_conversion) = PathConversion::from(path) {
            // generics.insert(TypeAndPathHolder(field_type.clone(), generic_path_conversion.path()));
            generics.insert(TypeHolder(field_type.clone()));
        }
    }
}

