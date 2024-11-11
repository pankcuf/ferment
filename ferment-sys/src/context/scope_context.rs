use std::fmt::Formatter;
use std::sync::{Arc, RwLock};
use syn::{Attribute, ImplItemMethod, Item, ItemType, parse_quote, Path, TraitBound, TraitItemMethod, Type, TypeBareFn, TypeParamBound, TypePath, TypeTraitObject, ItemTrait};
use syn::punctuated::Punctuated;
use crate::ast::{Depunctuated, TypeHolder};
use crate::composable::TraitModelPart1;
use crate::composer::ComposerLink;
use crate::context::{GlobalContext, ScopeChain, ScopeSearch};
use crate::conversion::{ObjectKind, ScopeItemKind, TypeModelKind};
use crate::ext::{Custom, DictionaryType, extract_trait_names, Fermented, FermentableDictionaryType, Join, ToObjectKind, ToType, AsType, Resolve, SpecialType, ResolveTrait};
use crate::lang::{LangFermentable, Specification};
use crate::presentation::{FFIFullDictionaryPath, FFIFullPath};
use crate::print_phase;

pub type ScopeContextLink = ComposerLink<ScopeContext>;
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
        context.config.current_crate.ident().eq(self.scope.crate_ident_ref())
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
        lock.custom.maybe_type(ty)
    }

    pub fn maybe_fn_sig(&self, full_ty: &Type) -> Option<TypeBareFn> {
        let scope_item = match full_ty {
            Type::Path(TypePath { path, .. }) => self.maybe_scope_item_obj_first(&path),
            _ => None,
        };
        match scope_item {
            Some(ScopeItemKind::Fn(..)) => None,
            Some(ScopeItemKind::Item(item, ..)) => match item {
                Item::Type(ItemType { ty, ..}) => {
                    match &*ty {
                        Type::BareFn(bare) => Some(bare.clone()),
                        _ => None
                    }
                },
                _ => None
            }
            None => None
        }
    }

    pub fn maybe_to_fn_type(&self) -> Option<Type> {
        match &self.scope.parent_object().unwrap() {
            ObjectKind::Type(ref ty_model_kind) |
            ObjectKind::Item(ref ty_model_kind, ..) => {
                let parent_scope = self.scope.parent_scope().unwrap();
                let context = self.context.read().unwrap();
                Some(context.maybe_scope_ref_obj_first(parent_scope.self_path())
                    .and_then(|parent_obj_scope| context.maybe_object_ref_by_tree_key(ty_model_kind.as_type(), parent_obj_scope)
                        .and_then(ObjectKind::maybe_type))
                    .unwrap_or(parent_scope.to_type()))

            },
            _ => None
        }
    }

    pub fn maybe_to_trait_fn_type<LANG, SPEC>(&self) -> Option<Type>
        where LANG: LangFermentable,
              SPEC: Specification<LANG>,
              FFIFullDictionaryPath<LANG, SPEC>: ToType {
        match &self.scope.parent_object().unwrap() {
            ObjectKind::Type(ref ty_conversion) |
            ObjectKind::Item(ref ty_conversion, ..) => {
                let full_parent_ty: Type = Resolve::resolve(ty_conversion.as_type(), self);
                match <Type as Resolve<SpecialType<LANG, SPEC>>>::maybe_resolve(&full_parent_ty, self) {
                    Some(special) => Some(special.to_type()),
                    None => match ty_conversion {
                        TypeModelKind::Trait(ty, ..) =>
                            Some(ty.as_type()
                                .maybe_trait_object(self)
                                .and_then(|oc| oc.maybe_type_model_kind_ref().map(TypeModelKind::to_type))
                                .unwrap_or(ty_conversion.to_type())),
                        _ => Some(ty_conversion.to_type())
                    }
                }
            },
            _ => None
        }
    }

    pub fn maybe_trait_or_regular_model_kind(&self) -> Option<TypeModelKind> {
        self.scope
            .parent_object()
            .and_then(|parent_obj|
                parent_obj.maybe_trait_or_regular_model_kind(self))
    }

    pub fn maybe_special_or_regular_ffi_full_path<LANG, SPEC>(&self, ty: &Type) -> Option<FFIFullPath<LANG, SPEC>>
        where LANG: LangFermentable,
              SPEC: Specification<LANG>,
              FFIFullDictionaryPath<LANG, SPEC>: ToType {
        self.maybe_special_ffi_full_path::<LANG, SPEC>(ty)
            .or_else(|| self.maybe_ffi_full_path(ty))
    }
    fn maybe_special_ffi_full_path<LANG, SPEC>(&self, ty: &Type) -> Option<FFIFullPath<LANG, SPEC>>
        where LANG: LangFermentable,
              SPEC: Specification<LANG>,
              FFIFullDictionaryPath<LANG, SPEC>: ToType {
        <Type as Resolve<SpecialType<LANG, SPEC>>>::maybe_resolve(ty, self)
            .map(FFIFullPath::from)
    }
    pub fn maybe_ffi_full_path<LANG, SPEC>(&self, ty: &Type) -> Option<FFIFullPath<LANG, SPEC>>
        where LANG: LangFermentable,
              SPEC: Specification<LANG> {
        <Type as Resolve<TypeModelKind>>::resolve(ty, self)
            .to_type()
            .maybe_resolve(self)
    }

    pub fn maybe_scope_item_obj_first(&self, path: &Path) -> Option<ScopeItemKind> {
        let lock = self.context.read().unwrap();
        lock.maybe_scope_item_ref_obj_first(path).cloned()
    }
    pub fn maybe_opaque_object<LANG, SPEC>(&self, ty: &Type) -> Option<Type>
        where LANG: LangFermentable, SPEC: Specification<LANG>,
              FFIFullDictionaryPath::<LANG, SPEC>: ToType {
        let resolve_opaque = |path: &Path| {
            let result = if path.is_void() {
                Some(FFIFullDictionaryPath::<LANG, SPEC>::Void.to_type())
            } else {
                match self.maybe_scope_item_obj_first(path) {
                    Some(item) => {
                        if item.is_fermented() || item.is_custom() {
                            None
                        } else {
                            Some(item.to_type())
                        }
                    },
                    None => {
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
        // match ScopeSearchKey::maybe_from_ref(ty) {
        //     Some(ScopeSearchKey::PathRef(path)) => resolve_opaque(path),
        //     _ => None
        // }
        match ty {
            Type::Path(TypePath { path, .. }) =>
                resolve_opaque(path),
            Type::TraitObject(TypeTraitObject { dyn_token, bounds, .. }) => match bounds.len() {
                1 => match bounds.first().unwrap() {
                    TypeParamBound::Trait(TraitBound { path, .. }) =>
                        resolve_opaque(path)
                            .map(|ty| {
                                match &ty {
                                    Type::ImplTrait(..) |
                                    Type::TraitObject(..) => ty,
                                    _ => parse_quote!(#dyn_token #ty),
                                }
                            }),
                    TypeParamBound::Lifetime(_) =>
                        panic!("maybe_opaque_object::error::lifetime")
                },
                _ => None
            },
            // Type::Ptr(TypePtr { elem, const_token, mutability, .. }) => {
            //     match &**elem {
            //         Type::Path(TypePath { path, .. }) => resolve_opaque(path).map(|ty| {
            //             if const_token.is_some() {
            //                 ty
            //                 // parse_quote!(*const #ty)
            //             } else {
            //                 ty
            //                 // parse_quote!(*mut #ty)
            //             }
            //         }),
            //         _ => None
            //     }
            // }
            _ => None
        }
    }

    pub fn maybe_object_by_key(&self, ty: &Type) -> Option<ObjectKind> {
        let lock = self.context.read().unwrap();
        let result = lock.maybe_object_ref_by_tree_key(ty, &self.scope).cloned();
        // println!("maybe_object: {} --- {} --- [{}]", ty.to_token_stream(), result.to_token_stream(), self.scope);
        result
    }

    pub fn maybe_object_by_value(&self, ty: &Type) -> Option<ObjectKind> {
        let lock = self.context.read().unwrap();
        let result = lock.maybe_object_ref_by_value(ty).cloned();
        // println!("maybe_object: {} --- {} --- [{}]", ty.to_token_stream(), result.to_token_stream(), self.scope);
        result
    }
    pub fn maybe_object_by_predicate_ref<'a>(&self, predicate: &'a ScopeSearch<'a>) -> Option<ObjectKind> {
        self.maybe_object_by_predicate(predicate.clone())
    }
    pub fn maybe_object_by_predicate<'a>(&self, predicate: ScopeSearch<'a>) -> Option<ObjectKind> {
        let lock = self.context.read().unwrap();
        let result = lock.maybe_object_ref_by_predicate(predicate).cloned();
        // println!("maybe_object: {} --- {} --- [{}]", ty.to_token_stream(), result.to_token_stream(), self.scope);
        result
    }

    pub fn maybe_type_model_kind(&self, ty: &Type) -> Option<TypeModelKind> {
        let lock = self.context.read().unwrap();
        lock.maybe_type_model_kind_ref_by_key(ty, &self.scope).cloned()
    }

    pub fn full_type_for(&self, ty: &Type) -> Type {
        let lock = self.context.read().unwrap();
        // println!("full_type_for.1: {} [{}]", ty.to_token_stream(), self.scope.self_path().to_token_stream());
        let full_ty = lock.maybe_object_ref_by_tree_key(ty, &self.scope)
            .and_then(ObjectKind::maybe_type)
            .unwrap_or(ty.clone());
        // println!("full_type_for.2: {}", full_ty.to_token_stream());
        full_ty
    }


    pub fn scope_type_for_path(&self, path: &Path) -> Option<Type> {
        let lock = self.context.read().unwrap();
        lock.scope_register.scope_key_type_for_path(path, &self.scope)
    }

    pub fn trait_items_from_attributes(&self, attrs: &[Attribute]) -> Depunctuated<(TraitModelPart1, ScopeChain)> {
        extract_trait_names(attrs)
            .iter()
            .filter_map(|trait_path| self.maybe_trait_scope_pair(&trait_path.to_type()))
            .collect()
    }

    pub fn maybe_trait_scope_pair(&self, trait_name: &Type) -> Option<(TraitModelPart1, ScopeChain)> {
        let lock = self.context.read().unwrap();
        lock.maybe_trait_scope_pair(trait_name, &self.scope)
    }

    pub fn maybe_item_trait(&self, trait_path: &Path) -> Option<ItemTrait> {
        let lock = self.context.read().unwrap();
        lock.maybe_item_trait(trait_path)
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
    //             parse_quote!(ferment::OpaqueContext_FFI),
    //         "OpaqueContextMut" =>
    //             parse_quote!(ferment::OpaqueContextMut_FFI),
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
}

// impl ScopeContext {
//     pub fn present_composition_in_context<T>(&self, composition: T, context: T::Context) -> T::Presentation
//         where T: Composition {
//         composition.present(context, self)
//     }
// }

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
