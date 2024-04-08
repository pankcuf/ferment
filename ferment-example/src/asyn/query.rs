use std::error::Error;
use std::time::Duration;
use crate::nested::ProtocolError;

#[derive(Debug, Clone, Copy)]
#[ferment_macro::export]
pub struct RequestSettings {
    pub timeout: Option<Duration>,
    pub retries: Option<usize>,
}
#[derive(Debug, Clone, Copy)]
#[ferment_macro::export]
pub struct AppliedRequestSettings {
    pub timeout: Duration,
    pub retries: usize,
}

#[derive(Clone)]
#[ferment_macro::export]
pub struct Uri {
    pub(crate) scheme: String,
}

#[ferment_macro::export]
pub trait CanRetry {
    fn can_retry(&self) -> bool;
}

#[ferment_macro::export]
pub trait TransportClient: Send + Sized {
    type Error: CanRetry + Send;
    fn with_uri(uri: Uri) -> Self;
}


#[ferment_macro::export]
pub trait TransportResponse: Clone + Send + Sync {}


#[ferment_macro::export]
pub trait TransportRequest: Clone + Send + Sync {
    type Client: TransportClient;
    type Response: TransportResponse;
    const SETTINGS_OVERRIDES: RequestSettings;
    fn execute_transport(self, client: &mut Self::Client, settings: &AppliedRequestSettings)
        -> Result<Self::Response, <Self::Client as TransportClient>::Error>;
}

#[ferment_macro::export]
pub trait Query<T: TransportRequest>: Send + Clone {
    fn query(self, prove: bool) -> Result<T, Box<dyn Error>>;
}

impl<T> Query<T> for T where T: TransportRequest + Sized + Send + Sync + Clone, T::Response: Send + Sync {
    fn query(self, prove: bool) -> Result<T, Box<dyn Error>> {
        if !prove {
            unimplemented!("queries without proofs are not supported yet");
        }
        Ok(self)
    }
}



// #[repr(C)]
// #[derive(Clone)]
// #[allow(non_camel_case_types)]
// pub struct Result_ok_crate_asyn_query_TransportRequest_Response_err_crate_asyn_query_TransportClient_Error {
//     pub ok: *mut crate::fermented::types::asyn::query::TransportResponse,
//     pub error: *mut crate::fermented::types::asyn::query::CanRetry,
// }
// // impl ferment_interfaces::FFIConversion<Result<TransportRequest::Response, TransportClient::Error>>
// impl ferment_interfaces::FFIConversion<Result<TransportRequest::Response, TransportClient::Error>>
// for Result_ok_crate_asyn_query_TransportRequest_Response_err_crate_asyn_query_TransportClient_Error {
//     unsafe fn ffi_from_const(ffi: *const Result_ok_crate_asyn_query_TransportRequest_Response_err_crate_asyn_query_TransportClient_Error)
//         -> Result<TransportRequest::Response, TransportClient::Error> {
//         let ffi_ref = &*ffi;
//         ferment_interfaces::fold_to_result(
//             ffi_ref.ok,
//             ffi_ref.error,
//             |o| ferment_interfaces::FFIConversion::ffi_from(o),
//             |o| ferment_interfaces::FFIConversion::ffi_from(o))
//     }
//     unsafe fn ffi_to_const(obj: Result<TransportRequest::Response, TransportClient::Error>)
//         -> *const Result_ok_crate_asyn_query_TransportRequest_Response_err_crate_asyn_query_TransportClient_Error {
//         let (ok, error) = match obj {
//             Ok(o) => (ferment_interfaces::FFIConversion::ffi_to(o), std::ptr::null_mut()),
//             Err(o) => (std::ptr::null_mut(), ferment_interfaces::FFIConversion::ffi_to(o))
//         };
//         ferment_interfaces::boxed(Self { ok, error })
//     }
//     unsafe fn destroy(
//         ffi: *mut Result_ok_crate_asyn_query_TransportRequest_Response_err_crate_asyn_query_TransportClient_Error) {
//         ferment_interfaces::unbox_any(ffi);
//     }
// }
// impl Drop for Result_ok_crate_asyn_query_TransportRequest_Response_err_crate_asyn_query_TransportClient_Error {
//     fn drop(&mut self) {
//         unsafe {
//             if !self.ok.is_null() {
//                 ferment_interfaces::unbox_any(self.ok);
//             }
//             if !self.error.is_null() {
//                 ferment_interfaces::unbox_any(self.error);
//             }
//         }
//     }
// }


// #[repr(C)]
// #[derive(Clone)]
// #[allow(non_camel_case_types)]
// pub struct Result_ok_crate_asyn_query_TransportRequest_err_Box_dyn_trait_std_error_Error2 {
//     pub ok: *mut crate::fermented::types::asyn::query::TransportRequest,
//     pub error: *mut crate::std_error_Error_FFI,
// }
// impl<T, E> ferment_interfaces::FFIConversion<Result<T, Box<E>>>
// for Result_ok_crate_asyn_query_TransportRequest_err_Box_dyn_trait_std_error_Error2
//     where
//         T: crate::asyn::query::TransportRequest,
//         E: std::error::Error {
//     unsafe fn ffi_from_const(ffi: * const Result_ok_crate_asyn_query_TransportRequest_err_Box_dyn_trait_std_error_Error2) -> Result<T, Box<E>> {
//         let ffi_ref = &*ffi;
//         ferment_interfaces::fold_to_result(
//             ffi_ref.ok,
//             ffi_ref.error,
//             |o| ferment_interfaces::FFIConversion::ffi_from(o),
//             |o| ferment_interfaces::FFIConversion::ffi_from(o),
//         )
//     }
//     unsafe fn ffi_to_const(obj: Result<T, Box<E>>) -> *const Result_ok_crate_asyn_query_TransportRequest_err_Box_dyn_trait_std_error_Error2 {
//         let (ok, error) = match obj {
//             Ok(o) => (
//                 ferment_interfaces::FFIConversion::ffi_to(o),
//                 std::ptr::null_mut(),
//             ),
//             Err(o) => (
//                 std::ptr::null_mut(),
//                 ferment_interfaces::FFIConversion::ffi_to(o),
//             ),
//         };
//         ferment_interfaces::boxed(Self { ok, error })
//     }
//     unsafe fn destroy(ffi: *mut Result_ok_crate_asyn_query_TransportRequest_err_Box_dyn_trait_std_error_Error2) {
//         ferment_interfaces::unbox_any(ffi);
//     }
// }
// impl Drop for Result_ok_crate_asyn_query_TransportRequest_err_Box_dyn_trait_std_error_Error2 {
//     fn drop(&mut self) {
//         unsafe {
//             if !self.ok.is_null() {
//                 ferment_interfaces::unbox_any(self.ok);
//             }
//             if !self.error.is_null() {
//                 ferment_interfaces::unbox_any(self.error);
//             }
//         }
//     }
// }

// #[repr(C)]
// #[derive(Clone)]
// #[allow(non_camel_case_types)]
// pub struct Result_ok_crate_asyn_query_TransportRequest_err_Box_dyn_trait_std_error_Error {
//     pub ok: *mut crate::fermented::types::asyn::query::TransportRequest,
//     pub error: *mut crate::std_error_Error_FFI,
// }
// impl
// ferment_interfaces::FFIConversion<
//     Result<
//         dyn crate::asyn::query::TransportRequest<
//             Client=dyn crate::asyn::query::TransportClient<
//                 Error=dyn crate::asyn::query::CanRetry>,
//             Response=dyn crate::asyn::query::TransportResponse>,
//         Box<dyn std::error::Error>
//     >,
// > for Result_ok_crate_asyn_query_TransportRequest_err_Box_dyn_trait_std_error_Error
// {
//     unsafe fn ffi_from_const(
//         ffi : * const Result_ok_crate_asyn_query_TransportRequest_err_Box_dyn_trait_std_error_Error,
//     ) -> Result<dyn crate::asyn::query::TransportRequest<Client=(), Response=()>, Box<dyn std::error::Error>> {
//         let ffi_ref = &*ffi;
//         ferment_interfaces::fold_to_result(
//             ffi_ref.ok,
//             ffi_ref.error,
//             |o| ferment_interfaces::FFIConversion::ffi_from(o),
//             |o| ferment_interfaces::FFIConversion::ffi_from(o),
//         )
//     }
//     unsafe fn ffi_to_const(
//         obj: Result<dyn crate::asyn::query::TransportRequest<Client=(), Response=()>, Box<dyn std::error::Error>>)
//         -> *const Result_ok_crate_asyn_query_TransportRequest_err_Box_dyn_trait_std_error_Error
//     {
//         let (ok, error) = match obj {
//             Ok(o) => (
//                 ferment_interfaces::FFIConversion::ffi_to(o),
//                 std::ptr::null_mut(),
//             ),
//             Err(o) => (
//                 std::ptr::null_mut(),
//                 ferment_interfaces::FFIConversion::ffi_to(o),
//             ),
//         };
//         ferment_interfaces::boxed(Self { ok, error })
//     }
//     unsafe fn destroy(
//         ffi: *mut Result_ok_crate_asyn_query_TransportRequest_err_Box_dyn_trait_std_error_Error,
//     ) {
//         ferment_interfaces::unbox_any(ffi);
//     }
// }
// impl Drop for Result_ok_crate_asyn_query_TransportRequest_err_Box_dyn_trait_std_error_Error {
//     fn drop(&mut self) {
//         unsafe {
//             if !self.ok.is_null() {
//                 ferment_interfaces::unbox_any(self.ok);
//             }
//             if !self.error.is_null() {
//                 ferment_interfaces::unbox_any(self.error);
//             }
//         }
//     }
// }
