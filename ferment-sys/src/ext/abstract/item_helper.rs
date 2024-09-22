use syn::Item;

pub trait ItemHelper {
    fn is_mod(&self) -> bool;
    #[allow(unused)]
    fn is_use(&self) -> bool;
}

impl ItemHelper for Item {
    fn is_mod(&self) -> bool {
        match self {
            Item::Mod(_) => true,
            _ => false,
        }
    }
    fn is_use(&self) -> bool {
        match self {
            Item::Use(_) => true,
            _ => false,
        }
    }
}