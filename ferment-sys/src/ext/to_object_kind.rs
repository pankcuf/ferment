use syn::{Type, TypePath};
use crate::composable::TypeModel;
use crate::composer::CommaPunctuatedNestedArguments;
use crate::kind::{ObjectKind, TypeModelKind};

pub trait ToObjectKind {
    fn to_unknown(self, nested_arguments: CommaPunctuatedNestedArguments) -> ObjectKind;
    fn to_object(self, nested_arguments: CommaPunctuatedNestedArguments) -> ObjectKind;
    fn to_trait(self, nested_arguments: CommaPunctuatedNestedArguments) -> ObjectKind;
}

impl ToObjectKind for Type {
    fn to_unknown(self, nested_arguments: CommaPunctuatedNestedArguments) -> ObjectKind {
        ObjectKind::Type(TypeModelKind::Unknown(handle_type_model(self, nested_arguments)))
    }

    fn to_object(self, nested_arguments: CommaPunctuatedNestedArguments) -> ObjectKind {
        ObjectKind::Type(TypeModelKind::Object(handle_type_model(self, nested_arguments)))
    }

    fn to_trait(self, nested_arguments: CommaPunctuatedNestedArguments) -> ObjectKind {
        // TODO: make it Unknown
        ObjectKind::Type(TypeModelKind::Unknown(handle_type_model(self, nested_arguments)))
    }
}

impl ToObjectKind for TypePath {
    fn to_unknown(self, nested_arguments: CommaPunctuatedNestedArguments) -> ObjectKind {
        ObjectKind::Type(TypeModelKind::Unknown(handle_type_path_model(self, nested_arguments)))
    }

    fn to_object(self, nested_arguments: CommaPunctuatedNestedArguments) -> ObjectKind {
        ObjectKind::Type(TypeModelKind::Object(handle_type_path_model(self, nested_arguments)))
    }

    fn to_trait(self, nested_arguments: CommaPunctuatedNestedArguments) -> ObjectKind {
        // TODO: make it Unknown
        ObjectKind::Type(TypeModelKind::Unknown(handle_type_path_model(self, nested_arguments)))
    }
}

pub(super) fn handle_type_model(ty: Type, nested_arguments: CommaPunctuatedNestedArguments) -> TypeModel {
    TypeModel::new(ty, None, nested_arguments)
}
pub(super) fn handle_type_path_model(type_path: TypePath, nested_arguments: CommaPunctuatedNestedArguments) -> TypeModel {
    TypeModel::new(Type::Path(type_path), None, nested_arguments)
}

