use std::collections::HashSet;
use quote::ToTokens;
use syn::{AngleBracketedGenericArguments, BareFnArg, Binding, GenericArgument, ParenthesizedGenericArguments, parse_quote, PathArguments, ReturnType, Type, TypeArray, TypeBareFn, TypePtr, TypeReference, TypeTuple};
use crate::context::VisitorContext;
use crate::conversion::{Conversion, TypeConversion};
use crate::holder::Holder;
use crate::impl_holder;

impl_holder!(TypeHolder, Type);

impl<'a> From<&'a TypeConversion> for TypeHolder {
    fn from(value: &'a TypeConversion) -> Self {
        TypeHolder(value.ty().clone())
    }
}
impl<'a> From<&'a Box<Type>> for TypeHolder {
    fn from(value: &'a Box<Type>) -> Self {
        TypeHolder(*value.clone())
    }
}
impl TypeHolder {

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
