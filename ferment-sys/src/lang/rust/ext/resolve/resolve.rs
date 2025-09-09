use quote::{quote_spanned, ToTokens};
use syn::{parse_quote, AngleBracketedGenericArguments, Path, PathArguments, PathSegment, Type, TypeArray, TypeParamBound, TypePath};
use syn::spanned::Spanned;
use crate::ast::Colon2Punctuated;
use crate::context::{ScopeContext, ScopeSearchKey};
use crate::ext::{DictionaryType, FFISpecialTypeResolve, Mangle, MaybeGenericType, Resolve, ToPath, ToType};
use crate::kind::{GenericTypeKind, SpecialType, TypeKind, TypeModelKind};
use crate::lang::RustSpecification;
use crate::presentation::{FFIFullDictionaryPath, FFIFullPath, FFIVariable};

impl Resolve<FFIVariable<RustSpecification, Type>> for ScopeSearchKey {
    fn maybe_resolve(&self, source: &ScopeContext) -> Option<FFIVariable<RustSpecification, Type>> {
        Some(self.resolve(source))
    }
    fn resolve(&self, source: &ScopeContext) -> FFIVariable<RustSpecification, Type> {
        Resolve::<SpecialType<RustSpecification>>::maybe_resolve(self, source)
            .map(FFIFullPath::from)
            .or_else(|| Resolve::<TypeModelKind>::resolve(self, source)
                .to_type()
                .maybe_resolve(source))
            .map(|ffi_path| ffi_path.to_type())
            .unwrap_or_else(|| self.to_type())
            .resolve(source)
    }
}

impl Resolve<FFIFullPath<RustSpecification>> for GenericTypeKind {
    fn maybe_resolve(&self, source: &ScopeContext) -> Option<FFIFullPath<RustSpecification>> {
        Some(self.resolve(source))
    }
    fn resolve(&self, source: &ScopeContext) -> FFIFullPath<RustSpecification> {
        match self {
            GenericTypeKind::Map(ty) |
            GenericTypeKind::Group(ty) |
            GenericTypeKind::Result(ty) |
            GenericTypeKind::Box(ty) |
            GenericTypeKind::AnyOther(ty) =>
                single_generic_ffi_full_path(ty),
            GenericTypeKind::Array(ty) |
            GenericTypeKind::Slice(ty) =>
                FFIFullPath::Generic { ffi_name: ty.mangle_ident_default().to_path() },
            GenericTypeKind::Callback(kind) =>
                FFIFullPath::Generic { ffi_name: kind.mangle_ident_default().to_path() },
            GenericTypeKind::Tuple(Type::Tuple(tuple)) => match tuple.elems.len() {
                0 => FFIFullPath::Dictionary { path: FFIFullDictionaryPath::Void },
                1 => single_generic_ffi_full_path(tuple.elems.first().unwrap()),
                _ => FFIFullPath::generic(tuple.mangle_ident_default().to_path())
            }
            GenericTypeKind::Optional(Type::Path(TypePath { path: Path { segments, .. }, .. })) => match segments.last() {
                Some(PathSegment { arguments: PathArguments::AngleBracketed(AngleBracketedGenericArguments { args, .. }), .. }) => args.first().and_then(MaybeGenericType::maybe_generic_type).map(|ty| match TypeKind::from(ty) {
                    TypeKind::Generic(gen) => gen.resolve(source),
                    _ => single_generic_ffi_full_path(ty),
                }).unwrap(),
                Some(PathSegment { arguments: PathArguments::Parenthesized(args), .. }) =>
                    FFIFullPath::Generic { ffi_name: args.mangle_ident_default().to_path() },
                _ => unimplemented!("TODO: Non-supported optional type as generic argument (Empty last segment): {}", segments.to_token_stream()),
            },
            GenericTypeKind::Optional(Type::Array(TypeArray { elem, .. })) =>
                single_generic_ffi_full_path(elem),
            GenericTypeKind::TraitBounds(bounds) => match bounds.len() {
                1 => if let Some(TypeParamBound::Trait(trait_bound)) = bounds.first() {
                    match FFISpecialTypeResolve::<RustSpecification>::maybe_special_type(&trait_bound.path.to_type(), source) {
                        Some(SpecialType::Opaque(..) | SpecialType::Custom(..)) =>
                            FFIFullPath::external(trait_bound.path.clone()),
                        _ =>
                            FFIFullPath::generic(trait_bound.mangle_ident_default().to_path())
                    }
                } else {
                    FFIFullPath::generic(bounds.mangle_ident_default().to_path())
                },
                _ =>
                    FFIFullPath::generic(bounds.mangle_ident_default().to_path())
            },
            gen_ty =>
                unimplemented!("TODO: TraitBounds when generic expansion: {}", gen_ty),
        }
    }
}

fn single_generic_ffi_full_path(ty: &Type) -> FFIFullPath<RustSpecification> {
    let path: Path = parse_quote!(#ty);
    match path.segments.first() {
        None => FFIFullPath::void(),
        Some(PathSegment { ident: first_ident, .. }) => match path.segments.iter().last() {
            None => FFIFullPath::void(),
            Some(PathSegment { ident: last_ident, .. }) => if last_ident.is_primitive() {
                FFIFullPath::external(last_ident.to_path())
            } else if last_ident.is_any_string() {
                FFIFullPath::c_char()
            } else if last_ident.is_special_generic() ||
                (last_ident.is_result() && path.segments.len() == 1) ||
                // TODO: avoid this hardcode
                (last_ident.eq("Map") && first_ident.eq("serde_json")) ||
                last_ident.is_smart_ptr() ||
                last_ident.is_lambda_fn() {
                FFIFullPath::generic(path.mangle_ident_default().to_path())
            } else {
                let new_segments = Colon2Punctuated::from_iter(path.segments
                    .into_iter()
                    .map(|segment| quote_spanned! { segment.span() => #segment }));
                FFIFullPath::external(parse_quote!(#new_segments))
            }
        }
    }
}
