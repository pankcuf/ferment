use syn::QSelf;
use crate::composable::TypeComposition;

pub struct QSelfComposition {
    #[allow(unused)]
    pub qs: TypeComposition,
    pub qself: QSelf
}