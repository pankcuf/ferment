use crate::nested::ProtocolError;

// #[ferment_macro::export]
pub trait FromProof {
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

#[ferment_macro::export]
pub struct Identity {
    pub platform_version: u32
}
#[derive(Debug)]
#[ferment_macro::export]
pub struct GetIdentityByPublicKeyHashRequestInit {

}
#[derive(Debug)]
#[ferment_macro::export]
pub struct GetIdentityByPublicKeyHashRequest {

}
#[derive(Debug)]
#[ferment_macro::export]
pub struct GetIdentityByPublicKeyHashResponse {

}

impl Into<GetIdentityByPublicKeyHashRequest> for GetIdentityByPublicKeyHashRequestInit {
    fn into(self) -> GetIdentityByPublicKeyHashRequest {
        todo!()
    }
}

// #[ferment_macro::export]
impl FromProof for Identity {
    type Request = GetIdentityByPublicKeyHashRequest;
    type Response = GetIdentityByPublicKeyHashResponse;

    fn maybe_from_proof<'a, I: Into<Self::Request>, O: Into<Self::Response>>(
        request: I,
        response: O,
        platform_version: u32
    ) -> Result<Option<Self>, ProtocolError> where Self: Sized + 'a {
        let req = request.into();
        let res = response.into();
        println!("maybe_from_proof: {:?} --- {:?}", req, res);
        Ok(Some(Identity { platform_version }))
    }

    fn from_proof<'a, I, O>(
        request: I,
        response: O,
        platform_version: u32
    ) -> Result<Self, ProtocolError> where Self: Sized + 'a, I: Into<Self::Request>, O: Into<Self::Response> {
        let req = request.into();
        let res = response.into();
        println!("maybe_from_proof: {:?} --- {:?}", req, res);
        Ok(Identity { platform_version })
    }
}


// mod fermented {
//     #[repr(C)]
//     #[derive(Clone)]
//     pub struct Into_GetIdentityByPublicKeyHashRequest {
//         pub object: *const (),
//         pub vtable: *const Into_GetIdentityByPublicKeyHashRequest_VTable,
//     }
//     #[repr(C)]
//     #[derive(Clone)]
//     pub struct Into_GetIdentityByPublicKeyHashRequest_VTable {
//         pub into: fn(*const Into_GetIdentityByPublicKeyHashRequest) -> crate::from_proof::from_proof::GetIdentityByPublicKeyHashResponse,
//     }
//
//     #[repr(C)]
//     #[derive(Clone)]
//     pub struct Into_GetIdentityByPublicKeyHashResponse_VTable {
//         pub into: fn(*const Into_GetIdentityByPublicKeyHashResponse) -> crate::from_proof::from_proof::GetIdentityByPublicKeyHashResponse,
//     }
//     #[repr(C)]
//     #[derive(Clone)]
//     pub struct Into_GetIdentityByPublicKeyHashResponse {
//         pub object: *const (),
//         pub vtable: *const Into_GetIdentityByPublicKeyHashResponse_VTable,
//     }
//
//     #[repr(C)]
//     #[derive(Clone)]
//     pub struct FromProof_FOR_Identity {
//         object: *const (),
//         vtable: *const FromProof_FOR_Identity_VTable,
//     }
//     #[repr(C)]
//     #[derive(Clone)]
//     pub struct FromProof_FOR_Identity_VTable {
//         pub maybe_from_proof: unsafe extern "C" fn (
//             request: *const Into_GetIdentityByPublicKeyHashRequest,
//             response: *const Into_GetIdentityByPublicKeyHashResponse,
//             platform_version: u32
//         ) -> *mut crate::fermented::generics::Result_ok_Option_ferment_example_traits_from_proof_from_proof_FromProof_err_ferment_example_traits_nested_ProtocolError,
//         pub from_proof: unsafe extern "C" fn (
//             request: *const Into_GetIdentityByPublicKeyHashRequest,
//             response: *const Into_GetIdentityByPublicKeyHashResponse,
//             platform_version: u32
//         ) -> *mut crate::fermented::generics::Result_ok_ferment_example_traits_from_proof_from_proof_FromProof_err_ferment_example_traits_nested_ProtocolError
//     }
//
//     pub const FROM_PROOF_FOR_IDENTITY_VTABLE: FromProof_FOR_Identity_VTable = FromProof_FOR_Identity_VTable {
//         maybe_from_proof: FromProof_FOR_Identity_maybe_from_proof,
//         from_proof: FromProof_FOR_Identity_from_proof,
//     };
//
//     #[no_mangle]
//     pub unsafe extern "C" fn FromProof_FOR_Identity_maybe_from_proof(
//         request: *const Into_GetIdentityByPublicKeyHashRequest,
//         response: *const Into_GetIdentityByPublicKeyHashResponse,
//         platform_version: u32,
//     ) -> *mut crate::fermented::generics::Result_ok_Option_ferment_example_traits_from_proof_from_proof_FromProof_err_ferment_example_traits_nested_ProtocolError {
//         ferment::FFIConversionTo::ffi_to(
//             crate::from_proof::from_proof::FromProof::maybe_from_proof(
//                 request,
//                 response,
//                 platform_version)
//         )
//     }
//     #[no_mangle]
//     pub unsafe extern "C" fn FromProof_FOR_Identity_from_proof(
//         request: *const Into_GetIdentityByPublicKeyHashRequest,
//         response: *const Into_GetIdentityByPublicKeyHashResponse,
//         platform_version: u32,
//     ) -> *mut crate::fermented::generics::Result_ok_ferment_example_traits_from_proof_from_proof_FromProof_err_ferment_example_traits_nested_ProtocolError {
//         ferment::FFIConversionTo::ffi_to(
//             crate::from_proof::from_proof::FromProof::from_proof(
//                 ferment::FFIConversionFrom::ffi_from_const(request),
//                 ferment::FFIConversionFrom::ffi_from_const(response),
//                 platform_version)
//         )
//     }
//
//     impl Into<crate::from_proof::from_proof::FromProof::Request> for Into_GetIdentityByPublicKeyHashRequest {
//         fn into(self) -> crate::from_proof::from_proof::FromProof::Request {
//             unsafe {
//                 let Self { vtable, object } = self;
//                 ((*vtable).into)(object as *const _)
//             }
//         }
//     }
//     impl Into<crate::from_proof::from_proof::FromProof::Response> for Into_GetIdentityByPublicKeyHashResponse {
//         fn into(self) -> crate::from_proof::from_proof::FromProof::Response {
//             unsafe {
//                 let Self { vtable, object} = self;
//                 ((*vtable).into)(object as *const _)
//             }
//         }
//     }
//
// }