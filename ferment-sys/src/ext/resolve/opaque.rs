use proc_macro2::Ident;
use syn::{Attribute, Path, PathSegment, Type, TypePath};
use crate::ext::{DictionaryType, ResolveMacro};
use crate::ext::item::ItemExtension;

pub trait Opaque {
    fn is_opaque(&self) -> bool;
}
impl<T> Opaque for T where T: ItemExtension {
    fn is_opaque(&self) -> bool {
        self.maybe_attrs().map_or(false, Opaque::is_opaque)

    }
}
impl Opaque for Vec<Attribute> {
    fn is_opaque(&self) -> bool {
        self.iter().any(ResolveMacro::is_labeled_for_opaque_export)
    }
}

pub trait Fermented {
    fn is_fermented(&self) -> bool;
}

impl<T> Fermented for T where T: ItemExtension {
    fn is_fermented(&self) -> bool {
        self.maybe_attrs().map_or(false, Fermented::is_fermented)

    }
}

impl Fermented for Vec<Attribute> {
    fn is_fermented(&self) -> bool {
        self.iter().any(ResolveMacro::is_labeled_for_export)
    }
}
pub trait Custom {
    fn is_custom(&self) -> bool;
}

impl<T> Custom for T where T: ItemExtension {
    fn is_custom(&self) -> bool {
        self.maybe_attrs().map_or(false, Custom::is_custom)

    }
}

impl Custom for Vec<Attribute> {
    fn is_custom(&self) -> bool {
        self.iter().any(ResolveMacro::is_labeled_for_register)
    }
}


#[allow(unused)]
pub trait Primitive {
    fn is_primitive(&self) -> bool;
}

impl<T> Primitive for T where T: ItemExtension {
    fn is_primitive(&self) -> bool {
        self.maybe_ident()
            .map_or(false, DictionaryType::is_primitive)

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

pub trait FermentableDictionaryType {
    fn is_fermentable_dictionary_type(&self) -> bool;
}

impl FermentableDictionaryType for Ident {
    fn is_fermentable_dictionary_type(&self) -> bool {
        self.is_special_generic() || self.is_result() || self.is_smart_ptr() || self.is_string() || self.is_str()
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
            self.is_lambda_fn() ||
            self.is_128_digit()
    }
}

impl FermentableDictionaryType for Path {
    fn is_fermentable_dictionary_type(&self) -> bool {
        self.segments.last().map_or(false, PathSegment::is_fermentable_dictionary_type)
    }
}
impl FermentableDictionaryType for Type {
    fn is_fermentable_dictionary_type(&self) -> bool {
        match self {
            Type::Path(TypePath { path, .. }) =>
                path.is_optional() || path.is_box() || path.is_fermentable_dictionary_type(),
            _ => false
        }
    }
}