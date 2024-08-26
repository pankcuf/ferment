use syn::QSelf;
use crate::composable::TypeModel;

pub struct QSelfModel {
    #[allow(unused)]
    pub qs: TypeModel,
    pub qself: QSelf
}