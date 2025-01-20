use std::fmt::Debug;
use proc_macro2::Ident;
use quote::{format_ident, ToTokens};
use syn::{AngleBracketedGenericArguments, BareFnArg, ConstParam, GenericArgument, GenericParam, Generics, Lifetime, LifetimeDef, ParenthesizedGenericArguments, Path, PathArguments, PathSegment, PredicateEq, PredicateLifetime, PredicateType, ReturnType, TraitBound, Type, TypeArray, TypeBareFn, TypeImplTrait, TypeParam, TypeParamBound, TypePath, TypePtr, TypeReference, TypeSlice, TypeTraitObject, TypeTuple, WhereClause, WherePredicate};
use syn::__private::TokenStream2;
use syn::punctuated::Punctuated;
use crate::composable::GenericBoundsModel;
use crate::conversion::ObjectKind;
use crate::ext::{AsType, LifetimeProcessor, ToPath};

#[derive(Default, Copy, Clone)]
pub struct MangleDefault; // "::" -> "_"

pub trait Mangle<T: Clone> where Self: Debug {
    fn mangle_string(&self, context: T) -> String;
    fn mangle_string_default(&self) -> String where T: Default {
        self.mangle_string(T::default())
    }
    fn mangle_ident_default(&self) -> Ident where T: Default {
        format_ident!("{}", self.mangle_string(T::default()))
    }
    fn mangle_tokens_default(&self) -> TokenStream2 where T: Default {
        self.mangle_ident_default()
            .to_token_stream()
    }
}

impl<T, SEP, CTX> Mangle<T> for Punctuated<CTX, SEP>
    where
        T: Clone + Copy + Default,
        CTX: Mangle<T>,
        SEP: Debug {
    fn mangle_string(&self, context: T) -> String {
        self.iter()
            .map(|item| item.mangle_string(context))
            .collect::<Vec<_>>()
            .join("_")
    }
}
impl Mangle<MangleDefault> for Type {
    fn mangle_string(&self, context: MangleDefault) -> String {
        // println!("Mangle Type: {} --- {:?}", self.to_token_stream(), self);
        let res = match self {
            // Here we expect BTreeMap<K, V> | HashMap<K, V> | Vec<V> for now
            Type::Path(TypePath { path, .. }) =>
                path.mangle_string(context),
            Type::Array(type_array) =>
                type_array.mangle_string(context),
            Type::Slice(type_slice) =>
                type_slice.mangle_string(context),
            Type::Tuple(type_tuple) =>
                type_tuple.mangle_string(context),
            Type::Reference(type_reference) =>
                type_reference.mangle_string(context),
            Type::BareFn(type_bare_fn) =>
                type_bare_fn.mangle_string(context),
            Type::Ptr(type_ptr) =>
                type_ptr.mangle_string(context),
            ty =>
                ty.to_path()
                    .get_ident()
                    .unwrap()
                    .to_string()
        };
        // println!("Mangle Type..222: {}", res);
        res
    }
}

impl Mangle<MangleDefault> for Path {
    fn mangle_string(&self, context: MangleDefault) -> String {
        self.segments.mangle_string(context)
    }
}

impl Mangle<MangleDefault> for TraitBound {
    fn mangle_string(&self, context: MangleDefault) -> String {
        format!("dyn_trait_{}", self.path.segments.mangle_string(context))
    }
}

impl Mangle<MangleDefault> for TypeTuple {
    fn mangle_string(&self, context: MangleDefault) -> String {
        format!("Tuple_{}", self.elems.mangle_string(context))
    }
}
impl Mangle<MangleDefault> for TypeArray {
    fn mangle_string(&self, context: MangleDefault) -> String {
        format!("Arr_{}_{}", self.elem.mangle_string(context), self.len.to_token_stream())
    }
}

impl Mangle<MangleDefault> for TypeSlice {
    fn mangle_string(&self, context: MangleDefault) -> String {
        format!("Slice_{}", self.elem.mangle_string(context))
        // format!("Vec_{}", self.elem.mangle_string(context))
    }
}

impl Mangle<MangleDefault> for TypeTraitObject {
    fn mangle_string(&self, context: MangleDefault) -> String {
        // TODO: need mixins impl to process multiple bounds
        self.bounds.iter().find_map(|b| match b {
            TypeParamBound::Trait(trait_bound) => Some(trait_bound.mangle_string(context)),
            TypeParamBound::Lifetime(_) => None,
        }).unwrap_or("Any".to_string())
    }
}

impl Mangle<MangleDefault> for TypeImplTrait {
    fn mangle_string(&self, context: MangleDefault) -> String {
        // TODO: need mixins impl to process multiple bounds
        self.bounds.iter().find_map(|b| match b {
            TypeParamBound::Trait(trait_bound) => Some(trait_bound.mangle_string(context)),
            TypeParamBound::Lifetime(_) => None,
        }).unwrap_or("Any".to_string())
    }
}

impl Mangle<MangleDefault> for TypeReference {
    fn mangle_string(&self, context: MangleDefault) -> String {
        self.elem.mangle_string(context)
    }
}
impl Mangle<MangleDefault> for TypeBareFn {
    fn mangle_string(&self, context: MangleDefault) -> String {
        format!(
            "FnPtr_ARGS_{}_RTRN_{}",
            &self.inputs.iter().map(|BareFnArg { ty, .. } | ty.mangle_string(context)).collect::<Vec<_>>().join("_"),
            match &self.output {
                ReturnType::Default => String::new(),
                ReturnType::Type(_, ty) => ty.mangle_string_default()
            })
    }
}

impl Mangle<MangleDefault> for TypePtr {
    fn mangle_string(&self, context: MangleDefault) -> String {
        self.elem.mangle_string(context)
    }
}


impl Mangle<((bool, bool), usize)> for TypePath {
    fn mangle_string(&self, context: ((bool, bool), usize)) -> String {
        let ((is_map, is_result), i) = context;
        let mangled = self.path.mangle_string_default();
        if is_map {
            format!("{}{}", if i == 0 { "keys_" } else { "values_" }, mangled)
        } else if is_result {
            format!("{}{}", if i == 0 { "ok_" } else { "err_" }, mangled)
        } else {
            mangled
        }

    }
}

impl Mangle<((bool, bool), usize)> for TypeArray {
    fn mangle_string(&self, context: ((bool, bool), usize)) -> String {
        let ((is_map, is_result), ..) = context;
        if let Type::Path(type_path) = &*self.elem {
            let mangled_type_path = type_path.mangle_string(context);
            if is_map || is_result {
                format!("{mangled_type_path}_arr_{}", self.len.to_token_stream().to_string())
            } else {
                format!("{mangled_type_path}_{}", self.len.to_token_stream().to_string())
            }
        } else {
            String::default()
        }
    }
}

impl Mangle<String> for PathArguments {
    fn mangle_string(&self, context: String) -> String {
        let mut segment_str = context.clone();
        let is_map = matches!(segment_str.as_str(), "BTreeMap" | "HashMap");
        if is_map {
            segment_str = String::from("Map");
        }
        let is_result = segment_str == "Result";
        match self {
            PathArguments::AngleBracketed(arguments) => {
                let args = arguments.lifetimes_cleaned();
                if args.args.is_empty() {
                    segment_str
                } else {
                    format!("{}_{}", segment_str, args.mangle_string((is_map, is_result)))
                }
            },
            PathArguments::Parenthesized(arguments) => {

                format!("{}_{}", segment_str, arguments.mangle_string((is_map, is_result)))
            },
            _ => segment_str,
        }
    }
}

impl Mangle<MangleDefault> for PathSegment {
    fn mangle_string(&self, _context: MangleDefault) -> String {
        self.arguments.mangle_string(self.ident.to_string())
    }
}

impl Mangle<(bool, bool)> for AngleBracketedGenericArguments {
    fn mangle_string(&self, context: (bool, bool)) -> String {
        self.args.iter()
            .enumerate()
            .filter_map(|(i, gen_arg)| match gen_arg {
                GenericArgument::Type(Type::Path(type_path)) =>
                    Some(type_path.mangle_string((context, i))),
                GenericArgument::Type(Type::Array(type_array)) =>
                    Some(type_array.mangle_string((context, i))),
                GenericArgument::Type(Type::Slice(type_slice)) =>
                    Some(type_slice.mangle_string_default()),
                GenericArgument::Type(Type::Tuple(type_tuple)) =>
                    Some(type_tuple.mangle_string_default()),
                GenericArgument::Type(Type::TraitObject(type_trait_object)) =>
                    Some(type_trait_object.mangle_string_default()),
                _ => None
            })
            .collect::<Vec<_>>()
            .join("_")
    }
}

impl Mangle<(bool, bool)> for ParenthesizedGenericArguments {
    fn mangle_string(&self, _context: (bool, bool)) -> String {
        format!(
            "ARGS_{}_RTRN_{}",
            &self.inputs.iter().map(|gen_arg| gen_arg.mangle_string_default()).collect::<Vec<_>>().join("_"),
            match &self.output {
                ReturnType::Default => String::new(),
                ReturnType::Type(_, ty) => ty.mangle_string_default()
            })
    }
}

impl Mangle<MangleDefault> for LifetimeDef {
    fn mangle_string(&self, _context: MangleDefault) -> String {
        "".to_string()
    }
}

impl Mangle<MangleDefault> for ConstParam {
    fn mangle_string(&self, context: MangleDefault) -> String {
        format!("where_CNST_{}_is_{}", self.ident, self.ty.mangle_string(context))
    }
}

impl Mangle<MangleDefault> for GenericParam {
    fn mangle_string(&self, context: MangleDefault) -> String {
        match self {
            GenericParam::Type(ty) => ty.mangle_string(context),
            GenericParam::Lifetime(lifetime_def) => lifetime_def.mangle_string(context),
            GenericParam::Const(const_param) => const_param.mangle_string(context)
        }
    }
}

impl Mangle<MangleDefault> for TypeParam {
    fn mangle_string(&self, context: MangleDefault) -> String {
        format!("where_{}_is_{}",
                self.ident,
                self.bounds.iter().map(|f| f.mangle_string(context)).collect::<Vec<_>>().join(""))
    }
}

impl Mangle<MangleDefault> for Lifetime {
    fn mangle_string(&self, _context: MangleDefault) -> String {
        "".to_string()
    }
}

impl Mangle<MangleDefault> for TypeParamBound {
    fn mangle_string(&self, context: MangleDefault) -> String {
        match self {
            TypeParamBound::Trait(trait_bound) => trait_bound.mangle_string(context),
            TypeParamBound::Lifetime(lifetime) => lifetime.mangle_string(context),
        }
    }
}

impl Mangle<MangleDefault> for PredicateType {
    fn mangle_string(&self, context: MangleDefault) -> String {
        format!("where_{}_is_{}",
                self.bounded_ty.mangle_string(context),
                self.bounds.iter().map(|f| f.mangle_string(context)).collect::<Vec<_>>().join(""))
    }
}

impl Mangle<MangleDefault> for PredicateLifetime {
    fn mangle_string(&self, _context: MangleDefault) -> String {
        "".to_string()
    }
}
impl Mangle<MangleDefault> for PredicateEq {
    fn mangle_string(&self, _context: MangleDefault) -> String {
        "".to_string()
    }
}

impl Mangle<MangleDefault> for WherePredicate {
    fn mangle_string(&self, context: MangleDefault) -> String {
        match self {
            WherePredicate::Type(predicate_ty) => predicate_ty.mangle_string(context),
            WherePredicate::Lifetime(predicate_lifetime) => predicate_lifetime.mangle_string(context),
            WherePredicate::Eq(predicate_eq) => predicate_eq.mangle_string(context),
        }
    }
}
impl Mangle<MangleDefault> for WhereClause {
    fn mangle_string(&self, context: MangleDefault) -> String {
        self.predicates.iter()
            .map(|predicate| predicate.mangle_string(context))
            .collect::<Vec<_>>()
            .join("_")
    }
}

impl Mangle<MangleDefault> for Generics {
    fn mangle_string(&self, context: MangleDefault) -> String {
        let mut chunks = vec![];
        chunks.extend(self.params.iter().map(|param| param.mangle_string(context)));
        if let Some(where_clause) = self.where_clause.as_ref() {
            chunks.push(where_clause.mangle_string(context));
        }
        chunks.join("_")
    }
}

impl Mangle<MangleDefault> for ObjectKind {
    fn mangle_string(&self, context: MangleDefault) -> String {
        match self {
            ObjectKind::Type(ty) |
            ObjectKind::Item(ty, _) => ty.as_type().mangle_string(context),
            ObjectKind::Empty => panic!("err"),
        }
    }
}

impl Mangle<MangleDefault> for GenericBoundsModel {
    fn mangle_string(&self, context: MangleDefault) -> String {

        let mut chunks = vec![];

        // chunks.extend(self.bounds.iter().map(|obj| obj.mangle_string(context)));
        if let Some(b) = self.bounds.first() {
            chunks.push(b.mangle_string(context));
        }
        chunks.extend(self.predicates.iter()
            .map(|(_predicate, objects)|
                     objects.iter()
                         .map(|obj| obj.mangle_string(context))
                         .collect::<Vec<_>>()
                         .join("_")
                // format!("where_{}_is_{}",
                //         predicate.mangle_string(context),
                //         objects.iter().map(|obj| obj.mangle_string(context)).collect::<Vec<_>>().join("_"))
            )
        );
        //println!("GenericBoundsModel::mangle({}) --> {}", self, chunks.join("_"));
        chunks.join("_")

        // format!("Mixin_{}", chunks.join("_"))

        // format!("{}", self.bounds.iter().map(|b| {
        //     match b {
        //         ObjectKind::Type(ty) |
        //         ObjectKind::Item(ty, _) => ty.ty().mangle_string(context),
        //         ObjectKind::Empty => panic!("err"),
        //     }
        // }).collect::<Vec<_>>().join("_"))
    }
}

