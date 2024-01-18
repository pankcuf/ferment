use crate::asyn::platform_version::PlatformVersion;
use crate::asyn::provider::ContextProvider;
use crate::nested::ProtocolError;

pub trait FromProof<Req> {
    type Request;
    type Response;
    fn maybe_from_proof<'a, I: Into<Self::Request>, O: Into<Self::Response>>(
        request: I,
        response: O,
        platform_version: &PlatformVersion,
        provider: &'a dyn ContextProvider,
    ) -> Result<Option<Self>, ProtocolError>
        where
            Self: Sized + 'a;

    fn from_proof<'a, I: Into<Self::Request>, O: Into<Self::Response>>(
        request: I,
        response: O,
        platform_version: &PlatformVersion,
        provider: &'a dyn ContextProvider,
    ) -> Result<Self, ProtocolError>
        where
            Self: Sized + 'a,
    {
        Self::maybe_from_proof(request, response, platform_version, provider)?
            .ok_or(ProtocolError::EmptyPublicKeyDataError)
    }
}
