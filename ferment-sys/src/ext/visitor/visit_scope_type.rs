use indexmap::IndexMap;
use proc_macro2::Ident;
use syn::{parse_quote, BareFnArg, GenericArgument, Generics, Path, PathArguments, PathSegment, PredicateType, QSelf, ReturnType, TraitBound, Type, TypeArray, TypeBareFn, TypeGroup, TypeImplTrait, TypeParam, TypeParamBound, TypeParen, TypePath, TypePtr, TypeSlice, TypeTraitObject, TypeTuple, WherePredicate};
use syn::punctuated::Punctuated;
use syn::token::PathSep;
use crate::ast::{AddPunctuated, Colon2Punctuated, CommaPunctuated};
use crate::composable::{GenericBoundsModel, NestedArgument, TypeModel, TypeModeled};
use crate::composer::CommaPunctuatedNestedArguments;
use crate::context::{GlobalContext, ScopeChain};
use crate::kind::{DictFermentableModelKind, GroupModelKind, ObjectKind, SmartPointerModelKind, TypeModelKind};
use crate::ext::{Accessory, AsType, CrateBased, CrateExtension, DictionaryType, GenericBoundKey, Join, MaybeTraitBound, PathTransform, Pop, PunctuateOne, ToPath, ToType};

pub trait VisitScopeType<'a> where Self: Sized + 'a {
    type Source;
    type Result;
    fn visit_scope_type(&self, source: &Self::Source) -> Self::Result;
}
impl<'a> VisitScopeType<'a> for Type {
    type Source = (&'a ScopeChain, &'a GlobalContext);
    type Result = ObjectKind;

    fn visit_scope_type(&self, source: &Self::Source) -> Self::Result {
        match self {
            Type::BareFn(TypeBareFn { inputs, output, .. }) => {
                let mut nested = CommaPunctuated::from_iter(inputs.iter().map(|BareFnArg { ty, .. }| NestedArgument::Object(ty.visit_scope_type(source))));
                if let ReturnType::Type(_, ty) = output {
                    nested.push(NestedArgument::Object(ty.visit_scope_type(source)))
                }
                ObjectKind::model_type(TypeModelKind::FnPointer, TypeModel::new_nested_ref(self, nested))

            },
            Type::Path(type_path) => type_path.visit_scope_type(source),
            Type::Ptr(TypePtr { elem, const_token, mutability, .. }) => {
                let mut obj = elem.visit_scope_type(source);
                if let ObjectKind::Type(tyc) | ObjectKind::Item(tyc, _) = &mut obj {
                    let ty = if let TypeModelKind::Imported(ty, import_path) = &tyc {
                        import_path.popped().joined(ty.as_type()).to_type()
                    } else {
                        tyc.to_type()
                    };
                    if const_token.is_some() {
                        tyc.replace_model_type(ty.joined_const())
                    } else if mutability.is_some() {
                        tyc.replace_model_type(ty.joined_mut())
                    }
                }
                obj
            },
            Type::Reference(type_reference) => {
                let mut new_type_reference = type_reference.clone();
                let mut obj = type_reference.elem.visit_scope_type(source);
                if let ObjectKind::Type(tyc) | ObjectKind::Item(tyc, _) = &mut obj {
                    new_type_reference.elem = Box::new(tyc.to_type());
                    tyc.replace_model_type(Type::Reference(new_type_reference));
                }
                obj
            },
            Type::ImplTrait(TypeImplTrait { impl_token, bounds }) => {
                let (bounds, nested_arguments) = bounds.visit_scope_type(source);
                ObjectKind::model_type(TypeModelKind::Unknown, TypeModel::new_nested(Type::ImplTrait(TypeImplTrait { impl_token: impl_token.clone(), bounds }), nested_arguments))
            }
            Type::TraitObject(TypeTraitObject { dyn_token, bounds }) => {
                let (bounds, nested_arguments) = bounds.visit_scope_type(source);
                ObjectKind::model_type(TypeModelKind::Unknown, TypeModel::new_nested(Type::TraitObject(TypeTraitObject { dyn_token: dyn_token.clone(), bounds }), nested_arguments))
            },
            Type::Array(TypeArray { elem, .. }) =>
                ObjectKind::model_type(TypeModelKind::Array, TypeModel::new_nested_ref(self, NestedArgument::Object(elem.visit_scope_type(source)).punctuate_one())),
            Type::Group(TypeGroup { elem, .. }) |
            Type::Paren(TypeParen { elem, .. }) =>
                ObjectKind::model_type(TypeModelKind::Unknown, TypeModel::new_nested_ref(self, NestedArgument::Object(elem.visit_scope_type(source)).punctuate_one())),
            Type::Slice(TypeSlice { elem, .. }) =>
                ObjectKind::model_type(TypeModelKind::Slice, TypeModel::new_nested_ref(self, NestedArgument::Object(elem.visit_scope_type(source)).punctuate_one())),
            Type::Tuple(TypeTuple { elems, .. }) =>
                ObjectKind::model_type(TypeModelKind::Tuple, TypeModel::new_nested_ref(self, Punctuated::from_iter(elems.iter().map(|elem| NestedArgument::Object(elem.visit_scope_type(source)))))),
            ty => ObjectKind::unknown_type(ty.clone())
        }
    }
}

impl<'a> VisitScopeType<'a> for AddPunctuated<TypeParamBound> {
    type Source = (&'a ScopeChain, &'a GlobalContext);
    type Result = (AddPunctuated<TypeParamBound>, CommaPunctuatedNestedArguments);

    fn visit_scope_type(&self, source: &Self::Source) -> Self::Result {
        let mut nested_arguments = CommaPunctuatedNestedArguments::new();
        let mut bounds = self.clone();
        bounds.iter_mut().for_each(|bound| if let TypeParamBound::Trait(TraitBound { path, .. }) = bound {
            let (scope, context) = source;
            let object = path.visit_scope_type(&(scope, context, None));
            match &object {
                ObjectKind::Type(tyc) |
                ObjectKind::Item(tyc, _) => match tyc.to_type() {
                    Type::Path(TypePath { path: ty_path, .. }) => {
                        *path = ty_path;
                    }
                    Type::ImplTrait(TypeImplTrait { bounds, .. }) |
                    Type::TraitObject(TypeTraitObject { bounds, .. }) => if let Some(first_bound) = bounds.first() {
                        *bound = first_bound.clone();
                    }
                    _ => {}
                }
                ObjectKind::Empty => {}
            }
            nested_arguments.push(NestedArgument::Constraint(object));
        });
        (bounds, nested_arguments)
    }
}

fn collect_trait_bounds(ty: &Type, ident_path: &Type, bounds: &AddPunctuated<TypeParamBound>, source: &(&ScopeChain, &GlobalContext, Option<QSelf>)) -> Vec<ObjectKind> {
    let mut has_bound = false;
    bounds.iter().filter_map(|b| b.maybe_trait_bound().and_then(|TraitBound { path, .. }| {
        let has = ident_path.eq(ty);
        if !has_bound && has {
            has_bound = true;
        }
        has.then(|| path.visit_scope_type(source))
    })).collect()
}

fn create_generics_chain(ident: &Ident, bound: TypeParam, generics: &Generics, source: &(&ScopeChain, &GlobalContext, Option<QSelf>)) -> (Vec<ObjectKind>, IndexMap<Type, Vec<ObjectKind>>) {
    // let ident_path = Type::Path(TypePath { qself: None, path: Path::from(PathSegment::from(bound.ident.clone())) });
    let ident_path = Path::from(PathSegment::from(bound.ident.clone())).to_type();
    let ty = ident.to_type();
    let bounds = collect_trait_bounds(&ty, &ident_path, &bound.bounds, source);
    let predicates = generics.where_clause
        .as_ref()
        .map(|where_clause|
            where_clause.predicates
                .iter()
                .filter_map(|predicate| match predicate {
                    WherePredicate::Type(PredicateType { bounded_ty, bounds, .. }) if ty.eq(bounded_ty) =>
                        Some((bounded_ty.clone(), collect_trait_bounds(&ty, &bounded_ty, bounds, source))),
                    _ => None
                })
                .collect())
        .unwrap_or_default();
    (bounds, predicates)
}

// fn collect_trait_bounds2(bounds: &AddPunctuated<TypeParamBound>, source: &(&ScopeChain, &GlobalContext, Option<QSelf>)) -> Vec<ObjectKind> {
//     let mut paths = Vec::<ObjectKind>::new();
//     bounds.iter().for_each(|b| match b {
//         TypeParamBound::Trait(TraitBound { modifier, path, .. }) => if let Some(path) = (matches!(modifier, TraitBoundModifier::None) && !path.segments.last().map(|PathSegment { ident, .. }| ident.eq("Sized")).unwrap_or_default()).then(|| path.clone()) {
//             paths.push(path.visit_scope_type(source));
//         },
//         _ => {}
//     });
//     paths
// }
//
// fn create_generics_chain2(bound: TypeParam, generics: &Generics, source: &(&ScopeChain, &GlobalContext, Option<QSelf>)) -> (Type, Vec<ObjectKind>) {
//     let Generics { params, where_clause, .. } = generics;
//     let ty = bound.ident.to_type();
//     let mut generics_chain = Vec::<ObjectKind>::new();
//     // 1) Bounds in angle brackets: `fn foo<T: Trait, U: A + B>() {}`
//     params.iter().for_each(|gp| match gp {
//         GenericParam::Type(type_param) => {
//             if type_param.eq(&bound) {
//                 generics_chain.extend(collect_trait_bounds2(&bound.bounds, source));
//             }
//         },
//         _ => {}
//     });
//     // 2) Where clause predicates: `where T: Trait, Vec<U>: Another`
//     if let Some(WhereClause { predicates, .. }) = where_clause {
//         predicates.iter().for_each(|pred| match pred {
//             WherePredicate::Type(PredicateType { bounded_ty, bounds, .. }) =>
//                 generics_chain.extend(collect_trait_bounds2(bounds, source)),
//             _ => {}
//         });
//     }
//     // Ensure each generic type parameter appears at least once; add unlimited if no restrictive bound collected
//     // params.iter().for_each(|gp| match gp {
//     //     GenericParam::Type(TypeParam { ident, .. }) if bound.ident.ne(ident) => {
//     //         generics_chain.entry(parse_quote!(#ident))
//     //             .or_default();
//     //     }
//     //     _ => {}
//     // });
//     // Dedup per-type trait paths by token string and order deterministically
//     // for trait_paths in generics_chain.iter_mut() {
//     let mut seen_p: HashSet<String> = HashSet::new();
//     generics_chain.retain(|p| seen_p.insert(p.to_token_stream().to_string()));
//     generics_chain.sort_by(|a, b| {
//         let a_s = a.to_token_stream().to_string();
//         let b_s = b.to_token_stream().to_string();
//         let norm = |s: &str| s.replace(' ', "");
//         let w = |s: &str| if norm(s).starts_with("::") { 1 } else { 0 };
//         w(&a_s).cmp(&w(&b_s)).then_with(|| a_s.cmp(&b_s))
//     });
//     // }
//     // If a bounded type has any restrictive trait bounds, drop its unlimited entries
//     let mut has_restrictive: HashMap<String, bool> = HashMap::new();
//     // for (bounded_ty, trait_paths) in &generics_chain {
//         let ty_s = bound.ident.to_token_stream().to_string();
//         let e = has_restrictive.entry(ty_s).or_insert(false);
//         if !generics_chain.is_empty() {
//             *e = true;
//         }
//     // }
//     generics_chain.retain(|trait_paths| {
//         if !trait_paths.is_empty() { return true; }
//         let ty_s = bound.ident.to_token_stream().to_string();
//         !has_restrictive.get(&ty_s).copied().unwrap_or_default()
//     });
//
//     // Deterministic order: first by bounded type, then by trait path (both token strings)
//     generics_chain.sort_by(|a_ty, a_paths, b_ty, b_paths| {
//         // Prefer simple type parameters (single-segment, no leading ::) before concrete/qualified types
//         let type_weight = |t: &Type| match t {
//             // simple ident like `T`, `U`
//             Type::Path(TypePath { qself: None, path: Path { leading_colon: None, segments } }) if segments.len() == 1 => 0,
//             _ => 1
//         };
//         let a_ty_s = a_ty.to_token_stream().to_string();
//         let a_tr_s = a_paths.iter().map(|p| p.to_token_stream().to_string()).collect::<Vec<_>>().join(" + ");
//         let b_ty_s = b_ty.to_token_stream().to_string();
//         let b_tr_s = b_paths.iter().map(|p| p.to_token_stream().to_string()).collect::<Vec<_>>().join(" + ");
//         let a_tw = type_weight(a_ty);
//         let b_tw = type_weight(b_ty);
//         // Prefer bare trait names over fully-qualified ones; unlimited last
//         let trait_weight = |s: &str| {
//             if s == "<unlimited>" { 2 }
//             else if s.replace(' ', "").starts_with("::") { 1 }
//             else { 0 }
//         };
//         a_tw
//             .cmp(&b_tw)
//             .then_with(|| a_ty_s.cmp(&b_ty_s))
//             .then_with(|| trait_weight(&a_tr_s).cmp(&trait_weight(&b_tr_s)))
//             .then_with(|| a_tr_s.cmp(&b_tr_s))
//     });
//     (ty, generics_chain)
// }
impl<'a> VisitScopeType<'a> for TypePath {
    type Source = (&'a ScopeChain, &'a GlobalContext);
    type Result = ObjectKind;

    fn visit_scope_type(&self, source: &Self::Source) -> Self::Result {
        let (scope, context) = source;
        let qself = self.qself.as_ref().and_then(|qself| qself.ty.visit_scope_type(source).maybe_type_model_kind_ref().map(|type_model_kind| {
            let mut qself = qself.clone();
            qself.ty = Box::new(type_model_kind.as_type().clone());
            qself
        }));

        self.path.visit_scope_type(&(scope, context, qself))
    }
}

impl<'a> VisitScopeType<'a> for PathArguments {
    type Source = (&'a ScopeChain, &'a GlobalContext);
    type Result = (PathArguments, CommaPunctuatedNestedArguments);

    fn visit_scope_type(&self, source: &Self::Source) -> Self::Result {
        let mut path_arguments = self.clone();
        let nested_arguments = match &mut path_arguments {
            PathArguments::None => Punctuated::new(),
            PathArguments::AngleBracketed(arguments) => {
                let mut nested_arguments = Punctuated::new();
                arguments.args.iter_mut().for_each(|arg| if let GenericArgument::Type(inner_type) = arg {
                    let obj_conversion = inner_type.visit_scope_type(source);
                    if let Some(ty_model_kind) = obj_conversion.maybe_type_model_kind_ref() {
                        let new_inner_ty = ty_model_kind.as_type().clone();
                        nested_arguments.push(NestedArgument::Object(obj_conversion));
                        *arg = GenericArgument::Type(new_inner_ty);
                    }
                });
                nested_arguments
            },
            PathArguments::Parenthesized(arguments) => {
                let mut nested_arguments = Punctuated::new();
                arguments.inputs.iter_mut().for_each(|arg| {
                    let obj_conversion = arg.visit_scope_type(source);
                    if let Some(ty_model_kind) = obj_conversion.maybe_type_model_kind_ref() {
                        let new_inner_ty = ty_model_kind.as_type().clone();
                        nested_arguments.push(NestedArgument::Object(obj_conversion));
                        *arg = new_inner_ty;
                    }
                });
                if let ReturnType::Type(_, ret) = &mut arguments.output {
                    let obj_conversion = ret.visit_scope_type(source);
                    if let Some(ty_model_kind) = obj_conversion.maybe_type_model_kind_ref() {
                        let new_inner_ty = ty_model_kind.as_type().clone();
                        nested_arguments.push(NestedArgument::Object(obj_conversion));
                        *ret = Box::new(new_inner_ty);
                    }
                }
                nested_arguments
            }
        };
        (path_arguments, nested_arguments)
    }
}

impl<'a> VisitScopeType<'a> for Colon2Punctuated<PathSegment> {
    type Source = (&'a ScopeChain, &'a GlobalContext);
    type Result = (Colon2Punctuated<PathSegment>, CommaPunctuatedNestedArguments);

    fn visit_scope_type(&self, source: &Self::Source) -> Self::Result {
        let mut segments = self.clone();
        let mut nested_arguments = Punctuated::new();
        segments.iter_mut().for_each(|PathSegment { arguments, .. }| {
            let (new_arguments, new_nested_arguments) = arguments.visit_scope_type(source);
            *arguments = new_arguments;
            nested_arguments.extend(new_nested_arguments);
        });
        (segments, nested_arguments)
    }
}

impl<'a> VisitScopeType<'a> for Path {
    type Source = (&'a ScopeChain, &'a GlobalContext, Option<QSelf>);
    type Result = ObjectKind;

    fn visit_scope_type(&self, source: &Self::Source) -> Self::Result {
        let (scope, context, qself) = source;
        let (mut segments, nested_arguments) = self.segments.visit_scope_type(&(scope, context));
        match (segments.first(), segments.last()) {
            (Some(PathSegment { ident, .. }), Some(PathSegment { ident: last_ident, arguments: last_arguments })) => {
                let generic_key = GenericBoundKey::ident(ident);
                let mut last_import_seg = last_ident.to_path();
                if let Some(PathSegment { arguments, .. }) = last_import_seg.segments.last_mut() {
                    *arguments = last_arguments.clone();
                }
                if let Some((generics, bound)) = scope.maybe_generic_bound_for_path(&generic_key) {
                    // let ty = bound.ident.to_type();
                    // let (ty, chain) = create_generics_chain2(bound, &generics, source);
                    let (bounds, predicates) = create_generics_chain(ident, bound, &generics, source);
                    ObjectKind::bounds(GenericBoundsModel::new(ident, bounds, predicates, generics, nested_arguments))
                    // ObjectKind::bounds(GenericBoundsModel::new(ident, ty, chain, generics, nested_arguments))
                } else if let Some(import_path) = context.maybe_import_path_ref(scope, &generic_key) {
                    // Can be reevaluated after processing entire scope tree:
                    // Because import path can have multiple aliases and we need the most complete one to use mangling correctly
                    // We can also determine the type after processing entire scope (if one in fermented crate)
                    ObjectKind::imported_model_type(handle_type_path_model(qself, None, segments, nested_arguments), import_path.crate_named(&scope.crate_ident_as_path()))
                } else if let Some(generic_bounds) = context.generics.maybe_generic_bounds(scope, &ident.to_type()) {
                    // TODO: multiple bounds handling
                    if let Some(first_bound) = generic_bounds.first() {
                        let key = GenericBoundKey::path(first_bound);
                        if let Some(Path { segments: import, .. }) = context.maybe_import_path_ref(scope, &key) {
                            segments.replace_last_with(import);
                        } else {
                            let scope_segments = &scope.self_path_ref().segments;
                            let new_segments = match &first_bound.segments.first() {
                                Some(PathSegment { ident, .. }) if ident.is_lambda_fn() =>
                                    scope_segments.joined(ident),
                                _ =>
                                    scope_segments.joined(first_bound)
                            };
                            segments.replace_last_with(&new_segments);
                        }
                    }
                    ObjectKind::unknown_model_type_path(qself, self.leading_colon, segments, nested_arguments)
                } else {
                    let obj_scope = scope.obj_root_chain().unwrap_or(scope);
                    let len = segments.len();
                    match ident.to_string().as_str() {
                        "Self" => if len <= 1 {
                            segments.replace_last_with(&obj_scope.self_path_ref().segments);
                            ObjectKind::unknown_model_type_path(qself, self.leading_colon, segments, nested_arguments)
                        } else {
                            let tail = segments.crate_less();
                            let last_segment = segments.pop().unwrap();
                            let new_segments = obj_scope.self_path_ref().segments.joined(&tail);
                            if let Some(PathSegment { arguments, .. }) = segments.last_mut() {
                                *arguments = last_segment.into_value().arguments;
                            }
                            segments.clear();
                            segments.extend(new_segments);
                            scope.obj_root_model_composer()(handle_type_path_model(qself, self.leading_colon, segments, nested_arguments))
                        },
                        "Vec" =>
                            ObjectKind::group_type(GroupModelKind::Vec(handle_type_path_model(qself, self.leading_colon, segments, nested_arguments))),
                        "Result" if len == 1 =>
                            ObjectKind::group_type(GroupModelKind::Result(handle_type_path_model(qself, self.leading_colon, segments, nested_arguments))),
                        "Option" =>
                            ObjectKind::optional_model_type(handle_type_path_model(qself, self.leading_colon, segments, nested_arguments)),
                        _ if last_ident.to_string().eq("Map") && ident.to_string().eq("serde_json") =>
                            ObjectKind::object_model_type(handle_type_path_model(qself, self.leading_colon, segments, nested_arguments)),
                        _ if last_ident.is_map() =>
                            ObjectKind::group_type(GroupModelKind::Map(handle_type_path_model(qself, self.leading_colon, segments, nested_arguments))),
                        _ if last_ident.is_btree_set() =>
                            ObjectKind::group_type(GroupModelKind::BTreeSet(handle_type_path_model(qself, self.leading_colon, segments, nested_arguments))),
                        _ if last_ident.is_hash_set() =>
                            ObjectKind::group_type(GroupModelKind::HashSet(handle_type_path_model(qself, self.leading_colon, segments, nested_arguments))),
                        _ if last_ident.eq("IndexMap") =>
                            ObjectKind::group_type(GroupModelKind::IndexMap(handle_type_path_model(qself, self.leading_colon, segments, nested_arguments))),
                        _ if last_ident.eq("IndexSet") =>
                            ObjectKind::group_type(GroupModelKind::IndexSet(handle_type_path_model(qself, self.leading_colon, segments, nested_arguments))),
                        _ if ident.is_box() =>
                            ObjectKind::smart_ptr_type(SmartPointerModelKind::Box(handle_type_path_model(qself, self.leading_colon, segments, nested_arguments))),
                        _ if ident.is_cow() =>
                            ObjectKind::non_primitive_fermentable_type(DictFermentableModelKind::Cow(handle_type_path_model(qself, self.leading_colon, segments, nested_arguments))),
                        _ if ident.is_primitive() =>
                            ObjectKind::primitive_type(ident),
                        _ if ident.eq("i128") =>
                            ObjectKind::i128_type(ident),
                        _ if ident.eq("u128") =>
                            ObjectKind::u128_type(ident),
                        _ if ident.is_special_std_trait() =>
                            ObjectKind::unknown_type(last_import_seg.to_type()),
                        _ if ident.is_str() =>
                            ObjectKind::str_type(last_import_seg.to_type()),
                        _ if ident.is_string() =>
                            ObjectKind::string_type(last_import_seg.to_type()),
                        _ if ident.is_lambda_fn() =>
                            ObjectKind::lambda_fn_model_type(handle_type_path_model(qself, self.leading_colon, segments, nested_arguments)),
                        _ => if len == 1 {
                            segments.replace_last_with(&match obj_scope.parent_scope() {
                                // Global
                                None if scope.is_crate_root() => scope.crate_ident_as_path().segments.joined(self),
                                None => scope.self_path_ref().segments.joined(self),
                                Some(parent) => parent.self_path_ref().segments.joined(self),
                            });
                            ObjectKind::unknown_model_type_path(qself, self.leading_colon, segments, nested_arguments)
                        } else if let Some(QSelf { ty, .. }) = qself {
                            let obj_parent_scope = obj_scope.parent_scope();
                            let tt = context.maybe_scope_import_path_ref(scope, &generic_key)
                                .or_else(|| context.maybe_scope_import_path_ref(obj_scope, &generic_key))
                                .or_else(|| obj_parent_scope.and_then(|parent_scope| context.maybe_scope_import_path_ref(parent_scope, &generic_key)))
                                .cloned()
                                .unwrap_or_else(|| obj_parent_scope.unwrap_or(scope).self_path_ref().joined(&generic_key));
                            let ty: Type = parse_quote!(<#ty as #tt>);
                            scope.obj_root_model_composer()(TypeModel::new_nested((len > 0).then(|| ty.joined(&segments.crate_less())).unwrap_or(ty), nested_arguments))
                        } else {
                            ObjectKind::unknown_model_type_path(qself, self.leading_colon, segments, nested_arguments)
                        }
                    }
                }
            },
            _ => ObjectKind::Empty
        }

    }
}

pub fn handle_type_path_model(qself: &Option<QSelf>, leading_colon: Option<PathSep>, segments: Colon2Punctuated<PathSegment>, nested_arguments: CommaPunctuatedNestedArguments) -> TypeModel {
    TypeModel::new_nested(Type::Path(TypePath { qself: qself.clone(), path: Path { leading_colon, segments }}), nested_arguments)
}
