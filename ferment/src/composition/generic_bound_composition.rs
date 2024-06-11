use syn::{Generics, parse_quote, Path, PathSegment, PredicateType, TraitBound, Type, TypeParam, TypeParamBound, WherePredicate};
use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};
use std::hash::{Hash, Hasher};
use quote::ToTokens;
use crate::composer::AddPunctuated;
use crate::composition::TypeComposition;
use crate::context::ScopeContext;
use crate::ext::{Conversion, Mangle, MangleDefault, ToPath};
use crate::formatter::{format_path_vec, format_predicates_dict};
use crate::presentation::context::FieldContext;

#[derive(Clone)]
pub struct GenericBoundComposition {
    pub type_composition: TypeComposition,
    pub bounds: Vec<Path>,
    pub predicates: HashMap<Type, Vec<Path>>
}

impl Debug for GenericBoundComposition {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let str = format!("GenericBoundComposition({}, {}, {})",
                            self.type_composition,
                            format_path_vec(&self.bounds),
                            format_predicates_dict(&self.predicates));
        f.write_str(str.as_str())
    }
}

impl Display for GenericBoundComposition {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}
impl PartialEq for GenericBoundComposition {
    fn eq(&self, other: &Self) -> bool {
        let self_tokens = [self.type_composition.ty.to_token_stream()];
        let other_tokens = [other.type_composition.ty.to_token_stream()];
        self_tokens.iter()
            .map(|t| t.to_string())
            .zip(other_tokens.iter().map(ToString::to_string))
            .all(|(a, b)| a == b)
    }
}

impl Eq for GenericBoundComposition {}

impl Hash for GenericBoundComposition {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.type_composition.ty.to_token_stream().to_string().hash(state);
        self.bounds.iter().for_each(|bound| bound.to_token_stream().to_string().hash(state));
    }
}

impl Mangle<MangleDefault> for GenericBoundComposition {
    fn mangle_string(&self, context: MangleDefault) -> String {
        // format!("Mixin_{}", self.bounds.iter().map(|b| b.mangle_string(context)).collect::<Vec<_>>().join("_"))
        format!("{}", self.bounds.iter().map(|b| b.mangle_string(context)).collect::<Vec<_>>().join("_"))
        // self.bounds.iter().map()
    }
}

impl GenericBoundComposition {
    pub fn new(path: &Path, type_param: &TypeParam, generics: &Generics) -> Self {
        let ty: Type = parse_quote!(#path);
        let ident = &type_param.ident;
        let segment = PathSegment::from(ident.clone());
        let ident_path = Path::from(segment);
        Self {
            bounds: generic_trait_bounds(path, &ident_path, &type_param.bounds),
            predicates: generics.where_clause
                .as_ref()
                .map(|where_clause|
                    where_clause.predicates
                        .iter()
                        .filter_map(|predicate| match predicate {
                            WherePredicate::Type(PredicateType { bounded_ty, bounds, .. }) =>
                                ty.eq(bounded_ty).then(||(bounded_ty.clone(), generic_trait_bounds(&path, &bounded_ty.to_path(), bounds))),
                            _ => None
                        })
                        .collect())
                .unwrap_or_default(),
            // TODO: it can have NestedArguments
            type_composition: TypeComposition::new_non_gen(ty, Some(generics.clone())),
        }
    }

    pub fn ffi_full_dictionary_type_presenter(&self, _source: &ScopeContext) -> Type {
        // unimplemented!("")
        let ffi_name = self.mangle_ident_default();
        // self.type_composition.ty.clone()
        parse_quote!(crate::fermented::generics::#ffi_name)
        // Determine mixin type
        //
    }

}


fn generic_trait_bounds(ty: &Path, ident_path: &Path, bounds: &AddPunctuated<TypeParamBound>) -> Vec<Path> {
    let mut has_bound = false;
    bounds.iter().filter_map(|b| match b {
        TypeParamBound::Trait(TraitBound { path, .. }) => {
            let has = ident_path.eq(ty);
            if !has_bound && has {
                has_bound = true;
            }
            has.then(|| path.clone())
        },
        TypeParamBound::Lifetime(_) => None
    }).collect()
}

impl Conversion for GenericBoundComposition {
    fn conversion_from(&self, field_path: FieldContext) -> FieldContext {
        field_path
        // FieldContext::FFICallbackExpr(FFICallbackMethodExpr::Get(quote!(&#ident)))
        // FieldContext::From(field_path.into())
    }

    fn conversion_to(&self, field_path: FieldContext) -> FieldContext {
        FieldContext::To(field_path.into())
    }

    fn conversion_destroy(&self, field_path: FieldContext) -> FieldContext {
        FieldContext::UnboxAny(field_path.into())
    }
}

