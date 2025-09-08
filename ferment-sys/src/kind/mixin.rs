use std::fmt::{Debug, Display, Formatter};
use std::hash::{Hash, Hasher};
use quote::ToTokens;
use crate::composable::GenericBoundsModel;
use crate::kind::GenericTypeKind;

#[derive(Clone)]
pub enum MixinKind {
    Generic(GenericTypeKind),
    Bounds(GenericBoundsModel)
}

impl Debug for MixinKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            MixinKind::Generic(kind) =>
                f.write_fmt(format_args!("MixinKind::Generic({})", kind.to_token_stream())),
            MixinKind::Bounds(model) =>
                f.write_fmt(format_args!("MixinKind::Bounds({})", model))
        }
    }
}
impl Display for MixinKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}

impl PartialEq for MixinKind {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (MixinKind::Generic(kind), MixinKind::Generic(other_kind)) =>
                kind.eq(other_kind),
            (MixinKind::Bounds(bounds), MixinKind::Bounds(other_bounds)) =>
                bounds.eq(other_bounds),
            _ => false
        }
    }
}

impl Eq for MixinKind {}

impl Hash for MixinKind {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            MixinKind::Generic(kind) =>
                kind.to_token_stream().to_string().hash(state),
            MixinKind::Bounds(model) =>
                model.hash(state)
        }
    }
}

impl MixinKind {
    pub fn bounds(bounds: &GenericBoundsModel) -> Self {
        Self::Bounds(bounds.clone())
    }
}