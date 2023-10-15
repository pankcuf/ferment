use std::collections::{HashMap, HashSet};
use std::fmt::Formatter;
use syn::{Attribute, BareFnArg, Data, DataEnum, DataStruct, DataUnion, DeriveInput, Expr, Field, Fields, FieldsNamed, FieldsUnnamed, FnArg, Ident, Item, ItemEnum, ItemFn, ItemMod, ItemStruct, ItemType, ItemUse, parse_quote, Pat, Path, PathSegment, PatIdent, PatType, ReturnType, Signature, Type, TypeBareFn, TypePath, UseGlob, UseGroup, UseName, UsePath, UseRename, UseTree, Variant};
use quote::{format_ident, quote, ToTokens};
use syn::__private::{Span, TokenStream2};
use crate::generics::{add_generic_type, TypePathComposition};
use crate::interface::{CURLY_BRACES_FIELDS_PRESENTER, EMPTY_FIELDS_PRESENTER, EMPTY_MAP_PRESENTER, EMPTY_PAIR_PRESENTER, ENUM_DESTROY_PRESENTER, ENUM_NAMED_VARIANT_PRESENTER, ENUM_PRESENTER, ENUM_UNIT_FIELDS_PRESENTER, ENUM_UNNAMED_VARIANT_PRESENTER, FFI_DICTIONARY_TYPE_PRESENTER, FFI_FROM_ROOT_PRESENTER, FFI_TO_ROOT_PRESENTER, MATCH_FIELDS_PRESENTER, NAMED_CONVERSION_PRESENTER, NAMED_VARIANT_FIELD_PRESENTER, NO_FIELDS_PRESENTER, obj, package_unboxed_root, ROOT_DESTROY_CONTEXT_PRESENTER, ROUND_BRACES_FIELDS_PRESENTER, SIMPLE_PAIR_PRESENTER, UNNAMED_VARIANT_FIELD_PRESENTER};
use crate::helper::{ffi_struct_name, from_path, path_arguments_to_types, to_path};
use crate::composer::RootComposer;
use crate::visitor::Visitor;
use crate::path_conversion::{GenericPathConversion, PathConversion};
use crate::presentation::{ConversionInterfacePresentation, DocPresentation, DropInterfacePresentation, Expansion, FFIObjectPresentation};
use crate::scope::{EMPTY, Scope};
use crate::import_conversion::{ImportConversion, ImportType};
use crate::scope_conversion::ScopeTreeItem;
use crate::type_conversion::TypeConversion;


#[derive(Clone)]
pub enum ItemConversion {
    Mod(ItemMod, Scope),
    Struct(ItemStruct, Scope),
    Enum(ItemEnum, Scope),
    Type(ItemType, Scope),
    Fn(ItemFn, Scope),
    Use(ItemUse, Scope),
}

impl ItemConversion {

    pub fn scope(&self) -> &Scope {
        match self {
            ItemConversion::Mod(_, scope) => scope,
            ItemConversion::Struct(_, scope) => scope,
            ItemConversion::Enum(_, scope) => scope,
            ItemConversion::Type(_, scope) => scope,
            ItemConversion::Fn(_, scope) => scope,
            ItemConversion::Use(_, scope) => scope,
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
        f.write_fmt(format_args!("{}: {}", self.name(), self.ident().to_token_stream()))
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
            ItemConversion::Use(item, ..) => item.to_tokens(tokens),
        }
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
        }
    }
    pub const fn r#type(&self) -> &str {
        match self {
            Self::Mod(..) => "export",
            Self::Struct(..) | Self::Enum(..) => "export",
            Self::Type(..) => "export",
            Self::Fn(..) => "export",
            Self::Use(..) => "",
        }
    }

    fn fold_use(tree: &UseTree) -> Vec<&Ident> {
        match tree {
            UseTree::Path(UsePath { ident, .. }) => vec![ident],
            UseTree::Name(UseName { ident }) => vec![ident],
            UseTree::Rename(UseRename { rename, .. }) => vec![rename],
            UseTree::Glob(UseGlob { .. }) => vec![],
            UseTree::Group(UseGroup { items , .. }) => {
                items.iter().flat_map(|tree| Self::fold_use(tree)).collect()
            }
        }
    }
    pub fn ident(&self) -> &Ident {
        match self {
            ItemConversion::Mod(ItemMod { ident, .. }, ..) => ident,
            ItemConversion::Struct(ItemStruct { ident, .. }, ..) => ident,
            ItemConversion::Enum(ItemEnum { ident, .. }, ..) => ident,
            ItemConversion::Type(ItemType { ident, .. }, ..) => ident,
            ItemConversion::Fn(ItemFn { sig: Signature { ident, .. }, .. }, ..) => ident,
            ItemConversion::Use(ItemUse { tree, .. }, ..) => Self::fold_use(tree).first().unwrap(),
        }
    }

    pub fn is_empty_mod(&self) -> bool {
        match self {
            ItemConversion::Mod(ItemMod { content, .. }, ..) => content.is_none(),
            _ => false
        }
    }
    pub fn is_non_empty_mod(&self) -> bool {
        !self.is_empty_mod()
    }

    fn macro_ident(&self) -> Ident {
        format_ident!("{}", self.r#type())
    }

    fn is_labeled_with_macro(&self, path: &Path) -> bool {
        path.segments
            .iter()
            .any(|segment| segment.ident == self.macro_ident())
    }

    fn handle_attributes_with_handler<F: FnMut(&Path)>(&self, attrs: &[Attribute], mut handler: F) {
        attrs.iter()
            .for_each(|Attribute { path, .. }|
                if self.is_labeled_with_macro(path) {
                    handler(path)
                }
            )
    }
}

impl From<(ItemConversion, HashMap<TypeConversion, Type>)> for Expansion {
    fn from(conversion: (ItemConversion, HashMap<TypeConversion, Type>)) -> Self {
        match &conversion.0 {
            ItemConversion::Mod(..) => Expansion::Empty,
            ItemConversion::Struct(item_struct, scope) => struct_expansion(item_struct, scope, conversion.1),
            ItemConversion::Enum(item_enum, scope) => enum_expansion(item_enum, scope, conversion.1),
            ItemConversion::Type(item_type, scope) => type_expansion(item_type, scope, conversion.1),
            ItemConversion::Fn(item_fn, scope) => fn_expansion(item_fn, scope, conversion.1),
            ItemConversion::Use(item_use, scope) => use_expansion(item_use, scope),
        }
    }
}


impl From<DeriveInput> for ItemConversion {
    fn from(input: DeriveInput) -> Self {
        let DeriveInput { attrs, vis, ident, generics, data } = input;
        match data {
            Data::Struct(DataStruct { fields,  semi_token, struct_token, .. }) =>
                Self::Struct(ItemStruct { attrs, vis, ident, generics, fields, semi_token, struct_token }, EMPTY),
            Data::Enum(DataEnum { enum_token, brace_token, variants }) =>
                Self::Enum(ItemEnum { attrs, vis, ident, generics, variants, enum_token, brace_token }, EMPTY),
            Data::Union(DataUnion { union_token, fields }) =>
                unimplemented!("Unions are not supported yet {}: {}", union_token.to_token_stream(), fields.to_token_stream()),
        }
    }
}
impl<'a> TryFrom<&'a ScopeTreeItem> for ItemConversion {
    type Error = String;
    fn try_from(value: &'a ScopeTreeItem) -> Result<Self, Self::Error> {
        match value {
            ScopeTreeItem::Tree { tree, item} => Ok(Self::Mod(parse_quote!(#item), tree.scope.clone())),
            ScopeTreeItem::Item { scope, item, scope_types: _ } => match item {
                Item::Mod(item_mod) => Ok(Self::Mod(item_mod.clone(), scope.clone())),
                Item::Struct(item_struct) => Ok(Self::Struct(item_struct.clone(), scope.clone())),
                Item::Enum(item_enum) => Ok(Self::Enum(item_enum.clone(), scope.clone())),
                Item::Type(item_type) => Ok(Self::Type(item_type.clone(), scope.clone())),
                Item::Fn(item_fn) => Ok(Self::Fn(item_fn.clone(), scope.clone())),
                item => Err(format!("Error: {}: {}", scope, item.to_token_stream().to_string()))
            }
        }
    }
}

impl<'a> TryFrom<&'a Item> for ItemConversion {
    type Error = String;
    fn try_from(value: &'a Item) -> Result<Self, Self::Error> {
        match value {
            Item::Mod(item_mod) => Ok(Self::Mod(item_mod.clone(), EMPTY)),
            Item::Struct(item_struct) => Ok(Self::Struct(item_struct.clone(), EMPTY)),
            Item::Enum(item_enum) => Ok(Self::Enum(item_enum.clone(), EMPTY)),
            Item::Type(item_type) => Ok(Self::Type(item_type.clone(), EMPTY)),
            Item::Fn(item_fn) => Ok(Self::Fn(item_fn.clone(), EMPTY)),
            item => Err(format!("Error: {}", item.to_token_stream().to_string()))
        }
    }
}

impl<'a> TryFrom<(&'a Item, &'a Scope)> for ItemConversion {
    type Error = String;
    fn try_from(value: (&'a Item, &'a Scope)) -> Result<Self, Self::Error> {
        match value.0 {
            Item::Mod(item_mod) => Ok(Self::Mod(item_mod.clone(), value.1.clone())),
            Item::Struct(item_struct) => Ok(Self::Struct(item_struct.clone(), value.1.clone())),
            Item::Enum(item_enum) => Ok(Self::Enum(item_enum.clone(), value.1.clone())),
            Item::Type(item_type) => Ok(Self::Type(item_type.clone(), value.1.clone())),
            Item::Fn(item_fn) => Ok(Self::Fn(item_fn.clone(), value.1.clone())),
            item => Err(format!("Error: {}", item.to_token_stream().to_string()))
        }
    }
}
impl<'a> TryFrom<(&'a Item, Scope)> for ItemConversion {
    type Error = String;
    fn try_from(value: (&'a Item, Scope)) -> Result<Self, Self::Error> {
        match value.0 {
            Item::Mod(item_mod) => Ok(Self::Mod(item_mod.clone(), value.1)),
            Item::Struct(item_struct) => Ok(Self::Struct(item_struct.clone(), value.1)),
            Item::Enum(item_enum) => Ok(Self::Enum(item_enum.clone(), value.1)),
            Item::Type(item_type) => Ok(Self::Type(item_type.clone(), value.1)),
            Item::Fn(item_fn) => Ok(Self::Fn(item_fn.clone(), value.1)),
            item => Err(format!("Error: {}", item.to_token_stream().to_string()))
        }
    }
}

impl<'a> From<&'a ItemStruct> for ItemConversion {
    fn from(item_struct: &'a ItemStruct) -> Self {
        Self::Struct(item_struct.clone(), EMPTY)
    }
}

impl Into<Item> for ItemConversion {
    fn into(self) -> Item {
        match self {
            ItemConversion::Mod(item, _) => parse_quote!(#item),
            ItemConversion::Struct(item, _) =>  parse_quote!(#item),
            ItemConversion::Enum(item, _) =>  parse_quote!(#item),
            ItemConversion::Type(item, _) =>  parse_quote!(#item),
            ItemConversion::Fn(item, _) =>  parse_quote!(#item),
            ItemConversion::Use(item, _) =>  parse_quote!(#item),
        }
    }
}
impl<'a> Into<Item> for &'a ItemConversion {
    fn into(self) -> Item {
        match self {
            ItemConversion::Mod(item, _) => parse_quote!(#item),
            ItemConversion::Struct(item, _) =>  parse_quote!(#item),
            ItemConversion::Enum(item, _) =>  parse_quote!(#item),
            ItemConversion::Type(item, _) =>  parse_quote!(#item),
            ItemConversion::Fn(item, _) =>  parse_quote!(#item),
            ItemConversion::Use(item, _) =>  parse_quote!(#item),
        }
    }
}
impl ItemConversion {

    pub fn collect_all_items(&self) -> Vec<ItemConversion> {
        // println!("collect_all_items {}", self);
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

    pub fn collect_compositions(&self) -> Vec<TypePathComposition> {
        let mut type_and_paths: Vec<TypePathComposition> = Vec::new();
        let mut cache_type = |ty: &Type, path: &Path|
            type_and_paths.push(TypePathComposition(ty.clone(), path.clone()));
        let mut cache_fields = |fields: &Fields, path: &Path| match fields {
            Fields::Unnamed(FieldsUnnamed { unnamed: fields, .. }) |
            Fields::Named(FieldsNamed { named: fields, .. }) =>
                fields.iter().for_each(|field| cache_type(&field.ty, path)),
            Fields::Unit => {}
        };

        match self {
            Self::Mod(ItemMod { content: Some((_, items)), .. }, scope) =>
                items.iter()
                    .flat_map(|m| Self::try_from((m, scope.clone())))
                    .for_each(|conversion|
                        type_and_paths.extend(conversion.collect_compositions())),
            Self::Struct(item_struct, ..) =>
                self.handle_attributes_with_handler(&item_struct.attrs, |path|
                    cache_fields(&item_struct.fields, path)),
            Self::Enum(item_enum, ..) =>
                self.handle_attributes_with_handler(&item_enum.attrs, |path|
                    item_enum.variants.iter().for_each(|Variant { fields, .. }|
                        cache_fields(&fields, path))),
            Self::Type(ItemType { attrs, ty, .. }, ..) =>
                self.handle_attributes_with_handler(attrs, |path|
                    cache_type(ty, path)),
            Self::Fn(item_fn, ..) =>
                self.handle_attributes_with_handler(&item_fn.attrs, |path| {
                    item_fn.sig.inputs.iter().for_each(|arg| match arg {
                        FnArg::Typed(PatType { ty, .. }) =>
                            cache_type(ty, path),
                        _ => {}
                    });
                    match &item_fn.sig.output {
                        ReturnType::Default => {},
                        ReturnType::Type(_, ty) => match &**ty {
                            Type::Path(TypePath { path, .. }) =>
                                cache_type(ty, path),
                            _ => {}
                        }
                    };
                }),
            _ => {}
        }

        type_and_paths
    }

    fn import_pair(path: &Path, imports: &HashMap<Ident, Path>) -> (ImportType, Scope) {
        let original_or_external_pair = |value| {
            let scope = Scope::from(value);
            (if scope.has_belong_to_current_crate() { ImportType::Original } else { ImportType::External }, scope)
        };
        match path.get_ident() {
            Some(ident) => match ident.to_string().as_str() {
                // accessible without specifying scope
                "i8" | "u8" | "i16" | "u16" | "i32" | "u32" | "i64" | "u64" | "i128" | "u128" | "isize"
                | "usize" | "bool" | "str" | "String" | "Vec" | "Option" =>
                    (ImportType::None, Scope::new(parse_quote!(#ident))),
                // "UInt128" | "UInt160" | "UInt256" | "UInt384" | "UInt512" | "UInt768" => (ImportType::Original, Scope::new()),
                // they are defined in the same scope, so it should be imported sometimes outside this scope (export-only)
                _ =>
                    imports.get(ident)
                        .map_or((ImportType::Inner, Scope::new(parse_quote!(#ident))), original_or_external_pair)
            },
            // partial chunk
            None => match path.segments.last() {
                Some(PathSegment { ident, .. }) => {
                    imports.get(ident)
                        .map_or(match ident.to_string().as_str() {
                            "Vec" | "Option" => (ImportType::None, Scope::new(parse_quote!(#ident))),
                            _ => (ImportType::ExternalChunk, Scope::from(path)),
                        }, original_or_external_pair)
                },
                _ => (ImportType::None, EMPTY),
            }
        }
    }

    fn cache_type_in(container: &mut HashMap<ImportType, HashSet<ImportConversion>>, ty: &Type, imports: &HashMap<Ident, Path>) {
        // Types which are used as a part of types (for generics and composite types)
        let involved_type_paths = TypeConversion::from(ty).get_all_type_paths_involved();
        involved_type_paths.iter().for_each(|type_path| {
            let path = &type_path.0.path;
            match path.segments.last() {
                Some(PathSegment { ident, .. }) => {
                    let (import_type, scope) = Self::import_pair(path, imports);
                    container.entry(import_type)
                        .or_insert_with(HashSet::new)
                        .insert(ImportConversion::from((ident, &scope)));
                },
                None => {},
            }
        });
    }



    fn cache_fields_in(mut container: &mut HashMap<ImportType, HashSet<ImportConversion>>, fields: &Fields, imports: &HashMap<Ident, Path>) {
        match fields {
            Fields::Unnamed(FieldsUnnamed { unnamed: fields, .. }) |
            Fields::Named(FieldsNamed { named: fields, .. }) =>
                fields.iter()
                    .for_each(|field| Self::cache_type_in(&mut container, &field.ty, imports)),
            Fields::Unit => {}
        }
    }

    pub fn get_used_imports(&self, imports: &HashMap<Ident, Path>) -> HashMap<ImportType, HashSet<ImportConversion>> {
        self.classify_imports(imports)
            .into_iter()
            .filter_map(|(import_type, used_imports)| import_type.get_imports_for(used_imports))
            .collect()
    }

    pub fn classify_imports(&self, imports: &HashMap<Ident, Path>) -> HashMap<ImportType, HashSet<ImportConversion>> {
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
                    Self::cache_fields_in(&mut container, &fields, imports))),
            Self::Type(ItemType { attrs, ty, .. }, ..) =>
                self.handle_attributes_with_handler(attrs, |_path|
                    Self::cache_type_in(&mut container, ty, imports)),
            Self::Fn(item_fn, ..) =>
                self.handle_attributes_with_handler(&item_fn.attrs, |_path| {
                    item_fn.sig.inputs.iter().for_each(|arg| match arg {
                        FnArg::Typed(PatType { ty, .. }) =>
                            Self::cache_type_in(&mut container, ty, imports),
                        _ => {}
                    });
                    if let ReturnType::Type(_, ty) = &item_fn.sig.output {
                        Self::cache_type_in(&mut container, &**ty, imports)
                    };
                }),
            _ => {}
        }
        container
    }

    fn find_generic_types_in_compositions(compositions: &Vec<TypePathComposition>) -> HashSet<TypePathComposition> {
        // collect all types with generics and ensure their uniqueness
        // since we don't want to implement interface multiple times for same object
        let mut generics: HashSet<TypePathComposition> = HashSet::new();
        compositions
            .iter()
            .for_each(|TypePathComposition(field_type, .. )| {
                // println!("find_generic_types_in_compositions: {}", quote!(#field_type));
                match field_type {
                    Type::Path(TypePath { path, .. }) => match PathConversion::from(path) {
                        PathConversion::Generic(GenericPathConversion::Vec(path)) |
                        PathConversion::Generic(GenericPathConversion::Map(path)) => {
                            path_arguments_to_types(&path.segments.last().unwrap().arguments)
                                .iter()
                                .for_each(|field_type|
                                    add_generic_type(field_type, &mut generics));
                            generics.insert(TypePathComposition(field_type.clone(), path.clone()));
                        },
                        _ => {}
                    },
                    _ => {}
                }
            });
        generics
    }

    pub(crate) fn find_generics(&self) -> HashSet<TypePathComposition> {
        Self::find_generic_types_in_compositions(&self.collect_compositions())
    }

    pub fn add_full_qualified_conversion<'ast>(self, visitor: &'ast mut Visitor) -> ItemConversion {
        let converted = match self {
            Self::Struct(item_struct, scope) => {
                item_struct.fields.iter().for_each(|Field { ty, .. }| visitor.add_full_qualified_type_match(scope.clone(), ty));
                // item_struct.fields.iter_mut().for_each(|Field { ty, .. }| visitor.convert_type(ty));
                Self::Struct(item_struct, scope)
            },
            Self::Enum(item_enum, scope) => {
                item_enum.variants.iter().for_each(|Variant { fields, .. }| fields.iter().for_each(|Field { ty, .. }| visitor.add_full_qualified_type_match(scope.clone(), ty)));
                // item_enum.variants.iter_mut().for_each(|Variant { fields, .. }| fields.iter_mut().for_each(|Field { ty, .. }| visitor.convert_type(ty)));
                Self::Enum(item_enum, scope)
            },
            Self::Type(item_type, scope) => {
                let ty = &*item_type.ty;
                visitor.add_full_qualified_type_match(scope.clone(), ty);
                // visitor.convert_type(ty);
                Self::Type(item_type, scope)
            },
            Self::Fn(item_fn, scope) => {
                match &item_fn.sig.output {
                    ReturnType::Default => {},
                    ReturnType::Type(_, ty) => {
                        visitor.add_full_qualified_type_match(scope.clone(), ty);
                        // visitor.convert_type(&mut *ty);
                    }
                }
                item_fn.sig.inputs.iter().for_each(|arg| match arg {
                    FnArg::Typed(PatType { ty, .. }) => {
                        visitor.add_full_qualified_type_match(scope.clone(), ty);
                        // visitor.convert_type(&mut *ty);
                    },
                    _ => {}
                });
                Self::Fn(item_fn, scope)
            },
            Self::Use(item_use, scope) => {
                Self::Use(item_use, scope)
            },
            Self::Mod(item_mod, scope) => {
                let ident = item_mod.ident.clone();
                let inner_scope = scope.joined(&ident);

                match &item_mod.content {
                    None => {},
                    Some((_, items)) => {
                        items.clone().into_iter().for_each(|item| {
                            match item {
                                Item::Use(node) => {
                                    visitor.fold_import_tree(&inner_scope, &node.tree, vec![]);
                                },
                                Item::Fn(item_fn) => {
                                    match &item_fn.sig.output {
                                        ReturnType::Default => {},
                                        ReturnType::Type(_, ty) => {
                                            visitor.add_full_qualified_type_match(inner_scope.clone(), ty);
                                            // visitor.convert_type(&mut *ty);
                                        }
                                    }
                                    item_fn.sig.inputs.iter().for_each(|arg| match arg {
                                        FnArg::Typed(PatType { ty, .. }) => {
                                            visitor.add_full_qualified_type_match(inner_scope.clone(), ty);
                                            // visitor.convert_type(&mut *ty);
                                        },
                                        _ => {}
                                    });
                                },
                                Item::Struct(item_struct) => {
                                    item_struct.fields.iter().for_each(|Field { ty, .. }| visitor.add_full_qualified_type_match(inner_scope.clone(), ty));
                                    // item_struct.fields.iter_mut().for_each(|Field { ty, .. }| visitor.convert_type(ty));
                                },
                                Item::Enum(item_enum) => {
                                    item_enum.variants.iter().for_each(|Variant { fields, .. }| fields.iter().for_each(|Field { ty, .. }| visitor.add_full_qualified_type_match(inner_scope.clone(), ty)));
                                    // item_enum.variants.iter_mut().for_each(|Variant { fields, .. }| fields.iter_mut().for_each(|Field { ty, .. }| visitor.convert_type(ty)));

                                },
                                Item::Type(item_type) => {
                                    let ty = &*item_type.ty;
                                    visitor.add_full_qualified_type_match(inner_scope.clone(), ty);
                                    // visitor.convert_type(ty);

                                },
                                Item::Mod(_item_mod) => {},
                                _ => {}
                            }
                        })
                    }
                }
                Self::Mod(item_mod, scope)
            },
        };
        converted
    }
}

fn enum_expansion(item_enum: &ItemEnum, _scope: &Scope, tree: HashMap<TypeConversion, Type>) -> Expansion {
    // println!("expansion (enum): in: {scope} => {}", quote!(#item_enum));
    // println!("enum_expansion: [{}]: {}", scope.to_token_stream(), item_enum.ident.to_token_stream());
    let ItemEnum { ident: target_name, variants, .. } = item_enum;
    let variants_count = variants.len();
    let ffi_name = ffi_struct_name(target_name);
    let mut conversions_to_ffi = Vec::<TokenStream2>::with_capacity(variants_count);
    let mut conversions_from_ffi = Vec::<TokenStream2>::with_capacity(variants_count);
    let mut variants_fields = Vec::<TokenStream2>::with_capacity(variants_count);
    let mut destroy_fields = Vec::<TokenStream2>::new();
    let mut drop_fields = Vec::<TokenStream2>::new();
    variants.iter().for_each(|Variant { ident: variant_name, fields, discriminant, .. }| {
        let target_variant_path = quote!(#target_name::#variant_name);
        let ffi_variant_path = quote!(#ffi_name::#variant_name);
        let (variant_presenter, fields_context) = match discriminant {
            Some((_, Expr::Lit(lit, ..))) => (ENUM_UNIT_FIELDS_PRESENTER, vec![quote!(#lit)]),
            None => match fields {
                Fields::Unit => (NO_FIELDS_PRESENTER, vec![]),
                Fields::Unnamed(FieldsUnnamed { unnamed, .. }) => (
                    ENUM_UNNAMED_VARIANT_PRESENTER,
                    unnamed
                        .iter()
                        .map(|f| UNNAMED_VARIANT_FIELD_PRESENTER(f, &tree))
                        .collect(),
                ),
                Fields::Named(FieldsNamed { named, .. }) => (
                    ENUM_NAMED_VARIANT_PRESENTER,
                    named
                        .iter()
                        .map(|f| NAMED_VARIANT_FIELD_PRESENTER(f, &tree))
                        .collect(),
                ),
            },
            _ => panic!("Error variant discriminant"),
        };
        let composer = match fields {
            Fields::Unit => RootComposer::enum_unit_variant_composer(
                quote!(#ffi_variant_path),
                quote!(#target_variant_path),
                tree.clone(),
                |_| quote!(),
            ),
            Fields::Unnamed(FieldsUnnamed { unnamed, .. }) => {
                RootComposer::enum_unnamed_variant_composer(
                    quote!(#ffi_variant_path),
                    quote!(#target_variant_path),
                    tree.clone(),
                    |_| quote!(),
                    unnamed.iter().enumerate().map(|(index, Field { ty, .. })| {
                        (ty, format_ident!("o_{}", index).to_token_stream())
                    }),
                )
            }
            Fields::Named(FieldsNamed { named, .. }) => {
                RootComposer::enum_named_variant_composer(
                    quote!(#ffi_variant_path),
                    quote!(#target_variant_path),
                    tree.clone(),
                    |_| quote!(),
                    named.iter().map(|Field { ident, ty, .. }|
                        (ty, ident.clone().unwrap().to_token_stream())),
                )
            }
        };
        let composer_owned = composer.borrow();
        variants_fields.push(variant_presenter((quote!(#variant_name), fields_context)));
        conversions_from_ffi.push(composer_owned.compose_from());
        conversions_to_ffi.push(composer_owned.compose_to());
        destroy_fields.push(composer_owned.compose_destroy());
        drop_fields.push(composer_owned.compose_drop());
    },
    );
    let input = quote!(#item_enum);
    let comment = DocPresentation::Default(quote!(#target_name));
    let ffi_presentation =
        FFIObjectPresentation::Full(ENUM_PRESENTER((quote!(#ffi_name), variants_fields)));
    let conversion = ConversionInterfacePresentation::Interface {
        ffi_name: quote!(#ffi_name),
        target_name: quote!(#target_name),
        from_presentation: FFI_FROM_ROOT_PRESENTER(
            quote!(&*ffi),
            MATCH_FIELDS_PRESENTER((quote!(ffi_ref), conversions_from_ffi)),
        ),
        to_presentation: FFI_TO_ROOT_PRESENTER(
            quote!(),
            MATCH_FIELDS_PRESENTER((quote!(obj), conversions_to_ffi)),
        ),
        destroy_presentation: package_unboxed_root(),
    };
    let drop = DropInterfacePresentation::Full(
        quote!(#ffi_name),
        ENUM_DESTROY_PRESENTER(drop_fields),
    );
    Expansion::Full { input, comment, ffi_presentation, conversion, drop }
}

fn struct_expansion(item_struct: &ItemStruct, _scope: &Scope, tree: HashMap<TypeConversion, Type>) -> Expansion {
    // println!("expansion (struct): in: {scope} => {}", quote!(#item_struct));
    // println!("struct_expansion: [{}]: {}", scope.to_token_stream(), item_struct.ident.to_token_stream());
    let ItemStruct { fields: ref f, ident: target_name, .. } = item_struct;
    let composer = match f {
        Fields::Unnamed(ref fields) => match target_name.clone().to_string().as_str() {
            // Hack used to simplify some structures
            // Main problem here that without special dictionary of predefined non-std structures
            // we unable to filter out structures and provide them conversions when they are used as field types inside parent structures
            // Solution would be to write build script to preprocess and collect dictionary before macro expansion
            // in order to match struct field types with this predefined dictionary
            "UInt128" | "UInt160" | "UInt256" | "UInt384" | "UInt512" | "UInt768"
            | "VarInt" => {
                let (
                    ffi_name,
                    ffi_from_presenter,
                    ffi_from_presentation_context,
                    ffi_to_presenter,
                    ffi_to_presentation_context,
                    destroy_code_context_presenter,
                ) = match fields.unnamed.first().unwrap().ty.clone() {
                    // VarInt
                    Type::Path(TypePath { path, .. }) => {
                        let ffi_name = ffi_struct_name(&target_name);
                        (
                            quote!(#ffi_name),
                            CURLY_BRACES_FIELDS_PRESENTER,
                            vec![from_path(quote!(ffi_ref.0), &path, None)],
                            CURLY_BRACES_FIELDS_PRESENTER,
                            quote!(#ffi_name),
                            ROOT_DESTROY_CONTEXT_PRESENTER,
                        )
                    }
                    // UInt256 etc
                    Type::Array(type_array) => (
                        quote!(#type_array),
                        ROUND_BRACES_FIELDS_PRESENTER,
                        vec![quote!(ffi_ref)],
                        NO_FIELDS_PRESENTER,
                        quote!(obj.0),
                        EMPTY_MAP_PRESENTER,
                    ),
                    _ => unimplemented!(
                        "from_unnamed_struct: not supported {:?}",
                        quote!(#fields)
                    ),
                };
                RootComposer::primitive_composer(
                    quote!(#ffi_name),
                    quote!(#target_name),
                    tree,
                    EMPTY_FIELDS_PRESENTER,
                    ffi_from_presenter,
                    ffi_to_presenter,
                    destroy_code_context_presenter,
                    EMPTY_PAIR_PRESENTER,
                    ffi_from_presentation_context,
                    ffi_to_presentation_context,
                )
            }
            _ => RootComposer::unnamed_struct_composer(
                ffi_struct_name(target_name).to_token_stream(),
                quote!(#target_name),
                tree,
                fields
                    .unnamed
                    .iter()
                    .enumerate()
                    .map(|(index, Field { ty, .. })| (ty, usize_to_tokenstream(index))),
            ),
        },
        Fields::Named(ref fields) => RootComposer::named_struct_composer(
            ffi_struct_name(target_name).to_token_stream(),
            quote!(#target_name),
            tree,
            fields
                .named
                .iter()
                .map(|Field { ident, ty, .. }| (ty, quote!(#ident))),
        ),
        Fields::Unit => panic!("Fields::Unit is not supported yet"),
    };
    let composer_owned = composer.borrow();
    composer_owned.make_expansion(quote!(#item_struct))
}

fn handle_arg_type(ty: &Type, pat: &Pat) -> TokenStream2 {
    match (ty, pat) {
        (Type::Path(TypePath { path, .. }), Pat::Ident(PatIdent { ident, .. })) =>
            from_path(quote!(#ident), &path, None),
        (Type::Reference(type_reference), pat) => handle_arg_type(&type_reference.elem, pat),
        // (Type::Ptr(TypePtr { star_token, const_token, mutability, elem }), Pat::Ident(PatIdent { ident, .. })) =>
        _ => panic!("error: Arg conversion not supported: {}", quote!(#ty)),
    }
}

fn fn_expansion(item_fn: &ItemFn, _scope: &Scope, tree: HashMap<TypeConversion, Type>) -> Expansion {
    // println!("fn_expansion: [{}]: {}", scope.to_token_stream(), item_fn.sig.ident.to_token_stream());
    // println!("fn_expansion: [{:?}]:", tree);
    let Signature {
        output,
        ident: fn_name,
        inputs,
        ..
    } = &item_fn.sig;
    let (output_expression, output_conversions) = match output {
        ReturnType::Default => (quote!(()), quote!(;)),
        ReturnType::Type(_, field_type) => (
            FFI_DICTIONARY_TYPE_PRESENTER(&field_type, &tree),
            match &**field_type {
                Type::Path(TypePath { path, .. }) => to_path(quote!(obj), &path, None),
                _ => panic!("error: output conversion: {}", quote!(#field_type)),
            },
        ),
    };

    // TODO: replace Fn arguments with crate::fermented::generics::#ident or #import
    let (fn_args, conversions) = inputs
        .iter()
        .map(|arg| match arg {
            FnArg::Typed(PatType { ty, pat, .. }) => (
                NAMED_CONVERSION_PRESENTER(pat.to_token_stream(), FFI_DICTIONARY_TYPE_PRESENTER(&ty, &tree)),
                handle_arg_type(&**ty, &**pat)
            ),
            _ => panic!("Arg type not supported: {:?}", quote!(#arg)),
        })
        .unzip();

    Expansion::Function {
        input: quote!(#item_fn),
        comment: DocPresentation::Safety(quote!(#fn_name)),
        ffi_presentation: FFIObjectPresentation::Function {
            name_and_arguments: ROUND_BRACES_FIELDS_PRESENTER((
                format_ident!("ffi_{}", fn_name).to_token_stream(),
                fn_args,
            )),
            input_conversions: ROUND_BRACES_FIELDS_PRESENTER((quote!(#fn_name), conversions)),
            output_expression,
            output_conversions,
        },
    }
}

fn use_expansion(item_use: &ItemUse, _scope: &Scope) -> Expansion {
    Expansion::Use { input: quote!(#item_use), comment: DocPresentation::Empty }
}

fn type_expansion(item_type: &ItemType, _scope: &Scope, tree: HashMap<TypeConversion, Type>) -> Expansion {
    // println!("type_expansion: [{}]: {}", scope.to_token_stream(), item_type.ident.to_token_stream());
    // println!("expansion (type): in: {scope} => {}", quote!(#item_type));
    let ItemType { ident, ty, .. } = item_type;
    let ffi_name = ffi_struct_name(ident);
    match &**ty {
        // Type::Tuple()
        Type::BareFn(TypeBareFn { inputs, output, .. }) => {
            Expansion::Callback {
                input: quote!(#item_type),
                comment: DocPresentation::Default(quote!(#ffi_name)),
                ffi_presentation: FFIObjectPresentation::Callback {
                    name: quote!(#ffi_name),
                    arguments: inputs
                        .iter()
                        .map(|BareFnArg { ty, name, .. }| NAMED_CONVERSION_PRESENTER(name.clone().unwrap().0.to_token_stream(), FFI_DICTIONARY_TYPE_PRESENTER(ty, &tree)))
                        .collect::<Vec<_>>(),
                    output_expression: match output {
                        ReturnType::Default => quote!(),
                        ReturnType::Type(token, field_type) =>
                            SIMPLE_PAIR_PRESENTER(quote!(#token), FFI_DICTIONARY_TYPE_PRESENTER(&field_type, &tree))
                    },
                }
            }
        },
        _ => RootComposer::type_alias_composer(
            quote!(#ffi_name),
            quote!(#ident),
            tree,
            IntoIterator::into_iter({
                vec![(&**ty, match &**ty {
                    Type::Path(TypePath { path, .. }) => match PathConversion::from(path) {
                        PathConversion::Primitive(..) => obj(),
                        _ => usize_to_tokenstream(0),
                    },
                    Type::Array(_type_array) => usize_to_tokenstream(0),
                    Type::Ptr(_type_ptr) => obj(),
                    _ => unimplemented!("from_type_alias: not supported {}", quote!(#ty)) })]
            }))
            .borrow()
            .make_expansion(quote!(#item_type))
    }
}

fn usize_to_tokenstream(value: usize) -> TokenStream2 {
    let lit = syn::LitInt::new(&value.to_string(), Span::call_site());
    lit.to_token_stream()
}
