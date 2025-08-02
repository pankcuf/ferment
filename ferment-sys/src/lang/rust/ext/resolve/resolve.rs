use quote::{quote_spanned, ToTokens};
use syn::{parse_quote, AngleBracketedGenericArguments, GenericArgument, Path, PathArguments, PathSegment, TraitBound, Type, TypeArray, TypeParamBound, TypePath};
use syn::spanned::Spanned;
use crate::context::{ScopeContext, ScopeSearchKey};
use crate::ext::{AsType, DictionaryType, FFISpecialTypeResolve, Mangle, Resolve, ToPath, ToType};
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
                FFIFullPath::Generic { ffi_name: kind.as_type().mangle_ident_default().to_path() },
            GenericTypeKind::Tuple(Type::Tuple(tuple)) => match tuple.elems.len() {
                0 => FFIFullPath::Dictionary { path: FFIFullDictionaryPath::Void },
                1 => single_generic_ffi_full_path(tuple.elems.first().unwrap()),
                _ => FFIFullPath::Generic { ffi_name: tuple.mangle_ident_default().to_path() }
            }
            GenericTypeKind::Optional(Type::Path(TypePath { path: Path { segments, .. }, .. })) => match segments.last() {
                Some(PathSegment { arguments: PathArguments::AngleBracketed(AngleBracketedGenericArguments { args, .. }), .. }) => match args.first() {
                    Some(GenericArgument::Type(ty)) => match TypeKind::from(ty) {
                        TypeKind::Generic(gen) => gen.resolve(source),
                        _ => single_generic_ffi_full_path(ty),
                    },
                    _ => panic!("TODO: Non-supported optional type as generic argument (PathArguments::AngleBracketed: Empty): {}", segments.to_token_stream()),
                },
                Some(PathSegment { arguments: PathArguments::Parenthesized(args), .. }) =>
                    FFIFullPath::Generic { ffi_name: args.mangle_ident_default().to_path() },
                _ => unimplemented!("TODO: Non-supported optional type as generic argument (Empty last segment): {}", segments.to_token_stream()),
            },
            GenericTypeKind::Optional(Type::Array(TypeArray { elem, .. })) =>
                single_generic_ffi_full_path(elem),
            GenericTypeKind::TraitBounds(bounds) => {
                println!("GenericTypeKind (TraitBounds): {}", bounds.to_token_stream());
                match bounds.len() {
                    1 => FFIFullPath::Generic {
                        ffi_name: {
                            if let Some(TypeParamBound::Trait(TraitBound  { path, .. })) = bounds.first() {
                                let ty = path.to_type();
                                let maybe_special: Option<SpecialType<RustSpecification>> = ty.maybe_special_type(source);
                                match maybe_special {
                                    Some(SpecialType::Opaque(..) | SpecialType::Custom(..)) => {
                                        println!("GenericTypeKind (TraitBounds: Special): {}", path.to_token_stream());
                                        return FFIFullPath::external(path.clone())
                                    },
                                    _ => {}
                                }
                            }
                            bounds.first().unwrap().mangle_ident_default().to_path()
                        }
                    },
                    _ => FFIFullPath::Generic { ffi_name: bounds.mangle_ident_default().to_path() }
                }
            },
            gen_ty =>
                unimplemented!("TODO: TraitBounds when generic expansion: {}", gen_ty),
        }
    }
}

fn single_generic_ffi_full_path(ty: &Type) -> FFIFullPath<RustSpecification> {
    let path: Path = parse_quote!(#ty);
    let first_segment = path.segments.first().unwrap();
    let mut cloned_segments = path.segments.clone();
    let first_ident = &first_segment.ident;
    let last_segment = cloned_segments.iter_mut().last().unwrap();
    let last_ident = &last_segment.ident;
    if last_ident.is_primitive() {
        FFIFullPath::external(last_ident.to_path())
    } else if last_ident.is_any_string() {
        FFIFullPath::Dictionary { path: FFIFullDictionaryPath::CChar }
    } else if last_ident.is_special_generic() ||
        (last_ident.is_result() && path.segments.len() == 1) ||
        // TODO: avoid this hardcode
        (last_ident.to_string().eq("Map") && first_ident.to_string().eq("serde_json")) ||
        last_ident.is_smart_ptr() ||
        last_ident.is_lambda_fn() {
        FFIFullPath::Generic { ffi_name: path.mangle_ident_default().to_path() }
    } else {
        let new_segments = cloned_segments
            .into_iter()
            .map(|segment| quote_spanned! { segment.span() => #segment })
            .collect::<Vec<_>>();
        FFIFullPath::external(parse_quote!(#(#new_segments)::*))

    }
}
