use syn::Type;
use std::fmt::{Debug, Display, Formatter};
use crate::composable::{TypeModel, TypeModeled};
use crate::ext::AsType;

#[derive(Clone)]
pub enum GroupModelKind {
    Result(TypeModel),
    Vec(TypeModel),
    Map(TypeModel),
    BTreeSet(TypeModel),
    HashSet(TypeModel),
    IndexMap(TypeModel),
    IndexSet(TypeModel)
}

impl<'a> AsType<'a> for GroupModelKind {
    fn as_type(&'a self) -> &'a Type {
        self.type_model_ref().as_type()
    }
}

impl TypeModeled for GroupModelKind {
    fn type_model_mut(&mut self) -> &mut TypeModel {
        match self {
            GroupModelKind::Result(model) |
            GroupModelKind::Vec(model) |
            GroupModelKind::Map(model) |
            GroupModelKind::BTreeSet(model) |
            GroupModelKind::HashSet(model) |
            GroupModelKind::IndexMap(model) |
            GroupModelKind::IndexSet(model) => model,
        }
    }

    fn type_model_ref(&self) -> &TypeModel {
        match self {
            GroupModelKind::Result(model) |
            GroupModelKind::Vec(model) |
            GroupModelKind::Map(model) |
            GroupModelKind::BTreeSet(model) |
            GroupModelKind::HashSet(model) |
            GroupModelKind::IndexMap(model) |
            GroupModelKind::IndexSet(model) => model
        }
    }
}

impl Debug for GroupModelKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", match self {
            GroupModelKind::Result(model) =>
                format!("Result({})", model),
            GroupModelKind::Vec(model) =>
                format!("Vec({})", model),
            GroupModelKind::Map(model) =>
                format!("Map({})", model),
            GroupModelKind::BTreeSet(model) =>
                format!("BTreeSet({})", model),
            GroupModelKind::HashSet(model) =>
                format!("HashSet({})", model),
            GroupModelKind::IndexMap(model) =>
                format!("IndexMap({})", model),
            GroupModelKind::IndexSet(model) =>
                format!("IndexSet({})", model),
        }))
    }
}

impl Display for GroupModelKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}