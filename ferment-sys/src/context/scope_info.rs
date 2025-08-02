use proc_macro2::Ident;
use syn::{Attribute, Path};
use crate::composer::MaybeMacroLabeled;
use crate::context::Scope;

#[derive(Clone, Eq)]
pub struct ScopeInfo {
    pub attrs: Vec<Attribute>,
    pub crate_ident: Ident,
    pub self_scope: Scope
}
impl PartialEq<Self> for ScopeInfo {
    fn eq(&self, other: &Self) -> bool {
        self.self_scope.eq(&other.self_scope) &&
            self.crate_ident.eq(&other.crate_ident)
    }
}

impl ScopeInfo {
    pub fn fmt_export_type(&self) -> String {
        self.attrs.is_labeled_for_opaque_export()
            .then(|| "Opaque")
            .or_else(|| self.attrs.is_labeled_for_export()
                .then(|| "Fermented"))
            .or_else(|| self.attrs.is_labeled_for_opaque_export().then(|| "Opaque"))
            .unwrap_or("Unknown").to_string()
    }
    pub fn self_path(&self) -> &Path {
        &self.self_scope.self_scope.0
    }
}
