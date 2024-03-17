use quote::{format_ident, quote, ToTokens};
use syn::{GenericArgument, parse_quote, Path, PathArguments, PathSegment, QSelf, TraitBound, Type, TypeParamBound, TypePath, TypeTraitObject, TypeTuple};
use syn::punctuated::Punctuated;
use syn::token::Colon2;
use crate::composition::{QSelfComposition, TypeComposition};
use crate::context::{GlobalContext, ScopeChain};
use crate::conversion::{ObjectConversion, TypeCompositionConversion};
use crate::formatter::{Emoji, format_token_stream};
use crate::holder::PathHolder;
use crate::nprint;

pub trait VisitScopeType<'a> where Self: Sized + 'a {
    type Source;
    type Result;
    fn update_nested_generics(&self, source: &Self::Source) -> Self::Result;
}

impl<'a> VisitScopeType<'a> for Type {
    type Source = (&'a ScopeChain, &'a GlobalContext);
    type Result = ObjectConversion;

    fn update_nested_generics(&self, source: &Self::Source) -> Self::Result {
        nprint!(1, Emoji::Node, "=== {} [{:?}]", format_token_stream(self), self);
        match self {
            Type::Path(type_path) => type_path.update_nested_generics(source),
            Type::TraitObject(type_trait_object) => type_trait_object.update_nested_generics(source),
            Type::Tuple(type_tuple) => type_tuple.update_nested_generics(source),
            tttt =>
                ObjectConversion::Type(TypeCompositionConversion::Unknown(TypeComposition::new(tttt.clone(), None)))
        }
    }
}

impl<'a> VisitScopeType<'a> for Path {
    type Source = (&'a ScopeChain, &'a GlobalContext, Option<QSelfComposition>);
    type Result = ObjectConversion;

    fn update_nested_generics(&self, source: &Self::Source) -> Self::Result {
        let (scope, context, qself) = source;
        let new_qself = qself.as_ref().map(|q| q.qself.clone());
        let mut segments = self.segments.clone();
        for segment in &mut segments {
            if let PathArguments::AngleBracketed(angle_bracketed_generic_arguments) = &mut segment.arguments {
                for arg in &mut angle_bracketed_generic_arguments.args {
                    match arg {
                        GenericArgument::Type(inner_type) => {
                            let obj_conversion = inner_type.update_nested_generics(&(scope, context));
                            *arg = GenericArgument::Type(obj_conversion.ty().cloned().unwrap())
                        },
                        _ => {}
                    }
                }
            }
        }
        let first_segment = &segments.first().unwrap();
        let first_ident = &first_segment.ident;
        let last_segment = &segments.last().unwrap();
        let last_ident = &last_segment.ident;
        let import_seg: PathHolder = parse_quote!(#first_ident);

        let obj_scope = scope.obj_root_chain().unwrap_or(scope);
        let object_self_scope = obj_scope.self_scope();
        if let Some(dict_type_composition) = scope.maybe_dictionary_type(&import_seg.0) {
            ObjectConversion::Type(dict_type_composition)
        } else if let Some(bounds_composition) = scope.maybe_generic_bound_for_path(&import_seg.0) {
            nprint!(1, Emoji::Local, "(Local Generic Bound) {}", bounds_composition);
            ObjectConversion::Type(TypeCompositionConversion::Bounds(bounds_composition))
        } else if let Some(replacement_path) = context.maybe_import(scope, &import_seg).cloned() {
            let last_segment = segments.pop().unwrap();
            if format_ident!("crate").eq(&replacement_path.segments.first().unwrap().ident) /*&& !lock.config.current_crate.ident().eq(crate_scope)*/ {
                nprint!(1, Emoji::Local, "(ScopeImport Local) {}", format_token_stream(&replacement_path));
                let crate_scope = scope.crate_scope();
                let replaced: Vec<_> = replacement_path.segments.iter().skip(1).collect();
                let mut new_path: Path = parse_quote!(#crate_scope::#(#replaced)::*);
                new_path.segments.last_mut().unwrap().arguments = last_segment.into_value().arguments;
                ObjectConversion::Type(
                    TypeCompositionConversion::Unknown(
                        TypeComposition::new(
                            Type::Path(
                                TypePath {
                                    qself: new_qself,
                                    path: new_path }),
                            None)))
            } else {
                nprint!(1, Emoji::Local, "(ScopeImport External) {}", format_token_stream(&replacement_path));
                segments.extend(replacement_path.segments.clone());
                segments.last_mut().unwrap().arguments = last_segment.into_value().arguments;
                ObjectConversion::Type(
                    TypeCompositionConversion::Unknown(
                        TypeComposition::new(
                            Type::Path(
                                TypePath {
                                    qself: new_qself,
                                    path: Path { leading_colon: self.leading_colon, segments } }),
                            None)))
            }

        } else if let Some(generic_bounds) = context.generics.maybe_generic_bounds(scope, &import_seg) {
            if let Some(first_bound) = generic_bounds.first() {
                let first_bound_as_scope = PathHolder::from(first_bound);
                if let Some(imported) = context.maybe_import(scope, &first_bound_as_scope).cloned() {
                    nprint!(1, Emoji::Local, "(Generic Bounds Imported) {}", format_token_stream(&segments));
                    let last_segment = segments.pop().unwrap();
                    let new_segments = imported.segments.clone();
                    segments.extend(new_segments);
                    segments.last_mut().unwrap().arguments = last_segment.into_value().arguments;
                } else {
                    nprint!(1, Emoji::Local, "(Generic Bounds Local) {}", format_token_stream(&segments));
                    let first_bound_ident = &first_bound.segments.first().unwrap().ident;

                    if matches!(first_bound_ident.to_string().as_str(), "FnOnce" | "Fn" | "FnMut") {
                        let last_segment = segments.pop().unwrap();
                        let scope = scope.self_path_holder();
                        let new_segments: Punctuated<PathSegment, Colon2> = parse_quote!(#scope::#first_bound_ident);

                        segments.extend(new_segments);
                        segments.last_mut().unwrap().arguments = last_segment.into_value().arguments;

                    } else {
                        let last_segment = segments.pop().unwrap();
                        let scope = scope.self_path_holder();
                        let new_segments: Punctuated<PathSegment, Colon2> = parse_quote!(#scope::#first_bound);
                        segments.extend(new_segments);
                        segments.last_mut().unwrap().arguments = last_segment.into_value().arguments;
                    }
                }
                ObjectConversion::Type(
                    TypeCompositionConversion::TraitType(
                        TypeComposition::new(Type::Path(TypePath { qself: new_qself, path: Path { leading_colon: self.leading_colon, segments } }),
                                             None)))
            } else {
                ObjectConversion::Type(
                    TypeCompositionConversion::TraitType(
                        TypeComposition::new(Type::Path(TypePath { qself: new_qself, path: Path { leading_colon: self.leading_colon, segments } }),
                                             None)))

            }
            // let first_bound = generic_bounds.first().unwrap();

        } else {
            // if let Some(same_mod_defined_obj) = lock.mayb
            nprint!(1, Emoji::Local, "(Local or Global ....) {}", segments.to_token_stream());

            let self_scope_path = &object_self_scope.self_scope;
            match first_ident.to_string().as_str() {
                "Self" if segments.len() <= 1 => {
                    nprint!(1, Emoji::Local, "(Self) {}", format_token_stream(first_ident));
                    let last_segment = segments.pop().unwrap();
                    let new_segments: Punctuated<PathSegment, Colon2> = parse_quote!(#self_scope_path);
                    println!("::: new_segments: {} ", new_segments.to_token_stream());
                    segments.extend(new_segments);
                    segments.last_mut().unwrap().arguments = last_segment.into_value().arguments;
                    println!("::: add_obj_self: {} scope: [{}]", object_self_scope, scope);
                    // object_self_scope.object.clone()

                    ObjectConversion::Type(TypeCompositionConversion::Unknown(TypeComposition::new(
                        Type::Path(
                            TypePath {
                                qself: new_qself,
                                path: Path { leading_colon: self.leading_colon, segments } }),
                        None)))

                },
                "Self" => {
                    let tail = segments.iter().skip(1).cloned().collect::<Vec<_>>();
                    let last_segment = segments.pop().unwrap();
                    nprint!(1, Emoji::Local, "(SELF::->) {}: {}", format_token_stream(&last_segment), format_token_stream(&last_segment.clone().into_value().arguments));
                    let new_path: Path = parse_quote!(#self_scope_path::#(#tail)::*);
                    segments.last_mut().unwrap().arguments = last_segment.into_value().arguments;
                    segments.clear();
                    segments.extend(new_path.segments);

                    match scope.obj_root_chain() {
                        Some(ScopeChain::Object { .. } | ScopeChain::Impl { .. }) =>
                            ObjectConversion::Type(
                                TypeCompositionConversion::Object(
                                    TypeComposition::new(
                                        Type::Path(TypePath { qself: new_qself, path: Path { leading_colon: self.leading_colon, segments } }),
                                        None))),
                        Some(ScopeChain::Trait { .. }) =>
                            ObjectConversion::Type(
                                TypeCompositionConversion::TraitType(
                                    TypeComposition::new(
                                        Type::Path(TypePath { qself: new_qself, path: Path { leading_colon: self.leading_colon, segments } }),
                                        None))),
                        _ => panic!("Unexpected scope obj root chain")
                    }

                },
                "Vec" => {
                    ObjectConversion::Type(TypeCompositionConversion::Object(TypeComposition::new(
                        Type::Path(
                            TypePath {
                                qself: new_qself,
                                path: Path { leading_colon: self.leading_colon, segments } }),
                        None)))
                    // nprint!(*counter, Emoji::Nothing, "(Global Object) {}", format_token_stream(&path));
                },
                "Option" => {
                    ObjectConversion::Type(TypeCompositionConversion::Object(TypeComposition::new(
                        Type::Path(
                            TypePath {
                                qself: new_qself,
                                path: Path { leading_colon: self.leading_colon, segments } }),
                        None)))
                },
                "Result" if segments.len() == 1 => {
                    ObjectConversion::Type(TypeCompositionConversion::Object(TypeComposition::new(
                        Type::Path(
                            TypePath {
                                qself: new_qself,
                                path: Path { leading_colon: self.leading_colon, segments } }),
                        None)))
                },
                _ if matches!(last_ident.to_string().as_str(), "BTreeMap" | "HashMap") => {
                    ObjectConversion::Type(TypeCompositionConversion::Object(TypeComposition::new(
                        Type::Path(
                            TypePath {
                                qself: new_qself,
                                path: Path { leading_colon: self.leading_colon, segments } }),
                        None)))
                },
                _ if matches!(first_ident.to_string().as_str(), "FnOnce" | "Fn" | "FnMut") => {
                    ObjectConversion::Type(TypeCompositionConversion::Object(TypeComposition::new(
                        Type::Path(
                            TypePath {
                                qself: new_qself,
                                path: Path { leading_colon: self.leading_colon, segments } }),
                        None)))

                },
                _ => {
                    let obj_parent_scope = obj_scope.parent_scope();
                    let len = segments.len();
                    if len == 1 {
                        nprint!(1, Emoji::Local, "(Local join single (has {} parent scope): {}) {} + {}",
                            if obj_parent_scope.is_some() { "some" } else { "no" },
                            first_ident,
                            scope,
                            self.to_token_stream());
                        let last_segment = segments.pop().unwrap();
                        let new_segments: Punctuated<PathSegment, Colon2> = match obj_parent_scope {
                            None => {
                                // Global
                                if scope.is_crate_root() {
                                    let scope = scope.crate_scope();
                                    parse_quote!(#scope::#self)
                                } else {
                                    parse_quote!(#scope::#self)
                                }
                            },
                            Some(parent) => {
                                let scope = parent.self_path_holder();
                                // nprint!(1, Emoji::Local, "(Local join single (has parent scope): {}) {} + {}", first_ident, scope, format_token_stream(&path));
                                parse_quote!(#scope::#self)
                            }
                        };
                        segments.extend(new_segments);
                        segments.last_mut().unwrap().arguments = last_segment.into_value().arguments;
                        ObjectConversion::Type(TypeCompositionConversion::Unknown(TypeComposition::new(
                            Type::Path(
                                TypePath {
                                    qself: new_qself,
                                    path: Path { leading_colon: self.leading_colon, segments } }),
                            None)))

                    } else {
                        let tail: Vec<_> = segments.iter().skip(1).cloned().collect();
                        if let Some(QSelfComposition { qs: _, qself: QSelf { ty, .. } }) = qself.as_ref() {
                            nprint!(1, Emoji::Local, "(Local join QSELF: {} [{}]) {} + {}", format_token_stream(ty), format_token_stream(&import_seg), format_token_stream(scope), format_token_stream(self));

                            println!("------ import local? {} in [{}]", import_seg.to_token_stream(), scope);
                            println!("------ import parent? {} in [{:?}]", import_seg.to_token_stream(), scope.parent_scope());
                            println!("------ import object? {} in [{:?}]", import_seg.to_token_stream(), obj_scope);
                            println!("------ import object parent? {} in [{:?}]", import_seg.to_token_stream(), obj_parent_scope);

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
                            let tail_path = quote!(#(#tail)::*);
                            // println!("{}: <{} as {}>::{}", tail.len(), format_token_stream(ty), format_token_stream(&tt), format_token_stream(&tail_path));
                            match scope.obj_root_chain() {
                                Some(ScopeChain::Trait { .. }) =>
                                    ObjectConversion::Type(TypeCompositionConversion::TraitType(TypeComposition {
                                        ty: match len {
                                            0 => parse_quote!(<#ty as #tt>),
                                            _ => parse_quote!(<#ty as #tt>::#tail_path)
                                        },
                                        generics: None,
                                    })),
                                Some(ScopeChain::Object { .. } | ScopeChain::Impl { .. }) =>
                                    ObjectConversion::Type(TypeCompositionConversion::Object(TypeComposition {
                                        ty: match len {
                                            0 => parse_quote!(<#ty as #tt>),
                                            _ => parse_quote!(<#ty as #tt>::#tail_path)
                                        },
                                        generics: None,
                                    })),
                                _ => ObjectConversion::Type(TypeCompositionConversion::Unknown(TypeComposition {
                                    ty: match len {
                                        0 => parse_quote!(<#ty as #tt>),
                                        _ => parse_quote!(<#ty as #tt>::#tail_path)
                                    },
                                    generics: None,
                                }))
                            }
                        } else {
                            nprint!(1, Emoji::Local, "(Local join multi: {}) {} + {}", first_ident, format_token_stream(scope), format_token_stream(self));
                            let last_segment = segments.last().cloned().unwrap();
                            let new_segments: Punctuated<PathSegment, Colon2> = if self.leading_colon.is_none() {
                                parse_quote!(#scope::#self)
                            } else {
                                parse_quote!(#scope #self)
                            };
                            segments.clear();
                            segments.extend(new_segments);
                            segments.last_mut().unwrap().arguments = last_segment.arguments;

                            ObjectConversion::Type(TypeCompositionConversion::Unknown(TypeComposition::new(
                                Type::Path(
                                    TypePath {
                                        qself: new_qself,
                                        path: Path { leading_colon: self.leading_colon, segments } }),
                                None)))
                        }
                    }
                },
            }
        }
    }
}

impl<'a> VisitScopeType<'a> for Option<QSelf> {
    type Source = (&'a ScopeChain, &'a GlobalContext);
    type Result = Option<QSelfComposition>;

    fn update_nested_generics(&self, source: &Self::Source) -> Self::Result {
        self.as_ref().map(|qself| {
            let mut new_qself = qself.clone();
            let qs = qself.ty.update_nested_generics(source);
            let qs = qs.type_conversion().unwrap().ty_composition().clone();
            new_qself.ty = qs.ty.clone().into();
            QSelfComposition { qs, qself: new_qself }
        })
    }
}

impl<'a> VisitScopeType<'a> for TypePath {
    type Source = (&'a ScopeChain, &'a GlobalContext);
    type Result = ObjectConversion;

    fn update_nested_generics(&self, source: &Self::Source) -> Self::Result {
        let (scope, context) = source;
        self.path.update_nested_generics(&(scope, context, self.qself.update_nested_generics(source)))
    }
}

impl<'a> VisitScopeType<'a> for TypeTraitObject {
    type Source = (&'a ScopeChain, &'a GlobalContext);
    type Result = ObjectConversion;

    fn update_nested_generics(&self, source: &Self::Source) -> Self::Result {
        let (scope, context) = source;
        let TypeTraitObject { dyn_token, bounds } = self;
        let mut bounds = bounds.clone();
        bounds.iter_mut().for_each(|bound| match bound {
            TypeParamBound::Trait(TraitBound { path, .. }) => {
                let full_path = path.update_nested_generics(&(scope, context, None));
                *path = parse_quote!(#full_path);
            },
            _ => {},
        });
        ObjectConversion::Type(TypeCompositionConversion::TraitType(TypeComposition::new(Type::TraitObject(TypeTraitObject {
            dyn_token: dyn_token.clone(),
            bounds
        }), None)))

    }
}
impl<'a> VisitScopeType<'a> for TypeTuple {
    type Source = (&'a ScopeChain, &'a GlobalContext);
    type Result = ObjectConversion;

    fn update_nested_generics(&self, source: &Self::Source) -> Self::Result {
        let elems = self.elems.iter().filter_map(|ty| ty.update_nested_generics(source).ty().cloned()).collect();
        ObjectConversion::Type(
            TypeCompositionConversion::Tuple(
                TypeComposition::new(Type::Tuple(TypeTuple { paren_token: Default::default(), elems }), None)))
    }
}




