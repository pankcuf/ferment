use proc_macro2::Ident;
use quote::ToTokens;
use syn::{Path, TraitBound, Type, TypePath, TypeReference, TypeTraitObject};
use crate::composable::TraitModel;
use crate::context::{ScopeContext, ScopeSearchKey};
use crate::kind::{GenericTypeKind, ObjectKind, SpecialType, TypeModelKind};
use crate::ext::{AsType, CRATE, CrateExtension, DictionaryType, Mangle, ResolveTrait, ToPath, ToType, Join, MaybeTraitBound, MaybeAngleBracketedArgs};
use crate::ext::maybe_generic_type::MaybeGenericType;
use crate::lang::Specification;
use crate::presentation::{FFIFullDictionaryPath, FFIFullPath};

pub trait Resolve<T> {
    fn maybe_resolve(&self, source: &ScopeContext) -> Option<T>;

    fn resolve(&self, source: &ScopeContext) -> T;
}

impl Resolve<Type> for Type {
    fn maybe_resolve(&self, source: &ScopeContext) -> Option<Type> {
        Some(self.resolve(source))
    }
    fn resolve(&self, source: &ScopeContext) -> Type {
        source.full_type_for(self)
    }
}
impl Resolve<ObjectKind> for Type {
    fn maybe_resolve(&self, source: &ScopeContext) -> Option<ObjectKind> {
        source.maybe_object_by_key(self)
    }
    fn resolve(&self, source: &ScopeContext) -> ObjectKind {
        self.maybe_resolve(source)
            .expect(format!("Can't resolve ObjectKind for Type({})", self.to_token_stream()).as_str())
    }
}

impl<SPEC> Resolve<SpecialType<SPEC>> for Type
    where SPEC: Specification,
          FFIFullDictionaryPath<SPEC>: ToType {
    fn maybe_resolve(&self, source: &ScopeContext) -> Option<SpecialType<SPEC>> {
        source.maybe_custom_conversion(self)
            .map(SpecialType::Custom)
            .or_else(|| source.maybe_opaque_object::<SPEC>(self)
                .map(SpecialType::Opaque))
    }
    fn resolve(&self, source: &ScopeContext) -> SpecialType<SPEC> {
        self.maybe_resolve(source)
            .expect(format!("Can't resolve SpecialType for Type({})", self.to_token_stream()).as_str())

    }
}

impl<SPEC> Resolve<SpecialType<SPEC>> for ScopeSearchKey
    where SPEC: Specification,
          FFIFullDictionaryPath<SPEC>: ToType {
    fn maybe_resolve(&self, source: &ScopeContext) -> Option<SpecialType<SPEC>> {
        let ty = self.to_type();
        source.maybe_custom_conversion(&ty)
            .map(SpecialType::Custom)
            .or_else(|| source.maybe_opaque_object::<SPEC>(&ty)
                .map(SpecialType::Opaque))
    }
    fn resolve(&self, source: &ScopeContext) -> SpecialType<SPEC> {
        self.maybe_resolve(source)
            .expect(format!("Can't resolve SpecialType for ScopeSearchKey({})", self.to_token_stream()).as_str())

    }
}

impl Resolve<TypeModelKind> for ScopeSearchKey  {
    fn maybe_resolve(&self, source: &ScopeContext) -> Option<TypeModelKind> {
        Some(self.resolve(source))
    }
    fn resolve(&self, source: &ScopeContext) -> TypeModelKind {
        self.to_type().resolve(source)
    }
}


impl Resolve<TypeModelKind> for Type {
    fn maybe_resolve(&self, source: &ScopeContext) -> Option<TypeModelKind> {
        Some(self.resolve(source))
    }
    fn resolve(&self, source: &ScopeContext) -> TypeModelKind {
        Resolve::<ObjectKind>::maybe_resolve(self, source)
            .and_then(|ext_obj_kind| ext_obj_kind.maybe_fn_or_trait_or_same_kind(source))
            .unwrap_or_else(|| TypeModelKind::unknown_type_ref(self))
    }
}

impl<SPEC> Resolve<FFIFullPath<SPEC>> for Type
    where SPEC: Specification {
    fn maybe_resolve(&self, source: &ScopeContext) -> Option<FFIFullPath<SPEC>> {
        match self {
            Type::Path(TypePath { path, .. }) =>
                path.maybe_resolve(source),
            Type::Reference(TypeReference { elem, .. }) =>
                elem.maybe_resolve(source),
            Type::Array(..) |
            Type::Slice(..) |
            Type::Tuple(..) =>
                Some(FFIFullPath::generic(self.mangle_ident_default().to_path())),
            Type::TraitObject(TypeTraitObject { bounds, .. }) => match bounds.len() {
                0 => unimplemented!("TODO: FFIResolver::resolve::Type::TraitObject (Empty)"),
                1 => bounds.first()
                    .and_then(MaybeTraitBound::maybe_trait_bound)
                    .and_then(|TraitBound { path, .. }| path.maybe_resolve(source)),
                _ => Some(FFIFullPath::generic(bounds.mangle_ident_default().to_path())),
            },
            _ => None
        }
    }
    fn resolve(&self, source: &ScopeContext) -> FFIFullPath<SPEC> {
        Resolve::<FFIFullPath<SPEC>>::maybe_resolve(self, source)
            .unwrap_or_else(|| FFIFullPath::external(self.to_path()))
    }
}

impl<SPEC> Resolve<SpecialType<SPEC>> for GenericTypeKind
    where SPEC: Specification,
          FFIFullDictionaryPath<SPEC>: ToType {
    fn maybe_resolve(&self, source: &ScopeContext) -> Option<SpecialType<SPEC>> {
        self.ty()
            .and_then(|ty| ty.maybe_resolve(source))
    }
    fn resolve(&self, source: &ScopeContext) -> SpecialType<SPEC> {
        self.maybe_resolve(source)
            .expect(format!("Can't resolve SpecialType for GenericTypeKind({})", self.to_token_stream()).as_str())

    }
}



impl<SPEC> Resolve<SpecialType<SPEC>> for TypeModelKind
    where SPEC: Specification,
          FFIFullDictionaryPath<SPEC>: ToType {
    fn maybe_resolve(&self, source: &ScopeContext) -> Option<SpecialType<SPEC>> {
        self.as_type().maybe_resolve(source)
    }
    fn resolve(&self, source: &ScopeContext) -> SpecialType<SPEC> {
        self.maybe_resolve(source)
            .expect(format!("Can't resolve SpecialType for TypeModelKind({})", self.to_token_stream()).as_str())

    }
}
impl<SPEC> Resolve<FFIFullPath<SPEC>> for TypeModelKind
    where SPEC: Specification,
          FFIFullDictionaryPath<SPEC>: ToType {
    fn maybe_resolve(&self, source: &ScopeContext) -> Option<FFIFullPath<SPEC>> {
        self.as_type().maybe_resolve(source)
    }
    fn resolve(&self, source: &ScopeContext) -> FFIFullPath<SPEC> {
        self.maybe_resolve(source)
            .expect(format!("Can't resolve SpecialType for TypeModelKind({})", self.to_token_stream()).as_str())

    }
}
impl Resolve<Type> for Ident {
    fn maybe_resolve(&self, source: &ScopeContext) -> Option<Type> {
        Resolve::<Type>::maybe_resolve(&self.to_type(), source)
    }

    fn resolve(&self, source: &ScopeContext) -> Type {
        Resolve::<Type>::resolve(&self.to_type(), source)
    }
}
impl Resolve<Type> for Path {
    fn maybe_resolve(&self, source: &ScopeContext) -> Option<Type> {
        Resolve::<Type>::maybe_resolve(&self.to_type(), source)
    }

    fn resolve(&self, source: &ScopeContext) -> Type {
        Resolve::<Type>::resolve(&self.to_type(), source)
    }
}
impl<SPEC> Resolve<FFIFullPath<SPEC>> for Path
where SPEC: Specification {
    fn maybe_resolve(&self, source: &ScopeContext) -> Option<FFIFullPath<SPEC>> {
        let segments = &self.segments;
        let first_segment = segments.first()?;
        let last_segment = segments.last()?;
        let first_ident = &first_segment.ident;
        let last_ident = &last_segment.ident;
        if last_ident.is_primitive() {
            None
        } else if last_ident.is_any_string() {
            Some(FFIFullPath::c_char())
        } else if last_ident.is_special_generic() ||
            (last_ident.is_result() && segments.len() == 1) ||
            last_ident.eq("Map") && first_ident.eq("serde_json") || last_ident.is_lambda_fn() {
            Some(FFIFullPath::generic(self.mangle_ident_default().to_path()))
        } else if last_ident.is_optional() || last_ident.is_box() || last_ident.is_cow() {
            last_segment.maybe_angle_bracketed_args()
                .and_then(MaybeGenericType::maybe_generic_type)
                .and_then(|ty| ty.maybe_resolve(source))
        } else if last_ident.is_smart_ptr() {
            Some(FFIFullPath::generic(self.mangle_ident_default().to_path()))
        } else {
            let chunk = if let Some(
                ObjectKind::Type(TypeModelKind::Trait(TraitModel { ty: model, .. })) |
                ObjectKind::Type(TypeModelKind::TraitType(model))
            ) = self.maybe_trait_object(source) {
                &model.as_type().to_path().segments
            } else {
                segments
            };
            maybe_crate_ident_replacement(&chunk.first()?.ident, source)
                .map(|crate_ident| {
                    let crate_local_segments = chunk.crate_and_ident_less();
                    FFIFullPath::r#type(crate_ident.clone(), if crate_local_segments.is_empty() {
                        crate_ident.to_path().joined(last_ident).mangle_ident_default().to_path()
                    } else {
                        crate_local_segments.joined(&chunk.ident_less().joined(last_ident).mangle_ident_default()).to_path()
                    })
                })
                .or_else(|| {
                    let segments = chunk.ident_less();
                    Some(FFIFullPath::external(if segments.is_empty() {
                        last_ident.to_path()
                    } else {
                        segments.joined(last_ident).to_path()
                    }))
                })
        }
    }
    fn resolve(&self, source: &ScopeContext) -> FFIFullPath<SPEC> {
        self.maybe_resolve(source)
            .expect(format!("Can't resolve FFIFullPath for Path({})", self.to_token_stream()).as_str())

    }
}


fn maybe_crate_ident_replacement<'a>(ident: &'a Ident, source: &'a ScopeContext) -> Option<&'a Ident> {
    let lock = source.context.read().unwrap();
    match ident.to_string().as_str() {
        CRATE | _ if lock.config.is_current_crate(ident) =>
            Some(source.scope.crate_ident_ref()),
        _ if lock.config.contains_fermented_crate(ident) =>
            Some(ident),
        _ =>
            None
    }
}

