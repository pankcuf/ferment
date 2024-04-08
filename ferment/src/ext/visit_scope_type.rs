use quote::ToTokens;
use syn::{GenericArgument, parse_quote, Path, PathArguments, QSelf, TraitBound, Type, TypeParamBound, TypePath, TypeTraitObject, TypeTuple};
use syn::punctuated::Punctuated;
use syn::token::Comma;
use crate::composition::{NestedArgument, QSelfComposition, TypeComposition};
use crate::context::{GlobalContext, ScopeChain};
use crate::conversion::{ObjectConversion, TypeCompositionConversion};
use crate::ext::{CrateExtension, ToPath};
use crate::formatter::{Emoji, format_token_stream};
use crate::holder::PathHolder;
use crate::nprint;

pub trait ToObjectConversion {
    fn to_unknown(self, nested_arguments: Punctuated<NestedArgument, Comma>) -> ObjectConversion;
    fn to_object(self, nested_arguments: Punctuated<NestedArgument, Comma>) -> ObjectConversion;
    fn to_trait(self, nested_arguments: Punctuated<NestedArgument, Comma>) -> ObjectConversion;

    // fn to_import(self) -> ObjectConversion;
}

impl ToObjectConversion for Type {
    fn to_unknown(self, nested_arguments: Punctuated<NestedArgument, Comma>) -> ObjectConversion {
        ObjectConversion::Type(TypeCompositionConversion::Unknown(handle_type_composition(self, nested_arguments)))
    }

    fn to_object(self, nested_arguments: Punctuated<NestedArgument, Comma>) -> ObjectConversion {
        ObjectConversion::Type(TypeCompositionConversion::Object(handle_type_composition(self, nested_arguments)))
    }

    fn to_trait(self, nested_arguments: Punctuated<NestedArgument, Comma>) -> ObjectConversion {
        ObjectConversion::Type(TypeCompositionConversion::TraitType(handle_type_composition(self, nested_arguments)))
    }

    // fn to_import(self) -> ObjectConversion {
    //     ObjectConversion::Type(TypeCompositionConversion::Imported(handle_type_composition(self)))
    // }
}

impl ToObjectConversion for TypePath {
    fn to_unknown(self, nested_arguments: Punctuated<NestedArgument, Comma>) -> ObjectConversion {
        ObjectConversion::Type(TypeCompositionConversion::Unknown(handle_type_path_composition(self, nested_arguments)))
    }

    fn to_object(self, nested_arguments: Punctuated<NestedArgument, Comma>) -> ObjectConversion {
        ObjectConversion::Type(TypeCompositionConversion::Object(handle_type_path_composition(self, nested_arguments)))
    }

    fn to_trait(self, nested_arguments: Punctuated<NestedArgument, Comma>) -> ObjectConversion {
        ObjectConversion::Type(TypeCompositionConversion::TraitType(handle_type_path_composition(self, nested_arguments)))
    }

    // fn to_import(self) -> ObjectConversion {
    //     ObjectConversion::Type(TypeCompositionConversion::Imported(handle_type_path_composition(self)))
    // }
}

pub trait VisitScopeType<'a> where Self: Sized + 'a {
    type Source;
    type Result;
    fn update_nested_generics(&self, source: &Self::Source) -> Self::Result;
}

impl<'a> VisitScopeType<'a> for Type {
    type Source = (&'a ScopeChain, &'a GlobalContext);
    type Result = ObjectConversion;

    fn update_nested_generics(&self, source: &Self::Source) -> Self::Result {
        nprint!(1, Emoji::Node, "=== {} [{:?}]", self.to_token_stream(), self);
        match self {
            Type::Path(type_path) => type_path.update_nested_generics(source),
            Type::TraitObject(type_trait_object) => type_trait_object.update_nested_generics(source),
            Type::Tuple(type_tuple) => type_tuple.update_nested_generics(source),
            ty => ty.clone().to_unknown(Punctuated::new())
        }
    }
}
fn handle_type_composition(ty: Type, nested_arguments: Punctuated<NestedArgument, Comma>) -> TypeComposition {
    TypeComposition::new(ty, None, nested_arguments)
}
fn handle_type_path_composition(type_path: TypePath, nested_arguments: Punctuated<NestedArgument, Comma>) -> TypeComposition {
    TypeComposition::new(Type::Path(type_path), None, nested_arguments)
}

impl<'a> VisitScopeType<'a> for Path {
    type Source = (&'a ScopeChain, &'a GlobalContext, Option<QSelfComposition>);
    type Result = ObjectConversion;

    fn update_nested_generics(&self, source: &Self::Source) -> Self::Result {
        // println!("PAth: update_nested_generics {}", self.to_token_stream());
        let (scope, context, qself) = source;
        let new_qself = qself.as_ref().map(|q| q.qself.clone());
        let mut segments = self.segments.clone();
        let mut nested_arguments = Punctuated::new();
        for segment in &mut segments {
            if let PathArguments::AngleBracketed(angle_bracketed_generic_arguments) = &mut segment.arguments {
                for arg in &mut angle_bracketed_generic_arguments.args {
                    // println!("PAth: update_nested_generics.2222 {}", arg.to_token_stream());
                    match arg {
                        GenericArgument::Type(inner_type) => {
                            let obj_conversion = inner_type.update_nested_generics(&(scope, context));
                            let ty = obj_conversion.to_ty().unwrap();
                            nested_arguments.push(NestedArgument::Object(obj_conversion));
                            *arg = GenericArgument::Type(ty)
                        },
                        _ => {}
                    }
                }
            }
        }
        let first_segment = segments.first().unwrap();
        let first_ident = &first_segment.ident;
        let last_segment = segments.last().unwrap();
        let last_ident = &last_segment.ident;
        // let last_ident_str = last_ident.to_string().as_str();
        let import_seg: PathHolder = parse_quote!(#first_ident);

        if let Some(dict_type_composition) = scope.maybe_dictionary_type(&import_seg.0, context) {
            nprint!(1, Emoji::Local, "(Dictionary Type) {}", dict_type_composition);
            ObjectConversion::Type(dict_type_composition)
        } else if let Some(bounds_composition) = scope.maybe_generic_bound_for_path(&import_seg.0) {
            nprint!(1, Emoji::Local, "(Local Generic Bound) {}", bounds_composition);
            ObjectConversion::Type(TypeCompositionConversion::Bounds(bounds_composition))
        } else if let Some(mut import_path) = context.maybe_import(scope, &import_seg).cloned() {
            // Can be reevaluated after processing entire scope tree:
            // Because import path can have multiple aliases and we need the most complete one to use mangling correctly
            // We can also determine the type after processing entire scope (if one in fermented crate)
            nprint!(1, Emoji::Local, "(ScopeImport) {}", format_token_stream(&import_path));
            if import_path.is_crate_based() {
                import_path.replace_first_with(&scope.crate_ident_as_path());
            }

            ObjectConversion::Type(TypeCompositionConversion::Imported(TypeComposition::new(Type::Path(TypePath { qself: new_qself, path: segments.to_path() }), None, nested_arguments), import_path))
            /*
            let last_segment = segments.pop().unwrap();
            let import_path = if import_path.is_crate_based() {
                nprint!(1, Emoji::Local, "(ScopeImport Local) {}", format_token_stream(&import_path));
                let path = import_path.replaced_first_with_ident(&scope.crate_ident_as_path());
                parse_quote!(#path)
            } else {
                nprint!(1, Emoji::Local, "(ScopeImport External) {}", format_token_stream(&import_path));
                segments.extend(import_path.segments.clone());
                parse_quote!(#segments)
            };
            let mut full_import_path = context.ensure_full_import(import_path);
            full_import_path.segments.last_mut().unwrap().arguments = last_segment.into_value().arguments;
            TypePath { qself: new_qself, path: full_import_path }
                .to_unknown()
            */

        } else if let Some(generic_bounds) = context.generics.maybe_generic_bounds(scope, &import_seg) {
            if let Some(first_bound) = generic_bounds.first() {
                let first_bound_as_scope = PathHolder::from(first_bound);
                let new_segments = if let Some(Path { segments, .. }) = context.maybe_import(scope, &first_bound_as_scope).cloned() {
                    nprint!(1, Emoji::Local, "(Generic Bounds Imported) {}", format_token_stream(&segments));
                    segments
                } else {
                    nprint!(1, Emoji::Local, "(Generic Bounds Local) {}", format_token_stream(&segments));
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
            TypePath { qself: new_qself, path: Path { leading_colon: self.leading_colon, segments } }
                .to_trait(nested_arguments)
        } else {
            nprint!(1, Emoji::Local, "(Local or Global ....) {}", segments.to_token_stream());
            let obj_scope = scope.obj_root_chain().unwrap_or(scope);
            let object_self_scope = obj_scope.self_scope();
            let self_scope_path = &object_self_scope.self_scope;
            match first_ident.to_string().as_str() {
                "Self" if segments.len() <= 1 => {
                    nprint!(1, Emoji::Local, "(Self) {}", format_token_stream(first_ident));
                    segments.replace_last_with(&self_scope_path.0.segments);
                    TypePath { qself: new_qself, path: Path { leading_colon: self.leading_colon, segments } }
                        .to_unknown(nested_arguments)
                },
                "Self" => {
                    let tail = segments.crate_less();
                    let last_segment = segments.pop().unwrap();
                    let new_path: Path = parse_quote!(#self_scope_path::#tail);
                    nprint!(1, Emoji::Local, "(SELF::->) {}: {}", format_token_stream(&last_segment), format_token_stream(&last_segment.clone().into_value().arguments));
                    segments.last_mut().unwrap().arguments = last_segment.into_value().arguments;
                    // TODO: why clear ????
                    segments.clear();
                    segments.extend(new_path.segments);

                    match scope.obj_root_chain() {
                        Some(ScopeChain::Object { .. } | ScopeChain::Impl { .. }) =>
                            TypePath { qself: new_qself, path: Path { leading_colon: self.leading_colon, segments } }
                                .to_object(nested_arguments),
                        Some(ScopeChain::Trait { .. }) =>
                            TypePath { qself: new_qself, path: Path { leading_colon: self.leading_colon, segments } }
                                .to_trait(nested_arguments),
                        _ => panic!("Unexpected scope obj root chain")
                    }

                },
                "Vec" | "Option" | "Result" if segments.len() == 1 => {
                    //println!("update_nested_generics (Vec): {}: {}", segments.to_token_stream(), nested_arguments.to_token_stream());
                    TypePath { qself: new_qself, path: Path { leading_colon: self.leading_colon, segments } }
                        .to_object(nested_arguments)
                },
                _ if matches!(last_ident.to_string().as_str(), "BTreeMap" | "HashMap") => {
                    TypePath { qself: new_qself, path: Path { leading_colon: self.leading_colon, segments } }
                        .to_object(nested_arguments)
                },
                _ if matches!(first_ident.to_string().as_str(), "FnOnce" | "Fn" | "FnMut") => {
                    TypePath { qself: new_qself, path: Path { leading_colon: self.leading_colon, segments } }
                        .to_trait(nested_arguments)
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
                        segments.replace_last_with(&match obj_parent_scope {
                            None => {
                                // Global
                                if scope.is_crate_root() {
                                    let scope = scope.crate_ident();
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

impl<'a> VisitScopeType<'a> for TypeTuple {
    type Source = (&'a ScopeChain, &'a GlobalContext);
    type Result = ObjectConversion;

    fn update_nested_generics(&self, source: &Self::Source) -> Self::Result {
        let nested_arguments = self.elems.iter().map(|ty| NestedArgument::Object(ty.update_nested_generics(source))).collect();
        ObjectConversion::Type(
            TypeCompositionConversion::Tuple(
                TypeComposition::new(Type::Tuple(self.clone()), None, nested_arguments)))
        // TypeComposition::new(Type::Tuple(TypeTuple { paren_token: Default::default(), elems }), None, elems)))
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
                *path = path.update_nested_generics(&(scope, context, None))
                    .to_path();
            },
            _ => {},
        });
        Type::TraitObject(TypeTraitObject { dyn_token: dyn_token.clone(), bounds })
            .to_trait(Punctuated::new())
    }
}
