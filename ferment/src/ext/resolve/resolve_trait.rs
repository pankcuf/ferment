use quote::ToTokens;
use syn::parse::Parse;
use syn::{parse_quote, Path, Type};
use syn::parse_quote::ParseQuote;
use crate::context::ScopeContext;
use crate::conversion::{ObjectConversion, TypeConversion};
use crate::ext::ToType;

pub trait ResolveTrait where Self: Sized + ToTokens + Parse + ParseQuote {
    fn maybe_trait_object(&self, source: &ScopeContext) -> Option<ObjectConversion> {
        // println!("FFI (check...1) for: {}", self.to_token_stream());
        let lock = source.context.read().unwrap();
        let ty: Type = parse_quote!(#self);
        let mut maybe_trait = lock.resolve_trait_type(&ty);
        // println!("FFI (trait) for: {}", maybe_trait.map_or(format!("None"), |m| m.to_string()));
        match maybe_trait {
            Some(ObjectConversion::Type(ty) | ObjectConversion::Item(ty, _)) => {
                // loc
                // check maybe it's really known
                let trait_scope = lock.actual_scope_for_type(&ty.to_type(), &source.scope);
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

    fn maybe_trait_ty(&self, source: &ScopeContext) -> Option<Type> {
        self.maybe_trait_object(source)
            .and_then(|full_trait_ty| full_trait_ty.to_ty())
    }
}

impl ResolveTrait for Path {}
impl ResolveTrait for Type {}
impl ResolveTrait for TypeConversion {}