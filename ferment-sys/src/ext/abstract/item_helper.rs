use syn::Item;

pub trait ItemHelper {
    fn is_mod(&self) -> bool;
}

impl ItemHelper for Item {
    fn is_mod(&self) -> bool {
        matches!(self, Item::Mod(_))
    }
}