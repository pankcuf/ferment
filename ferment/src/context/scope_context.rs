use std::collections::{HashMap, HashSet};
use std::fmt::Formatter;
use std::sync::{Arc, RwLock};
use proc_macro2::Ident;
use quote::{quote, quote_spanned, ToTokens};
use syn::{Attribute, Item, parse_quote, Path, PathSegment, spanned::Spanned, TraitBound, Type, TypeArray, TypeParamBound, TypePath, TypePtr, TypeReference, TypeSlice, TypeTraitObject};
use syn::punctuated::Punctuated;
use syn::token::Colon2;
use crate::composition::{Composition, GenericConversion, ImportComposition, TraitCompositionPart1, TypeComposition};
use crate::context::{GlobalContext, ScopeChain};
use crate::conversion::{GenericPathConversion, ImportConversion, ObjectConversion, TypeConversion};
use crate::ext::{Accessory, extract_trait_names, Mangle};
use crate::helper::{path_arguments_to_paths, path_arguments_to_types};
use crate::holder::PathHolder;

#[derive(Clone)]
pub struct ScopeContext {
    pub scope: ScopeChain,
    pub context: Arc<RwLock<GlobalContext>>
}

impl std::fmt::Debug for ScopeContext {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ScopeContext")
            .field("scope", &self.scope)
            .field("context", &self.context)
            .finish()
    }
}

impl std::fmt::Display for ScopeContext {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}

impl ScopeContext {
    pub fn is_from_current_crate(&self) -> bool {
        let context = self.context.read().unwrap();
        context.config.current_crate.ident().eq(self.scope.crate_scope())
    }
    pub fn with(scope: ScopeChain, context: Arc<RwLock<GlobalContext>>) -> Self {
        Self { scope, context }
    }
    pub fn add_custom_conversion(&self, scope: ScopeChain, path: PathHolder, ffi_type: Type) {
        // Here we don't know about types in pass 1, we can only use imports
        let mut lock = self.context.write().unwrap();
        let regular_type = lock.maybe_import(&scope, &path)
            .unwrap_or(&path.0).clone();
        lock.custom.add_conversion(regular_type, ffi_type, scope);
    }
    pub fn full_type_for(&self, ty: &Type) -> Type {
        let lock = self.context.read().unwrap();
        let full_ty = lock.maybe_type(ty, &self.scope)
            .and_then(|full_type| full_type.ty().cloned())
            .unwrap_or(ty.clone());
        // println!("[{}] [{}] full_type_for: {} is: [{}]", self.scope.crate_scope(), self.scope.self_path_holder(), quote!(#ty), quote!(#full_ty));

        full_ty
        // lock.maybe_scope_type_or_parent_type(ty, &self.scope)
        //     .and_then(|sty| sty.ty().cloned())
        //     .unwrap_or(ty.clone())
    }

    fn trait_ty(&self, ty: &Type) -> Option<ObjectConversion> {
        // println!("FFI (check...1) for: {}", format_token_stream(ty));
        let lock = self.context.read().unwrap();
        let mut maybe_trait = lock.resolve_trait_type(ty);
        // println!("FFI (trait) for: {}", maybe_trait.map_or(format!("None"), |m| m.to_string()));
        match maybe_trait {
            Some(ObjectConversion::Type(ty) | ObjectConversion::Item(ty, _)) => {
                // loc
                // check maybe it's really known
                let trait_scope = lock.actual_scope_for_type(ty.ty(), &self.scope);
                if let Some(obj) = lock.maybe_scope_object(&parse_quote!(Self), &trait_scope) {
                    maybe_trait = Some(obj);
                }
                // if let Some(tt) = lock.maybe_scope_type(&parse_quote!(Self), &parse_quote!(#ty)) {
                //     maybe_trait = Some(tt);
                // }
                // maybe_trait = lock.maybe_scope_type(&parse_quote!(Self), &parse_quote!(#ty));
                // println!("FFI (trait unknown but maybe known) for: {}", maybe_trait.map_or(format!("None"), |m| m.to_string()));
                // if let Some(ty) = maybe_trait {
                //     println!("FFI (trait unknown but known) for: {}", ty.to_string());
                // }
            },
            _ => {}
        }
        maybe_trait.cloned()
    }

    fn ffi_internal_type_for(&self, ty: &Type) -> Type {
        let lock = self.context.read().unwrap();
        let tyty = lock.maybe_type(ty, &self.scope)
            .and_then(|external_type| {
                match external_type.type_conversion() {
                    Some(TypeConversion::Trait(ty, _decomposition, _super_bounds)) =>
                        self.trait_ty(&ty.ty)
                            .map(|oc| oc.type_conversion().cloned()),
                    _ => None
                }.unwrap_or(external_type.type_conversion().cloned())

            }).unwrap_or(TypeConversion::Unknown(TypeComposition::new(ty.clone(), None)));
        // let tyty = lock.maybe_scope_type(ty, &self.scope)
        //     .and_then(|external_type| {
        //         match external_type.type_conversion() {
        //             Some(TypeConversion::Trait(ty, _decomposition)) =>
        //                 self.trait_ty(&ty.ty)
        //                     .map(|oc| oc.type_conversion().cloned()),
        //             _ => None
        //         }.unwrap_or(external_type.type_conversion().cloned())
        //
        //     }).unwrap_or(TypeConversion::Unknown(TypeComposition::new(ty.clone(), None)));
        let result = match tyty.ty() {
            Type::Path(TypePath { path, .. }) =>
                self.ffi_external_path_converted(path, self.scope.crate_scope()),
            _ => None
        };

        let result = result.unwrap_or(ty.clone());

        // let mangled = result.to_mangled_ident_default();
        // parse_quote!(#mangled)
        //println!("FFI (ffi_internal_type_for) for: {} in [{}] = {}", ty.to_token_stream(), self.scope, format_token_stream(&result));
        result
    }

    pub fn ffi_custom_or_internal_type(&self, ty: &Type) -> Type {
        let lock = self.context.read().unwrap();
        lock.custom.maybe_conversion(ty)
            .unwrap_or(self.ffi_internal_type_for(ty))
    }

    // pub fn find_item_trait_in_scope(&self, trait_name: &Path, scope: &ScopeChain) -> (TraitCompositionPart1, ScopeChain) {
    //     let trait_ty = parse_quote!(#trait_name);
    //     let lock = self.context.read().unwrap();
    //     let full_trait_ty = lock.maybe_type(&trait_ty, scope).unwrap();
    //     let trait_ident = parse_quote!(#trait_name);
    //     let trait_scope = full_trait_ty.as_scope();
    //
    //     let trait_scope = lock.actual_scope_for_path(full_trait_ty);
    //
    //     //let trait_scope = ScopeChain::Trait { self_scope: trait_scope, parent_scope_chain: Box::new(scope.clone()) };
    //     println!("find_item_trait_in_scope.2: {}: {}", format_token_stream(&trait_ident), &trait_scope);
    //     let item_trait = self.item_trait_with_ident_for(&trait_ident, &trait_scope).unwrap();
    //     // let trait_scope_chain = ScopeChain::Trait {
    //     //     self_scope: trait_scope,
    //     //     parent_scope_chain: Box::new(ScopeChain::Mod { self_scope: self.scope.self_scope().clone() }),
    //     // };
    //     (item_trait, trait_scope)
    // }
    // pub fn find_item_trait_scope_pair(&self, trait_name: &Path) -> (TraitCompositionPart1, ScopeChain) {
    //     println!("find_item_trait_scope_pair.1: {}", format_token_stream(trait_name));
    //     let trait_ty = parse_quote!(#trait_name);
    //     let lock = self.context.read().unwrap();
    //     // let full_trait_ty = lock.maybe_type(&trait_ty, &self.scope).unwrap();
    //     let trait_scope = lock.actual_scope_for_type(&trait_ty, &self.scope);
    //     // trait_scope.se
    //     // let trait_ident = parse_quote!(#trait_name);
    //     // let trait_scope = full_trait_ty.as_scope();
    //     println!("find_item_trait_scope_pair.2: {}", trait_scope);
    //     let item_trait = self.item_trait_with_ident_for(&trait_ident, &trait_scope).unwrap();
    //     // let trait_scope_chain = ScopeChain::Trait {
    //     //     self_scope: trait_scope,
    //     //     parent_scope_chain: Box::new(ScopeChain::Mod { self_scope: self.scope.self_scope().clone() }),
    //     // };
    //     (item_trait, trait_scope)
    // }

    pub fn scope_type_for_path(&self, path: &Path) -> Option<Type> {
        let lock = self.context.read().unwrap();
        lock.scope_register.scope_type_for_path(path, &self.scope)
    }

    // pub fn item_trait_with_ident_for(&self, ident: &Ident, scope: &ScopeChain) -> Option<TraitCompositionPart1> {
    //     println!("item_trait_with_ident_for: {} in [{}] ", format_token_stream(ident), format_token_stream(scope));
    //     let lock = self.context.read().unwrap();
    //     lock.traits.item_trait_with_ident_for(ident, scope).cloned()
    // }

    pub fn find_generics_fq_in(&self, item: &Item, scope: &ScopeChain) -> HashSet<GenericConversion> {
        // println!("find_generics_fq_in: {} in [{}]", item.ident(), format_token_stream(scope));
        let lock = self.context.read().unwrap();
        lock.scope_register.find_generics_fq_in(item, scope)
    }

    pub fn find_used_imports(&self, item: &Item) -> Option<HashMap<ImportConversion, HashSet<ImportComposition>>> {
        let lock = self.context.read().unwrap();
        lock.imports.find_used_imports(item, &self.scope)
    }

    pub fn populate_imports_and_generics(&self, scope: &ScopeChain, item: &Item, imported: &mut HashMap<ImportConversion, HashSet<ImportComposition>>, generics: &mut HashSet<GenericConversion>) {
        if let Some(scope_imports) = self.find_used_imports(item) {
            scope_imports
                .iter()
                .for_each(|(import_type, imports)|
                    imported.entry(import_type.clone())
                        .or_insert_with(HashSet::new)
                        .extend(imports.clone()));
        }
        generics.extend(self.find_generics_fq_in(item, &scope));
    }

    pub fn ffi_path_converted_or_same(&self, path: &Path) -> Type {
        self.resolve_ffi_path(path)
            .unwrap_or(parse_quote!(#path))
    }

    pub fn convert_to_ffi_path(&self, generic_path_conversion: &GenericPathConversion) -> Type {
        // println!("convert_to_ffi_path: {}", format_token_stream(generic_path_conversion));
        let path = generic_path_conversion.as_path();
        let mut cloned_segments = path.segments.clone();
        let last_segment = cloned_segments.iter_mut().last().unwrap();
        let last_ident = &last_segment.ident;
        let result = match last_ident.to_string().as_str() {
            // simple primitive type
            "i8" | "u8" | "i16" | "u16" | "i32" | "u32" | "i64" | "u64" | "i128" | "u128" |
            "isize" | "usize" | "bool" => parse_quote!(#last_ident),
            // complex special type
            "str" | "String" => parse_quote!(std::os::raw::c_char),
            "Vec" | "BTreeMap" | "HashMap" => {
                let ffi_name = path.to_mangled_ident_default();
                parse_quote!(crate::fermented::generics::#ffi_name)
            },
            "Result" if cloned_segments.len() == 1 => {
                let ffi_name = path.to_mangled_ident_default();
                parse_quote!(crate::fermented::generics::#ffi_name)

            },
            _ => {
                let new_segments = cloned_segments
                    .into_iter()
                    .map(|segment| quote_spanned! { segment.span() => #segment })
                    .collect::<Vec<_>>();
                parse_quote!(#(#new_segments)::*)
            }
        };
        // println!("•• [FFI] convert_to_ffi_path (generic): {} --> {}", format_token_stream(path), format_token_stream(&result));
        result
    }

    fn resolve_ffi_path(&self, path: &Path) -> Option<Type> {
        let segments = &path.segments;
        let last_segment = segments.iter().last().unwrap();
        let last_ident = &last_segment.ident;
        // println!("ffi_path_converted: {}", format_token_stream(path));
        let result = match last_ident.to_string().as_str() {
            "i8" | "u8" | "i16" | "u16" | "i32" | "u32" | "i64" | "u64" | "i128" | "u128"
            | "isize" | "usize" | "bool" => None,
            "str" | "String" => Some(parse_quote!(std::os::raw::c_char)),
            "Vec" | "BTreeMap" | "HashMap" => {
                let ffi_name = path.to_mangled_ident_default();
                Some(parse_quote!(crate::fermented::generics::#ffi_name))
            },
            "Result" if segments.len() == 1 => {
                let ffi_name = path.to_mangled_ident_default();
                Some(parse_quote!(crate::fermented::generics::#ffi_name))
            },
            "Box" => path_arguments_to_types(&last_segment.arguments)
                .first()
                .cloned()
                .and_then(|item| {
                    let path = match item {
                        Type::TraitObject(TypeTraitObject { bounds , .. }) => {
                            let b = bounds.iter().find_map(|bound| match bound {
                                TypeParamBound::Trait(TraitBound { path, .. }) => {
                                    let p: Type = parse_quote!(#path);
                                    Some(p)
                                }
                                TypeParamBound::Lifetime(_) => None
                            }).unwrap();
                            b
                        },
                        _ => parse_quote!(#item)
                    };
                    Some(self.ffi_custom_or_internal_type(&path))
                }),
            "Option" => path_arguments_to_paths(&last_segment.arguments)
                .first()
                .cloned()
                .and_then(|item| self.resolve_ffi_path(item)),
            "OpaqueContext" => Some(parse_quote!(ferment_interfaces::fermented::types::OpaqueContext_FFI)),
            "OpaqueContextMut" => Some(parse_quote!(ferment_interfaces::fermented::types::OpaqueContextMut_FFI)),
            _ => {
                let ty = parse_quote!(#path);
                let ffi_type = if let Some(ObjectConversion::Type(TypeConversion::Trait(tc, ..)) |
                            ObjectConversion::Type(TypeConversion::TraitType(tc))) = self.trait_ty(&ty) {
                    let trait_ty = &tc.ty;
                    let trait_path: Path = parse_quote!(#trait_ty);
                    ffi_chunk_converted(&trait_path.segments)
                } else {
                    ffi_chunk_converted(&path.segments)
                };
                // println!("[{}] [{}] resolve_ffi_path: {} ----> {}", self.scope.crate_scope(), self.scope.self_path_holder(), ty.to_token_stream(), ffi_type.to_token_stream());
                Some(ffi_type)
            }
        };
        result
    }

    fn ffi_external_path_converted(&self, path: &Path, crate_scope: &Ident) -> Option<Type> {
        // println!("ffi_external_path_converted: {}", format_token_stream(path));
        let lock = self.context.read().unwrap();
        let segments = &path.segments;
        let first_segment = segments.first().unwrap();
        let first_ident = &first_segment.ident;

        let last_segment = segments.iter().last().unwrap();
        let last_ident = &last_segment.ident;

        match last_ident.to_string().as_str() {
            "i8" | "u8" | "i16" | "u16" | "i32" | "u32" | "i64" | "u64" | "i128" | "u128" |
            "isize" | "usize" | "bool" => None,
            "str" | "String" => Some(parse_quote!(std::os::raw::c_char)),
            "Vec" | "BTreeMap" | "HashMap" => {
                let ffi_name = path.to_mangled_ident_default();
                Some(parse_quote!(crate::fermented::generics::#ffi_name))
            },
            "Result" if segments.len() == 1 => {
                let ffi_name = path.to_mangled_ident_default();
                Some(parse_quote!(crate::fermented::generics::#ffi_name))
            },
            "Option" => path_arguments_to_paths(&last_segment.arguments)
                .first()
                .cloned()
                .and_then(|ty| self.ffi_external_path_converted(ty, crate_scope)),
            "OpaqueContext" => Some(parse_quote!(ferment_interfaces::fermented::types::OpaqueContext_FFI)),
            "OpaqueContextMut" => Some(parse_quote!(ferment_interfaces::fermented::types::OpaqueContextMut_FFI)),
            _ => match first_ident.to_string().as_str() {
                "crate" | _ if lock.config.is_current_crate(first_ident) =>
                    Some(ffi_external_chunk(crate_scope, segments)),
                _ if lock.config.contains_fermented_crate(first_ident) =>
                    Some(ffi_external_chunk(first_ident, segments)),
                _ => {
                    let segments: Vec<_> = segments.iter().take(segments.len() - 1).collect();
                    Some(if segments.is_empty() { parse_quote!(#last_ident) } else { parse_quote!(#(#segments)::*::#last_ident) })
                }
            }
        }
    }


    pub fn ffi_full_dictionary_type_presenter(&self, ty: &Type) -> Type {
        let full_ty = self.ffi_custom_or_internal_type(ty);
        self.ffi_dictionary_type_presenter(&full_ty)
    }

    fn ffi_dictionary_type_presenter(&self, field_type: &Type) -> Type {
        // println!("ffi_dictionary_field_type_presenter: {:?}", format_token_stream(field_type));
        match field_type {
            Type::Path(TypePath { path, .. }) =>
                self.ffi_dictionary_type(path),
            Type::Array(TypeArray { elem, len, .. }) =>
                parse_quote!(*mut [#elem; #len]),
            Type::Reference(TypeReference { elem, .. }) =>
                self.ffi_dictionary_type_presenter(elem),
            Type::Ptr(TypePtr { star_token, const_token, mutability, elem }) =>
                match &**elem {
                    Type::Path(TypePath { path, .. }) => match path.segments.last().unwrap().ident.to_string().as_str() {
                        "c_void" => match (star_token, const_token, mutability) {
                            (_, Some(_const_token), None) => parse_quote!(OpaqueContext_FFI),
                            (_, None, Some(_mut_token)) => parse_quote!(OpaqueContextMut_FFI),
                            _ => panic!("extract_struct_field: c_void with {} {} not supported", quote!(#const_token), quote!(#mutability))
                        },
                        _ => parse_quote!(*mut #path)
                    },
                    Type::Ptr(type_ptr) => parse_quote!(*mut #type_ptr),
                    _ => parse_quote!(#field_type)
                },
            Type::Slice(TypeSlice { elem, .. }) => self.ffi_dictionary_type_presenter(elem),
            Type::TraitObject(TypeTraitObject { bounds, .. }) => {
                let bound = bounds.iter().find_map(|bound| match bound {
                    TypeParamBound::Trait(TraitBound { path, .. }) => {
                        let p: Type = parse_quote!(#path);
                        Some(p)
                    }
                    TypeParamBound::Lifetime(_) => None
                }).unwrap();
                self.ffi_dictionary_type_presenter(&bound)
            },
            _ => panic!("FFI_DICTIONARY_TYPE_PRESENTER: type not supported: {}", field_type.to_token_stream())
        }
    }

    pub fn ffi_dictionary_type(&self, path: &Path) -> Type {
        // println!("ffi_dictionary_field_type: {}", format_token_stream(path));
        match path.segments.last().unwrap().ident.to_string().as_str() {
            "i8" | "u8" | "i16" | "u16" | "i32" | "u32" | "i64" | "u64" | "i128" | "u128" |
            "isize" | "usize" | "bool" =>
                parse_quote!(#path),
            "OpaqueContext" =>
                parse_quote!(ferment_interfaces::OpaqueContext_FFI),
            "OpaqueContextMut" =>
                parse_quote!(ferment_interfaces::OpaqueContextMut_FFI),
            "Option" =>
                self.ffi_dictionary_type(path_arguments_to_paths(&path.segments.last().unwrap().arguments).first().unwrap()),
            "Vec" | "BTreeMap" | "HashMap" => {
                let path = self.scope_type_for_path(path)
                    .map_or(path.to_token_stream(), |full_type| full_type.to_mangled_ident_default().to_token_stream())
                    .joined_mut();
                parse_quote!(#path)
            },
            "Result" /*if path.segments.len() == 1*/ => {
                let path = self.scope_type_for_path(path)
                    .map_or(path.to_token_stream(), |full_type| full_type.to_mangled_ident_default().to_token_stream())
                    .joined_mut();
                parse_quote!(#path)
            },
            _ => {
                let ty: Type = parse_quote!(#path);
                ty.joined_mut()
            }
        }
    }

    pub fn trait_items_from_attributes(&self, attrs: &[Attribute]) -> Vec<(TraitCompositionPart1, ScopeChain)> {
        let attr_traits = extract_trait_names(attrs);
        // println!("trait_items_from_attributes: [{}]: [{}]", self.scope, format_path_vec(&attr_traits));
        attr_traits.iter()
            .map(|trait_name| {

                // self.find_item_trait_scope_pair(trait_name)

                let trait_ty = parse_quote!(#trait_name);
                // let oc = ObjectConversion::Type(TypeConversion::TraitType(TypeComposition::new(trait_ty, None)));
                let lock = self.context.read().unwrap();
                // let full_trait_ty = lock.maybe_type(&trait_ty, &self.scope).unwrap();
                let parent_scope = self.scope.parent_scope().unwrap();
                let trait_scope = lock.actual_scope_for_type(&trait_ty, parent_scope);
                // let trait_scope = lock.actual_scope_for_type(&trait_ty, &self.scope);
                // trait_scope
                // trait_scope.se
                // let trait_ident = parse_quote!(#trait_name);
                // let trait_scope = full_trait_ty.as_scope();
                println!("find_item_trait_scope_pair: {} ::: {}", trait_name.to_token_stream(), trait_scope);
                // let item_trait = self.item_trait_with_ident_for(&trait_ident, &trait_scope).unwrap();
                // let trait_scope_chain = ScopeChain::Trait {
                //     self_scope: trait_scope,
                //     parent_scope_chain: Box::new(ScopeChain::Mod { self_scope: self.scope.self_scope().clone() }),
                // };
                let ident = trait_name.get_ident().unwrap();
                (lock.traits
                     .item_trait_with_ident_for(ident, &trait_scope)
                     .cloned()
                     .unwrap(), trait_scope)

            })
            .collect()
    }

}

impl ScopeContext {
    pub fn present_composition_in_context<T>(&self, composition: T, context: T::Context) -> T::Presentation
        where T: Composition {
        composition.present(context, self)
    }
}

fn ffi_chunk_converted(segments: &Punctuated<PathSegment, Colon2>) -> Type {
    let crate_local_segments: Vec<_> = match segments.first().unwrap().ident.to_string().as_str() {
        "crate" => segments.iter().take(segments.len() - 1).skip(1).collect(),
        _ => segments.iter().take(segments.len() - 1).collect()
    };
    let ffi_path_chunk = if crate_local_segments.is_empty() {
        segments.iter().last().unwrap().to_token_stream()
    } else {
        let mangled_ty = segments.to_mangled_ident_default();
        quote!(#(#crate_local_segments)::*::#mangled_ty)
    };
    parse_quote!(crate::fermented::types::#ffi_path_chunk)
}
fn ffi_external_chunk(crate_ident: &Ident, segments: &Punctuated<PathSegment, Colon2>) -> Type {
    let crate_local_segments: Vec<_> = segments.iter().take(segments.len() - 1).skip(1).collect();
    let last_ident = &segments.iter().last().unwrap().ident;
    let ffi_chunk_path = if crate_local_segments.is_empty() {
        last_ident.to_token_stream()
    } else {
        let no_ident_segments = segments.iter().take(segments.len() - 1).collect::<Vec<_>>();
        let ty: Type = parse_quote!(#(#no_ident_segments)::*::#last_ident);
        let mangled_ty = ty.to_mangled_ident_default();
        quote!(#(#crate_local_segments)::*::#mangled_ty)
    };
    parse_quote!(crate::fermented::types::#crate_ident::#ffi_chunk_path)
}



