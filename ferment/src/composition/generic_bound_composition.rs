use syn::{ParenthesizedGenericArguments, parse_quote, PathArguments, PathSegment, Type, TypePath};
use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};
use std::hash::{Hash, Hasher};
use quote::ToTokens;
use crate::composer::{CommaPunctuated};
use crate::composition::{NestedArgument, TypeComposition};
use crate::context::ScopeContext;
use crate::conversion::{ObjectConversion, TypeCompositionConversion};
use crate::ext::{Conversion, DictionaryType, Mangle, MangleDefault};
use crate::formatter::{format_obj_vec, format_predicates_obj_dict};
use crate::presentation::context::FieldContext;

#[derive(Clone)]
pub struct GenericBoundComposition {
    // 'T'
    pub type_composition: TypeComposition,
    // 'Fn(u32) -> Result<bool, ProtocolError>' or 'Clone + Debug + Smth'
    pub bounds: Vec<ObjectConversion>,
    pub predicates: HashMap<Type, Vec<ObjectConversion>>,
    // pub bounds: Vec<Path>,
    // pub predicates: HashMap<Type, Vec<Path>>,
    pub nested_arguments: CommaPunctuated<NestedArgument>,
    // pub nested_arguments: HashMap<Path, CommaPunctuated<NestedArgument>>,
}

impl Debug for GenericBoundComposition {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!(
            "GenericBoundComposition(\n\tty: {},\n\tbounds: {},\n\tpredicates: {},\n\tnested_args: {})",
            self.type_composition,
            format_obj_vec(&self.bounds),
            format_predicates_obj_dict(&self.predicates),
            self.nested_arguments.to_token_stream()
        ).as_str())
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
        format!("{}", self.bounds.iter().map(|b| {
            match b {
                ObjectConversion::Type(ty) |
                ObjectConversion::Item(ty, _) => ty.ty().mangle_string(context),
                ObjectConversion::Empty => panic!("err"),
            }
        }).collect::<Vec<_>>().join("_"))
    }
}

impl GenericBoundComposition {
    // pub fn new(path: &Path, type_param: &TypeParam, generics: Generics, nested_arguments: CommaPunctuated<NestedArgument>) -> Self {
    //     let ty: Type = parse_quote!(#path);
    //     let ident = &type_param.ident;
    //     let segment = PathSegment::from(ident.clone());
    //     let ident_path = Path::from(segment);
    //     let bounds = generic_trait_bounds(path, &ident_path, &type_param.bounds);
    //     let predicates = generics.where_clause
    //         .as_ref()
    //         .map(|where_clause|
    //             where_clause.predicates
    //                 .iter()
    //                 .filter_map(|predicate| match predicate {
    //                     WherePredicate::Type(PredicateType { bounded_ty, bounds, .. }) =>
    //                         ty.eq(bounded_ty).then(||(bounded_ty.clone(), generic_trait_bounds(&path, &bounded_ty.to_path(), bounds))),
    //                     _ => None
    //                 })
    //                 .collect())
    //         .unwrap_or_default();
    //     let s = Self {
    //         bounds,
    //         predicates,
    //         // TODO: it can have NestedArguments
    //         type_composition: TypeComposition::new(ty, Some(generics.clone()), nested_arguments.clone()),
    //         nested_arguments
    //     };
    //     println!("GenericBoundComposition::new({})", s);
    //     s
    // }

    pub fn ffi_full_dictionary_type_presenter(&self, _source: &ScopeContext) -> Type {
        // unimplemented!("")
        let ffi_name = self.mangle_ident_default();
        println!("GenericBound: ffi_full_dictionary_type_presenter: {} --- {}", ffi_name, self);
        parse_quote!(crate::fermented::generics::#ffi_name)
        // Determine mixin type
        //
    }

    pub fn maybe_bound_is_callback<'a>(&self, bound: &'a ObjectConversion) -> Option<&'a ParenthesizedGenericArguments> {
        if let ObjectConversion::Type(TypeCompositionConversion::Callback(ty)) |
        ObjectConversion::Item(TypeCompositionConversion::Callback(ty), _) = bound {
            if let Type::Path(TypePath { path, .. }) = &ty.ty {
                if let Some(PathSegment { arguments, ident: last_ident, ..}) = &path.segments.last() {
                    if last_ident.is_lambda_fn() {
                        if let PathArguments::Parenthesized(args) = arguments {
                            return Some(args)
                        }
                    }
                }
            }
        }
        None
    }
}


// pub fn generic_trait_bounds(ty: &Path, ident_path: &Path, bounds: &AddPunctuated<TypeParamBound>) -> Vec<Path> {
//     let mut has_bound = false;
//     bounds.iter().filter_map(|b| match b {
//         TypeParamBound::Trait(TraitBound { path, .. }) => {
//             let has = ident_path.eq(ty);
//             if !has_bound && has {
//                 has_bound = true;
//             }
//             has.then(|| path.clone())
//         },
//         TypeParamBound::Lifetime(_) => None
//     }).collect()
// }

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