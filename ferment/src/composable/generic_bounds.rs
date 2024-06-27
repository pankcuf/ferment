use syn::{Attribute, Generics, ParenthesizedGenericArguments, parse_quote, PathArguments, PathSegment, Type, TypePath};
use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Display, Formatter};
use std::hash::{Hash, Hasher};
use quote::ToTokens;
use syn::__private::TokenStream2;
use crate::ast::Depunctuated;
use crate::composable::{FieldTypeComposition, FieldTypeConversionKind, TypeComposition};
use crate::composer::{CommaPunctuatedNestedArguments, ParentComposer};
use crate::context::ScopeContext;
use crate::conversion::{compose_generic_presentation, dictionary_generic_arg_pair, expand_attributes, ObjectConversion, TypeCompositionConversion};
use crate::ext::{Conversion, DictionaryType, Mangle, Terminated, ToType, usize_to_tokenstream};
use crate::formatter::{format_obj_vec, format_predicates_obj_dict};
use crate::naming::Name;
use crate::presentable::{Expression, ScopeContextPresentable};
use crate::presentation::{DestroyPresentation, FromConversionPresentation, InterfacePresentation, ToConversionPresentation};

#[derive(Clone)]
pub struct GenericBoundComposition {
    // 'T'
    pub type_composition: TypeComposition,
    // 'Fn(u32) -> Result<bool, ProtocolError>' or 'Clone + Debug + Smth'
    pub bounds: Vec<ObjectConversion>,
    pub predicates: HashMap<Type, Vec<ObjectConversion>>,
    // pub bounds: Vec<Path>,
    // pub predicates: HashMap<Type, Vec<Path>>,
    pub nested_arguments: CommaPunctuatedNestedArguments,
    // pub nested_arguments: HashMap<Path, CommaPunctuated<NestedArgument>>,
}

impl Debug for GenericBoundComposition {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!(
            "GenericBoundComposition(ty: {}, bounds: {}, predicates: {}, nested_args: {})",
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
        // self.predicates.iter().for_each(||)
    }
}

impl GenericBoundComposition {
    pub fn new(ty: Type, bounds: Vec<ObjectConversion>, predicates: HashMap<Type, Vec<ObjectConversion>>, generics: Generics, nested_arguments: CommaPunctuatedNestedArguments) -> Self {
        Self {
            type_composition: TypeComposition::new(ty, Some(generics), nested_arguments.clone()),
            bounds,
            predicates,
            nested_arguments,
        }
    }
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
        if let ObjectConversion::Type(TypeCompositionConversion::FnPointer(ty)) |
        ObjectConversion::Item(TypeCompositionConversion::FnPointer(ty), _) = bound {
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
    fn conversion_from(&self, field_path: Expression) -> Expression {
        field_path
        // FieldContext::FFICallbackExpr(FFICallbackMethodExpr::Get(quote!(&#ident)))
        // FieldContext::From(field_path.into())
    }

    fn conversion_to(&self, field_path: Expression) -> Expression {
        Expression::To(field_path.into())
    }

    fn conversion_destroy(&self, field_path: Expression) -> Expression {
        Expression::UnboxAny(field_path.into())
    }
}

impl GenericBoundComposition {
    pub fn expand(&self, attrs: &HashSet<Option<Attribute>>, scope_context: &ParentComposer<ScopeContext>) -> TokenStream2 {
        // println!("Mixin::Expand: {} ---- {:?}", self, attrs);
        let source = scope_context.borrow();
        let attrs = expand_attributes(attrs);
        let ffi_name = self.mangle_ident_default();
        let self_ty = &self.type_composition.ty;
        // let ffi_name = self_ty.mangle_ident_default();
        let ffi_as_type = ffi_name.to_type();
        println!("Mixin::Expand: {} ---- \n\tattrs: {:?}\n\tname: {}", self, attrs, ffi_name);

        let mixin_items = self.predicates.iter()
            .enumerate()
            .map(|(index, (predicate_ty, _bounds))|
                dictionary_generic_arg_pair(
                    Name::UnnamedArg(index),
                    usize_to_tokenstream(index),
                    predicate_ty,
                    &source))
            .collect::<Depunctuated<_>>();
        compose_generic_presentation(
            ffi_name,
            attrs.clone(),
            Depunctuated::from_iter(
                mixin_items.iter()
                    .enumerate()
                    .map(|(index, (root_path, _))| FieldTypeComposition::unnamed(Name::UnnamedArg(index), FieldTypeConversionKind::Type(root_path.clone())))),
            Depunctuated::from_iter([
                InterfacePresentation::Conversion {
                    attrs,
                    types: (ffi_as_type, parse_quote!(#self_ty)),
                    conversions: (
                        FromConversionPresentation::Tuple(mixin_items.iter().flat_map(|(_, args)| args.iter().map(|item| item.from_conversion.present(&source))).collect()),
                        ToConversionPresentation::Tuple(mixin_items.iter().flat_map(|(_, args)| args.iter().map(|item| item.to_conversion.present(&source))).collect()),
                        DestroyPresentation::Default,
                        None
                    )
                }
            ]),
            Depunctuated::from_iter(mixin_items.iter().flat_map(|(_, args)| args.iter().map(|item| item.destructor.present(&source).terminated()))),
            &source
        ).to_token_stream()
    }
}