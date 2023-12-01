use std::collections::{HashMap, HashSet};
use std::fmt::Formatter;
use syn::{Attribute, BareFnArg, Expr, Field, Fields, FieldsNamed, FieldsUnnamed, FnArg, Ident, Item, ItemEnum, ItemFn, ItemMod, ItemStruct, ItemTrait, ItemType, ItemUse, Meta, NestedMeta, parse_quote, Pat, Path, PathSegment, PatIdent, PatType, Receiver, ReturnType, Signature, TraitItem, TraitItemMethod, TraitItemType, Type, TypeBareFn, TypePath, TypeReference, UseGlob, UseGroup, UseName, UsePath, UseRename, UseTree, Variant};
use quote::{format_ident, quote, ToTokens};
use syn::__private::{Span, TokenStream2};
use syn::punctuated::Punctuated;
use syn::token::Comma;
use crate::generics::{add_generic_type, GenericConversion, TypePathComposition};
use crate::interface::{CURLY_BRACES_FIELDS_PRESENTER, EMPTY_FIELDS_PRESENTER, EMPTY_MAP_PRESENTER, EMPTY_PAIR_PRESENTER, ENUM_DESTROY_PRESENTER, ENUM_NAMED_VARIANT_PRESENTER, ENUM_PRESENTER, ENUM_UNIT_FIELDS_PRESENTER, ENUM_UNNAMED_VARIANT_PRESENTER, FFI_DICTIONARY_TYPE_PRESENTER, NAMED_CONVERSION_PRESENTER, NAMED_VARIANT_FIELD_PRESENTER, NO_FIELDS_PRESENTER, package_unboxed_root, ROOT_DESTROY_CONTEXT_PRESENTER, ROUND_BRACES_FIELDS_PRESENTER, ROUND_ITER_PRESENTER, SIMPLE_PAIR_PRESENTER, UNNAMED_VARIANT_FIELD_PRESENTER};
use crate::helper::{ffi_destructor_name, ffi_fn_name, ffi_struct_name, ffi_trait_obj_name, ffi_vtable_name, from_path, path_arguments_to_types, to_path};
use crate::composer::{ConversionsComposer, ItemComposer};
use crate::context::Context;
use crate::formatter::{format_types_dict, format_used_traits};
use crate::visitor::Visitor;
use crate::path_conversion::{GenericPathConversion, PathConversion};
use crate::presentation::{ConversionInterfacePresentation, DocPresentation, DropInterfacePresentation, Expansion, FFIObjectPresentation, FromConversionPresentation, ToConversionPresentation, TraitVTablePresentation};
use crate::scope::{EMPTY, Scope};
use crate::import_conversion::{ImportConversion, ImportType};
use crate::type_conversion::TypeConversion;

pub struct MacroAttributes {
    pub path: Path,
    pub arguments: Vec<Path>,
}

#[derive(Clone, Default)]
pub struct ItemContext {
    pub context: Context,
    pub scope_types: HashMap<TypeConversion, Type>,
    pub traits_dictionary: HashMap<Scope, HashMap<Ident, ItemTrait>>,
}

impl ItemContext {
    pub(crate) fn with_context(context: Context) -> Self {
        Self {
            context,
            ..Default::default()
        }
    }
}

impl std::fmt::Debug for ItemContext {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ItemContext")
            .field("scope_types", &format_types_dict(&self.scope_types))
            .field("traits_dictionary", &format_used_traits(&self.traits_dictionary))
            .finish()
    }
}


impl ItemContext {
    pub fn new(context: Context, scope_types: HashMap<TypeConversion, Type>, traits_dictionary: HashMap<Scope, HashMap<Ident, ItemTrait>>) -> Self {
        Self { context, scope_types, traits_dictionary }
    }
    pub fn full_type_for(&self, ty: &Type) -> Type {
        // TODO: should transform to ffi package path
        let tc = match ty {
            Type::Reference(TypeReference { elem, .. }) => TypeConversion::from(elem),
            _ => TypeConversion::from(ty)
        };
        let full_ty = self.scope_types.get(&tc).cloned().unwrap_or(ty.clone());
        println!("full_type_for: {}: {}" , quote!(#tc), quote!(#full_ty));
        println!(" ------------: {}" , format_types_dict(&self.scope_types));
        full_ty
    }

    pub fn ffi_full_type_for(&self, ty: &Type) -> Type {
        let tc = match ty {
            Type::Reference(TypeReference { elem, .. }) => TypeConversion::from(elem),
            _ => TypeConversion::from(ty)
        };
        let full_ty = self.scope_types.get(&tc).cloned().unwrap_or(ty.clone());
        // let ffi_full_ty = Scope::ffi_type_converted_or_same(&full_ty);
        let ffi_full_ty = Scope::ffi_external_type_converted_or_same(&full_ty, &self.context);
        println!("## ffi_full_type_for: {} ==> {}", quote!(#ty), quote!(#ffi_full_ty));
        ffi_full_ty
    }
}

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
    pub ident: Option<Ident>,
    pub scope: Scope,
    pub return_type: FnReturnTypeDecomposition,
    pub arguments: Vec<FnArgDecomposition>,
}

fn handle_fn_return_type(output: &ReturnType, context: &ItemContext) -> FnReturnTypeDecomposition {
    match output {
        ReturnType::Default => FnReturnTypeDecomposition { presentation: quote!(()), conversion: quote!(;) },
        ReturnType::Type(_, field_type) => {
            let full_ty = context.ffi_full_type_for(field_type);
            let presentation = FFI_DICTIONARY_TYPE_PRESENTER(&full_ty, context);
            let conversion = match &**field_type {
                Type::Path(TypePath { path, .. }) => to_path(quote!(obj), path, None, context),
                _ => panic!("error: output conversion: {}", quote!(#field_type)),
            };
            FnReturnTypeDecomposition { presentation, conversion }
        },
    }
}
fn handle_bare_fn_return_type(output: &ReturnType, context: &ItemContext) -> FnReturnTypeDecomposition {
    match output {
        ReturnType::Default => FnReturnTypeDecomposition { presentation: quote!(), conversion: quote!() },
        ReturnType::Type(token, field_type) => {
            let full_ty = context.ffi_full_type_for(field_type);
            let pres = FFI_DICTIONARY_TYPE_PRESENTER(&full_ty, context);
            let presentation = SIMPLE_PAIR_PRESENTER(quote!(#token), pres);
            FnReturnTypeDecomposition { presentation, conversion: quote!() }
        }
    }
}
fn handle_fn_args(inputs: &Punctuated<FnArg, Comma>, context: &ItemContext) -> Vec<FnArgDecomposition> {
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
            FnArg::Typed(PatType { ty, pat, .. }) => {
                let full_ty = context.ffi_full_type_for(ty);
                let pres = FFI_DICTIONARY_TYPE_PRESENTER(&full_ty, context);
                // println!("handle_fn_args: {} --> {} ---> {}", quote!(#ty), quote!(#full_ty), quote!(#pres));
                let name_type_original = NAMED_CONVERSION_PRESENTER(pat.to_token_stream(), quote!(#pres));
                let name_type_conversion = handle_arg_type(ty, pat, context);
                // println!("handle_fn_args: [{}: {}] ---> {} +++ {}", quote!(#pat), quote!(#ty), quote!(#name_type_original), quote!(#name_type_conversion));
                FnArgDecomposition {
                    name: Some(pat.to_token_stream()),
                    name_type_original,
                    name_type_conversion
                }
            },
        })
        .collect()
}

fn handle_bare_fn_args(inputs: &Punctuated<BareFnArg, Comma>, context: &ItemContext) -> Vec<FnArgDecomposition> {
    inputs
        .iter()
        .map(|BareFnArg { ty, name, .. }| {
            let name = name.clone().map(|(ident, _)| ident.to_token_stream());
            let full_ty = context.ffi_full_type_for(ty);
            let pres = FFI_DICTIONARY_TYPE_PRESENTER(&full_ty, context);
            FnArgDecomposition {
                name: name.clone(),
                name_type_original: NAMED_CONVERSION_PRESENTER(name.unwrap(), pres),
                name_type_conversion: quote!()
            }
        })
        .collect::<Vec<_>>()
}

impl FnSignatureDecomposition {
    pub fn from_signature(sig: &Signature, scope: Scope, context: &ItemContext) -> Self {
        let Signature { output, ident, inputs, .. } = sig;
        // TODO: make a path
        FnSignatureDecomposition {
            ident: Some(ident.clone()),
            scope,
            return_type: handle_fn_return_type(output, context),
            arguments: handle_fn_args(inputs, context)
        }
    }

    pub fn from_bare_fn(bare_fn: &TypeBareFn, ident: &Ident, scope: Scope, context: &ItemContext) -> Self {
        let TypeBareFn { inputs, output, .. } = bare_fn;
        let arguments = handle_bare_fn_args(inputs, context);
        let return_type = handle_bare_fn_return_type(output, context);
        FnSignatureDecomposition {
            ident: Some(ident.clone()),
            scope,
            arguments,
            return_type
        }
    }

    pub fn present_callback(self) -> FFIObjectPresentation {
        let arguments = self.arguments.iter().map(|arg| arg.name_type_original.clone()).collect();
        let output_expression = self.return_type.presentation;
        FFIObjectPresentation::Callback {
            name: self.ident.clone().unwrap().to_token_stream(),
            arguments,
            output_expression
        }
    }

    pub fn present_fn(self) -> FFIObjectPresentation {
        let arguments = self.arguments.iter().map(|arg| arg.name_type_original.clone()).collect();
        let fn_name = self.ident.unwrap();
        let full_fn_path = self.scope.joined(&fn_name);
        let argument_conversions = self.arguments.iter().map(|arg| arg.name_type_conversion.clone()).collect();
        let name_and_arguments = ROUND_BRACES_FIELDS_PRESENTER((ffi_fn_name(&fn_name).to_token_stream(), arguments));
        let input_conversions = ROUND_BRACES_FIELDS_PRESENTER((quote!(#full_fn_path), argument_conversions));
        let output_expression = self.return_type.presentation;
        let output_conversions = self.return_type.conversion;
        FFIObjectPresentation::Function { name_and_arguments, input_conversions, output_expression, output_conversions }
    }

    pub fn present_trait_vtable_inner_fn(self) -> TokenStream2 {
        let arguments = self.arguments.iter().map(|arg| arg.name_type_original.clone()).collect();
        let fn_name = self.ident.unwrap();
        let name_and_args = ROUND_BRACES_FIELDS_PRESENTER((quote!(unsafe extern "C" fn), arguments));
        let output_expression = self.return_type.presentation;
        quote!(pub #fn_name: #name_and_args -> #output_expression)
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
            item => Err(format!("Error: {}", item.to_token_stream()))
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
            item => Err(format!("Error: {}", item.to_token_stream()))
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
            item => Err(format!("Error: {}", item.to_token_stream()))
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
            UseTree::Group(UseGroup { items , .. }) =>
                items.iter().flat_map(Self::fold_use).collect()
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

        let result = path.segments
            .iter()
            .any(|segment| segment.ident == self.macro_ident());
        //println!("is_labeled_with_macro: {} => {}", result, quote!(#path));
        result
    }

    pub fn is_owner_labeled_with_trait_implementation(&self, path: &Path) -> bool {
        path.segments
            .iter()
            .any(|segment| segment.ident == self.macro_ident())
    }

    pub fn has_macro_attribute(&self) -> bool {
        self.attrs().iter().filter(|Attribute { path, .. }| self.is_labeled_with_macro(path)).count() > 0
    }

    pub fn handle_attributes_with_handler<F: FnMut(MacroAttributes)>(&self, attrs: &[Attribute], mut handler: F) {
        attrs.iter()
            .for_each(|attr|
                if self.is_labeled_with_macro(&attr.path) || self.is_owner_labeled_with_trait_implementation(&attr.path) {
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

    pub fn collect_compositions(&self) -> Vec<TypePathComposition> {
        let mut type_and_paths: Vec<TypePathComposition> = Vec::new();
        let mut cache_type = |ty: &Type, path: &Path|
            type_and_paths.push(TypePathComposition(ty.clone(), path.clone()));
        let mut cache_fields = |fields: &Fields, attrs: &MacroAttributes| match fields {
            Fields::Unnamed(FieldsUnnamed { unnamed: fields, .. }) |
            Fields::Named(FieldsNamed { named: fields, .. }) =>
                fields.iter().for_each(|field| cache_type(&field.ty, &attrs.path)),
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
                    cache_type(ty, &attrs.path)),
            Self::Fn(item_fn, ..) =>
                self.handle_attributes_with_handler(&item_fn.attrs, |attrs| {
                    item_fn.sig.inputs.iter().for_each(|arg|
                        if let FnArg::Typed(PatType { ty, .. }) = arg {
                            cache_type(ty, &attrs.path);
                        });
                    if let ReturnType::Type(_, ty) = &item_fn.sig.output {
                        // if let Type::Path(TypePath { path, .. }) = &**ty {
                            cache_type(ty, &attrs.path);
                        // }
                    }
                }),
            Self::Trait(item_trait, ..) => self.handle_attributes_with_handler(&item_trait.attrs, |attrs| {
                item_trait.items.iter().for_each(|trait_item| match trait_item {
                    TraitItem::Type(TraitItemType { default: Some((_, ty)), .. }) =>
                        cache_type(ty, &attrs.path),
                    TraitItem::Method(TraitItemMethod { sig, .. }) => {
                        sig.inputs.iter().for_each(|arg|
                            if let FnArg::Typed(PatType { ty, .. }) = arg {
                                cache_type(ty, &attrs.path);
                            });
                        if let ReturnType::Type(_, ty) = &sig.output {
                            // if let Type::Path(TypePath { path, .. }) = &**ty {
                                cache_type(ty, &attrs.path);
                            // }
                        }
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
            if let Some(PathSegment { ident, .. }) = path.segments.last() {
                let (import_type, scope) = Self::import_pair(path, imports);
                container.entry(import_type)
                    .or_insert_with(HashSet::new)
                    .insert(ImportConversion::from((ident, &scope)));
            }
        });
    }

    fn cache_fields_in(container: &mut HashMap<ImportType, HashSet<ImportConversion>>, fields: &Fields, imports: &HashMap<Ident, Path>) {
        match fields {
            Fields::Unnamed(FieldsUnnamed { unnamed: fields, .. }) |
            Fields::Named(FieldsNamed { named: fields, .. }) =>
                fields.iter()
                    .for_each(|field| Self::cache_type_in(container, &field.ty, imports)),
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
    fn collect_generic_types_in_type(field_type: &Type, generics: &mut HashSet<TypePathComposition>) {
        if let Type::Path(TypePath { path, .. }) = field_type {
            match PathConversion::from(path) {
                PathConversion::Complex(path) => {
                    if let Some(last_segment) = path.segments.last() {
                        if last_segment.ident.to_string().as_str() == "Option" {
                            Self::collect_generic_types_in_type(path_arguments_to_types(&last_segment.arguments)[0], generics);
                        }
                    }
                },
                PathConversion::Generic(GenericPathConversion::Result(path)) |
                PathConversion::Generic(GenericPathConversion::Vec(path)) |
                PathConversion::Generic(GenericPathConversion::Map(path)) => {
                    path_arguments_to_types(&path.segments.last().unwrap().arguments)
                        .iter()
                        .for_each(|field_type|
                            add_generic_type(field_type, generics));
                    generics.insert(TypePathComposition(field_type.clone(), path.clone()));
                },
                _ => {}
            }
        }
    }

    fn find_generic_types_in_compositions(compositions: &[TypePathComposition]) -> HashSet<TypePathComposition> {
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

    pub fn find_generics_fq(&self, scope_types: &HashMap<TypeConversion, Type>) -> HashSet<GenericConversion> {
        self.find_generics()
            .iter()
            .filter_map(|TypePathComposition { 0: value, .. }| scope_types.get(&TypeConversion::from(value)))
            .map(GenericConversion::from)
            .collect()
    }

    fn add_full_qualified_signature(visitor: &mut Visitor, sig: &Signature, scope: &Scope) {
        if let ReturnType::Type(_, ty) = &sig.output {
            visitor.add_full_qualified_type_match(scope.clone(), ty)
        }
        sig.inputs.iter().for_each(|arg| if let FnArg::Typed(PatType { ty, .. }) = arg {
            visitor.add_full_qualified_type_match(scope.clone(), ty);
        });
    }
    fn add_full_qualified_trait(visitor: &mut Visitor, item_trait: &ItemTrait, scope: &Scope) {
        visitor.add_full_qualified_trait_match(scope.clone(), item_trait);
        item_trait.items.iter().for_each(|trait_item|
            if let TraitItem::Method(TraitItemMethod { sig, .. }) = trait_item {
                Self::add_full_qualified_signature(visitor, sig, scope)
            })
    }

    fn add_full_qualified_trait_type_from_macro(visitor: &mut Visitor, item_trait_attrs: &[Attribute], scope: &Scope) {
        let trait_names = extract_trait_names(item_trait_attrs);
        trait_names.iter().for_each(|trait_name|
            visitor.add_full_qualified_type_match(scope.clone(), &parse_quote!(#trait_name)));
    }

    fn add_full_qualified_type_from_enum(visitor: &mut Visitor, item_enum: &ItemEnum, scope: &Scope) {
        item_enum.variants.iter().for_each(|Variant { fields, .. }|
            fields.iter().for_each(|Field { ty, .. }|
                visitor.add_full_qualified_type_match(scope.clone(), ty)));
    }

    fn add_full_qualified_type_from_struct(visitor: &mut Visitor, item_struct: &ItemStruct, scope: &Scope) {
        item_struct.fields.iter().for_each(|Field { ty, .. }|
            visitor.add_full_qualified_type_match(scope.clone(), ty));
    }

    fn add_itself_conversion(visitor: &mut Visitor, scope: &Scope, ident: &Ident, ty: Type) {
        visitor.usage_info.used_types_at_scopes.entry(scope.clone())
            .or_insert_with(HashMap::new)
            .insert(TypeConversion::new(parse_quote!(#ident)), ty);
    }

    pub fn add_full_qualified_conversion(self, visitor: &mut Visitor) -> ItemConversion {
        match self {
            Self::Struct(item_struct, scope) => {
                let ident = &item_struct.ident;
                let itself = scope.joined(ident);
                Self::add_itself_conversion(visitor, &scope, ident, itself.to_type());
                Self::add_full_qualified_trait_type_from_macro(visitor, &item_struct.attrs, &scope);
                Self::add_full_qualified_type_from_struct(visitor, &item_struct, &scope);
                Self::Struct(item_struct, scope)
            },
            Self::Enum(item_enum, scope) => {
                let ident = &item_enum.ident;
                let itself = scope.joined(ident);
                println!("add_full_qualified_conversion [Enum]: {}: [{}] -> {}", quote!(#ident), quote!(#scope), quote!(#itself));
                Self::add_itself_conversion(visitor, &scope, ident, itself.to_type());
                Self::add_full_qualified_trait_type_from_macro(visitor, &item_enum.attrs, &scope);
                Self::add_full_qualified_type_from_enum(visitor, &item_enum, &scope);
                Self::Enum(item_enum, scope)
            },
            Self::Type(item_type, scope) => {
                let ident = &item_type.ident;
                let itself = scope.joined(ident);
                Self::add_itself_conversion(visitor, &scope, ident, itself.to_type());
                visitor.add_full_qualified_type_match(scope.clone(), &item_type.ty);
                Self::Type(item_type, scope)
            },
            Self::Fn(item_fn, scope) => {
                let ident = &item_fn.sig.ident;
                let itself = scope.joined(ident);
                Self::add_itself_conversion(visitor, &scope, ident, itself.to_type());
                Self::add_full_qualified_signature(visitor, &item_fn.sig, &scope);
                Self::Fn(item_fn, scope)
            },
            Self::Trait(item_trait, scope) => {
                let ident = &item_trait.ident;
                let itself = scope.joined(ident);
                Self::add_itself_conversion(visitor, &scope, ident, itself.to_type());
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
                            Item::Fn(ItemFn { sig, .. }) => {
                                let ident = &sig.ident;
                                let itself = inner_scope.joined(ident);
                                Self::add_itself_conversion(visitor, &inner_scope, ident, itself.to_type());
                                Self::add_full_qualified_signature(visitor, &sig, &inner_scope)
                            },
                            Item::Trait(item_trait) => {
                                let ident = &item_trait.ident;
                                let itself = inner_scope.joined(ident);
                                Self::add_itself_conversion(visitor, &inner_scope, ident, itself.to_type());
                                Self::add_full_qualified_trait(visitor, &item_trait, &inner_scope)
                            },
                            Item::Struct(item_struct) => {
                                let ident = &item_struct.ident;
                                let itself = inner_scope.joined(ident);
                                Self::add_itself_conversion(visitor, &inner_scope, ident, itself.to_type());
                                Self::add_full_qualified_trait_type_from_macro(visitor, &item_struct.attrs, &inner_scope);
                                Self::add_full_qualified_type_from_struct(visitor, &item_struct, &inner_scope);
                            },
                            Item::Enum(item_enum) => {
                                let ident = &item_enum.ident;
                                let itself = inner_scope.joined(ident);
                                println!("add_full_qualified_conversion [Enum inner]: {}: [{}] -> {}", quote!(#ident), quote!(#inner_scope), quote!(#itself));
                                Self::add_itself_conversion(visitor, &inner_scope, ident, itself.to_type());
                                Self::add_full_qualified_trait_type_from_macro(visitor, &item_enum.attrs, &inner_scope);
                                Self::add_full_qualified_type_from_enum(visitor, &item_enum, &inner_scope);
                            },
                            Item::Type(item_type) => {
                                let ident = &item_type.ident;
                                let itself = scope.joined(ident);
                                Self::add_itself_conversion(visitor, &inner_scope, ident, itself.to_type());
                                visitor.add_full_qualified_type_match(inner_scope.clone(), &item_type.ty)
                            },
                            _ => {}
                        })
                    }
                }
                Self::Mod(item_mod, scope)
            },
        }
    }

    pub fn make_expansion(&self, item_context: ItemContext) -> Expansion {
        match self {
            ItemConversion::Mod(..) => Expansion::Empty,
            ItemConversion::Struct(item, scope) => struct_expansion(item, scope, item_context),
            ItemConversion::Enum(item, scope) => enum_expansion(item, scope, item_context),
            ItemConversion::Type(item, scope) => type_expansion(item, scope, item_context),
            ItemConversion::Fn(item, scope) => {
                let signature = FnSignatureDecomposition::from_signature(&item.sig, scope.popped(), &item_context);
                Expansion::Function {
                    comment: DocPresentation::Safety(signature.ident.to_token_stream()),
                    ffi_presentation: signature.present_fn(),
                }
            },
            ItemConversion::Trait(item, scope) => trait_expansion(item, scope, item_context),
            ItemConversion::Use(_item, _scope) => Expansion::Use { comment: DocPresentation::Empty },
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


fn enum_expansion(item_enum: &ItemEnum, item_scope: &Scope, context: ItemContext) -> Expansion {
    let ItemEnum { ident: target_name, variants, .. } = item_enum;
    let variants_count = variants.len();
    let ffi_name = ffi_struct_name(target_name);
    let traits = item_traits_expansions((target_name, item_scope), &item_enum.attrs, &context);
    let mut conversions_to_ffi = Vec::<TokenStream2>::with_capacity(variants_count);
    let mut conversions_from_ffi = Vec::<TokenStream2>::with_capacity(variants_count);
    let mut variants_fields = Vec::<TokenStream2>::with_capacity(variants_count);
    let mut destroy_fields = Vec::<TokenStream2>::new();
    let mut drop_fields = Vec::<TokenStream2>::new();
    variants.iter().for_each(|Variant { ident: variant_name, fields, discriminant, .. }| {
        let full_ty = context.full_type_for(&parse_quote!(#target_name));
        //println!("variant: {} -> {}", quote!(#variant_name), quote!(#full_ty));
        let target_variant_path = quote!(#full_ty::#variant_name);
        let ffi_variant_path = quote!(#ffi_name::#variant_name);
        let (variant_presenter, fields_context) = match discriminant {
            Some((_, Expr::Lit(lit, ..))) => (ENUM_UNIT_FIELDS_PRESENTER, vec![quote!(#lit)]),
            None => match fields {
                Fields::Unit => (NO_FIELDS_PRESENTER, vec![]),
                Fields::Unnamed(FieldsUnnamed { unnamed, .. }) => (
                    ENUM_UNNAMED_VARIANT_PRESENTER,
                    unnamed
                        .iter()
                        .map(|f| UNNAMED_VARIANT_FIELD_PRESENTER(f, &context))
                        .collect(),
                ),
                Fields::Named(FieldsNamed { named, .. }) => (
                    ENUM_NAMED_VARIANT_PRESENTER,
                    named
                        .iter()
                        .map(|f| NAMED_VARIANT_FIELD_PRESENTER(f, &context))
                        .collect(),
                ),
            },
            _ => panic!("Error variant discriminant"),
        };
        let composer = match fields {
            Fields::Unit => ItemComposer::enum_unit_variant_composer(
                parse_quote!(#ffi_variant_path),
                parse_quote!(#target_variant_path),
                context.clone()
            ),
            Fields::Unnamed(fields) => {
                ItemComposer::enum_unnamed_variant_composer(
                    parse_quote!(#ffi_variant_path),
                    parse_quote!(#target_variant_path),
                    context.clone(),
                    ConversionsComposer::UnnamedEnumVariant(fields),
                )
            }
            Fields::Named(fields) => {
                ItemComposer::enum_named_variant_composer(
                    parse_quote!(#ffi_variant_path),
                    parse_quote!(#target_variant_path),
                    context.clone(),
                    ConversionsComposer::NamedStruct(fields),
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
    // let ffi_full_type = item_context.ffi_full_type_for(&parse_quote!(#ffi_name));
    let target_full_type = context.full_type_for(&parse_quote!(#target_name));
    // println!("enummm: {}: {}", quote!(#target_name), quote!(#target_full_type));
    // println!("context: {:#?}", item_context);

    let conversion = ConversionInterfacePresentation::Interface {
        ffi_type: parse_quote!(#ffi_name),
        target_type: quote!(#target_full_type),
        from_presentation: FromConversionPresentation::Enum(conversions_from_ffi),
        to_presentation: ToConversionPresentation::Enum(conversions_to_ffi),
        destroy_presentation: package_unboxed_root(),
    };
    let drop = DropInterfacePresentation::Full(
        quote!(#ffi_name),
        ENUM_DESTROY_PRESENTER(drop_fields),
    );
    // let constructor = ConversionInterfacePresentation::Constructor {
    //     ffi_ident: ffi_name.clone(),
    //     ctor_arguments: vec![],
    //     body_presentation: quote!()
    //     // constructor_ident: ffi_constructor_name(&ffi_name).to_token_stream(),
    //     // fields_presentation: quote!(),
    //     // arguments_presentation: quote!(),
    // };
    let constructor = ConversionInterfacePresentation::Empty;
    let destructor = ConversionInterfacePresentation::Destructor {
        ffi_name: quote!(#ffi_name),
        destructor_ident: ffi_destructor_name(&ffi_name).to_token_stream()
    };
    Expansion::Full { comment, ffi_presentation, conversion, drop, constructor, destructor, traits }
}

fn implement_trait_for_item(item_trait: (&ItemTrait, &Scope), item: (&Ident, &Scope), context: &ItemContext) -> TraitVTablePresentation {
    let (item_trait, trait_scope) = item_trait;
    let (item_name, item_scope) = item;
    let field_compositions = trait_fields_compositions(&item_trait.items, &EMPTY, context);
    let trait_ident = &item_trait.ident;
    let item_full_ty = context.full_type_for(&parse_quote!(#item_name));
    let trait_full_ty = context.full_type_for(&parse_quote!(#trait_ident));
    let (vtable_methods_implentations, vtable_methods_declarations): (Vec<TokenStream2>, Vec<TokenStream2>) = field_compositions.into_iter()
        .map(|signature_decomposition| {
            let FnReturnTypeDecomposition { presentation: output_expression, conversion: output_conversions } = signature_decomposition.return_type;
            let fn_name = signature_decomposition.ident.unwrap();
            let ffi_method_ident = format_ident!("{}_{}", item_name, fn_name);
            let arguments = signature_decomposition.arguments.iter().map(|arg| arg.name_type_original.clone()).collect();
            let mut argument_conversions = vec![quote!(cast_obj)];
            argument_conversions.extend(signature_decomposition.arguments.iter().filter(|arg| arg.name.is_some()).map(|arg| arg.name_type_conversion.clone()));
            let name_and_args = ROUND_BRACES_FIELDS_PRESENTER((quote!(unsafe extern "C" fn #ffi_method_ident), arguments));
            let argument_names = ROUND_ITER_PRESENTER(argument_conversions);
                // let cast_obj = &(*(obj as *const crate::chain::common::chain_type::DevnetType));
                // let obj = <crate::chain::common::chain_type::DevnetType as crate::chain::common::chain_type::IHaveChainSettings>::should_process_llmq_of_type(cast_obj, llmq_type);
                // obj
                (quote!(#name_and_args -> #output_expression {
                let cast_obj = &(*(obj as *const #item_full_ty));
                let obj = <#item_full_ty as #trait_full_ty>::#fn_name #argument_names;
                #output_conversions
            }), quote!(#fn_name: #ffi_method_ident))
            // (quote!(#name_and_args -> #output_expression {
            //     let cast_obj = &(*(obj as *const #item_name));
            //     let obj = cast_obj.#fn_name #argument_names;
            //     #output_conversions
            // }), quote!(#fn_name: #ffi_method_ident))
    }).unzip();
    let trait_vtable_ident = ffi_vtable_name(trait_ident);
    let trait_object_ident = ffi_trait_obj_name(trait_ident);
    let trait_implementor_vtable_ident = format_ident!("{}_{}", item_name, trait_vtable_ident);
    let item_module = item_scope.popped();
    let (fq_trait_vtable, fq_trait_object) = if item_module.eq(trait_scope) {
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

pub fn trait_items_from_attributes(attrs: &[Attribute], item_context: &ItemContext) -> Vec<(ItemTrait, Scope)> {
    let attr_traits = extract_trait_names(attrs);
    attr_traits.iter()
        .map(|trait_name| {
            let trait_ty = parse_quote!(#trait_name);
            let trait_ty_conversion = TypeConversion::new(trait_ty);
            let full_trait_ty = item_context.scope_types.get(&trait_ty_conversion).unwrap();
            let trait_scope = Scope::extract_type_scope(full_trait_ty);
            let scope_traits = item_context.traits_dictionary.get(&trait_scope).unwrap();
            let trait_ident = parse_quote!(#trait_name);
            let item_trait = scope_traits.get(&trait_ident).cloned().unwrap();
            (item_trait, trait_scope)
        })
        .collect()
}

fn item_traits_expansions(item: (&Ident, &Scope), attrs: &[Attribute], context: &ItemContext) -> Vec<TraitVTablePresentation> {
    let trait_types = trait_items_from_attributes(attrs, context);
    trait_types.iter()
        .map(|(item_trait, trait_scope)| implement_trait_for_item((item_trait, trait_scope), item, &context))
        .collect()
}

fn struct_expansion(item_struct: &ItemStruct, _scope: &Scope, item_context: ItemContext) -> Expansion {
    // println!("expansion (struct): in: {scope} => {}", quote!(#item_struct));
    // println!("struct_expansion: [{}]: {}", scope.to_token_stream(), item_struct.ident.to_token_stream());
    let ItemStruct { fields: ref f, ident: target_name, .. } = item_struct;
    let traits = item_traits_expansions((target_name, _scope), &item_struct.attrs, &item_context);
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

                        let ffi_name = ffi_struct_name(target_name);
                        (
                            quote!(#ffi_name),
                            CURLY_BRACES_FIELDS_PRESENTER,
                            vec![from_path(quote!(ffi_ref.0), &path, None, &item_context)],
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
                    _ => unimplemented!("from_unnamed_struct: not supported {:?}", quote!(#fields)),
                };
                println!("primitive_composer: {} --> {}", quote!(#target_name), quote!(#ffi_name));
                let ffi_name = ffi_struct_name(target_name).to_token_stream();
                ItemComposer::primitive_composer(
                    parse_quote!(#ffi_name),
                    parse_quote!(#target_name),
                    item_context,
                    EMPTY_FIELDS_PRESENTER,
                    ffi_from_presenter,
                    ffi_to_presenter,
                    destroy_code_context_presenter,
                    EMPTY_PAIR_PRESENTER,
                    ffi_from_presentation_context,
                    parse_quote!(#ffi_to_presentation_context),
                )
            }
            _ => ItemComposer::unnamed_struct_composer(
                {
                    // let full_ty = item_context.ffi_full_type_for(&parse_quote!(#target_name));
                    let ffi_name = ffi_struct_name(target_name).to_token_stream();
                    println!("unnamed_struct_composer.2: {} --> {}", quote!(#target_name), quote!(#ffi_name));
                    parse_quote!(#ffi_name)
                },
                {
                    println!("unnamed_struct_composer.3: {}", quote!(#target_name));
                    let full_ty = item_context.full_type_for(&parse_quote!(#target_name));
                    parse_quote!(#full_ty)
                },
                item_context,
                ConversionsComposer::UnnamedStruct(fields)
            ),
        },
        Fields::Named(ref fields) => ItemComposer::named_struct_composer(
            {
                // let full_ty = item_context.ffi_full_type_for(&parse_quote!(#target_name));
                let ffi_name = ffi_struct_name(target_name).to_token_stream();
                println!("named_struct_composer: {} --> {}", quote!(#target_name), quote!(#ffi_name));
                parse_quote!(#ffi_name)
            },
            {
                let full_ty = item_context.full_type_for(&parse_quote!(#target_name));
                parse_quote!(#full_ty)
            },
            item_context,
            ConversionsComposer::NamedStruct(fields)
        ),
        Fields::Unit => panic!("Fields::Unit is not supported yet"),
    };
    let composer_owned = composer.borrow();

    composer_owned.make_expansion(/*ffi_destructor_name(&ffi_struct_name(target_name)).to_token_stream(), */traits)
}

fn handle_arg_type(ty: &Type, pat: &Pat, context: &ItemContext) -> TokenStream2 {
    match (ty, pat) {
        (Type::Path(TypePath { path, .. }), Pat::Ident(PatIdent { ident, .. })) =>
            from_path(quote!(#ident), path, None, context),
        (Type::Reference(type_reference), pat) => {
            let arg_type = handle_arg_type(&type_reference.elem, pat, context);
            // arg_type
            if let Some(_mutable) = type_reference.mutability {
                quote!(&mut #arg_type)
            } else {
                quote!(&#arg_type)
            }
        },
        // (Type::Ptr(TypePtr { star_token, const_token, mutability, elem }), Pat::Ident(PatIdent { ident, .. })) =>
        _ => panic!("error: Arg conversion not supported: {}", quote!(#ty)),
    }
}

fn trait_item_presentation(trait_item: &TraitItem, scope: Scope, context: &ItemContext)
    -> Option<FnSignatureDecomposition> {
    match trait_item {
        TraitItem::Method(TraitItemMethod { sig, .. } ) =>
            Some(FnSignatureDecomposition::from_signature(sig, scope, context)),
        TraitItem::Type(TraitItemType { ident: _, type_token: _, .. }) =>
            None,
        _ => None
    }
}

fn trait_fields_compositions(trait_items: &[TraitItem], scope: &Scope, context: &ItemContext) -> Vec<FnSignatureDecomposition> {
    trait_items
        .iter()
        .filter_map(|trait_item| trait_item_presentation(trait_item, scope.clone(), context))
        .collect::<Vec<_>>()
}

fn trait_expansion(item_trait: &ItemTrait, scope: &Scope, context: ItemContext) -> Expansion {
    let field_compositions = trait_fields_compositions(&item_trait.items, scope, &context);
    let fields = field_compositions.into_iter().map(FnSignatureDecomposition::present_trait_vtable_inner_fn).collect();

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

fn type_expansion(item_type: &ItemType, scope: &Scope, context: ItemContext) -> Expansion {
    // println!("type_expansion: [{}]: {}", scope.to_token_stream(), item_type.ident.to_token_stream());
    // println!("expansion (type): in: {scope} => {}", quote!(#item_type));
    let ItemType { ident, ty, .. } = item_type;
    let ffi_name = ffi_struct_name(ident);
    match &**ty {
        // Type::Tuple()
        Type::BareFn(bare_fn) => {
            let decomposition = FnSignatureDecomposition::from_bare_fn(bare_fn, &ffi_name, scope.clone(), &context);
            Expansion::Callback {
                comment: DocPresentation::Default(quote!(#ffi_name)),
                ffi_presentation: decomposition.present_callback(),
            }
        },
        _ => {
            let traits = item_traits_expansions((ident, scope), &item_type.attrs, &context);
            ItemComposer::type_alias_composer(
                // {
                //     let full_ty = item_context.ffi_full_type_for(&parse_quote!(#target_name));
                //     // let ffi_name = ffi_struct_name(target_name).to_token_stream();
                //     parse_quote!(#full_ty)
                // },
                // {
                //     let full_ty = item_context.full_type_for(&parse_quote!(#target_name));
                //     parse_quote!(#full_ty)
                // },

                {
                    let ffi_name = ffi_struct_name(ident).to_token_stream();
                    // println!("type_alias_composer: {} --> {}", quote!(#ident), quote!(#ffi_name));
                    parse_quote!(#ffi_name)
                },
                {
                    let full_ty = context.full_type_for(&parse_quote!(#ident));
                    // println!("typealias: {}: {}", quote!(#ident), quote!(#full_ty));
                    // println!("context: {:?}", context);
                    parse_quote!(#full_ty)
                },
                context,
                ConversionsComposer::TypeAlias(ty))
                .borrow()
                .make_expansion(/*&ffi_destructor_name(&ffi_name).to_token_stream(), */traits)
        }
    }
}

pub fn usize_to_tokenstream(value: usize) -> TokenStream2 {
    let lit = syn::LitInt::new(&value.to_string(), Span::call_site());
    lit.to_token_stream()
}
