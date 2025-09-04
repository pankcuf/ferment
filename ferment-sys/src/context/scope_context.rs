use std::cell::RefCell;
use std::fmt::Formatter;
use std::rc::Rc;
use std::sync::{Arc, RwLock};
use syn::{Attribute, Item, ItemType, parse_quote, Path, TraitBound, Type, TypeBareFn, TypeParamBound, TypePath, TypeTraitObject, ItemTrait};
use crate::ast::{CommaPunctuated, Depunctuated, TypeHolder};
use crate::composable::TraitModelPart1;
use crate::composer::{ComposerLink, MaybeMacroLabeled};
use crate::context::{GlobalContext, ScopeChain, ScopeSearch, ScopeSearchKey};
use crate::kind::{ObjectKind, ScopeItemKind, SpecialType, TypeModelKind};
use crate::ext::{DictionaryType, extract_trait_names, FermentableDictionaryType, ToType, AsType, Resolve, ResolveTrait, LifetimeProcessor, MaybeLambdaArgs};
use crate::lang::Specification;
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
    }
    pub fn is_from_current_crate(&self) -> bool {
        let context = self.context.read().unwrap();
        context.config.current_crate.ident().eq(self.scope.crate_ident_ref())
    }
    pub fn with(scope: ScopeChain, context: Arc<RwLock<GlobalContext>>) -> Self {
        Self { scope, context }
    }
    pub fn cell_with(scope: ScopeChain, context: Arc<RwLock<GlobalContext>>) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self::with(scope, context)))
    }
    pub fn add_custom_conversion(&self, scope: ScopeChain, custom_type: TypeHolder, ffi_type: Type) {
        // Here we don't know about types in pass 1, we can only use imports
        let mut lock = self.context.write().unwrap();
        lock.custom.add_conversion(custom_type, ObjectKind::unknown_type(ffi_type), scope);
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
            Some(ScopeItemKind::Item(Item::Type(ItemType { ty, ..}), ..)) => match &*ty {
                Type::BareFn(bare) => Some(bare.clone()),
                _ => None
            }
            _ => None
        }
    }

    pub fn maybe_lambda_args<SPEC>(&self, ty: &Type) -> Option<CommaPunctuated<SPEC::Name>>
        where SPEC: Specification {
        self.maybe_fn_sig(ty)
            .and_then(|ty| MaybeLambdaArgs::<SPEC>::maybe_lambda_arg_names(&ty))
    }

    pub fn maybe_to_fn_type(&self) -> Option<Type> {
        match &self.scope.parent_object() {
            Some(ObjectKind::Type(ref ty_model_kind) | ObjectKind::Item(ref ty_model_kind, ..)) => {
                self.scope.parent_scope().map(|parent_scope| {
                    let context = self.context.read().unwrap();
                    context.maybe_scope_ref_obj_first(parent_scope.self_path())
                        .and_then(|parent_obj_scope| context.maybe_object_ref_by_tree_key(ty_model_kind.as_type(), parent_obj_scope)
                            .and_then(ObjectKind::maybe_type))
                        .unwrap_or_else(|| parent_scope.to_type())
                })
            },
            _ => None
        }
    }

    pub fn maybe_to_trait_fn_type<SPEC>(&self) -> Option<Type>
        where SPEC: Specification,
              FFIFullDictionaryPath<SPEC>: ToType {
        match &self.scope.parent_object() {
            Some(ObjectKind::Type(ref ty_conversion) | ObjectKind::Item(ref ty_conversion, ..)) => {
                let full_parent_ty: Type = Resolve::resolve(ty_conversion.as_type(), self);
                match Resolve::<SpecialType<SPEC>>::maybe_resolve(&full_parent_ty, self) {
                    Some(special) => Some(special.to_type()),
                    None => match ty_conversion {
                        TypeModelKind::Trait(model) =>
                            Some(model.as_type()
                                .maybe_trait_object(self)
                                .and_then(|oc| oc.maybe_type_model_kind_ref().map(TypeModelKind::to_type))
                                .unwrap_or_else(|| ty_conversion.to_type())),
                        _ => Some(ty_conversion.to_type())
                    }
                }
            },
            _ => None
        }
    }

    pub fn maybe_parent_trait_or_regular_model_kind(&self) -> Option<TypeModelKind> {
        self.scope
            .parent_object()
            .and_then(|parent_obj| parent_obj.maybe_fn_or_trait_or_same_kind(self))
    }

    pub fn maybe_special_or_regular_ffi_full_path<SPEC>(&self, ty: &Type) -> Option<FFIFullPath<SPEC>>
        where SPEC: Specification,
              FFIFullDictionaryPath<SPEC>: ToType {
        self.maybe_special_ffi_full_path::<SPEC>(ty)
            .or_else(|| self.maybe_ffi_full_path(ty))
    }
    fn maybe_special_ffi_full_path<SPEC>(&self, ty: &Type) -> Option<FFIFullPath<SPEC>>
        where SPEC: Specification,
              FFIFullDictionaryPath<SPEC>: ToType {
        Resolve::<SpecialType<SPEC>>::maybe_resolve(ty, self)
            .map(FFIFullPath::from)
    }
    pub fn maybe_ffi_full_path<SPEC>(&self, ty: &Type) -> Option<FFIFullPath<SPEC>>
        where SPEC: Specification {
        Resolve::<TypeModelKind>::resolve(ty, self)
            .to_type()
            .maybe_resolve(self)
    }

    pub fn maybe_scope_item_obj_first(&self, path: &Path) -> Option<ScopeItemKind> {
        let lock = self.context.read().unwrap();
        lock.maybe_scope_item_ref_obj_first(path).cloned()
    }
    pub fn maybe_opaque_object<SPEC>(&self, ty: &Type) -> Option<Type>
        where SPEC: Specification,
              FFIFullDictionaryPath<SPEC>: ToType {
        let resolve_opaque = |path: &Path| {
            let result = if path.is_void() {
                Some(FFIFullDictionaryPath::<SPEC>::Void.to_type())
            } else {
                match self.maybe_scope_item_obj_first(path)
                    .or_else(|| self.maybe_scope_item_obj_first(&path.lifetimes_cleaned())) {
                    Some(item) => {
                        if item.is_labeled_for_export() || item.is_labeled_for_register() {
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
        match ty {
            Type::Path(TypePath { path, .. }) =>
                resolve_opaque(path),
            Type::TraitObject(TypeTraitObject { dyn_token, bounds, .. }) => match bounds.len() {
                1 => match bounds.first()? {
                    TypeParamBound::Trait(TraitBound { path, .. }) =>
                        resolve_opaque(path)
                            .map(|ty| {
                                match &ty {
                                    Type::ImplTrait(..) |
                                    Type::TraitObject(..) => ty,
                                    _ => parse_quote!(#dyn_token #ty),
                                }
                            }),
                    _ =>
                        panic!("maybe_opaque_object::error::lifetime")
                },
                _ => None
            },
            _ => None
        }
    }

    pub fn maybe_object_by_key(&self, ty: &Type) -> Option<ObjectKind> {
        let lock = self.context.read().unwrap();
        let result = lock.maybe_object_ref_by_tree_key(ty, &self.scope).cloned();
        result
    }

    pub fn maybe_object_ref_by_key_in_scope(&self, search_key: ScopeSearchKey, scope: &ScopeChain) -> Option<ObjectKind> {
        let lock = self.context.read().unwrap();
        let result = lock.scope_register.maybe_object_ref_by_key_in_scope(search_key, scope);
        result.cloned()
    }

    pub fn maybe_object_ref_by_value(&self, search_key: ScopeSearchKey) -> Option<ObjectKind> {
        let lock = self.context.read().unwrap();
        let result = lock.scope_register.maybe_object_ref_by_value(search_key);
        result.cloned()
    }

    pub fn maybe_object_by_value(&self, ty: &Type) -> Option<ObjectKind> {
        let lock = self.context.read().unwrap();
        let result = lock.maybe_object_ref_by_value(ty).cloned();
        result
    }
    pub fn maybe_object_by_predicate_ref(&self, predicate: &ScopeSearch) -> Option<ObjectKind> {
        match predicate {
            ScopeSearch::KeyInScope(search_key, scope) =>
                self.maybe_object_ref_by_key_in_scope(search_key.clone(), scope),
            ScopeSearch::Value(search_key) =>
                self.maybe_object_ref_by_value(search_key.clone()),
            ScopeSearch::KeyInComposerScope(search_key) => {
                self.maybe_object_ref_by_key_in_scope(search_key.clone(), &self.scope)
            }
        }

    }

    pub fn maybe_type_model_kind(&self, ty: &Type) -> Option<TypeModelKind> {
        let lock = self.context.read().unwrap();
        lock.maybe_type_model_kind_ref_by_key(ty, &self.scope).cloned()
    }

    pub fn full_type_for(&self, ty: &Type) -> Type {
        let lock = self.context.read().unwrap();
        let full_ty = lock.maybe_object_ref_by_tree_key(ty, &self.scope)
            .and_then(ObjectKind::maybe_type)
            .unwrap_or_else(|| ty.clone());
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
}
