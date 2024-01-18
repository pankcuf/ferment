use syn::QSelf;
use crate::composition::TypeComposition;

pub struct QSelfComposition {
    pub qs: TypeComposition,
    pub qself: QSelf
}