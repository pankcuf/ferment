use crate::state_transition::errors::invalid_identity_public_key_type_error::InvalidIdentityPublicKeyTypeError;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[ferment_macro::export]
pub enum ProtocolError {
    InvalidPKT(InvalidIdentityPublicKeyTypeError)
}