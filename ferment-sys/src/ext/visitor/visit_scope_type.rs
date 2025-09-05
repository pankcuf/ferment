use syn::{parse_quote, BareFnArg, GenericArgument, ParenthesizedGenericArguments, Path, PathArguments, PathSegment, PredicateType, QSelf, ReturnType, TraitBound, Type, TypeArray, TypeBareFn, TypeGroup, TypeImplTrait, TypeParamBound, TypeParen, TypePath, TypePtr, TypeReference, TypeSlice, TypeTraitObject, WherePredicate};
use syn::punctuated::Punctuated;
use syn::token::PathSep;
use crate::ast::{AddPunctuated, CommaPunctuated, PathHolder};
use crate::composable::{GenericBoundsModel, NestedArgument, QSelfModel, TypeModel, TypeModeled};
use crate::composer::CommaPunctuatedNestedArguments;
use crate::context::{GlobalContext, ScopeChain};
use crate::kind::{DictFermentableModelKind, GroupModelKind, ObjectKind, SmartPointerModelKind, TypeModelKind};
use crate::ext::{Accessory, AsType, CrateExtension, DictionaryType, Pop, ToPath, ToType};
use crate::nprint;



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
            Type::BareFn(type_bare_fn) => type_bare_fn.visit_scope_type(source),
            Type::ImplTrait(type_impl_trait) => type_impl_trait.visit_scope_type(source),
            Type::Path(type_path) => type_path.visit_scope_type(source),
            Type::Ptr(type_ptr) => type_ptr.visit_scope_type(source),
            Type::Reference(type_reference) => type_reference.visit_scope_type(source),
            Type::TraitObject(type_trait_object) => type_trait_object.visit_scope_type(source),
            Type::Array(TypeArray { elem, .. }) =>
                ObjectKind::array_model_type(
                    handle_type_model(
                        self.clone(),
                        Punctuated::from_iter([NestedArgument::Object(elem.visit_scope_type(source))]))),
            Type::Group(TypeGroup { elem, .. }) |
            Type::Paren(TypeParen { elem, .. }) =>
                ObjectKind::unknown_model_type(
                    handle_type_model(
                        self.clone(),
                        Punctuated::from_iter([NestedArgument::Object(elem.visit_scope_type(source))]))),
            Type::Slice(TypeSlice { elem, .. }) =>
                ObjectKind::slice_model_type(
                    handle_type_model(
                        self.clone(),
                        Punctuated::from_iter([NestedArgument::Object(elem.visit_scope_type(source))]))),
            Type::Tuple(type_tuple) =>
                ObjectKind::tuple_model_type(
                    handle_type_model(
                        self.clone(),
                        type_tuple.elems
                            .iter()
                            .map(|ty| NestedArgument::Object(ty.visit_scope_type(source)))
                            .collect())),
            ty => ObjectKind::unknown_type(ty.clone())
        }
    }
}
impl<'a> VisitScopeType<'a> for Path {
    type Source = (&'a ScopeChain, &'a GlobalContext, Option<QSelfModel>);
    type Result = ObjectKind;

    fn visit_scope_type(&self, source: &Self::Source) -> Self::Result {
        let (scope, context, qself) = source;
        let new_qself = qself.as_ref().map(|q| q.qself.clone());
        let mut segments = self.segments.clone();
        let mut nested_arguments = Punctuated::new();
        for segment in &mut segments {
            match &mut segment.arguments {
                PathArguments::None => {}
                PathArguments::AngleBracketed(arguments) => {
                    arguments.args.iter_mut().for_each(|arg| match arg {
                        GenericArgument::Type(inner_type) => {
                            let obj_conversion = inner_type.visit_scope_type(&(scope, context));
                            if let Some(ty_model_kind) = obj_conversion.maybe_type_model_kind_ref() {
                                let new_inner_ty = ty_model_kind.as_type().clone();
                                nested_arguments.push(NestedArgument::Object(obj_conversion));
                                *arg = GenericArgument::Type(new_inner_ty);
                            }
                        },
                        _ => {}
                    });
                }
                PathArguments::Parenthesized(ParenthesizedGenericArguments { inputs, output, .. }) => {
                    inputs.iter_mut().for_each(|arg| {
                        let obj_conversion = arg.visit_scope_type(&(scope, context));
                        if let Some(ty_model_kind) = obj_conversion.maybe_type_model_kind_ref() {
                            let new_inner_ty = ty_model_kind.as_type().clone();
                            nested_arguments.push(NestedArgument::Object(obj_conversion));
                            *arg = new_inner_ty;
                        }
                    });

                    if let ReturnType::Type(_, ret) = output {
                        let obj_conversion = ret.visit_scope_type(&(scope, context));
                        if let Some(ty_model_kind) = obj_conversion.maybe_type_model_kind_ref() {
                            let new_inner_ty = ty_model_kind.as_type().clone();
                            nested_arguments.push(NestedArgument::Object(obj_conversion));
                            *ret = Box::new(new_inner_ty);
                        }
                    }
                }
            }
        }
        match (segments.first(), segments.last()) {
            (Some(PathSegment { ident: first_ident, .. }), Some(PathSegment { ident: last_ident, arguments: last_arguments })) => {
                let first_import_seg = first_ident.to_path();
                let mut last_import_seg = last_ident.to_path();
                if let Some(PathSegment { arguments, .. }) = last_import_seg.segments.last_mut() {
                    *arguments = last_arguments.clone();
                }
                if let Some((generics, bound)) = scope.maybe_generic_bound_for_path(&first_import_seg) {
                    nprint!(1, crate::formatter::Emoji::Local, "(Local Generic Bound) {}: {}", generics.to_token_stream(), bound.to_token_stream());
                    let generic_trait_bounds = |ty: &Type, ident_path: &Type, bounds: &AddPunctuated<TypeParamBound>| {
                        let mut has_bound = false;
                        bounds.iter().filter_map(|b| match b {
                            TypeParamBound::Trait(TraitBound { path, .. }) => {
                                let has = ident_path.eq(ty);
                                if !has_bound && has {
                                    has_bound = true;
                                }
                                has.then(|| path.visit_scope_type(source))
                            },
                            _ => None
                        }).collect()
                    };
                    let ident_path = Type::Path(TypePath { qself: None, path: Path::from(PathSegment::from(bound.ident.clone())) });
                    let ty: Type = first_import_seg.to_type();
                    let bounds = generic_trait_bounds(&ty, &ident_path, &bound.bounds);
                    let predicates = generics.where_clause
                        .as_ref()
                        .map(|where_clause|
                            where_clause.predicates
                                .iter()
                                .filter_map(|predicate| match predicate {
                                    WherePredicate::Type(PredicateType { bounded_ty, bounds, .. }) =>
                                        ty.eq(bounded_ty).then(||(bounded_ty.clone(), generic_trait_bounds(&ty, &bounded_ty, bounds))),
                                    _ => None
                                })
                                .collect())
                        .unwrap_or_default();
                    ObjectKind::bounds_type(GenericBoundsModel::new(ty, bounds, predicates, generics, nested_arguments))
                } else if let Some(mut import_path) = context.maybe_import_path_ref(scope, &first_import_seg).cloned() {
                    // Can be reevaluated after processing entire scope tree:
                    // Because import path can have multiple aliases and we need the most complete one to use mangling correctly
                    // We can also determine the type after processing entire scope (if one in fermented crate)
                    nprint!(1, crate::formatter::Emoji::Local, "(ScopeImport) {}", import_path.to_token_stream());
                    if import_path.is_crate_based() {
                        import_path.replace_first_with(&scope.crate_ident_as_path());
                    }
                    ObjectKind::imported_model_type(handle_type_path_model(new_qself, None, segments, nested_arguments), import_path)
                } else if let Some(generic_bounds) = context.generics.maybe_generic_bounds(scope, &first_ident.to_type()) {
                    // TODO: multiple bounds handling
                    if let Some(first_bound) = generic_bounds.first() {
                        let first_bound_as_scope = PathHolder::from(first_bound);
                        if let Some(Path { segments: new_segments, .. }) = context.maybe_import_path_ref(scope, &first_bound_as_scope.0) {
                            nprint!(1, crate::formatter::Emoji::Local, "(Generic Bounds Imported) {}", format_token_stream(&iseg));
                            segments.replace_last_with(new_segments);
                        } else {
                            nprint!(1, crate::formatter::Emoji::Local, "(Generic Bounds Local) {}", format_token_stream(&segments));
                            let new_segments = match &first_bound.segments.first() {
                                Some(PathSegment { ident, .. }) if matches!(ident.to_string().as_str(), "FnOnce" | "Fn" | "FnMut") =>
                                    parse_quote!(#scope::#ident),
                                _ =>
                                    parse_quote!(#scope::#first_bound)
                            };
                            segments.replace_last_with(&new_segments);
                        }
                    }
                    ObjectKind::unknown_model_type_path(new_qself, self.leading_colon, segments, nested_arguments)
                } else {
                    nprint!(1, crate::formatter::Emoji::Local, "(Local or Global ....) {}", segments.to_token_stream());
                    let obj_scope = scope.obj_root_chain().unwrap_or(scope);
                    let self_scope_path = obj_scope.self_path_holder_ref();
                    match first_ident.to_string().as_str() {
                        "Self" => if segments.len() <= 1 {
                            nprint!(1, crate::formatter::Emoji::Local, "(Self) {}", format_token_stream(first_ident));
                            segments.replace_last_with(&self_scope_path.0.segments);
                            ObjectKind::unknown_model_type_path(new_qself, self.leading_colon, segments, nested_arguments)
                        } else {
                            let tail = segments.crate_less();
                            let last_segment = segments.pop().unwrap();
                            let new_path: Path = parse_quote!(#self_scope_path::#tail);
                            nprint!(1, crate::formatter::Emoji::Local, "(SELF::->) {}: {}", format_token_stream(&last_segment), format_token_stream(&last_segment.clone().into_value().arguments));
                            if let Some(PathSegment { arguments, .. }) = segments.last_mut() {
                                *arguments = last_segment.into_value().arguments;
                            }
                            segments.clear();
                            segments.extend(new_path.segments);
                            scope.obj_root_model_composer()(handle_type_path_model(new_qself, self.leading_colon, segments, nested_arguments))
                        },
                        "Vec" =>
                            ObjectKind::group_type(GroupModelKind::Vec(handle_type_path_model(new_qself, self.leading_colon, segments, nested_arguments))),
                        "Result" if segments.len() == 1 =>
                            ObjectKind::group_type(GroupModelKind::Result(handle_type_path_model(new_qself, self.leading_colon, segments, nested_arguments))),
                        "Option" =>
                            ObjectKind::optional_model_type(handle_type_path_model(new_qself, self.leading_colon, segments, nested_arguments)),
                        _ if last_ident.to_string().eq("Map") && first_ident.to_string().eq("serde_json") =>
                            ObjectKind::object_model_type(handle_type_path_model(new_qself, self.leading_colon, segments, nested_arguments)),
                        _ if last_ident.is_map() =>
                            ObjectKind::group_type(GroupModelKind::Map(handle_type_path_model(new_qself, self.leading_colon, segments, nested_arguments))),
                        _ if last_ident.is_btree_set() =>
                            ObjectKind::group_type(GroupModelKind::BTreeSet(handle_type_path_model(new_qself, self.leading_colon, segments, nested_arguments))),
                        _ if last_ident.is_hash_set() =>
                            ObjectKind::group_type(GroupModelKind::HashSet(handle_type_path_model(new_qself, self.leading_colon, segments, nested_arguments))),
                        _ if last_ident.eq("IndexMap") =>
                            ObjectKind::group_type(GroupModelKind::IndexMap(handle_type_path_model(new_qself, self.leading_colon, segments, nested_arguments))),
                        _ if last_ident.eq("IndexSet") =>
                            ObjectKind::group_type(GroupModelKind::IndexSet(handle_type_path_model(new_qself, self.leading_colon, segments, nested_arguments))),
                        _ if first_ident.is_box() =>
                            ObjectKind::smart_ptr_type(SmartPointerModelKind::Box(handle_type_path_model(new_qself, self.leading_colon, segments, nested_arguments))),
                        _ if first_ident.is_cow() =>
                            ObjectKind::non_primitive_fermentable_type(DictFermentableModelKind::Cow(handle_type_path_model(new_qself, self.leading_colon, segments, nested_arguments))),
                        _ if first_ident.is_primitive() =>
                            ObjectKind::primitive_type(first_ident.to_type()),
                        _ if first_ident.eq("i128") =>
                            ObjectKind::i128_type(first_ident.to_type()),
                        _ if first_ident.eq("u128") =>
                            ObjectKind::u128_type(first_ident.to_type()),
                        _ if first_ident.is_special_std_trait() =>
                            ObjectKind::unknown_type(last_import_seg.to_type()),
                        _ if first_ident.is_str() =>
                            ObjectKind::str_type(last_import_seg.to_type()),
                        _ if first_ident.is_string() =>
                            ObjectKind::string_type(last_import_seg.to_type()),
                        _ if first_ident.is_lambda_fn() =>
                            ObjectKind::lambda_fn_model_type(handle_type_path_model(new_qself, self.leading_colon, segments, nested_arguments)),
                        _ => {
                            let obj_parent_scope = obj_scope.parent_scope();
                            let len = segments.len();
                            if len == 1 {
                                nprint!(1, crate::formatter::Emoji::Local, "(Local join single (has {} parent scope): {}) {} + {}", obj_parent_scope.is_some().then_some("some").unwrap_or("no"), first_ident, scope, self.to_token_stream());
                                segments.replace_last_with(&match obj_parent_scope {
                                    // Global
                                    None if scope.is_crate_root() => {
                                        let scope = scope.crate_ident_ref();
                                        parse_quote!(#scope::#self)
                                    }
                                    None => parse_quote!(#scope::#self),
                                    Some(parent) => {
                                        let scope = parent.self_path_holder_ref();
                                        parse_quote!(#scope::#self)
                                    }
                                });
                                ObjectKind::unknown_model_type_path(new_qself, self.leading_colon, segments, nested_arguments)

                            } else if let Some(QSelfModel { qs: _, qself: QSelf { ty, .. } }) = qself.as_ref() {
                                nprint!(1, crate::formatter::Emoji::Local, "(Local join QSELF: {} [{}]) {} + {}", format_token_stream(ty), format_token_stream(&import_seg), format_token_stream(scope), format_token_stream(self));
                                let tt = context.maybe_scope_import_path(scope, &first_import_seg)
                                    .or_else(|| context.maybe_scope_import_path(obj_scope, &first_import_seg))
                                    .or_else(|| obj_parent_scope.and_then(|parent_scope| context.maybe_scope_import_path(parent_scope, &first_import_seg)))
                                    .cloned()
                                    .unwrap_or_else(|| obj_parent_scope.unwrap_or(scope).self_path_holder_ref().joined_path(first_import_seg));

                                scope.obj_root_model_composer()(handle_type_model(Type::Path(match len {
                                    0 => parse_quote!(<#ty as #tt>),
                                    _ => {
                                        let tail = segments.crate_less();
                                        parse_quote!(<#ty as #tt>::#tail)
                                    }
                                }), nested_arguments))

                            } else {
                                ObjectKind::unknown_model_type_path(new_qself, self.leading_colon.clone(), self.segments.clone(), nested_arguments)
                            }
                        },
                    }
                }
            },
            _ => ObjectKind::Empty
        }

    }
}

impl<'a> VisitScopeType<'a> for Option<QSelf> {
    type Source = (&'a ScopeChain, &'a GlobalContext);
    type Result = Option<QSelfModel>;

    fn visit_scope_type(&self, source: &Self::Source) -> Self::Result {
        self.as_ref().and_then(|qself| qself.ty.visit_scope_type(source).maybe_type_model_kind_ref().map(|type_model_kind| {
            let qs = type_model_kind.type_model_ref().clone();
            let mut new_qself = qself.clone();
            new_qself.ty = Box::new(qs.as_type().clone());
            QSelfModel { qs, qself: new_qself }
        }))
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

impl<'a> VisitScopeType<'a> for TypeBareFn {
    type Source = (&'a ScopeChain, &'a GlobalContext);
    type Result = ObjectKind;

    fn visit_scope_type(&self, source: &Self::Source) -> Self::Result {
        let mut nested = self.inputs.iter().map(|BareFnArg { ty, .. }| NestedArgument::Object(ty.visit_scope_type(source))).collect::<CommaPunctuated<_>>();
        if let ReturnType::Type(_, ty) = &self.output {
            nested.push(NestedArgument::Object(ty.visit_scope_type(source)))
        }
        ObjectKind::fn_pointer_model_type(handle_type_model(Type::BareFn(self.clone()), nested))
    }
}
impl<'a> VisitScopeType<'a> for TypeReference {
    type Source = (&'a ScopeChain, &'a GlobalContext);
    type Result = ObjectKind;
    fn visit_scope_type(&self, source: &Self::Source) -> Self::Result {
        let mut obj = self.elem.visit_scope_type(source);
        if let ObjectKind::Type(tyc) | ObjectKind::Item(tyc, _) = &mut obj {
            tyc.replace_model_type(Type::Reference(TypeReference {
                and_token: Default::default(),
                lifetime: self.lifetime.clone(),
                mutability: self.mutability.clone(),
                elem: Box::new(tyc.to_type()),
            }));
        }
        obj
    }
}
impl<'a> VisitScopeType<'a> for TypePtr {
    type Source = (&'a ScopeChain, &'a GlobalContext);
    type Result = ObjectKind;
    fn visit_scope_type(&self, source: &Self::Source) -> Self::Result {
        let mut obj = self.elem.visit_scope_type(source);
        if let ObjectKind::Type(tyc) | ObjectKind::Item(tyc, _) = &mut obj {
            let ty = match &tyc {
                TypeModelKind::Imported(ty, import_path) => {
                    let ty = ty.as_type();
                    let path = import_path.popped();
                    parse_quote!(#path::#ty)
                },
                _ => tyc.to_type()
            };
            match (self.const_token, self.mutability) {
                (Some(..), _) =>
                    tyc.replace_model_type(ty.joined_const()),
                (_, Some(..)) =>
                    tyc.replace_model_type(ty.joined_mut()),
                _ => {}
            }
        }
        obj
    }
}

impl<'a> VisitScopeType<'a> for TypeImplTrait {
    type Source = (&'a ScopeChain, &'a GlobalContext);
    type Result = ObjectKind;

    fn visit_scope_type(&self, source: &Self::Source) -> Self::Result {
        let (scope, context) = source;
        let TypeImplTrait { impl_token, bounds } = self;
        let mut bounds = bounds.clone();
        let mut nested_arguments = CommaPunctuatedNestedArguments::new();
        bounds.iter_mut().for_each(|bound| match bound {
            TypeParamBound::Trait(TraitBound { path, .. }) => {
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
            },
            _ => {},
        });
        // TODO: make it Unknown
        ObjectKind::unknown_model_type(handle_type_model(Type::ImplTrait(TypeImplTrait { impl_token: impl_token.clone(), bounds }), nested_arguments))
    }
}

impl<'a> VisitScopeType<'a> for TypeTraitObject {
    type Source = (&'a ScopeChain, &'a GlobalContext);
    type Result = ObjectKind;

    fn visit_scope_type(&self, source: &Self::Source) -> Self::Result {
        let (scope, context) = source;
        let TypeTraitObject { dyn_token, bounds } = self;
        let mut bounds = bounds.clone();
        let mut nested_arguments = CommaPunctuatedNestedArguments::new();
        bounds.iter_mut().for_each(|bound| match bound {
            TypeParamBound::Trait(TraitBound { path, .. }) => {
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
            },
            _ => {},
        });
        // TODO: make it Unknown
        ObjectKind::unknown_model_type(handle_type_model(Type::TraitObject(TypeTraitObject { dyn_token: dyn_token.clone(), bounds }), nested_arguments))
    }
}

pub fn handle_type_model(ty: Type, nested_arguments: CommaPunctuatedNestedArguments) -> TypeModel {
    TypeModel::new_nested(ty, nested_arguments)
}
pub fn handle_type_path_model(qself: Option<QSelf>, leading_colon: Option<PathSep>, segments: Punctuated<PathSegment, PathSep>, nested_arguments: CommaPunctuatedNestedArguments) -> TypeModel {
    handle_type_model(Type::Path(TypePath { qself, path: Path { leading_colon, segments }}), nested_arguments)
}
