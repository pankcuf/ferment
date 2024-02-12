use std::collections::HashMap;
use syn::{GenericArgument, parse_quote, Path, PathArguments, Type, TypePath};
use crate::composition::TypeComposition;
use crate::context::ScopeChain;
use crate::conversion::{ObjectConversion, TypeConversion};
use crate::holder::TypeHolder;

#[derive(Clone, Default)]
pub struct CustomResolver {
    pub inner: HashMap<ScopeChain, HashMap<TypeHolder, ObjectConversion>>,
}

impl CustomResolver {
    pub fn add_conversion(&mut self, path: Path, ffi_type: Type, scope: ScopeChain) {
        self.inner
            .entry(scope.clone())
            .or_default()
            .insert(parse_quote!(#path), ObjectConversion::Type(TypeConversion::Unknown(TypeComposition::new(ffi_type, None))));
    }
    pub fn maybe_conversion(&self, ty: &Type) -> Option<Type> {
        //println!("maybe_custom_conversion: {}", format_token_stream(ty));
        self.inner.keys()
            .find_map(|scope| self.replace_conversion(scope, ty))
    }

    fn replacement_for<'a>(&'a self, ty: &'a Type, scope: &'a ScopeChain) -> Option<&'a ObjectConversion> {
        let tc = TypeHolder::from(ty);
        self.inner
            .get(scope)
            .and_then(|conversion_pairs| conversion_pairs.get(&tc))
    }

    fn replace_conversion(&self, scope: &ScopeChain, ty: &Type) -> Option<Type> {
        let mut custom_type = ty.clone();
        let mut replaced = false;
        if let Type::Path(TypePath { path: Path { segments, .. }, .. }) = &mut custom_type {
            for segment in &mut *segments {
                if let PathArguments::AngleBracketed(angle_bracketed_generic_arguments) = &mut segment.arguments {
                    for arg in &mut angle_bracketed_generic_arguments.args {
                        if let GenericArgument::Type(inner_type) = arg {
                            if let Some(replaced_type) = self.replace_conversion(scope, inner_type) {
                                *arg = GenericArgument::Type(replaced_type);
                                replaced = true;
                            }
                        }
                    }
                }
            }
            if let Some(type_type) = self.replacement_for(ty, scope) {
                if let Some(Type::Path(TypePath { path: Path { segments: new_segments, .. }, .. })) = type_type.ty() {
                    *segments = new_segments.clone();
                    replaced = true;
                }
            }
        }
        replaced.then_some(custom_type)
    }

}