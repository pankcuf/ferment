use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};
use syn::{GenericArgument, Path, PathArguments, TraitBound, Type, TypeParamBound, TypePath, TypeTraitObject};
use crate::context::{ScopeChain, TypeChain};
use crate::kind::ObjectKind;
use crate::formatter::types_dict;

#[derive(Clone, Default)]
pub struct CustomResolver {
    pub inner: HashMap<ScopeChain, TypeChain>,
}
impl Debug for CustomResolver {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut iter = self.inner.iter()
            .map(|(key, value)| format!("\t{}:\n\t\t{}", key, types_dict(&value.inner).join("\n\t\t")))
            .collect::<Vec<String>>();
        iter.sort();
        f.write_str( iter.join("\n\n").as_str())
    }
}

impl Display for CustomResolver {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

impl CustomResolver {
    pub fn add_conversion(&mut self, regular_ty: Type, ffi_object: ObjectKind, scope: ScopeChain) {
        self.inner
            .entry(scope.clone())
            .or_default()
            .insert(regular_ty, ffi_object);
    }
    pub fn maybe_type(&self, ty: &Type) -> Option<Type> {
        self.inner.keys()
            .find_map(|scope| self.replace_conversion(scope, ty))
    }

    fn replacement_for<'a>(&'a self, ty: &'a Type, scope: &'a ScopeChain) -> Option<&'a ObjectKind> {
        self.inner
            .get(scope)
            .and_then(|chain| chain.get(ty))
    }

    fn replace_conversion(&self, scope: &ScopeChain, ty: &Type) -> Option<Type> {
        let mut custom_type = ty.clone();
        let mut replaced = false;
        let mut replace_segments = |path: &mut Path| {
            let segments = &mut path.segments;
            for segment in &mut *segments {
                if let PathArguments::AngleBracketed(angle_bracketed_generic_arguments) = &mut segment.arguments {
                    for arg in &mut angle_bracketed_generic_arguments.args {
                        if let GenericArgument::Type(inner_type) = arg {
                            if let Some(replaced_type) = self.replace_conversion(scope, inner_type) {
                                *arg = GenericArgument::Type(replaced_type);
                            }
                        }
                    }
                }
            }
            if let Some(Type::Path(TypePath { path: Path { segments: new_segments, .. }, .. })) = self.replacement_for(ty, scope).and_then(ObjectKind::maybe_type) {
                *segments = new_segments.clone();
                replaced = true;
            }

        };
        match &mut custom_type {
            Type::Path(TypePath { path, .. }) => {
                replace_segments(path)
            },
            Type::TraitObject(TypeTraitObject { bounds, .. }) => {
                bounds.iter_mut().for_each(|bound| match bound {
                    TypeParamBound::Trait(TraitBound { path, .. }) => {
                        replace_segments(path);
                    },
                    _ => {}
                })
            },
            _ => {}
        }
        replaced.then(|| custom_type)
    }

}