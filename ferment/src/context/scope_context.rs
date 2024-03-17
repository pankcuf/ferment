use std::collections::{HashMap, HashSet};
use std::fmt::Formatter;
use std::sync::{Arc, RwLock};
use quote::{format_ident, quote, ToTokens};
use syn::{Attribute, Item, parse_quote, Path, TraitBound, Type, TypeArray, TypeParamBound, TypePath, TypePtr, TypeReference, TypeSlice, TypeTraitObject, TypeTuple};
use crate::composition::{Composition, GenericConversion, ImportComposition, TraitCompositionPart1};
use crate::context::{GlobalContext, ScopeChain};
use crate::conversion::ImportConversion;
use crate::ext::{Accessory, extract_trait_names, FFIResolver, Mangle};
use crate::helper::path_arguments_to_paths;
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
        // println!("full_type_for: {} ---> {}", ty.to_token_stream(), full_ty.to_token_stream());

        full_ty
        // lock.maybe_scope_type_or_parent_type(ty, &self.scope)
        //     .and_then(|sty| sty.ty().cloned())
        //     .unwrap_or(ty.clone())
    }

    // fn trait_ty(&self, ty: &Type) -> Option<ObjectConversion> {
    //     // println!("FFI (check...1) for: {}", format_token_stream(ty));
    //     let lock = self.context.read().unwrap();
    //     let mut maybe_trait = lock.resolve_trait_type(ty);
    //     // println!("FFI (trait) for: {}", maybe_trait.map_or(format!("None"), |m| m.to_string()));
    //     match maybe_trait {
    //         Some(ObjectConversion::Type(ty) | ObjectConversion::Item(ty, _)) => {
    //             // loc
    //             // check maybe it's really known
    //             let trait_scope = lock.actual_scope_for_type(ty.ty(), &self.scope);
    //             if let Some(obj) = lock.maybe_scope_object(&parse_quote!(Self), &trait_scope) {
    //                 maybe_trait = Some(obj);
    //             }
    //             // if let Some(tt) = lock.maybe_scope_type(&parse_quote!(Self), &parse_quote!(#ty)) {
    //             //     maybe_trait = Some(tt);
    //             // }
    //             // maybe_trait = lock.maybe_scope_type(&parse_quote!(Self), &parse_quote!(#ty));
    //             // println!("FFI (trait unknown but maybe known) for: {}", maybe_trait.map_or(format!("None"), |m| m.to_string()));
    //             // if let Some(ty) = maybe_trait {
    //             //     println!("FFI (trait unknown but known) for: {}", ty.to_string());
    //             // }
    //         },
    //         _ => {}
    //     }
    //     maybe_trait.cloned()
    // }
    //
    // fn ffi_internal_type_for(&self, ty: &Type) -> Type {
    //     let lock = self.context.read().unwrap();
    //     let tyty = lock.maybe_type(ty, &self.scope)
    //         .and_then(|external_type| {
    //             match external_type.type_conversion() {
    //                 Some(TypeCompositionConversion::Trait(ty, _decomposition, _super_bounds)) =>
    //                     self.trait_ty(&ty.ty)
    //                         .map(|oc| oc.type_conversion().cloned()),
    //                 _ => None
    //             }.unwrap_or(external_type.type_conversion().cloned())
    //
    //         }).unwrap_or(TypeCompositionConversion::Unknown(TypeComposition::new(ty.clone(), None)));
    //     // let tyty = lock.maybe_scope_type(ty, &self.scope)
    //     //     .and_then(|external_type| {
    //     //         match external_type.type_conversion() {
    //     //             Some(TypeCompositionConversion::Trait(ty, _decomposition)) =>
    //     //                 self.trait_ty(&ty.ty)
    //     //                     .map(|oc| oc.type_conversion().cloned()),
    //     //             _ => None
    //     //         }.unwrap_or(external_type.type_conversion().cloned())
    //     //
    //     //     }).unwrap_or(TypeCompositionConversion::Unknown(TypeComposition::new(ty.clone(), None)));
    //     let result = match tyty.ty() {
    //         Type::Path(TypePath { path, .. }) =>
    //             path.ffi_external_path_converted(self).map(|path| parse_quote!(#path)),
    //         _ => None
    //     };
    //
    //     let result = result.unwrap_or(ty.clone());
    //
    //     // let mangled = result.to_mangled_ident_default();
    //     // parse_quote!(#mangled)
    //     //println!("FFI (ffi_internal_type_for) for: {} in [{}] = {}", ty.to_token_stream(), self.scope, format_token_stream(&result));
    //     result
    // }
    //
    // pub fn ffi_custom_or_internal_type(&self, ty: &Type) -> Type {
    //     let lock = self.context.read().unwrap();
    //     lock.custom.maybe_conversion(ty)
    //         .unwrap_or(self.ffi_internal_type_for(ty))
    // }

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

    // pub fn ffi_path_converted_or_same(&self, path: &Path) -> Type {
    //     self.resolve_ffi_path(path)
    //         .unwrap_or(parse_quote!(#path))
    // }
    //
    //
    //
    // fn resolve_ffi_path(&self, path: &Path) -> Option<Type> {
    //     path.resolve(self)
    //         .map(|path| parse_quote!(#path))
    // }



    pub fn ffi_full_dictionary_type_presenter(&self, ty: &Type) -> Type {
        let full_ty = ty.ffi_custom_or_internal_type(self);
        // let full_ty = self.ffi_custom_or_internal_type(ty);
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
            Type::Tuple(TypeTuple { elems, .. }) => {
                let ffi_types = elems.iter().map(|ty| {
                    let ident = ty.to_mangled_ident_default();
                    ident.to_string()
                    // self.ffi_dictionary_type_presenter(ty)
                }).collect::<Vec<String>>().join("_");
                let ffi_ident = format_ident!("{}_Tuple", ffi_types);
                parse_quote!(#ffi_ident)
            },
            _ => panic!("FFI_DICTIONARY_TYPE_PRESENTER: type not supported: {}", field_type.to_token_stream())
        }
    }

    pub fn ffi_dictionary_type(&self, path: &Path) -> Type {
        // println!("ffi_dictionary_field_type: {}", format_token_stream(path));
        match path.segments.last().unwrap().ident.to_string().as_str() {
            "i8" | "u8" | "i16" | "u16" | "i32" | "u32" | "i64" | "u64" | "f64" | "i128" | "u128" |
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




