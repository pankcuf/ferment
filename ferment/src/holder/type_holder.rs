use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use quote::ToTokens;
use syn::__private::TokenStream2;
use syn::parse::ParseStream;
use syn::parse_quote::ParseQuote;
use syn::{AngleBracketedGenericArguments, BareFnArg, Binding, GenericArgument, ParenthesizedGenericArguments, parse_quote, PathArguments, ReturnType, Type, TypeArray, TypeBareFn, TypePtr, TypeReference, TypeTuple};
use crate::context::VisitorContext;
use crate::conversion::{Conversion, TypeConversion};

#[derive(Clone)]
pub struct TypeHolder(pub Type);

impl PartialEq for TypeHolder {
    fn eq(&self, other: &Self) -> bool {
        self.0.to_token_stream().to_string().eq(&other.0.to_token_stream().to_string())
    }
}

impl Eq for TypeHolder {}

impl Hash for TypeHolder {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.to_token_stream().to_string().hash(state);
    }
}


impl std::fmt::Debug for TypeHolder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.0.to_token_stream().to_string().as_str())
    }
}

impl std::fmt::Display for TypeHolder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}

impl ParseQuote for TypeHolder {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Type::parse(input)
            .map(TypeHolder::new)
    }
}

impl ToTokens for TypeHolder {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        self.0.to_tokens(tokens)
    }
}

impl<'a> From<&'a TypeConversion> for TypeHolder {
    fn from(value: &'a TypeConversion) -> Self {
        TypeHolder(value.ty().clone())
    }
}
impl<'a> From<&'a Type> for TypeHolder {
    fn from(value: &'a Type) -> Self {
        TypeHolder(value.clone())
    }
}

impl<'a> From<&'a Box<Type>> for TypeHolder {
    fn from(value: &'a Box<Type>) -> Self {
        TypeHolder(*value.clone())
    }
}
impl TypeHolder {
    pub fn new(ty: Type) -> Self {
        TypeHolder(ty)
    }

    // pub fn get_all_type_paths_involved(&self) -> HashSet<TypePathConversion> {
    //     let mut involved = HashSet::from([TypePathConversion(parse_quote!(Self))]);
    //     match &self.0 {
    //         Type::Array(TypeArray { elem: ty, .. }) =>
    //             add_involved_types_into_container(ty, &mut involved),
    //         Type::Ptr(TypePtr { elem: ty, .. }) =>
    //             add_involved_types_into_container(ty,&mut involved),
    //         Type::Reference(TypeReference { elem: ty, .. }) =>
    //             add_involved_types_into_container(ty,&mut involved),
    //         Type::Tuple(TypeTuple { elems, .. }) =>
    //             elems.iter().for_each(|ty|
    //                 add_involved_types_into_container(ty,&mut involved)),
    //         Type::BareFn(TypeBareFn { inputs, output, .. }) => {
    //             inputs
    //                 .iter()
    //                 .for_each(|BareFnArg { ty, .. }|
    //                     add_involved_types_into_container(ty, &mut involved));
    //             if let ReturnType::Type(_, ty) = output {
    //                 add_involved_types_into_container(ty,&mut involved);
    //             }
    //         }
    //         Type::Path(type_path) => {
    //             involved.insert(TypePathConversion::from(type_path));
    //             if let Some(last_segment) = type_path.path.segments.last() {
    //                 match &last_segment.arguments {
    //                     PathArguments::AngleBracketed(AngleBracketedGenericArguments { args, .. }) =>
    //                         args.iter().for_each(|arg| match arg {
    //                             GenericArgument::Type(ty) =>
    //                                 add_involved_types_into_container(ty, &mut involved),
    //                             GenericArgument::Binding(Binding { ty, .. }) =>
    //                                 add_involved_types_into_container(ty, &mut involved),
    //                             _ => {}
    //                         }),
    //                     PathArguments::Parenthesized(ParenthesizedGenericArguments { inputs, output, .. }) => {
    //                         inputs
    //                             .iter()
    //                             .for_each(|ty|
    //                                 add_involved_types_into_container(ty, &mut involved));
    //                         if let ReturnType::Type(_, ty) = output {
    //                             add_involved_types_into_container(ty, &mut involved);
    //                         }
    //                     },
    //                     PathArguments::None => {}
    //                 }
    //             }
    //         },
    //         _ => {}
    //     }
    //     involved
    // }
}

// fn add_involved_types_into_container(ty: &Type, container: &mut HashSet<TypePathConversion>) {
//     //println!("add_involved_types_into_container: {}", ty.to_token_stream());
//     container.extend(TypeHolder::from(ty).get_all_type_paths_involved());
// }

impl Conversion for TypeHolder {
    type Item = Type;

    fn nested_items_into_container(ty: &Self::Item, visitor_context: &VisitorContext, container: &mut HashSet<Self::Item>) {
        container.extend(Self::nested_items(ty, visitor_context));
    }

    fn nested_items(item: &Self::Item, visitor_context: &VisitorContext) -> HashSet<Self::Item> {
        let mut involved = HashSet::from([parse_quote!(Self)]);
        match item {
            Type::Array(TypeArray { elem: ty, .. }) =>
                Self::nested_items_into_container(ty, visitor_context, &mut involved),
            Type::Ptr(TypePtr { elem: ty, .. }) =>
                Self::nested_items_into_container(ty, visitor_context, &mut involved),
            Type::Reference(TypeReference { elem: ty, .. }) =>
                Self::nested_items_into_container(ty, visitor_context, &mut involved),
            Type::Tuple(TypeTuple { elems, .. }) =>
                elems.iter().for_each(|ty|
                    Self::nested_items_into_container(ty, visitor_context, &mut involved)),
            Type::BareFn(TypeBareFn { inputs, output, .. }) => {
                inputs
                    .iter()
                    .for_each(|BareFnArg { ty, .. }|
                        Self::nested_items_into_container(ty, visitor_context, &mut involved));
                if let ReturnType::Type(_, ty) = output {
                    Self::nested_items_into_container(ty, visitor_context, &mut involved);
                }
            }
            Type::Path(type_path) => {
                involved.insert(item.clone());
                if let Some(last_segment) = type_path.path.segments.last() {
                    match &last_segment.arguments {
                        PathArguments::AngleBracketed(AngleBracketedGenericArguments { args, .. }) =>
                            args.iter().for_each(|arg| match arg {
                                GenericArgument::Type(ty) =>
                                    Self::nested_items_into_container(ty, visitor_context, &mut involved),
                                GenericArgument::Binding(Binding { ty, .. }) =>
                                    Self::nested_items_into_container(ty, visitor_context, &mut involved),
                                _ => {}
                            }),
                        PathArguments::Parenthesized(ParenthesizedGenericArguments { inputs, output, .. }) => {
                            inputs
                                .iter()
                                .for_each(|ty|
                                    Self::nested_items_into_container(ty, visitor_context, &mut involved));
                            if let ReturnType::Type(_, ty) = output {
                                Self::nested_items_into_container(ty, visitor_context, &mut involved);
                            }
                        },
                        PathArguments::None => {}
                    }
                }
            },
            _ => {}
        }
        involved
    }
}
