use syn::{parse_quote, Path, Type};
use crate::context::{ScopeContext, ScopeSearchKey};
use crate::kind::{ObjectKind, TypeKind, TypeModelKind};
use crate::ext::{AsType, ToType};

pub trait ResolveTrait where Self: Sized + ToType {
    fn maybe_trait_object(&self, source: &ScopeContext) -> Option<ObjectKind> {
        let ScopeContext { context, scope} = source;
        let lock = context.borrow();
        let ty = self.to_type();
        let mut maybe_trait = lock.resolve_trait_type(&ty);
        if let Some(ObjectKind::Type(model) | ObjectKind::Item(model, _)) = maybe_trait {
            if let Some(trait_scope) = lock.actual_scope_for_type(model.as_type(), scope) {
                if let Some(search_key) = ScopeSearchKey::maybe_from(parse_quote!(Self)) {
                    if let Some(obj) = lock.scope_register.maybe_object_ref_by_key_in_scope(search_key, trait_scope) {
                        maybe_trait = Some(obj);
                    }
                }
            }
        }
        maybe_trait.cloned()
    }

    fn maybe_trait_object_model_kind(&self, source: &ScopeContext) -> Option<TypeModelKind> {
        self.maybe_trait_object(source)
            .and_then(|oc| oc.maybe_type_model_kind())
    }
    fn maybe_trait_object_maybe_model_kind(&self, source: &ScopeContext) -> Option<Option<TypeModelKind>> {
        self.maybe_trait_object(source)
            .map(|oc| oc.maybe_type_model_kind())
    }


    fn maybe_trait_ty(&self, source: &ScopeContext) -> Option<Type> {
        self.maybe_trait_object(source)
            .and_then(|full_trait_ty| full_trait_ty.maybe_type())
    }
}

impl ResolveTrait for Path {}
impl ResolveTrait for Type {}
impl ResolveTrait for TypeKind {}