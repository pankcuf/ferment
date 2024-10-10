use quote::ToTokens;
use syn::{BareFnArg, GenericArgument, ParenthesizedGenericArguments, parse_quote, Path, PathArguments, PathSegment, PredicateType, QSelf, ReturnType, TraitBound, Type, TypeArray, TypeBareFn, TypeImplTrait, TypeParamBound, TypePath, TypePtr, TypeReference, TypeSlice, TypeTraitObject, TypeTuple, WherePredicate};
use syn::punctuated::Punctuated;
use crate::ast::{AddPunctuated, CommaPunctuated, PathHolder, TypePathHolder};
use crate::composable::{GenericBoundsModel, NestedArgument, QSelfModel, TypeModel, TypeModeled};
use crate::composer::CommaPunctuatedNestedArguments;
use crate::context::{GlobalContext, ScopeChain};
use crate::conversion::{DictFermentableModelKind, DictTypeModelKind, GroupModelKind, ObjectKind, SmartPointerModelKind, TypeModelKind};
use crate::ext::{AsType, CrateExtension, DictionaryType, ToPath, ToType};
use crate::nprint;

pub trait ToObjectKind {
    fn to_unknown(self, nested_arguments: CommaPunctuatedNestedArguments) -> ObjectKind;
    fn to_object(self, nested_arguments: CommaPunctuatedNestedArguments) -> ObjectKind;
    fn to_trait(self, nested_arguments: CommaPunctuatedNestedArguments) -> ObjectKind;
}

impl ToObjectKind for Type {
    fn to_unknown(self, nested_arguments: CommaPunctuatedNestedArguments) -> ObjectKind {
        ObjectKind::Type(TypeModelKind::Unknown(handle_type_model(self, nested_arguments)))
    }

    fn to_object(self, nested_arguments: CommaPunctuatedNestedArguments) -> ObjectKind {
        ObjectKind::Type(TypeModelKind::Object(handle_type_model(self, nested_arguments)))
    }

    fn to_trait(self, nested_arguments: CommaPunctuatedNestedArguments) -> ObjectKind {
        // TODO: make it Unknown
        ObjectKind::Type(TypeModelKind::Unknown(handle_type_model(self, nested_arguments)))
    }
}

impl ToObjectKind for TypePath {
    fn to_unknown(self, nested_arguments: CommaPunctuatedNestedArguments) -> ObjectKind {
        ObjectKind::Type(TypeModelKind::Unknown(handle_type_path_model(self, nested_arguments)))
    }

    fn to_object(self, nested_arguments: CommaPunctuatedNestedArguments) -> ObjectKind {
        ObjectKind::Type(TypeModelKind::Object(handle_type_path_model(self, nested_arguments)))
    }

    fn to_trait(self, nested_arguments: CommaPunctuatedNestedArguments) -> ObjectKind {
        // TODO: make it Unknown
        ObjectKind::Type(TypeModelKind::Unknown(handle_type_path_model(self, nested_arguments)))
    }
}

pub trait VisitScopeType<'a> where Self: Sized + 'a {
    type Source;
    type Result;
    fn visit_scope_type(&self, source: &Self::Source) -> Self::Result;
}

impl<'a> VisitScopeType<'a> for Type {
    type Source = (&'a ScopeChain, &'a GlobalContext);
    type Result = ObjectKind;

    fn visit_scope_type(&self, source: &Self::Source) -> Self::Result {
        nprint!(1, crate::formatter::Emoji::Node, "=== {}", self.to_token_stream());
        match self {
            Type::Path(type_path) => type_path.visit_scope_type(source),
            Type::TraitObject(type_trait_object) => type_trait_object.visit_scope_type(source),
            Type::Tuple(type_tuple) => type_tuple.visit_scope_type(source),
            Type::Array(type_array) => type_array.visit_scope_type(source),
            Type::Slice(type_slice) => type_slice.visit_scope_type(source),
            Type::BareFn(type_bare_fn) => type_bare_fn.visit_scope_type(source),
            Type::Reference(type_reference) => type_reference.visit_scope_type(source),
            Type::Ptr(type_ptr) => type_ptr.visit_scope_type(source),
            ty => ty.clone().to_unknown(Punctuated::new())
        }
    }
}
fn handle_type_model(ty: Type, nested_arguments: CommaPunctuatedNestedArguments) -> TypeModel {
    TypeModel::new(ty, None, nested_arguments)
}
fn handle_type_path_model(type_path: TypePath, nested_arguments: CommaPunctuatedNestedArguments) -> TypeModel {
    TypeModel::new(Type::Path(type_path), None, nested_arguments)
}

impl<'a> VisitScopeType<'a> for Path {
    type Source = (&'a ScopeChain, &'a GlobalContext, Option<QSelfModel>);
    type Result = ObjectKind;

    fn visit_scope_type(&self, source: &Self::Source) -> Self::Result {
        let (scope, context, qself) = source;
        // println!("{}: Path: visit_scope_type {}", scope.fmt_short(), self.to_token_stream());
        let new_qself = qself.as_ref().map(|q| q.qself.clone());
        let mut segments = self.segments.clone();
        let mut nested_arguments = Punctuated::new();
        for segment in &mut segments {
            match &mut segment.arguments {
                PathArguments::None => {}
                PathArguments::AngleBracketed(angle_bracketed_generic_arguments) => {
                    for arg in &mut angle_bracketed_generic_arguments.args {
                        // println!("Path: visit_scope_type.2222 {}", arg.to_token_stream());
                        match arg {
                            GenericArgument::Type(inner_type) => {
                                let obj_conversion = inner_type.visit_scope_type(&(scope, context));
                                let ty = obj_conversion.maybe_type().unwrap();
                                //println!("nested object::::: {}", obj_conversion);
                                nested_arguments.push(NestedArgument::Object(obj_conversion));
                                *arg = GenericArgument::Type(ty);
                            },
                            _ => {}
                        }
                    }
                }
                PathArguments::Parenthesized(ParenthesizedGenericArguments { inputs, output, .. }) => {
                    //println!("Path::Parenthesized::visit_scope_type: {} --- {}", inputs.to_token_stream(), output.to_token_stream());
                    for arg in inputs {
                        let obj_conversion = arg.visit_scope_type(&(scope, context));
                        let ty = obj_conversion.maybe_type().unwrap();
                        //println!("Path::Parenthesized: {} --- {} --- {}", arg.to_token_stream(), obj_conversion, ty.to_token_stream());
                        nested_arguments.push(NestedArgument::Object(obj_conversion));
                        *arg = ty;
                    }
                    if let ReturnType::Type(_, ret) = output {
                        let obj_conversion = ret.visit_scope_type(&(scope, context));
                        let ty = obj_conversion.maybe_type().unwrap();
                        nested_arguments.push(NestedArgument::Object(obj_conversion));
                        *ret = Box::new(ty);
                    }
                }
            }
        }
        let first_segment = segments.first().unwrap();
        let first_ident = &first_segment.ident;
        let last_segment = segments.last().unwrap();
        let last_ident = &last_segment.ident;
        let import_seg: PathHolder = parse_quote!(#first_ident);
        let import_type_path: TypePathHolder = parse_quote!(#first_ident);

        let mut nested_import_seg: Path = parse_quote!(#last_ident);
        nested_import_seg.segments.last_mut().unwrap().arguments = last_segment.arguments.clone();
        if let Some((generics, bound)) = scope.maybe_generic_bound_for_path(&import_seg.0) {
            nprint!(1, crate::formatter::Emoji::Local, "(Local Generic Bound) {}: {}", generics.to_token_stream(), bound.to_token_stream());
            let path = &import_seg.0;
            let ty: Type = parse_quote!(#path);
            let ident_path = Path::from(PathSegment::from(bound.ident.clone()));
            let generic_trait_bounds = |ty: &Path, ident_path: &Path, bounds: &AddPunctuated<TypeParamBound>| {
                let mut has_bound = false;
                bounds.iter().filter_map(|b| match b {
                    TypeParamBound::Trait(TraitBound { path, .. }) => {
                        let has = ident_path.eq(ty);
                        if !has_bound && has {
                            has_bound = true;
                        }
                        has.then(|| path.visit_scope_type(source))
                    },
                    TypeParamBound::Lifetime(_) => None
                }).collect()
            };
            let bounds = generic_trait_bounds(path, &ident_path, &bound.bounds);
            let predicates = generics.where_clause
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
                .unwrap_or_default();
            // GenericBoundsModel::new(&import_seg.0, &bound, generics, nested_arguments)
            ObjectKind::Type(TypeModelKind::Bounds(GenericBoundsModel::new(ty, bounds, predicates, generics, nested_arguments)))
        } else if let Some(mut import_path) = context.maybe_import_path_ref(scope, &import_seg).cloned() {
            // Can be reevaluated after processing entire scope tree:
            // Because import path can have multiple aliases and we need the most complete one to use mangling correctly
            // We can also determine the type after processing entire scope (if one in fermented crate)
            nprint!(1, crate::formatter::Emoji::Local, "(ScopeImport) {}", import_path.to_token_stream());
            if import_path.is_crate_based() {
                import_path.replace_first_with(&scope.crate_ident_as_path());
            }
            ObjectKind::Type(TypeModelKind::Imported(TypeModel::new(Type::Path(TypePath { qself: new_qself, path: segments.to_path() }), None, nested_arguments), import_path))
        } else if let Some(generic_bounds) = context.generics.maybe_generic_bounds(scope, &import_type_path) {
            // TODO: multiple bounds handling
            if let Some(first_bound) = generic_bounds.first() {
                let first_bound_as_scope = PathHolder::from(first_bound);
                let new_segments = if let Some(Path { segments, .. }) = context.maybe_import_path_ref(scope, &first_bound_as_scope) {
                    nprint!(1, crate::formatter::Emoji::Local, "(Generic Bounds Imported) {}", format_token_stream(&segments));
                    segments.clone()
                } else {
                    nprint!(1, crate::formatter::Emoji::Local, "(Generic Bounds Local) {}", format_token_stream(&segments));
                    let first_bound_ident = &first_bound.segments.first().unwrap().ident;
                    let bounds = if matches!(first_bound_ident.to_string().as_str(), "FnOnce" | "Fn" | "FnMut") {
                        first_bound_ident.to_token_stream()
                    } else {
                        first_bound.to_token_stream()
                    };
                    parse_quote!(#scope::#bounds)
                };
                segments.replace_last_with(&new_segments);
            }
            println!("TO TRAIT: (BOUNDS): {} -- {:?}", segments.to_token_stream(), nested_arguments);
            TypePath { qself: new_qself, path: Path { leading_colon: self.leading_colon, segments } }
                .to_trait(nested_arguments)
        } else {
            nprint!(1, crate::formatter::Emoji::Local, "(Local or Global ....) {}", segments.to_token_stream());
            let obj_scope = scope.obj_root_chain().unwrap_or(scope);
            let object_self_scope = obj_scope.self_scope();
            let self_scope_path = &object_self_scope.self_scope;
            match first_ident.to_string().as_str() {
                "Self" if segments.len() <= 1 => {
                    nprint!(1, crate::formatter::Emoji::Local, "(Self) {}", format_token_stream(first_ident));
                    segments.replace_last_with(&self_scope_path.0.segments);
                    TypePath { qself: new_qself, path: Path { leading_colon: self.leading_colon, segments } }
                        .to_unknown(nested_arguments)
                },
                "Self" => {
                    let tail = segments.crate_less();
                    let last_segment = segments.pop().unwrap();
                    let new_path: Path = parse_quote!(#self_scope_path::#tail);
                    nprint!(1, crate::formatter::Emoji::Local, "(SELF::->) {}: {}", format_token_stream(&last_segment), format_token_stream(&last_segment.clone().into_value().arguments));
                    segments.last_mut().unwrap().arguments = last_segment.into_value().arguments;
                    // TODO: why clear ????
                    segments.clear();
                    segments.extend(new_path.segments);

                    match scope.obj_root_chain() {
                        Some(ScopeChain::Object { .. } | ScopeChain::Impl { .. }) => {
                            // println!("New Local Object: {}", segments.to_token_stream());
                            TypePath { qself: new_qself, path: Path { leading_colon: self.leading_colon, segments } }
                                .to_object(nested_arguments)
                        },
                        Some(ScopeChain::Trait { .. }) =>
                            TypePath { qself: new_qself, path: Path { leading_colon: self.leading_colon, segments } }
                                .to_trait(nested_arguments),
                        _ => panic!("Unexpected scope obj root chain")
                    }

                },
                "Vec" => {
                    ObjectKind::Type(TypeModelKind::Dictionary(DictTypeModelKind::NonPrimitiveFermentable(DictFermentableModelKind::Group(GroupModelKind::Vec(TypeModel::new(Type::Path(TypePath { qself: new_qself, path: Path { leading_colon: self.leading_colon, segments } }), None, nested_arguments))))))
                },
                "Result" if segments.len() == 1 => {
                    ObjectKind::Type(TypeModelKind::Dictionary(DictTypeModelKind::NonPrimitiveFermentable(DictFermentableModelKind::Group(GroupModelKind::Result(TypeModel::new(Type::Path(TypePath { qself: new_qself, path: Path { leading_colon: self.leading_colon, segments } }), None, nested_arguments))))))

                    // println!("visit_scope_type (Option): {}: {}", segments.to_token_stream(), nested_arguments.to_token_stream());
                    // TypePath { qself: new_qself, path: Path { leading_colon: self.leading_colon, segments } }
                    //     .to_object(nested_arguments)
                },
                "Option" => {
                    //println!("visit_scope_type (Option): {} === {}", segments.to_token_stream(), nested_arguments.to_token_stream());
                    ObjectKind::Type(
                        TypeModelKind::Optional(
                            handle_type_path_model(
                                TypePath {
                                    qself: new_qself,
                                    path: Path {
                                        leading_colon: self.leading_colon,
                                        segments
                                    }
                                },
                                nested_arguments)))
                    // TypePath { qself: new_qself, path: Path { leading_colon: self.leading_colon, segments } }
                    //     .to_object(nested_arguments)

                },
                _ if last_ident.to_string().eq("Map") && first_ident.to_string().eq("serde_json") => {
                    TypePath { qself: new_qself, path: Path { leading_colon: self.leading_colon, segments } }
                        .to_object(nested_arguments)
                },
                // _ if first_ident.is_special_generic() => {
                //     ObjectKind::Type(TypeModelKind::Dictionary(DictTypeModelKind::NonPrimitiveFermentable(TypeComposition::new(Type::Path(TypePath { qself: new_qself, path: Path { leading_colon: self.leading_colon, segments } }), None, nested_arguments))))
                // },
                _ if last_ident.is_map() =>
                    ObjectKind::Type(TypeModelKind::Dictionary(DictTypeModelKind::NonPrimitiveFermentable(DictFermentableModelKind::Group(GroupModelKind::Map(TypeModel::new(Type::Path(TypePath { qself: new_qself, path: Path { leading_colon: self.leading_colon, segments } }), None, nested_arguments)))))),
                _ if last_ident.is_btree_set() =>
                    ObjectKind::Type(TypeModelKind::Dictionary(DictTypeModelKind::NonPrimitiveFermentable(DictFermentableModelKind::Group(GroupModelKind::BTreeSet(TypeModel::new(Type::Path(TypePath { qself: new_qself, path: Path { leading_colon: self.leading_colon, segments } }), None, nested_arguments)))))),
                _ if last_ident.is_hash_set() =>
                    ObjectKind::Type(TypeModelKind::Dictionary(DictTypeModelKind::NonPrimitiveFermentable(DictFermentableModelKind::Group(GroupModelKind::HashSet(TypeModel::new(Type::Path(TypePath { qself: new_qself, path: Path { leading_colon: self.leading_colon, segments } }), None, nested_arguments)))))),

                _ if matches!(last_ident.to_string().as_str(), "IndexMap") =>
                    ObjectKind::Type(TypeModelKind::Dictionary(DictTypeModelKind::NonPrimitiveFermentable(DictFermentableModelKind::Group(GroupModelKind::IndexMap(TypeModel::new(Type::Path(TypePath { qself: new_qself, path: Path { leading_colon: self.leading_colon, segments } }), None, nested_arguments)))))),
                _ if first_ident.is_box() => {
                    ObjectKind::Type(TypeModelKind::Dictionary(DictTypeModelKind::NonPrimitiveFermentable(DictFermentableModelKind::SmartPointer(SmartPointerModelKind::Box(TypeModel::new(Type::Path(TypePath { qself: new_qself, path: Path { leading_colon: self.leading_colon, segments } }), None, nested_arguments))))))
                },
                _ if first_ident.is_primitive() => {
                    ObjectKind::Type(TypeModelKind::Dictionary(DictTypeModelKind::Primitive(TypeModel::new_non_gen(first_ident.to_type(), None))))
                },
                _ if matches!(first_ident.to_string().as_str(), "i128") => {
                    ObjectKind::Type(TypeModelKind::Dictionary(DictTypeModelKind::NonPrimitiveFermentable(DictFermentableModelKind::I128(TypeModel::new_non_gen(first_ident.to_type(), None)))))
                },
                _ if matches!(first_ident.to_string().as_str(), "u128") => {
                    ObjectKind::Type(TypeModelKind::Dictionary(DictTypeModelKind::NonPrimitiveFermentable(DictFermentableModelKind::U128(TypeModel::new_non_gen(first_ident.to_type(), None)))))
                },
                _ if first_ident.is_special_std_trait() => {
                    ObjectKind::Type(TypeModelKind::unknown_type(nested_import_seg.to_type()))
                },
                _ if first_ident.is_str() => {
                    ObjectKind::Type(TypeModelKind::Dictionary(DictTypeModelKind::NonPrimitiveFermentable(DictFermentableModelKind::Str(TypeModel::new_non_gen(nested_import_seg.to_type(), None)))))
                },
                _ if first_ident.is_string() => {
                    ObjectKind::Type(TypeModelKind::Dictionary(DictTypeModelKind::NonPrimitiveFermentable(DictFermentableModelKind::String(TypeModel::new_non_gen(nested_import_seg.to_type(), None)))))
                },
                _ if first_ident.is_lambda_fn() => {
                    //println!("first_ident.is_lambda_fn: {}", segments.to_token_stream());
                    ObjectKind::Type(TypeModelKind::Dictionary(DictTypeModelKind::LambdaFn(handle_type_path_model(TypePath { qself: new_qself, path: Path { leading_colon: self.leading_colon, segments } }, nested_arguments))))
                },
                _ => {
                    let obj_parent_scope = obj_scope.parent_scope();
                    let len = segments.len();
                    if len == 1 {
                        nprint!(1, crate::formatter::Emoji::Local, "(Local join single (has {} parent scope): {}) {} + {}",
                            if obj_parent_scope.is_some() { "some" } else { "no" },
                            first_ident,
                            scope,
                            self.to_token_stream());
                        segments.replace_last_with(&match obj_parent_scope {
                            None => {
                                // Global
                                if scope.is_crate_root() {
                                    let scope = scope.crate_ident_ref();
                                    parse_quote!(#scope::#self)
                                } else {
                                    parse_quote!(#scope::#self)
                                }
                            },
                            Some(parent) => {
                                let scope = parent.self_path_holder_ref();
                                // nprint!(1, Emoji::Local, "(Local join single (has parent scope): {}) {} + {}", first_ident, scope, format_token_stream(&path));
                                parse_quote!(#scope::#self)
                            }
                        });
                        // let last_segment = segments.pop().unwrap();
                        // let new_segments: Punctuated<PathSegment, Colon2> = match obj_parent_scope {
                        //     None => {
                        //         // Global
                        //         if scope.is_crate_root() {
                        //             let scope = scope.crate_ident();
                        //
                        //             parse_quote!(#scope::#self)
                        //         } else {
                        //             parse_quote!(#scope::#self)
                        //         }
                        //     },
                        //     Some(parent) => {
                        //         let scope = parent.self_path_holder();
                        //         // nprint!(1, Emoji::Local, "(Local join single (has parent scope): {}) {} + {}", first_ident, scope, format_token_stream(&path));
                        //         parse_quote!(#scope::#self)
                        //     }
                        // };
                        // segments.extend(new_segments);
                        // segments.last_mut().unwrap().arguments = last_segment.into_value().arguments;
                        TypePath { qself: new_qself, path: Path { leading_colon: self.leading_colon, segments } }
                            .to_unknown(nested_arguments)

                    } else {
                        let tail = segments.crate_less();
                        if let Some(QSelfModel { qs: _, qself: QSelf { ty, .. } }) = qself.as_ref() {
                            nprint!(1, crate::formatter::Emoji::Local, "(Local join QSELF: {} [{}]) {} + {}", format_token_stream(ty), format_token_stream(&import_seg), format_token_stream(scope), format_token_stream(self));

                            // println!("------ import local? {} in [{}]", import_seg.to_token_stream(), scope);
                            // println!("------ import parent? {} in [{:?}]", import_seg.to_token_stream(), scope.parent_scope());
                            // println!("------ import object? {} in [{:?}]", import_seg.to_token_stream(), obj_scope);
                            // println!("------ import object parent? {} in [{:?}]", import_seg.to_token_stream(), obj_parent_scope);

                            let maybe_import = context.maybe_scope_import_path(scope, &import_seg)
                                .or(context.maybe_scope_import_path(obj_scope, &import_seg))
                                .or(obj_parent_scope.and_then(|obj_parent_scope|
                                    context.maybe_scope_import_path(obj_parent_scope, &import_seg)));

                            let tt = if let Some(import) = maybe_import {
                                import.clone()
                            } else {
                                let local = obj_parent_scope.unwrap_or(scope);
                                parse_quote!(#local::#import_seg)
                            };
                            let converted: TypePath = match len {
                                0 => parse_quote!(<#ty as #tt>),
                                _ => parse_quote!(<#ty as #tt>::#tail)
                            };

                            match scope.obj_root_chain() {
                                Some(ScopeChain::Trait { .. }) =>
                                    converted.to_trait(nested_arguments),
                                Some(ScopeChain::Object { .. } | ScopeChain::Impl { .. }) =>
                                    converted.to_object(nested_arguments),
                                _ =>
                                    converted.to_unknown(nested_arguments)
                            }
                        } else {
                            // println!("No root chain: {} --- {}", self.to_token_stream(), nested_arguments.to_token_stream());
                            // println!("------ import local? {} in [{}]", import_seg.to_token_stream(), scope);
                            // println!("------ import parent? {} in [{:?}]", import_seg.to_token_stream(), scope.parent_scope());
                            // println!("------ import object? {} in [{:?}]", import_seg.to_token_stream(), obj_scope);
                            // println!("------ import object parent? {} in [{:?}]", import_seg.to_token_stream(), obj_parent_scope);

                            TypePath { qself: new_qself, path: self.clone() }
                                .to_unknown(nested_arguments)

                            //(Local join multi: std) ferment_example::std_error_Error_FFI + std::fmt::Result
                            // nprint!(1, Emoji::Local, "(Local or ExternalChunks join multi) {} + {}", format_token_stream(scope), format_token_stream(self));
                            // let last_segment = segments.last().cloned().unwrap();
                            // let new_segments: Punctuated<PathSegment, Colon2> = if self.leading_colon.is_none() {
                            //     parse_quote!(#scope::#self)
                            // } else {
                            //     parse_quote!(#scope #self)
                            // };
                            // segments.clear();
                            // segments.extend(new_segments);
                            // segments.last_mut().unwrap().arguments = last_segment.arguments;
                            // TypePath { qself: new_qself, path: Path { leading_colon: self.leading_colon, segments } }
                            //     .to_unknown()
                        }
                    }
                },
            }
        }
    }
}

impl<'a> VisitScopeType<'a> for Option<QSelf> {
    type Source = (&'a ScopeChain, &'a GlobalContext);
    type Result = Option<QSelfModel>;

    fn visit_scope_type(&self, source: &Self::Source) -> Self::Result {
        self.as_ref().map(|qself| {
            let mut new_qself = qself.clone();
            let qs = qself.ty.visit_scope_type(source);
            let qs = qs.maybe_type_model_kind_ref().unwrap().type_model_ref().clone();
            new_qself.ty = Box::new(qs.as_type().clone());
            QSelfModel { qs, qself: new_qself }
        })
    }
}

impl<'a> VisitScopeType<'a> for TypePath {
    type Source = (&'a ScopeChain, &'a GlobalContext);
    type Result = ObjectKind;

    fn visit_scope_type(&self, source: &Self::Source) -> Self::Result {
        let (scope, context) = source;
        self.path.visit_scope_type(&(scope, context, self.qself.visit_scope_type(source)))
    }
}

impl<'a> VisitScopeType<'a> for TypeArray {
    type Source = (&'a ScopeChain, &'a GlobalContext);
    type Result = ObjectKind;

    fn visit_scope_type(&self, source: &Self::Source) -> Self::Result {
        ObjectKind::Type(
            TypeModelKind::Slice(
                TypeModel::new(
                    Type::Array(self.clone()),
                    None,
                    Punctuated::from_iter([NestedArgument::Object(self.elem.visit_scope_type(source))]))))
    }
}

impl<'a> VisitScopeType<'a> for TypeSlice {
    type Source = (&'a ScopeChain, &'a GlobalContext);
    type Result = ObjectKind;

    fn visit_scope_type(&self, source: &Self::Source) -> Self::Result {
        ObjectKind::Type(
            TypeModelKind::Slice(
                TypeModel::new(
                    Type::Slice(self.clone()),
                    None,
                    Punctuated::from_iter([NestedArgument::Object(self.elem.visit_scope_type(source))]))))

    }
}
impl<'a> VisitScopeType<'a> for TypeBareFn {
    type Source = (&'a ScopeChain, &'a GlobalContext);
    type Result = ObjectKind;

    fn visit_scope_type(&self, source: &Self::Source) -> Self::Result {
        let mut nested = self.inputs.iter().map(|BareFnArg { ty, .. }| NestedArgument::Object(ty.visit_scope_type(source))).collect::<CommaPunctuated<_>>();
        if let ReturnType::Type(_, ty) = &self.output {
            nested.push(NestedArgument::Object(ty.visit_scope_type(source)))
        }
        println!("TypeBareFn::visit_scope_type: {} --- {}", self.to_token_stream(), nested.to_token_stream());
        ObjectKind::Type(
            TypeModelKind::FnPointer(
                TypeModel::new(Type::BareFn(self.clone()), None, nested)))
    }
}
impl<'a> VisitScopeType<'a> for TypeReference {
    type Source = (&'a ScopeChain, &'a GlobalContext);
    type Result = ObjectKind;
    fn visit_scope_type(&self, source: &Self::Source) -> Self::Result {
        let mut obj = self.elem.visit_scope_type(source);
        match &mut obj {
            ObjectKind::Type(tyc) |
            ObjectKind::Item(tyc, _) => {
                tyc.replace_model_type(Type::Reference(TypeReference {
                    and_token: Default::default(),
                    lifetime: self.lifetime.clone(),
                    mutability: self.mutability.clone(),
                    elem: Box::new(tyc.to_type()),
                }));
            }
            ObjectKind::Empty => {}
        }
        obj
    }
}
impl<'a> VisitScopeType<'a> for TypePtr {
    type Source = (&'a ScopeChain, &'a GlobalContext);
    type Result = ObjectKind;
    fn visit_scope_type(&self, source: &Self::Source) -> Self::Result {
        let mut obj = self.elem.visit_scope_type(source);
        match &mut obj {
            ObjectKind::Type(tyc) |
            ObjectKind::Item(tyc, _) => {
                let ty = tyc.to_type();
                match (self.const_token, self.mutability) {
                    (Some(..), _) => {
                        tyc.replace_model_type(parse_quote!(*const #ty))
                    },
                    (_, Some(..)) => {
                        tyc.replace_model_type(parse_quote!(*mut #ty))
                    },
                    _ => {}
                }
            }
            ObjectKind::Empty => {}
        }
        obj
    }
}

impl<'a> VisitScopeType<'a> for TypeTuple {
    type Source = (&'a ScopeChain, &'a GlobalContext);
    type Result = ObjectKind;

    fn visit_scope_type(&self, source: &Self::Source) -> Self::Result {
        ObjectKind::Type(
            TypeModelKind::Tuple(
                TypeModel::new(
                    Type::Tuple(self.clone()),
                    None,
                    self.elems
                        .iter()
                        .map(|ty| NestedArgument::Object(ty.visit_scope_type(source)))
                        .collect())))
    }
}

impl<'a> VisitScopeType<'a> for TypeTraitObject {
    type Source = (&'a ScopeChain, &'a GlobalContext);
    type Result = ObjectKind;

    fn visit_scope_type(&self, source: &Self::Source) -> Self::Result {
        //println!("visit_scope_type (TypeTraitObject): {}", self.to_token_stream());
        let (scope, context) = source;
        let TypeTraitObject { dyn_token, bounds } = self;
        let mut bounds = bounds.clone();
        let nested_arguments = CommaPunctuatedNestedArguments::new();
        bounds.iter_mut().for_each(|bound| match bound {
            TypeParamBound::Trait(TraitBound { path, .. }) => {
                let object = path.visit_scope_type(&(scope, context, None));
                match &object {
                    ObjectKind::Type(tyc) |
                    ObjectKind::Item(tyc, _) => {
                        let ty = tyc.to_type();
                        match ty {
                            Type::Path(TypePath { path: ty_path, .. }) => {
                                *path = ty_path;
                            }
                            Type::ImplTrait(TypeImplTrait { bounds, .. }) => {
                                *bound = bounds.first().unwrap().clone();
                            }
                            Type::TraitObject(TypeTraitObject { bounds, .. }) => {
                                *bound = bounds.first().unwrap().clone();
                            }
                            _ => {}
                        }
                    }
                    ObjectKind::Empty => {}
                }
                //nested_arguments.push(NestedArgument::Constraint(object));
            },
            _ => {},
        });
        // TODO: make it Unknown
        ObjectKind::Type(
            TypeModelKind::Unknown(
                handle_type_model(
                    Type::TraitObject(
                        TypeTraitObject {
                            dyn_token: dyn_token.clone(),
                            bounds
                        }),
                    nested_arguments)))
    }
}
