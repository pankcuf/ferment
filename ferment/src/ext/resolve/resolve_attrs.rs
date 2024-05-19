use syn::Attribute;

pub trait ResolveAttrs where Self: Sized {
    fn resolve_attrs(&self) -> Vec<Option<Attribute>>;
}
