use syn::{AngleBracketedGenericArguments, ParenthesizedGenericArguments, PathArguments, PathSegment};

pub trait MaybeParenthesizedArgs {
    fn maybe_parenthesized_args(&self) -> Option<&ParenthesizedGenericArguments>;
}
pub trait MaybeParenthesizedArgsMut {
    fn maybe_parenthesized_args_mut(&mut self) -> Option<&mut ParenthesizedGenericArguments>;
}

pub trait MaybeAngleBracketedArgs {
    fn maybe_angle_bracketed_args(&self) -> Option<&AngleBracketedGenericArguments>;
}
pub trait MaybeAngleBracketedArgsMut {
    fn maybe_angle_bracketed_args_mut(&mut self) -> Option<&mut AngleBracketedGenericArguments>;
}

impl MaybeParenthesizedArgs for PathArguments {
    fn maybe_parenthesized_args(&self) -> Option<&ParenthesizedGenericArguments> {
        match self {
            PathArguments::Parenthesized(args) => Some(args),
            _ => None
        }
    }
}
impl MaybeParenthesizedArgsMut for PathArguments {
    fn maybe_parenthesized_args_mut(&mut self) -> Option<&mut ParenthesizedGenericArguments> {
        match self {
            PathArguments::Parenthesized(args) => Some(args),
            _ => None
        }
    }
}
impl MaybeParenthesizedArgs for PathSegment {
    fn maybe_parenthesized_args(&self) -> Option<&ParenthesizedGenericArguments> {
        self.arguments.maybe_parenthesized_args()
    }
}
impl MaybeParenthesizedArgsMut for PathSegment {
    fn maybe_parenthesized_args_mut(&mut self) -> Option<&mut ParenthesizedGenericArguments> {
        self.arguments.maybe_parenthesized_args_mut()
    }
}
impl MaybeAngleBracketedArgs for PathArguments {
    fn maybe_angle_bracketed_args(&self) -> Option<&AngleBracketedGenericArguments> {
        match self {
            PathArguments::AngleBracketed(args) => Some(args),
            _ => None
        }
    }
}
impl MaybeAngleBracketedArgsMut for PathArguments {
    fn maybe_angle_bracketed_args_mut(&mut self) -> Option<&mut AngleBracketedGenericArguments> {
        match self {
            PathArguments::AngleBracketed(args) => Some(args),
            _ => None
        }
    }
}

impl MaybeAngleBracketedArgs for PathSegment {
    fn maybe_angle_bracketed_args(&self) -> Option<&AngleBracketedGenericArguments> {
        self.arguments.maybe_angle_bracketed_args()
    }
}
impl MaybeAngleBracketedArgsMut for PathSegment {
    fn maybe_angle_bracketed_args_mut(&mut self) -> Option<&mut AngleBracketedGenericArguments> {
        self.arguments.maybe_angle_bracketed_args_mut()
    }
}
