use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use quote::ToTokens;
use syn::__private::TokenStream2;
use syn::parse::ParseStream;
use syn::parse_quote::ParseQuote;
use syn::{AngleBracketedGenericArguments, BareFnArg, Binding, GenericArgument, ParenthesizedGenericArguments, PathArguments, ReturnType, Type, TypeArray, TypeBareFn, TypePath, TypePtr, TypeReference, TypeTuple};

#[derive(Clone)]
pub struct TypeConversion(pub Type);

impl PartialEq for TypeConversion {
    fn eq(&self, other: &Self) -> bool {
        self.0.to_token_stream().to_string().eq(&other.0.to_token_stream().to_string())
    }
}

impl Eq for TypeConversion {}

impl Hash for TypeConversion {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.to_token_stream().to_string().hash(state);
    }
}


impl std::fmt::Debug for TypeConversion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.0.to_token_stream().to_string().as_str())
    }
}

impl std::fmt::Display for TypeConversion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}

impl ParseQuote for TypeConversion {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Type::parse(input)
            .map(TypeConversion::new)
    }
}

impl ToTokens for TypeConversion {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        self.0.to_tokens(tokens)
    }
}

impl<'a> From<&'a Type> for TypeConversion {
    fn from(value: &'a Type) -> Self {
        TypeConversion(value.clone())
    }
}

impl<'a> From<&'a Box<Type>> for TypeConversion {
    fn from(value: &'a Box<Type>) -> Self {
        TypeConversion(*value.clone())
    }
}
impl TypeConversion {
    pub fn new(ty: Type) -> Self {
        TypeConversion(ty)
    }

    fn add_involved_types_into_container(ty: &Type, container: &mut HashSet<TypePath>) {
        container.extend(TypeConversion::from(ty).get_all_type_paths_involved());
    }

    pub fn get_all_type_paths_involved(&self) -> HashSet<TypePath> {
        let mut involved = HashSet::new();
        match &self.0 {
            Type::Array(TypeArray { elem: ty, .. }) =>
                Self::add_involved_types_into_container(ty, &mut involved),
            Type::BareFn(TypeBareFn { inputs, output, .. }) => {
                inputs
                    .iter()
                    .for_each(|BareFnArg { ty, .. }| {
                        Self::add_involved_types_into_container(ty, &mut involved);
                    });
                if let ReturnType::Type(_, ty) = output {
                    Self::add_involved_types_into_container(ty,&mut involved);
                }
            }
            Type::Path(type_path) => {
                involved.insert(type_path.clone());
                match type_path.path.segments.last() {
                    Some(last_segment) =>
                        match &last_segment.arguments {
                            PathArguments::AngleBracketed(AngleBracketedGenericArguments { args, .. }) =>
                                args.iter().for_each(|arg| match arg {
                                    GenericArgument::Type(ty) =>
                                        Self::add_involved_types_into_container(ty, &mut involved),
                                    GenericArgument::Binding(Binding { ty, .. }) =>
                                        Self::add_involved_types_into_container(ty, &mut involved),
                                    _ => {}
                                }),
                            PathArguments::Parenthesized(ParenthesizedGenericArguments { inputs, output, .. }) => {
                                inputs
                                    .iter()
                                    .for_each(|ty|
                                        Self::add_involved_types_into_container(ty, &mut involved));
                                if let ReturnType::Type(_, ty) = output {
                                    Self::add_involved_types_into_container(ty, &mut involved);
                                }
                            },
                            PathArguments::None => {}
                        },
                    None => {},
                }
            },
            Type::Ptr(TypePtr { elem: ty, .. }) =>
                Self::add_involved_types_into_container(ty,&mut involved),
            Type::Reference(TypeReference { elem: ty, .. }) =>
                Self::add_involved_types_into_container(ty,&mut involved),
            Type::Tuple(TypeTuple { elems, .. }) => {
                elems.iter().for_each(|ty|
                    Self::add_involved_types_into_container(ty,&mut involved))
            },
            _ => {}
        }
        involved
    }
}
