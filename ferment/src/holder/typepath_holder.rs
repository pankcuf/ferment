use std::collections::HashSet;
use quote::ToTokens;
use syn::{AngleBracketedGenericArguments, BareFnArg, Binding, GenericArgument, ParenthesizedGenericArguments, PathArguments, ReturnType, Type, TypeArray, TypeBareFn, TypePath, TypePtr, TypeReference, TypeTuple};
use crate::conversion::Conversion;
use crate::holder::Holder;
use crate::impl_holder;

impl_holder!(TypePathHolder, TypePath);

impl Conversion for TypePathHolder {
    type Item = Type;

    fn nested_items(item: &Self::Item) -> HashSet<Self::Item> {
        let mut container = HashSet::from([]);
        match item {
            Type::Array(TypeArray { elem: ty, .. }) =>
                Self::nested_items_into_container(ty, &mut container),
            Type::Ptr(TypePtr { elem: ty, .. }) =>
                Self::nested_items_into_container(ty, &mut container),
            Type::Reference(TypeReference { elem: ty, .. }) =>
                Self::nested_items_into_container(ty, &mut container),
            Type::Tuple(TypeTuple { elems, .. }) =>
                elems.iter().for_each(|ty|
                    Self::nested_items_into_container(ty, &mut container)),
            Type::BareFn(TypeBareFn { inputs, output, .. }) => {
                inputs
                    .iter()
                    .for_each(|BareFnArg { ty, .. }|
                        Self::nested_items_into_container(ty, &mut container));
                if let ReturnType::Type(_, ty) = output {
                    Self::nested_items_into_container(ty, &mut container);
                }
            },
            Type::Path(type_path) => {
                container.insert(item.clone());
                if let Some(last_segment) = type_path.path.segments.last() {
                    match &last_segment.arguments {
                        PathArguments::AngleBracketed(AngleBracketedGenericArguments { args, .. }) =>
                            args.iter().for_each(|arg| match arg {
                                GenericArgument::Type(ty) =>
                                    Self::nested_items_into_container(ty, &mut container),
                                GenericArgument::Binding(Binding { ty, .. }) =>
                                    Self::nested_items_into_container(ty, &mut container),
                                _ => {}
                            }),
                        PathArguments::Parenthesized(ParenthesizedGenericArguments { inputs, output, .. }) => {
                            inputs
                                .iter()
                                .for_each(|ty|
                                    Self::nested_items_into_container(ty, &mut container));
                            if let ReturnType::Type(_, ty) = output {
                                Self::nested_items_into_container(ty, &mut container);
                            }
                        },
                        PathArguments::None => {}
                    }
                }
            },
            _ => {}
        }
        container
    }
}
