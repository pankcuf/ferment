use std::collections::{HashMap, HashSet};
use std::fmt::Formatter;
use std::sync::{Arc, RwLock};
use quote::ToTokens;
use syn::{Attribute, Item, parse_quote, Path, Type};
use crate::composition::{Composition, GenericConversion, ImportComposition, TraitCompositionPart1};
use crate::context::{GlobalContext, ScopeChain};
use crate::conversion::ImportConversion;
use crate::ext::extract_trait_names;
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
        full_ty
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

    // pub fn ffi_dictionary_type(&self, path: &Path) -> Type {
    //     // println!("ffi_dictionary_field_type: {}", format_token_stream(path));
    //     match path.segments.last().unwrap().ident.to_string().as_str() {
    //         "i8" | "u8" | "i16" | "u16" | "i32" | "u32" | "i64" | "u64" | "f64" | "i128" | "u128" |
    //         "isize" | "usize" | "bool" =>
    //             parse_quote!(#path),
    //         "OpaqueContext" =>
    //             parse_quote!(ferment_interfaces::OpaqueContext_FFI),
    //         "OpaqueContextMut" =>
    //             parse_quote!(ferment_interfaces::OpaqueContextMut_FFI),
    //         "Option" =>
    //             self.ffi_dictionary_type(path_arguments_to_paths(&path.segments.last().unwrap().arguments).first().unwrap()),
    //         "Vec" | "BTreeMap" | "HashMap" => {
    //             let path = self.scope_type_for_path(path)
    //                 .map_or(path.to_token_stream(), |full_type| full_type.mangle_ident_default().to_token_stream())
    //                 .joined_mut();
    //             parse_quote!(#path)
    //         },
    //         "Result" /*if path.segments.len() == 1*/ => {
    //             let path = self.scope_type_for_path(path)
    //                 .map_or(path.to_token_stream(), |full_type| full_type.mangle_ident_default().to_token_stream())
    //                 .joined_mut();
    //             parse_quote!(#path)
    //         },
    //         _ => {
    //             let ty: Type = parse_quote!(#path);
    //             ty.joined_mut()
    //         }
    //     }
    // }

    pub fn trait_items_from_attributes(&self, attrs: &[Attribute]) -> Vec<(TraitCompositionPart1, ScopeChain)> {
        let attr_traits = extract_trait_names(attrs);
        // println!("trait_items_from_attributes: [{}]: [{}]", self.scope, format_path_vec(&attr_traits));
        attr_traits.iter()
            .map(|trait_name| {

                // self.find_item_trait_scope_pair(trait_name)

                let trait_ty = parse_quote!(#trait_name);
                // let oc = ObjectConversion::Type(TypeCompositionConversion::TraitType(TypeComposition::new(trait_ty, None)));
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




