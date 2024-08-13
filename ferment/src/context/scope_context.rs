use std::fmt::Formatter;
use std::sync::{Arc, RwLock};
use syn::{Attribute, ImplItemMethod, parse_quote, Path, TraitBound, TraitItemMethod, Type, TypeParamBound, TypePath, TypeTraitObject};
use syn::punctuated::Punctuated;
use crate::ast::{Depunctuated, TypeHolder};
use crate::composable::{Composition, TraitCompositionPart1};
use crate::context::{GlobalContext, ScopeChain};
use crate::conversion::{ObjectConversion, TypeCompositionConversion};
use crate::ext::{Custom, DictionaryType, extract_trait_names, Fermented, FermentableDictionaryType, Join, ToObjectConversion, ToType};
use crate::presentation::FFIFullDictionaryPath;
use crate::print_phase;

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
    pub fn print_with_message(&self, message: &str) {
        print_phase!(message, "{}", self);
        // println!("\n•• {} ••\n", message);
        // println!("{}", self);

    }
    pub fn is_from_current_crate(&self) -> bool {
        let context = self.context.read().unwrap();
        context.config.current_crate.ident().eq(self.scope.crate_ident())
    }
    pub fn with(scope: ScopeChain, context: Arc<RwLock<GlobalContext>>) -> Self {
        Self { scope, context }
    }
    pub fn add_custom_conversion(&self, scope: ScopeChain, custom_type: TypeHolder, ffi_type: Type) {
        // Here we don't know about types in pass 1, we can only use imports
        // let path = PathHolder::from(custom_type.0.to_path());
        let mut lock = self.context.write().unwrap();

        // let regular_type = lock.maybe_import(&scope, &path)
        //     .unwrap_or(&path.0).clone();
        lock.custom.add_conversion(
            custom_type,
            ffi_type.to_unknown(Punctuated::new()),
            scope);
    }

    pub fn maybe_custom_conversion(&self, ty: &Type) -> Option<Type> {
        let lock = self.context.read().unwrap();
        lock.custom.maybe_conversion(ty)
    }
    pub fn maybe_opaque_object(&self, ty: &Type) -> Option<Type> {
        //println!("maybe_opaque_object: {}", ty.to_token_stream());
        let resolve_opaque = |path: &Path| {
            // println!("resolve_opaque: {}", path.to_token_stream());
            let lock = self.context.read().unwrap();
            let result = if path.is_void() {
                Some(FFIFullDictionaryPath::Void.to_type())
            } else {
                match lock.maybe_item_obj_first(path) {
                    Some(item) => {
                        if item.is_fermented() || item.is_custom() {
                            // println!("resolve_opaque: (non opaque) {} ", path.to_token_stream());
                            None
                        } else {
                            // println!("resolve_opaque: (opaque by macro) {} ", path.to_token_stream());
                            Some(item.to_type())
                        }
                    },
                    None => {
                        //println!("resolve_opaque: (unknown: ferment: {}) {}", path.is_fermentable_dictionary_type(), path.to_token_stream());
                        if path.is_fermentable_dictionary_type() {
                            None
                        } else if path.is_primitive() {
                            None
                        } else {
                            Some(ty.clone())
                        }
                    }
                }
            };
            result
        };
        match ty {
            Type::Path(TypePath { path, .. }) =>
                resolve_opaque(path),
            Type::TraitObject(TypeTraitObject { dyn_token, bounds, .. }) => match bounds.len() {
                1 => match bounds.first().unwrap() {
                    TypeParamBound::Trait(TraitBound { path, .. }) =>
                        resolve_opaque(path)
                            .map(|ty| parse_quote!(#dyn_token #ty)),
                    TypeParamBound::Lifetime(_) =>
                        panic!("maybe_opaque_object::error::lifetime")
                },
                _ => None
            },
            _ => None
        }
    }

    pub fn maybe_object(&self, ty: &Type) -> Option<ObjectConversion> {
        let lock = self.context.read().unwrap();
        let result = lock.maybe_object(ty, &self.scope).cloned();
        // println!("maybe_object: {} --- {} --- [{}]", ty.to_token_stream(), result.to_token_stream(), self.scope);
        result
    }

    pub fn maybe_type_conversion(&self, ty: &Type) -> Option<TypeCompositionConversion> {
        let lock = self.context.read().unwrap();
        lock.maybe_type_composition_conversion(ty, &self.scope).cloned()
    }

    pub fn full_type_for(&self, ty: &Type) -> Type {
        let lock = self.context.read().unwrap();
        // println!("full_type_for.1: {} [{}]", ty.to_token_stream(), self.scope.self_path().to_token_stream());
        let full_ty = lock.maybe_object(ty, &self.scope)
            .and_then(ObjectConversion::maybe_type)
            .unwrap_or(ty.clone());
        // println!("full_type_for.2: {}", full_ty.to_token_stream());
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

    // pub fn find_generics_fq_in(&self, item: &Item, scope: &ScopeChain) -> HashSet<GenericConversion> {
    //     let lock = self.context.read().unwrap();
    //     lock.scope_register.find_generics_fq_in(item, scope)
    // }

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

    pub fn trait_items_from_attributes(&self, attrs: &[Attribute]) -> Depunctuated<(TraitCompositionPart1, ScopeChain)> {
        let global = self.context.read().unwrap();
        extract_trait_names(attrs)
            .iter()
            .filter_map(|link| global.maybe_trait_scope_pair(link, &self.scope))
            .collect()
    }

}

impl ScopeContext {
    pub fn present_composition_in_context<T>(&self, composition: T, context: T::Context) -> T::Presentation
        where T: Composition {
        composition.present(context, self)
    }
}

impl Join<ImplItemMethod> for ScopeContext {
    fn joined(&self, other: &ImplItemMethod) -> Self {
        Self::with(self.scope.joined(other), self.context.clone())
    }
}

impl Join<TraitItemMethod> for ScopeContext {
    fn joined(&self, other: &TraitItemMethod) -> Self {
        Self::with(self.scope.joined(other), self.context.clone())
    }
}

// impl<'a, T, S> Composer<'a> for ScopeContext
//     where S: ScopeContextPresentable<Presentation=T>, T: ToTokens {
//     type Source = S;
//     type Result = <S as ScopeContextPresentable>::Presentation;
//
//     fn compose(&self, source: &'a Self::Source) -> Self::Result {
//         source.present(self)
//     }
// }
