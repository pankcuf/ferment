use syn::Attribute;
use crate::ext::ResolveMacro;
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


