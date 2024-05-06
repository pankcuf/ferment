use crate::state_transition::errors::invalid_identity_public_key_type_error::InvalidIdentityPublicKeyTypeError;

#[ferment_macro::export]
pub enum ProtocolError {
    InvalidPKT(InvalidIdentityPublicKeyTypeError)
}