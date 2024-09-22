use syn::{parse_quote, Path, Type};
use crate::context::{ScopeContext, ScopeSearch, ScopeSearchKey};
use crate::conversion::{ObjectKind, TypeKind, TypeModelKind};
use crate::ext::{AsType, ToType};

pub trait ResolveTrait where Self: Sized + ToType {
    fn maybe_trait_object(&self, source: &ScopeContext) -> Option<ObjectKind> {
        let ScopeContext { context, scope} = source;
        let lock = context.read().unwrap();
        let ty = self.to_type();
        let mut maybe_trait = lock.resolve_trait_type(&ty);
        match maybe_trait {
            Some(ObjectKind::Type(model) | ObjectKind::Item(model, _)) => {
                // check maybe it's really known
                // let trait_scope = lock.actual_scope_for_type(model.as_type(), scope).unwrap();
                if let Some(trait_scope) = lock.actual_scope_for_type(model.as_type(), scope) {
                    let search_key = ScopeSearchKey::maybe_from(parse_quote!(Self)).unwrap();
                    if let Some(obj) = lock.maybe_object_ref_by_predicate(ScopeSearch::KeyInScope(search_key, trait_scope)) {
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

                }


            },
            _ => {}
        }
        maybe_trait.cloned()
    }

    fn maybe_trait_object_model_kind(&self, source: &ScopeContext) -> Option<TypeModelKind> {
        self.maybe_trait_object(source)
            .and_then(|oc| oc.maybe_type_model_kind_ref().cloned())
    }
    fn maybe_trait_object_maybe_model_kind(&self, source: &ScopeContext) -> Option<Option<TypeModelKind>> {
        self.maybe_trait_object(source)
            .map(|oc| oc.maybe_type_model_kind_ref().cloned())
    }


    fn maybe_trait_ty(&self, source: &ScopeContext) -> Option<Type> {
        self.maybe_trait_object(source)
            .and_then(|full_trait_ty| full_trait_ty.maybe_type())
    }
}

impl ResolveTrait for Path {}
impl ResolveTrait for Type {}
impl ResolveTrait for TypeKind {}