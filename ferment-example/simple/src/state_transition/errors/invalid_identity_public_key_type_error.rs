#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[ferment_macro::export]
pub struct InvalidIdentityPublicKeyTypeError {
    pub public_key_type: String,
}
