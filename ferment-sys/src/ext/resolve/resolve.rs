use proc_macro2::Ident;
use quote::{quote_spanned, ToTokens};
use syn::{AngleBracketedGenericArguments, GenericArgument, parse_quote, Path, PathArguments, PathSegment, TraitBound, Type, TypeArray, TypeParamBound, TypePath, TypeReference, TypeTraitObject};
use syn::spanned::Spanned;
use crate::context::{ScopeContext, ScopeSearchKey};
use crate::conversion::{GenericTypeKind, ObjectKind, ScopeItemKind, TypeModelKind, TypeKind};
use crate::ext::{AsType, CrateExtension, DictionaryType, FFISpecialTypeResolve, Mangle, ResolveTrait, SpecialType, ToPath, ToType};
use crate::lang::{RustSpecification, Specification};
use crate::presentation::{FFIFullDictionaryPath, FFIFullPath};

pub trait Resolve<T> {
    fn maybe_resolve(&self, source: &ScopeContext) -> Option<T>;

    fn resolve(&self, source: &ScopeContext) -> T;
}

impl Resolve<Type> for Type {
    fn resolve(&self, source: &ScopeContext) -> Type {
        source.full_type_for(self)
    }
    fn maybe_resolve(&self, source: &ScopeContext) -> Option<Type> {
        Some(self.resolve(source))
    }
}
impl Resolve<ObjectKind> for Type {
    fn maybe_resolve(&self, source: &ScopeContext) -> Option<ObjectKind> {
        source.maybe_object_by_key(self)
    }
    fn resolve(&self, source: &ScopeContext) -> ObjectKind {
        self.maybe_resolve(source).unwrap()
    }
}

impl<SPEC> Resolve<SpecialType<SPEC>> for Type
    where SPEC: Specification,
          FFIFullDictionaryPath<SPEC>: ToType {
    fn maybe_resolve(&self, source: &ScopeContext) -> Option<SpecialType<SPEC>> {
        let result = source.maybe_custom_conversion(self)
            .map(SpecialType::Custom)
            .or_else(|| source.maybe_opaque_object::<SPEC>(self)
                .map(SpecialType::Opaque));
        // println!("Type::<Option<SpecialType>>::resolve.2({})", result.to_token_stream());
        result
    }
    // #[ferment_macro::debug_io]
    fn resolve(&self, source: &ScopeContext) -> SpecialType<SPEC> {
        self.maybe_resolve(source).unwrap()
    }
}

impl<'a, SPEC> Resolve<SpecialType<SPEC>> for ScopeSearchKey<'a>
    where SPEC: Specification,
          FFIFullDictionaryPath<SPEC>: ToType {
    fn maybe_resolve(&self, source: &ScopeContext) -> Option<SpecialType<SPEC>> {
        let ty = self.to_type();
        let result = source.maybe_custom_conversion(&ty)
            .map(SpecialType::Custom)
            .or_else(|| source.maybe_opaque_object::<SPEC>(&ty)
                .map(SpecialType::Opaque));
        result
    }
    fn resolve(&self, source: &ScopeContext) -> SpecialType<SPEC> {
        self.maybe_resolve(source).unwrap()
    }
}

impl<'a> Resolve<TypeModelKind> for ScopeSearchKey<'a>  {
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
        // println!("Type::<TypeModelKind>::resolve.1({}) in {}", self.to_token_stream(), source.scope.fmt_short());
        let result = Resolve::<ObjectKind>::maybe_resolve(self, source)
            .and_then(|ext_obj_kind| {
                match ext_obj_kind {
                    ObjectKind::Item(.., ScopeItemKind::Fn(..)) =>
                        source.maybe_trait_or_regular_model_kind(),
                    ObjectKind::Type(ref kind) |
                    ObjectKind::Item(ref kind, ..) => {
                        match kind {
                            TypeModelKind::Trait(ty, ..) =>
                                ty.maybe_trait_object_maybe_model_kind(source),
                            _ => None,
                        }.unwrap_or_else(|| ext_obj_kind.maybe_type_model_kind_ref().cloned())
                    },
                    ObjectKind::Empty => None
                }
            })
            .unwrap_or_else(|| TypeModelKind::unknown_type_ref(self));
        // println!("Type::<TypeModelKind>::resolve.2({}) in {} --> {}", self.to_token_stream(), source.scope.fmt_short(), result);
        result
    }
}

impl<SPEC> Resolve<FFIFullPath<SPEC>> for Type
    where SPEC: Specification {
    fn maybe_resolve(&self, source: &ScopeContext) -> Option<FFIFullPath<SPEC>> {
        let res = match self {
            Type::Path(TypePath { path, .. }) =>
                path.maybe_resolve(source),
            Type::Reference(TypeReference { elem, .. }) =>
                elem.maybe_resolve(source),
            Type::Array(..) |
            Type::Slice(..) |
            Type::Tuple(..) =>
                Some(FFIFullPath::Generic { ffi_name: self.mangle_ident_default().to_path() }),
            Type::TraitObject(TypeTraitObject { bounds, .. }) => {
                match bounds.len() {
                    0 => unimplemented!("TODO: FFIResolver::resolve::Type::TraitObject (Empty)"),
                    1 => match bounds.first()? {
                        TypeParamBound::Trait(TraitBound { path, .. }) => path.maybe_resolve(source),
                        TypeParamBound::Lifetime(_) => unimplemented!("TODO: FFIResolver::resolve::Type::TraitObject (Lifetime)"),
                    },
                    _ => Some(FFIFullPath::Generic { ffi_name: bounds.mangle_ident_default().to_path() }),
                }

            },
            _ => None
        };
        //println!("Type::<Option<FFIFullPath>>::resolve {} --> {:?}", self.to_token_stream(), res);
        res
    }
    fn resolve(&self, source: &ScopeContext) -> FFIFullPath<SPEC> {
        //println!("Type::<FFIFullPath>::resolve({})", self.to_token_stream());
        Resolve::<FFIFullPath<SPEC>>::maybe_resolve(self, source)
            .unwrap_or_else(|| {
                // println!("Type::<FFIFullPath>::resolve else ({})", self.to_token_stream());

                FFIFullPath::External { path: parse_quote!(#self) }
            })
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
        self.maybe_resolve(source).unwrap()
    }
}

impl Resolve<FFIFullPath<RustSpecification>> for GenericTypeKind {
    fn maybe_resolve(&self, source: &ScopeContext) -> Option<FFIFullPath<RustSpecification>> {
        Some(self.resolve(source))
    }
    fn resolve(&self, source: &ScopeContext) -> FFIFullPath<RustSpecification> {
        //println!("GenericTypeKind -> FFIFullPath --> {}", self);
        let result = match self {
            GenericTypeKind::Map(ty) |
            GenericTypeKind::Group(ty) |
            GenericTypeKind::Result(ty) |
            GenericTypeKind::Box(ty) |
            GenericTypeKind::AnyOther(ty) =>
                single_generic_ffi_type(ty),
            GenericTypeKind::Callback(ty) |
            GenericTypeKind::Array(ty) |
            GenericTypeKind::Slice(ty) =>
                FFIFullPath::Generic { ffi_name: ty.mangle_ident_default().to_path() },
            GenericTypeKind::Tuple(Type::Tuple(tuple)) => match tuple.elems.len() {
                0 => FFIFullPath::Dictionary { path: FFIFullDictionaryPath::Void },
                1 => single_generic_ffi_type(tuple.elems.first().unwrap()),
                _ => FFIFullPath::Generic { ffi_name: tuple.mangle_ident_default().to_path() }
            }
            GenericTypeKind::Optional(Type::Path(TypePath { path: Path { segments, .. }, .. })) => match segments.last() {
                Some(PathSegment { arguments: PathArguments::AngleBracketed(AngleBracketedGenericArguments { args, .. }), .. }) => match args.first() {
                    Some(GenericArgument::Type(ty)) => match TypeKind::from(ty) {
                        TypeKind::Generic(gen) => gen.resolve(source),
                        _ => single_generic_ffi_type(ty),
                    },
                    _ => panic!("TODO: Non-supported optional type as generic argument (PathArguments::AngleBracketed: Empty): {}", segments.to_token_stream()),
                },
                Some(PathSegment { arguments: PathArguments::Parenthesized(args), .. }) =>
                    FFIFullPath::Generic { ffi_name: args.mangle_ident_default().to_path() },
                _ => unimplemented!("TODO: Non-supported optional type as generic argument (Empty last segment): {}", segments.to_token_stream()),
            },
            GenericTypeKind::Optional(Type::Array(TypeArray { elem, .. })) =>
                single_generic_ffi_type(elem),
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
        };
        //println!("GenericTypeKind -> FFIFullPath <-- {}", result.to_token_stream());
        result
    }
}


impl<SPEC> Resolve<SpecialType<SPEC>> for TypeModelKind
    where SPEC: Specification,
          FFIFullDictionaryPath<SPEC>: ToType {
    fn maybe_resolve(&self, source: &ScopeContext) -> Option<SpecialType<SPEC>> {
        self.as_type().maybe_resolve(source)
    }
    fn resolve(&self, source: &ScopeContext) -> SpecialType<SPEC> {
        // println!("Type::<Option<SpecialType>>::resolve({})", self.to_token_stream());
        self.maybe_resolve(source).unwrap()
    }
}

impl<SPEC> Resolve<FFIFullPath<SPEC>> for Path
    where SPEC: Specification {
    fn maybe_resolve(&self, source: &ScopeContext) -> Option<FFIFullPath<SPEC>> {
        // let config = &source.context.read().unwrap().config;

        let segments = &self.segments;
        let first_segment = segments.first()?;
        let last_segment = segments.last()?;
        let first_ident = &first_segment.ident;
        let last_ident = &last_segment.ident;
        if last_ident.is_primitive() {
            None
        } else if last_ident.is_any_string() {
            Some(FFIFullPath::Dictionary { path: FFIFullDictionaryPath::CChar })
        } else if last_ident.is_special_generic() ||
            (last_ident.is_result() && segments.len() == 1) ||
            last_ident.to_string().eq("Map") && first_ident.to_string().eq("serde_json") || last_ident.is_lambda_fn() {
            Some(FFIFullPath::Generic { ffi_name: self.mangle_ident_default().to_path() })
        } else if last_ident.is_optional() || last_ident.is_box() {
            match &last_segment.arguments {
                PathArguments::AngleBracketed(AngleBracketedGenericArguments { args, .. }) =>
                    args.iter().find_map(|arg| match arg {
                        GenericArgument::Type(ty) =>
                            ty.maybe_resolve(source),
                        _ => None
                    }),
                _ => None
            }
        } else if last_ident.is_smart_ptr() {
            Some(FFIFullPath::Generic { ffi_name: self.mangle_ident_default().to_path() })
        } else {
            let chunk =  if let Some(
                ObjectKind::Type(TypeModelKind::Trait(tc, ..)) |
                ObjectKind::Type(TypeModelKind::TraitType(tc))
            ) = self.maybe_trait_object(source) {
                &tc.as_type().to_path().segments
            } else {
                segments
            };
            maybe_crate_ident_replacement(&chunk.first()?.ident, source)
                .map(|crate_ident| {
                    let crate_local_segments = chunk.crate_and_ident_less();
                    FFIFullPath::Type {
                        crate_ident: crate_ident.clone(),
                        ffi_name: if crate_local_segments.is_empty() {
                            let ty: Type = parse_quote!(#crate_ident::#last_ident);
                            ty.mangle_ident_default().to_path()
                        } else {
                            let no_ident_segments = chunk.ident_less();
                            let ty: Type = parse_quote!(#no_ident_segments::#last_ident);
                            let mangled_ty = ty.mangle_ident_default();
                            parse_quote!(#crate_local_segments::#mangled_ty)
                        }
                    }
                })
                .or_else(|| {
                    let segments = chunk.ident_less();
                    Some(FFIFullPath::external(if segments.is_empty() { last_ident.to_path() } else { parse_quote!(#segments::#last_ident) }))
                })
        }
    }
    fn resolve(&self, source: &ScopeContext) -> FFIFullPath<SPEC> {
        // println!("Path::<Option<FFIFullPath>>::resolve({})", self.to_token_stream());
        self.maybe_resolve(source).unwrap()

    }
}


fn single_generic_ffi_type(ty: &Type) -> FFIFullPath<RustSpecification> {
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
fn maybe_crate_ident_replacement<'a>(ident: &'a Ident, source: &'a ScopeContext) -> Option<&'a Ident> {
    let lock = source.context.read().unwrap();
    match ident.to_string().as_str() {
        "crate" | _ if lock.config.is_current_crate(ident) => Some(source.scope.crate_ident_ref()),
        _ if lock.config.contains_fermented_crate(ident) =>
            Some(ident),
        _ => None
    }
}

