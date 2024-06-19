use syn::Attribute;
use crate::ext::ResolveMacro;
use crate::helper::ItemExtension;

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