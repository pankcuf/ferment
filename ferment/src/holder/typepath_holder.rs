use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use quote::ToTokens;
use syn::__private::TokenStream2;
use syn::parse::ParseStream;
use syn::parse_quote::ParseQuote;
use syn::{AngleBracketedGenericArguments, BareFnArg, Binding, GenericArgument, ParenthesizedGenericArguments, parse_quote, PathArguments, ReturnType, Type, TypeArray, TypeBareFn, TypePath, TypePtr, TypeReference, TypeTuple};
use crate::conversion::Conversion;
use crate::context::VisitorContext;

#[derive(Clone)]
pub struct TypePathHolder(pub TypePath);

impl PartialEq for TypePathHolder {
    fn eq(&self, other: &Self) -> bool {
        self.0.to_token_stream().to_string().eq(&other.0.to_token_stream().to_string())
    }
}
impl Eq for TypePathHolder {}

impl Hash for TypePathHolder {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.to_token_stream().to_string().hash(state);
    }
}

impl std::fmt::Debug for TypePathHolder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.0.to_token_stream().to_string().as_str())
    }
}

impl std::fmt::Display for TypePathHolder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}
impl ParseQuote for TypePathHolder {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        TypePath::parse(input)
            .map(TypePathHolder::from)
    }
}
impl ToTokens for TypePathHolder {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        self.0.to_tokens(tokens)
    }
}

impl From<TypePath> for TypePathHolder {
    fn from(value: TypePath) -> Self {
        TypePathHolder(value)
    }
}

impl<'a> From<&'a TypePath> for TypePathHolder {
    fn from(value: &'a TypePath) -> Self {
        TypePathHolder(value.clone())
    }
}

// impl TypePathConversion {
//     pub fn first_ident(&self) -> Option<&Ident> {
//         self.0.path.segments.first().map(|segment| &segment.ident)
//     }
//     pub fn last_ident(&self) -> Option<&Ident> {
//         self.0.path.segments.last().map(|segment| &segment.ident)
//     }
// }


impl Conversion for TypePathHolder {
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
