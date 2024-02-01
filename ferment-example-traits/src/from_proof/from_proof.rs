use crate::nested::ProtocolError;

#[ferment_macro::export]
pub trait FromProof<Req> {
    type Request;
    type Response;
    fn maybe_from_proof<'a, I: Into<Self::Request>, O: Into<Self::Response>>(
        request: I,
        response: O,
        platform_version: u32,
    ) -> Result<Option<Self>, ProtocolError>
        where
            Self: Sized + 'a;

    fn from_proof<'a, I, O>(
        request: I,
        response: O,
        platform_version: u32,
    ) -> Result<Self, ProtocolError>
        where
            Self: Sized + 'a,
            I: Into<Self::Request>,
            O: Into<Self::Response>;

}
