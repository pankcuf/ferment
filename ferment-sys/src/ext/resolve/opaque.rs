use proc_macro2::Ident;
use syn::{Path, PathSegment, Type, TypePath};
use crate::ext::DictionaryType;
use crate::ext::maybe_ident::MaybeIdent;

#[allow(unused)]
pub trait Primitive {
    fn is_primitive(&self) -> bool;
}

impl<T> Primitive for T where T: MaybeIdent
{
    fn is_primitive(&self) -> bool {
        self.maybe_ident()
            .map(DictionaryType::is_primitive)
            .unwrap_or_default()

    }
}

impl Primitive for Type {
    fn is_primitive(&self) -> bool {
        match self {
            Type::Path(TypePath { path, .. }) => path.is_primitive(),
            _ => false
        }
    }
}

#[allow(unused)]
pub trait Optional {
    fn is_optional(&self) -> bool;
}

impl<T> Optional for T where T: MaybeIdent
{
    fn is_optional(&self) -> bool {
        self.maybe_ident()
            .map(DictionaryType::is_optional)
            .unwrap_or_default()
    }
}

impl Optional for Type {
    fn is_optional(&self) -> bool {
        match self {
            Type::Path(TypePath { path, .. }) => path.is_primitive(),
            _ => false
        }
    }
}

pub trait FermentableDictionaryType {
    fn is_fermentable_dictionary_type(&self) -> bool;
    fn is_fermentable_string(&self) -> bool;
}

impl FermentableDictionaryType for Ident {
    fn is_fermentable_dictionary_type(&self) -> bool {
        self.is_special_generic() || self.is_result() || self.is_smart_ptr() || self.is_string() || self.is_str()
    }
    fn is_fermentable_string(&self) -> bool {
        self.is_string() || self.is_str()
    }
}
impl FermentableDictionaryType for PathSegment {
    fn is_fermentable_dictionary_type(&self) -> bool {
        self.is_special_generic() ||
            self.is_result() ||
            self.is_smart_ptr() ||
            self.is_string() ||
            self.is_str() ||
            self.is_optional() ||
            self.is_box() ||
            self.is_cow() ||
            self.is_lambda_fn() ||
            self.is_128_digit()
    }

    fn is_fermentable_string(&self) -> bool {
        self.is_string() || self.is_str()
    }
}

impl FermentableDictionaryType for Path {
    fn is_fermentable_dictionary_type(&self) -> bool {
        self.segments.last().map(PathSegment::is_fermentable_dictionary_type).unwrap_or_default()
    }

    fn is_fermentable_string(&self) -> bool {
        self.segments.last().map(PathSegment::is_fermentable_string).unwrap_or_default()
    }
}
impl FermentableDictionaryType for Type {
    fn is_fermentable_dictionary_type(&self) -> bool {
        match self {
            Type::Path(TypePath { path, .. }) =>
                path.is_optional() || path.is_box() || path.is_cow() || path.is_fermentable_dictionary_type(),
            _ => false
        }
    }

    fn is_fermentable_string(&self) -> bool {
        match self {
            Type::Path(TypePath { path, .. }) =>
                path.is_string() || path.is_str(),
            _ => false
        }
    }
}