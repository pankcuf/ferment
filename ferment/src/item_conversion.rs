use std::collections::{HashMap, HashSet};
use std::fmt::Formatter;
use syn::{Attribute, BareFnArg, Expr, Field, Fields, FieldsNamed, FieldsUnnamed, FnArg, Ident, Item, ItemEnum, ItemFn, ItemMod, ItemStruct, ItemTrait, ItemType, ItemUse, Meta, NestedMeta, parse_quote, Pat, Path, PathSegment, PatIdent, PatType, Receiver, ReturnType, Signature, TraitItem, TraitItemMethod, TraitItemType, Type, TypeBareFn, TypePath, UseGlob, UseGroup, UseName, UsePath, UseRename, UseTree, Variant};
use quote::{format_ident, quote, ToTokens};
use syn::__private::{Span, TokenStream2};
use syn::punctuated::Punctuated;
use syn::token::Comma;
use crate::generics::{add_generic_type, TypePathComposition};
use crate::interface::{CURLY_BRACES_FIELDS_PRESENTER, EMPTY_FIELDS_PRESENTER, EMPTY_MAP_PRESENTER, EMPTY_PAIR_PRESENTER, ENUM_DESTROY_PRESENTER, ENUM_NAMED_VARIANT_PRESENTER, ENUM_PRESENTER, ENUM_UNIT_FIELDS_PRESENTER, ENUM_UNNAMED_VARIANT_PRESENTER, FFI_DICTIONARY_TYPE_PRESENTER, FFI_FROM_ROOT_PRESENTER, FFI_TO_ROOT_PRESENTER, MATCH_FIELDS_PRESENTER, NAMED_CONVERSION_PRESENTER, NAMED_VARIANT_FIELD_PRESENTER, NO_FIELDS_PRESENTER, obj, package_unboxed_root, ROOT_DESTROY_CONTEXT_PRESENTER, ROUND_BRACES_FIELDS_PRESENTER, ROUND_ITER_PRESENTER, SIMPLE_PAIR_PRESENTER, UNNAMED_VARIANT_FIELD_PRESENTER};
use crate::helper::{ffi_destructor_name, ffi_fn_name, ffi_struct_name, ffi_trait_obj_name, ffi_unnamed_arg_name, ffi_vtable_name, from_path, path_arguments_to_types, to_path};
use crate::composer::ItemComposer;
use crate::visitor::Visitor;
use crate::path_conversion::{GenericPathConversion, PathConversion};
use crate::presentation::{ConversionInterfacePresentation, DocPresentation, DropInterfacePresentation, Expansion, FFIObjectPresentation, TraitVTablePresentation};
use crate::scope::{EMPTY, Scope};
use crate::import_conversion::{ImportConversion, ImportType};
use crate::type_conversion::TypeConversion;

pub struct FnReturnTypeDecomposition {
    pub presentation: TokenStream2,
    pub conversion: TokenStream2
}
pub struct FnArgDecomposition {
    pub name: Option<TokenStream2>,
    pub name_type_original: TokenStream2,
    pub name_type_conversion: TokenStream2,
}
pub struct FnSignatureDecomposition {
    pub ident: Ident,
    pub return_type: FnReturnTypeDecomposition,
    pub arguments: Vec<FnArgDecomposition>,
}

impl FnSignatureDecomposition {
    pub fn from_signature(sig: &Signature, tree: &HashMap<TypeConversion, Type>) -> Self {
        let Signature { output, ident, inputs, .. } = sig;
        FnSignatureDecomposition {
            ident: ident.clone(),
            return_type: handle_fn_return_type(output, tree),
            arguments: handle_fn_args(inputs, tree)
        }
    }

    pub fn present_fn(self) -> FFIObjectPresentation {
        let arguments = self.arguments.iter().map(|arg| arg.name_type_original.clone()).collect();
        let fn_name = self.ident;
        let argument_conversions = self.arguments.iter().map(|arg| arg.name_type_conversion.clone()).collect();
        let name_and_arguments = ROUND_BRACES_FIELDS_PRESENTER((ffi_fn_name(&fn_name).to_token_stream(), arguments));
        let input_conversions = ROUND_BRACES_FIELDS_PRESENTER((quote!(#fn_name), argument_conversions));
        let output_expression = self.return_type.presentation;
        let output_conversions = self.return_type.conversion;
        FFIObjectPresentation::Function { name_and_arguments, input_conversions, output_expression, output_conversions }
    }
}

#[derive(Clone)]
pub enum ItemConversion {
    Mod(ItemMod, Scope),
    Struct(ItemStruct, Scope),
    Enum(ItemEnum, Scope),
    Type(ItemType, Scope),
    Fn(ItemFn, Scope),
    Use(ItemUse, Scope),
    Trait(ItemTrait, Scope),
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
            ItemConversion::Use(item, ..) => item.to_tokens(tokens),
        }
    }
}

impl From<(ItemConversion, HashMap<TypeConversion, Type>, HashMap<Scope, HashMap<Ident, ItemTrait>>)> for Expansion {
    fn from(conversion: (ItemConversion, HashMap<TypeConversion, Type>, HashMap<Scope, HashMap<Ident, ItemTrait>>)) -> Self {
        match &conversion.0 {
            ItemConversion::Mod(..) => Expansion::Empty,
            ItemConversion::Struct(item, scope) => struct_expansion(item, scope, conversion.1, conversion.2),
            ItemConversion::Enum(item, scope) => enum_expansion(item, scope, conversion.1, conversion.2),
            ItemConversion::Type(item, scope) => type_expansion(item, scope, conversion.1, conversion.2),
            ItemConversion::Fn(item, scope) => fn_expansion(item, scope, conversion.1, conversion.2),
            ItemConversion::Trait(item, scope) => trait_expansion(item, scope, conversion.1, conversion.2),
            ItemConversion::Use(item, scope) => use_expansion(item, scope),
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
            item => Err(format!("Error: {}", item.to_token_stream().to_string()))
        }
    }
}

impl<'a> TryFrom<(&'a Item, &'a Scope)> for ItemConversion {
    type Error = String;
    fn try_from(value: (&'a Item, &'a Scope)) -> Result<Self, Self::Error> {
        match value.0 {
            Item::Mod(item) => Ok(Self::Mod(item.clone(), value.1.clone())),
            Item::Struct(item) => Ok(Self::Struct(item.clone(), value.1.clone())),
            Item::Enum(item) => Ok(Self::Enum(item.clone(), value.1.clone())),
            Item::Type(item) => Ok(Self::Type(item.clone(), value.1.clone())),
            Item::Fn(item) => Ok(Self::Fn(item.clone(), value.1.clone())),
            Item::Trait(item) => Ok(Self::Trait(item.clone(), value.1.clone())),
            item => Err(format!("Error: {}", item.to_token_stream().to_string()))
        }
    }
}

impl<'a> TryFrom<(Item, &'a Scope)> for ItemConversion {
    type Error = String;
    fn try_from(value: (Item, &'a Scope)) -> Result<Self, Self::Error> {
        match value.0 {
            Item::Mod(item) => Ok(Self::Mod(item, value.1.clone())),
            Item::Struct(item) => Ok(Self::Struct(item, value.1.clone())),
            Item::Enum(item) => Ok(Self::Enum(item, value.1.clone())),
            Item::Type(item) => Ok(Self::Type(item, value.1.clone())),
            Item::Fn(item) => Ok(Self::Fn(item, value.1.clone())),
            Item::Trait(item) => Ok(Self::Trait(item, value.1.clone())),
            item => Err(format!("Error: {}", item.to_token_stream().to_string()))
        }
    }
}

impl<'a> TryFrom<(&'a Item, Scope)> for ItemConversion {
    type Error = String;
    fn try_from(value: (&'a Item, Scope)) -> Result<Self, Self::Error> {
        match value.0 {
            Item::Mod(item) => Ok(Self::Mod(item.clone(), value.1)),
            Item::Struct(item) => Ok(Self::Struct(item.clone(), value.1)),
            Item::Enum(item) => Ok(Self::Enum(item.clone(), value.1)),
            Item::Type(item) => Ok(Self::Type(item.clone(), value.1)),
            Item::Fn(item) => Ok(Self::Fn(item.clone(), value.1)),
            Item::Trait(item) => Ok(Self::Trait(item.clone(), value.1)),
            item => Err(format!("Error: {}", item.to_token_stream().to_string()))
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
            ItemConversion::Trait(item, _) =>  parse_quote!(#item),
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
            Self::Trait(..) => "trait",
        }
    }
    pub const fn r#type(&self) -> &str {
        match self {
            Self::Mod(..) |
            Self::Struct(..) |
            Self::Enum(..) |
            Self::Type(..) |
            Self::Fn(..) |
            Self::Trait(..) => "export",
            Self::Use(..) => "",
        }
    }

    pub fn scope(&self) -> &Scope {
        match self {
            ItemConversion::Mod(_, scope) => scope,
            ItemConversion::Struct(_, scope) => scope,
            ItemConversion::Enum(_, scope) => scope,
            ItemConversion::Type(_, scope) => scope,
            ItemConversion::Fn(_, scope) => scope,
            ItemConversion::Trait(_, scope) => scope,
            ItemConversion::Use(_, scope) => scope,
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
            ItemConversion::Trait(ItemTrait { ident, .. }, ..) => ident,
            ItemConversion::Use(ItemUse { tree, .. }, ..) => Self::fold_use(tree).first().unwrap(),
        }
    }

    fn macro_ident(&self) -> Ident {
        format_ident!("{}", self.r#type())
    }

    pub fn is_labeled_with_macro(&self, path: &Path) -> bool {
        path.segments
            .iter()
            .any(|segment| segment.ident == self.macro_ident())
    }

    pub fn has_macro_attribute(&self) -> bool {
        self.attrs().iter().filter(|Attribute { path, .. }| self.is_labeled_with_macro(path)).count() > 0
    }

    pub fn handle_attributes_with_handler<F: FnMut(&Path)>(&self, attrs: &[Attribute], mut handler: F) {
        attrs.iter()
            .for_each(|Attribute { path, .. }|
                if self.is_labeled_with_macro(path) {
                    handler(path)
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
            Self::Trait(item_trait, ..) => self.handle_attributes_with_handler(&item_trait.attrs, |path| {
                item_trait.items.iter().for_each(|trait_item| match trait_item {
                    TraitItem::Type(TraitItemType { default: Some((_, ty)), .. }) =>
                        cache_type(ty, path),
                    TraitItem::Method(TraitItemMethod { sig, .. }) => {
                        sig.inputs.iter().for_each(|arg| match arg {
                            FnArg::Typed(PatType { ty, .. }) =>
                                cache_type(ty, path),
                            _ => {}
                        });
                        match &sig.output {
                            ReturnType::Default => {},
                            ReturnType::Type(_, ty) => match &**ty {
                                Type::Path(TypePath { path, .. }) =>
                                    cache_type(ty, path),
                                _ => {}
                            }
                        };

                    },
                    _ => {}
                });
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
            Self::Trait(item_trait, ..) =>
                self.handle_attributes_with_handler(&item_trait.attrs, |_path| {
                    item_trait.items.iter().for_each(|trait_item| match trait_item {
                        TraitItem::Method(TraitItemMethod { sig, .. }) => {
                            sig.inputs.iter().for_each(|arg| match arg {
                                FnArg::Typed(PatType { ty, .. }) =>
                                    Self::cache_type_in(&mut container, ty, imports),
                                _ => {}
                            });
                            if let ReturnType::Type(_, ty) = &sig.output {
                                Self::cache_type_in(&mut container, &**ty, imports)
                            };
                        },
                        _ => {}
                    });
                }),
            _ => {}
        }
        container
    }
    fn collect_generic_types_in_type(field_type: &Type, generics: &mut HashSet<TypePathComposition>) {
        match field_type {
            Type::Path(TypePath { path, .. }) => match PathConversion::from(path) {
                PathConversion::Complex(path) => {
                    match path.segments.last().unwrap().ident.to_string().as_str() {
                        "Option" =>
                            Self::collect_generic_types_in_type(path_arguments_to_types(&path.segments.last().unwrap().arguments)[0], generics),
                        _ => {}
                    }
                },
                PathConversion::Generic(GenericPathConversion::Vec(path)) |
                PathConversion::Generic(GenericPathConversion::Map(path)) => {
                    path_arguments_to_types(&path.segments.last().unwrap().arguments)
                        .iter()
                        .for_each(|field_type|
                            add_generic_type(field_type, generics));
                    generics.insert(TypePathComposition(field_type.clone(), path.clone()));
                },
                _ => {}
            },
            _ => {}
        }
    }

    fn find_generic_types_in_compositions(compositions: &Vec<TypePathComposition>) -> HashSet<TypePathComposition> {
        // collect all types with generics and ensure their uniqueness
        // since we don't want to implement interface multiple times for same object
        let mut generics: HashSet<TypePathComposition> = HashSet::new();
        compositions
            .iter()
            .for_each(|TypePathComposition(field_type, .. )|
                Self::collect_generic_types_in_type(field_type, &mut generics));
        generics
    }

    pub(crate) fn find_generics(&self) -> HashSet<TypePathComposition> {
        Self::find_generic_types_in_compositions(&self.collect_compositions())
    }

    fn add_full_qualified_signature<'ast>(visitor: &'ast mut Visitor, sig: &Signature, scope: &Scope) {
        if let ReturnType::Type(_, ty) = &sig.output {
            visitor.add_full_qualified_type_match(scope.clone(), ty)
        }
        sig.inputs.iter().for_each(|arg| match arg {
            FnArg::Typed(PatType { ty, .. }) => {
                visitor.add_full_qualified_type_match(scope.clone(), ty);
            },
            _ => {}
        });
    }
    fn add_full_qualified_trait<'ast>(visitor: &'ast mut Visitor, item_trait: &ItemTrait, scope: &Scope) {
        visitor.add_full_qualified_trait_match(scope.clone(), item_trait);
        item_trait.items.iter().for_each(|trait_item| match trait_item {
            TraitItem::Method(TraitItemMethod { sig, .. }) =>
                Self::add_full_qualified_signature(visitor, sig, &scope),
            _ => {}
        })
    }

    fn add_full_qualified_trait_type_from_macro<'ast>(visitor: &'ast mut Visitor, item_trait_attrs: &[Attribute], scope: &Scope) {
        let trait_names = extract_trait_names(item_trait_attrs);
        trait_names.iter().for_each(|trait_name|
            visitor.add_full_qualified_type_match(scope.clone(), &parse_quote!(#trait_name)));

    }

    pub fn add_full_qualified_conversion<'ast>(self, visitor: &'ast mut Visitor) -> ItemConversion {
        let converted = match self {
            Self::Struct(item_struct, scope) => {
                Self::add_full_qualified_trait_type_from_macro(visitor, &item_struct.attrs, &scope);
                item_struct.fields.iter().for_each(|Field { ty, .. }| visitor.add_full_qualified_type_match(scope.clone(), ty));
                Self::Struct(item_struct, scope)
            },
            Self::Enum(item_enum, scope) => {
                Self::add_full_qualified_trait_type_from_macro(visitor, &item_enum.attrs, &scope);
                item_enum.variants.iter().for_each(|Variant { fields, .. }|
                    fields.iter().for_each(|Field { ty, .. }|
                        visitor.add_full_qualified_type_match(scope.clone(), ty)));
                Self::Enum(item_enum, scope)
            },
            Self::Type(item_type, scope) => {
                visitor.add_full_qualified_type_match(scope.clone(), &*item_type.ty);
                Self::Type(item_type, scope)
            },
            Self::Fn(item_fn, scope) => {
                Self::add_full_qualified_signature(visitor, &item_fn.sig, &scope);
                Self::Fn(item_fn, scope)
            },
            Self::Trait(item_trait, scope) => {
                Self::add_full_qualified_trait(visitor, &item_trait, &scope);
                Self::Trait(item_trait, scope)
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
                            Item::Fn(ItemFn { sig, .. }) =>
                                Self::add_full_qualified_signature(visitor, &sig, &inner_scope),
                            Item::Trait(item_trait) =>
                                Self::add_full_qualified_trait(visitor, &item_trait, &inner_scope),
                            Item::Struct(item_struct) =>
                                item_struct.fields.iter().for_each(|Field { ty, .. }|
                                    visitor.add_full_qualified_type_match(inner_scope.clone(), ty)),
                            Item::Enum(item_enum) =>
                                item_enum.variants.iter().for_each(|Variant { fields, .. }|
                                    fields.iter().for_each(|Field { ty, .. }|
                                        visitor.add_full_qualified_type_match(inner_scope.clone(), ty))),
                            Item::Type(item_type) =>
                                visitor.add_full_qualified_type_match(inner_scope.clone(), &*item_type.ty),
                            _ => {}
                        })
                    }
                }
                Self::Mod(item_mod, scope)
            },
        };
        converted
    }
}


fn extract_trait_names(attrs: &[Attribute]) -> Vec<Path> {
    let mut paths = Vec::<Path>::new();
    attrs.iter().for_each(|attr| {
        if attr.path.segments
            .iter()
            .any(|segment| segment.ident == format_ident!("export")) {
            match attr.parse_meta() {
                Ok(Meta::List(meta_list)) => {
                    meta_list.nested.iter().for_each(|meta| {
                        if let NestedMeta::Meta(Meta::Path(path)) = meta {
                            paths.push(path.clone());
                        }
                    });
                },
                _ => {}
            }
        }
    });
    paths
}


fn enum_expansion(item_enum: &ItemEnum, _scope: &Scope, tree: HashMap<TypeConversion, Type>, traits: HashMap<Scope, HashMap<Ident, ItemTrait>>) -> Expansion {
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
            Fields::Unit => ItemComposer::enum_unit_variant_composer(
                quote!(#ffi_variant_path),
                quote!(#target_variant_path),
                tree.clone()
            ),
            Fields::Unnamed(FieldsUnnamed { unnamed, .. }) => {
                ItemComposer::enum_unnamed_variant_composer(
                    quote!(#ffi_variant_path),
                    quote!(#target_variant_path),
                    tree.clone(),
                    unnamed.iter().enumerate().map(|(index, Field { ty, .. })|
                        (ty, ffi_unnamed_arg_name(index).to_token_stream())
                    ),
                )
            }
            Fields::Named(FieldsNamed { named, .. }) => {
                ItemComposer::enum_named_variant_composer(
                    quote!(#ffi_variant_path),
                    quote!(#target_variant_path),
                    tree.clone(),
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
    });
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
    let destructor = ConversionInterfacePresentation::Destructor {
        ffi_name: quote!(#ffi_name),
        destructor_ident: ffi_destructor_name(&ffi_name).to_token_stream()
    };
    let traits = item_traits_expansions(target_name, _scope, &item_enum.attrs, &tree, traits);
    Expansion::Full { comment, ffi_presentation, conversion, drop, destructor, traits }
}

fn implement_trait_for_item(item_trait: &ItemTrait, item_name: &Ident, item_scope: &Scope, trait_export_scope: &Scope, tree: &HashMap<TypeConversion, Type>) -> TraitVTablePresentation {
    let field_compositions = trait_fields_compositions(&item_trait.items, &tree);
    let trait_ident = &item_trait.ident;
    let (vtable_methods_implentations, vtable_methods_declarations): (Vec<TokenStream2>, Vec<TokenStream2>) = field_compositions.into_iter()
        .map(|signature_decomposition| {
        let FnReturnTypeDecomposition { presentation: output_expression, conversion: output_conversions } = signature_decomposition.return_type;
        let fn_name = signature_decomposition.ident;
        let ffi_method_ident = format_ident!("{}_{}", item_name, fn_name);
        let arguments = signature_decomposition.arguments.iter().map(|arg| arg.name_type_original.clone()).collect();
        let argument_conversions = signature_decomposition.arguments.iter().filter(|arg| arg.name.is_some()).map(|arg| arg.name_type_conversion.clone()).collect();
        let name_and_args = ROUND_BRACES_FIELDS_PRESENTER((quote!(unsafe extern "C" fn #ffi_method_ident), arguments));
        let argument_names = ROUND_ITER_PRESENTER(argument_conversions);
        (quote!(#name_and_args -> #output_expression {
            let cast_obj = &(*(obj as *const #item_name));
            let obj = cast_obj.#fn_name #argument_names;
            #output_conversions
        }), quote!(#fn_name: #ffi_method_ident))
    }).unzip();
    let trait_vtable_ident = ffi_vtable_name(trait_ident);
    let trait_object_ident = ffi_trait_obj_name(trait_ident);
    let trait_implementor_vtable_ident = format_ident!("{}_{}", item_name, trait_vtable_ident);
    let item_module = item_scope.popped();
    let (fq_trait_vtable, fq_trait_object) = if item_module.eq(trait_export_scope) {
        (quote!(#trait_vtable_ident), quote!(#trait_object_ident))
    } else {
        (quote!(#trait_export_scope::#trait_vtable_ident), quote!(#trait_export_scope::#trait_object_ident))
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
        #[allow(non_snake_case)]
        #[no_mangle]
        pub extern "C" fn #binding_ident(obj: *const #item_name) -> #fq_trait_object {
            #fq_trait_object {
                object: obj as *const (),
                vtable: &#trait_implementor_vtable_ident,
            }
        }
    };
    let destructor = quote! {
        #[allow(non_snake_case)]
        #[no_mangle]
        pub unsafe extern "C" fn #destructor_binding_ident(obj: #fq_trait_object) {
            ferment_interfaces::unbox_any(obj.object as *mut #item_name);
        }
    };
    TraitVTablePresentation::Full { vtable, export, destructor }
}

fn item_traits_expansions(item_name: &Ident, item_scope: &Scope, attrs: &[Attribute], tree: &HashMap<TypeConversion, Type>, traits: HashMap<Scope, HashMap<Ident, ItemTrait>>) -> Vec<TraitVTablePresentation> {
    let attr_traits = extract_trait_names(attrs);
    attr_traits.iter()
        .map(|trait_name| {
            let tc = TypeConversion::new(parse_quote!(#trait_name));
            let full_trait_path = tree.get(&tc).unwrap();
            let trait_export_scope = Scope::extract_type_scope(full_trait_path);
            let scope_trait = traits.get(&trait_export_scope).unwrap();
            let ident = parse_quote!(#trait_name);
            let item_trait = scope_trait.get(&ident).unwrap();
            implement_trait_for_item(item_trait, item_name, item_scope, &trait_export_scope,  tree)
        })
        .collect()

}

fn struct_expansion(item_struct: &ItemStruct, _scope: &Scope, tree: HashMap<TypeConversion, Type>, traits: HashMap<Scope, HashMap<Ident, ItemTrait>>) -> Expansion {
    // println!("expansion (struct): in: {scope} => {}", quote!(#item_struct));
    // println!("struct_expansion: [{}]: {}", scope.to_token_stream(), item_struct.ident.to_token_stream());
    let ItemStruct { fields: ref f, ident: target_name, .. } = item_struct;
    let traits = item_traits_expansions(target_name, _scope, &item_struct.attrs, &tree, traits);
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
                ItemComposer::primitive_composer(
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
            _ => ItemComposer::unnamed_struct_composer(
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
        Fields::Named(ref fields) => ItemComposer::named_struct_composer(
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

    composer_owned.make_expansion(ffi_destructor_name(&ffi_struct_name(target_name)).to_token_stream(), traits)
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

fn handle_fn_return_type(output: &ReturnType, tree: &HashMap<TypeConversion, Type>) -> FnReturnTypeDecomposition {
    match output {
        ReturnType::Default => FnReturnTypeDecomposition { presentation: quote!(()), conversion: quote!(;) },
        ReturnType::Type(_, field_type) => FnReturnTypeDecomposition {
            presentation: FFI_DICTIONARY_TYPE_PRESENTER(&field_type, tree),
            conversion: match &**field_type {
                Type::Path(TypePath { path, .. }) => to_path(quote!(obj), &path, None),
                _ => panic!("error: output conversion: {}", quote!(#field_type)),
            },
        },
    }
}

fn handle_fn_args(inputs: &Punctuated<FnArg, Comma>, tree: &HashMap<TypeConversion, Type>) -> Vec<FnArgDecomposition> {
    // TODO: replace Fn arguments with crate::fermented::generics::#ident or #import
    inputs
        .iter()
        .map(|arg| match arg {
            FnArg::Receiver(Receiver { mutability, .. }) => FnArgDecomposition {
                name: None,
                name_type_original: match mutability {
                    Some(..) => quote!(obj: *mut ()),
                    _ => quote!(obj: *const ())
                },
                name_type_conversion: quote!()
            },
            FnArg::Typed(PatType { ty, pat, .. }) => FnArgDecomposition {
                name: Some(pat.to_token_stream()),
                name_type_original: NAMED_CONVERSION_PRESENTER(pat.to_token_stream(), FFI_DICTIONARY_TYPE_PRESENTER(&ty, tree)),
                name_type_conversion: handle_arg_type(&**ty, &**pat)
            },
        })
        .collect()
}

fn fn_expansion(item_fn: &ItemFn, _scope: &Scope, tree: HashMap<TypeConversion, Type>, _traits: HashMap<Scope, HashMap<Ident, ItemTrait>>) -> Expansion {
    let Signature { output, ident, inputs, .. } = &item_fn.sig;
    let signature_decomposition = FnSignatureDecomposition {
        ident: ident.clone(),
        return_type: handle_fn_return_type(output, &tree),
        arguments: handle_fn_args(inputs, &tree)
    };
    Expansion::Function {
        comment: DocPresentation::Safety(quote!(#ident)),
        ffi_presentation: signature_decomposition.present_fn(),
    }
}


fn trait_item_presentation(trait_item: &TraitItem, tree: &HashMap<TypeConversion, Type>)
    -> Option<FnSignatureDecomposition> {
    match trait_item {
        TraitItem::Method(TraitItemMethod { sig, .. } ) =>
            Some(FnSignatureDecomposition::from_signature(sig, tree)),
        TraitItem::Type(TraitItemType { ident: _, type_token: _, .. }) =>
            None,
        _ => None
    }
}

fn trait_fields_compositions(trait_items: &Vec<TraitItem>, tree: &HashMap<TypeConversion, Type>) -> Vec<FnSignatureDecomposition> {
    trait_items
        .iter()
        .filter_map(|trait_item| trait_item_presentation(trait_item, tree))
        .collect::<Vec<_>>()
}

fn trait_expansion(item_trait: &ItemTrait, _scope: &Scope, tree: HashMap<TypeConversion, Type>, _traits: HashMap<Scope, HashMap<Ident, ItemTrait>>) -> Expansion {
    let field_compositions = trait_fields_compositions(&item_trait.items, &tree);

    let fields = field_compositions.into_iter().map(|signature| {
        let name_and_args = ROUND_BRACES_FIELDS_PRESENTER((quote!(unsafe extern "C" fn), signature.arguments.iter().map(|arg| arg.name_type_original.clone()).collect()));
        let fn_name = signature.ident;
        let output_expression = signature.return_type.presentation;
        quote!(pub #fn_name: #name_and_args -> #output_expression)

    }).collect();

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

fn use_expansion(_item_use: &ItemUse, _scope: &Scope) -> Expansion {
    Expansion::Use { comment: DocPresentation::Empty }
}

fn type_expansion(item_type: &ItemType, scope: &Scope, tree: HashMap<TypeConversion, Type>, traits: HashMap<Scope, HashMap<Ident, ItemTrait>>) -> Expansion {
    // println!("type_expansion: [{}]: {}", scope.to_token_stream(), item_type.ident.to_token_stream());
    // println!("expansion (type): in: {scope} => {}", quote!(#item_type));
    let ItemType { ident, ty, .. } = item_type;
    let ffi_name = ffi_struct_name(ident);
    match &**ty {
        // Type::Tuple()
        Type::BareFn(TypeBareFn { inputs, output, .. }) => {
            Expansion::Callback {
                comment: DocPresentation::Default(quote!(#ffi_name)),
                ffi_presentation: FFIObjectPresentation::Callback {
                    name: quote!(#ffi_name),
                    arguments: inputs
                        .iter()
                        .map(|BareFnArg { ty, name, .. }|
                            NAMED_CONVERSION_PRESENTER(
                                name.clone().unwrap().0.to_token_stream(),
                                FFI_DICTIONARY_TYPE_PRESENTER(ty, &tree)))
                        .collect::<Vec<_>>(),
                    output_expression: match output {
                        ReturnType::Default => quote!(),
                        ReturnType::Type(token, field_type) =>
                            SIMPLE_PAIR_PRESENTER(
                                quote!(#token),
                                FFI_DICTIONARY_TYPE_PRESENTER(&field_type, &tree))
                    },
                }
            }
        },
        _ => {
            let traits = item_traits_expansions(ident, scope, &item_type.attrs, &tree, traits);
            ItemComposer::type_alias_composer(
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
                .make_expansion(ffi_destructor_name(&ffi_name).to_token_stream(), traits)
        }
    }
}

fn usize_to_tokenstream(value: usize) -> TokenStream2 {
    let lit = syn::LitInt::new(&value.to_string(), Span::call_site());
    lit.to_token_stream()
}
