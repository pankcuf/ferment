#[allow(
    clippy::let_and_return,
    clippy::suspicious_else_formatting,
    clippy::redundant_field_names,
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    redundant_semicolons,
    unused_braces,
    unused_imports,
    unused_unsafe,
    unused_variables,
    unused_qualifications
)]
pub mod types {
    pub mod transport {
        pub mod transport_request {
            #[doc = "FFI-representation of the # [doc = \"FFI-representation of the crate :: transport :: transport_request :: GetDocumentsResponse\"]"]
            #[repr(C)]
            #[derive(Clone)]
            #[allow(non_camel_case_types)]
            pub struct GetDocumentsResponse {
                pub version: u32,
            }
            impl
                ferment_interfaces::FFIConversion<
                    crate::transport::transport_request::GetDocumentsResponse,
                > for GetDocumentsResponse
            {
                unsafe fn ffi_from_const(
                    ffi: *const GetDocumentsResponse,
                ) -> crate::transport::transport_request::GetDocumentsResponse {
                    let ffi_ref = &*ffi;
                    crate::transport::transport_request::GetDocumentsResponse {
                        version: ffi_ref.version,
                    }
                }
                unsafe fn ffi_to_const(
                    obj: crate::transport::transport_request::GetDocumentsResponse,
                ) -> *const GetDocumentsResponse {
                    ferment_interfaces::boxed(GetDocumentsResponse {
                        version: obj.version,
                    })
                }
                unsafe fn destroy(ffi: *mut GetDocumentsResponse) {
                    ferment_interfaces::unbox_any(ffi);
                }
            }
            impl Drop for GetDocumentsResponse {
                fn drop(&mut self) {
                    unsafe {
                        let ffi_ref = self;
                    }
                }
            }
            #[doc = r" # Safety"]
            #[allow(non_snake_case)]
            #[no_mangle]
            pub unsafe extern "C" fn GetDocumentsResponse_ctor(
                version: u32,
            ) -> *mut GetDocumentsResponse {
                ferment_interfaces::boxed(GetDocumentsResponse { version })
            }
            #[doc = r" # Safety"]
            #[allow(non_snake_case)]
            #[no_mangle]
            pub unsafe extern "C" fn GetDocumentsResponse_destroy(ffi: *mut GetDocumentsResponse) {
                ferment_interfaces::unbox_any(ffi);
            }
            #[doc = "FFI-representation of the # [doc = \"FFI-representation of the crate :: transport :: transport_request :: Uri\"]"]
            #[repr(C)]
            #[derive(Clone)]
            #[allow(non_camel_case_types)]
            pub struct Uri {
                pub scheme: *mut std::os::raw::c_char,
            }
            impl ferment_interfaces::FFIConversion<crate::transport::transport_request::Uri> for Uri {
                unsafe fn ffi_from_const(
                    ffi: *const Uri,
                ) -> crate::transport::transport_request::Uri {
                    let ffi_ref = &*ffi;
                    crate::transport::transport_request::Uri {
                        scheme: ferment_interfaces::FFIConversion::ffi_from(ffi_ref.scheme),
                    }
                }
                unsafe fn ffi_to_const(
                    obj: crate::transport::transport_request::Uri,
                ) -> *const Uri {
                    ferment_interfaces::boxed(Uri {
                        scheme: ferment_interfaces::FFIConversion::ffi_to(obj.scheme),
                    })
                }
                unsafe fn destroy(ffi: *mut Uri) {
                    ferment_interfaces::unbox_any(ffi);
                }
            }
            impl Drop for Uri {
                fn drop(&mut self) {
                    unsafe {
                        let ffi_ref = self;
                        < std :: os :: raw :: c_char as ferment_interfaces :: FFIConversion < String >> :: destroy (ffi_ref . scheme) ;
                    }
                }
            }
            #[doc = r" # Safety"]
            #[allow(non_snake_case)]
            #[no_mangle]
            pub unsafe extern "C" fn Uri_ctor(scheme: *mut std::os::raw::c_char) -> *mut Uri {
                ferment_interfaces::boxed(Uri { scheme })
            }
            #[doc = r" # Safety"]
            #[allow(non_snake_case)]
            #[no_mangle]
            pub unsafe extern "C" fn Uri_destroy(ffi: *mut Uri) {
                ferment_interfaces::unbox_any(ffi);
            }
            #[doc = "FFI-representation of the Status"]
            #[repr(C)]
            #[allow(non_camel_case_types)]
            #[derive(Clone)]
            pub enum Status {
                Error,
                Success,
            }
            impl ferment_interfaces::FFIConversion<crate::transport::transport_request::Status> for Status {
                unsafe fn ffi_from_const(
                    ffi: *const Status,
                ) -> crate::transport::transport_request::Status {
                    let ffi_ref = &*ffi;
                    match ffi_ref {
                        Status::Error => crate::transport::transport_request::Status::Error,
                        Status::Success => crate::transport::transport_request::Status::Success,
                    }
                }
                unsafe fn ffi_to_const(
                    obj: crate::transport::transport_request::Status,
                ) -> *const Status {
                    ferment_interfaces::boxed(match obj {
                        crate::transport::transport_request::Status::Error => Status::Error,
                        crate::transport::transport_request::Status::Success => Status::Success,
                    })
                }
                unsafe fn destroy(ffi: *mut Status) {
                    ferment_interfaces::unbox_any(ffi);
                }
            }
            impl Drop for Status {
                fn drop(&mut self) {
                    unsafe {
                        match self {
                            Status::Error => {}
                            Status::Success => {}
                        }
                    }
                }
            }
            #[doc = r" # Safety"]
            #[allow(non_snake_case)]
            #[no_mangle]
            pub unsafe extern "C" fn Status_Error_ctor() -> *mut Status {
                ferment_interfaces::boxed(Status::Error)
            }
            #[doc = r" # Safety"]
            #[allow(non_snake_case)]
            #[no_mangle]
            pub unsafe extern "C" fn Status_Success_ctor() -> *mut Status {
                ferment_interfaces::boxed(Status::Success)
            }
            #[doc = r" # Safety"]
            #[allow(non_snake_case)]
            #[no_mangle]
            pub unsafe extern "C" fn Status_destroy(ffi: *mut Status) {
                ferment_interfaces::unbox_any(ffi);
            }
            #[doc = "FFI-representation of the # [doc = \"FFI-representation of the crate :: transport :: transport_request :: DocumentQuery\"]"]
            #[repr(C)]
            #[derive(Clone)]
            #[allow(non_camel_case_types)]
            pub struct DocumentQuery {
                pub version: u32,
            }
            impl
                ferment_interfaces::FFIConversion<
                    crate::transport::transport_request::DocumentQuery,
                > for DocumentQuery
            {
                unsafe fn ffi_from_const(
                    ffi: *const DocumentQuery,
                ) -> crate::transport::transport_request::DocumentQuery {
                    let ffi_ref = &*ffi;
                    crate::transport::transport_request::DocumentQuery {
                        version: ffi_ref.version,
                    }
                }
                unsafe fn ffi_to_const(
                    obj: crate::transport::transport_request::DocumentQuery,
                ) -> *const DocumentQuery {
                    ferment_interfaces::boxed(DocumentQuery {
                        version: obj.version,
                    })
                }
                unsafe fn destroy(ffi: *mut DocumentQuery) {
                    ferment_interfaces::unbox_any(ffi);
                }
            }
            impl Drop for DocumentQuery {
                fn drop(&mut self) {
                    unsafe {
                        let ffi_ref = self;
                    }
                }
            }
            #[doc = r" # Safety"]
            #[allow(non_snake_case)]
            #[no_mangle]
            pub unsafe extern "C" fn DocumentQuery_ctor(version: u32) -> *mut DocumentQuery {
                ferment_interfaces::boxed(DocumentQuery { version })
            }
            #[doc = r" # Safety"]
            #[allow(non_snake_case)]
            #[no_mangle]
            pub unsafe extern "C" fn DocumentQuery_destroy(ffi: *mut DocumentQuery) {
                ferment_interfaces::unbox_any(ffi);
            }
            #[doc = "FFI-representation of the # [doc = \"FFI-representation of the crate :: transport :: transport_request :: GetDocumentsRequest\"]"]
            #[repr(C)]
            #[derive(Clone)]
            #[allow(non_camel_case_types)]
            pub struct GetDocumentsRequest {
                pub version: u32,
            }
            impl
                ferment_interfaces::FFIConversion<
                    crate::transport::transport_request::GetDocumentsRequest,
                > for GetDocumentsRequest
            {
                unsafe fn ffi_from_const(
                    ffi: *const GetDocumentsRequest,
                ) -> crate::transport::transport_request::GetDocumentsRequest {
                    let ffi_ref = &*ffi;
                    crate::transport::transport_request::GetDocumentsRequest {
                        version: ffi_ref.version,
                    }
                }
                unsafe fn ffi_to_const(
                    obj: crate::transport::transport_request::GetDocumentsRequest,
                ) -> *const GetDocumentsRequest {
                    ferment_interfaces::boxed(GetDocumentsRequest {
                        version: obj.version,
                    })
                }
                unsafe fn destroy(ffi: *mut GetDocumentsRequest) {
                    ferment_interfaces::unbox_any(ffi);
                }
            }
            impl Drop for GetDocumentsRequest {
                fn drop(&mut self) {
                    unsafe {
                        let ffi_ref = self;
                    }
                }
            }
            #[doc = r" # Safety"]
            #[allow(non_snake_case)]
            #[no_mangle]
            pub unsafe extern "C" fn GetDocumentsRequest_ctor(
                version: u32,
            ) -> *mut GetDocumentsRequest {
                ferment_interfaces::boxed(GetDocumentsRequest { version })
            }
            #[doc = r" # Safety"]
            #[allow(non_snake_case)]
            #[no_mangle]
            pub unsafe extern "C" fn GetDocumentsRequest_destroy(ffi: *mut GetDocumentsRequest) {
                ferment_interfaces::unbox_any(ffi);
            }
            #[doc = "FFI-representation of the # [doc = \"FFI-representation of the crate :: transport :: transport_request :: CoreGrpcClient\"]"]
            #[repr(C)]
            #[derive(Clone)]
            #[allow(non_camel_case_types)]
            pub struct CoreGrpcClient {
                pub uri: *mut crate::fermented::types::transport::transport_request::Uri,
            }
            impl
                ferment_interfaces::FFIConversion<
                    crate::transport::transport_request::CoreGrpcClient,
                > for CoreGrpcClient
            {
                unsafe fn ffi_from_const(
                    ffi: *const CoreGrpcClient,
                ) -> crate::transport::transport_request::CoreGrpcClient {
                    let ffi_ref = &*ffi;
                    crate::transport::transport_request::CoreGrpcClient {
                        uri: ferment_interfaces::FFIConversion::ffi_from(ffi_ref.uri),
                    }
                }
                unsafe fn ffi_to_const(
                    obj: crate::transport::transport_request::CoreGrpcClient,
                ) -> *const CoreGrpcClient {
                    ferment_interfaces::boxed(CoreGrpcClient {
                        uri: ferment_interfaces::FFIConversion::ffi_to(obj.uri),
                    })
                }
                unsafe fn destroy(ffi: *mut CoreGrpcClient) {
                    ferment_interfaces::unbox_any(ffi);
                }
            }
            impl Drop for CoreGrpcClient {
                fn drop(&mut self) {
                    unsafe {
                        let ffi_ref = self;
                        ferment_interfaces::unbox_any(ffi_ref.uri);
                    }
                }
            }
            #[doc = r" # Safety"]
            #[allow(non_snake_case)]
            #[no_mangle]
            pub unsafe extern "C" fn CoreGrpcClient_ctor(
                uri: *mut crate::fermented::types::transport::transport_request::Uri,
            ) -> *mut CoreGrpcClient {
                ferment_interfaces::boxed(CoreGrpcClient { uri })
            }
            #[doc = r" # Safety"]
            #[allow(non_snake_case)]
            #[no_mangle]
            pub unsafe extern "C" fn CoreGrpcClient_destroy(ffi: *mut CoreGrpcClient) {
                ferment_interfaces::unbox_any(ffi);
            }
            #[doc = "FFI-representation of the # [doc = \"FFI-representation of the crate :: transport :: transport_request :: Identifier\"]"]
            #[repr(C)]
            #[derive(Clone)]
            #[allow(non_camel_case_types)]
            pub struct Identifier(u32);
            impl ferment_interfaces::FFIConversion<crate::transport::transport_request::Identifier>
                for Identifier
            {
                unsafe fn ffi_from_const(
                    ffi: *const Identifier,
                ) -> crate::transport::transport_request::Identifier {
                    let ffi_ref = &*ffi;
                    crate::transport::transport_request::Identifier(ffi_ref.0)
                }
                unsafe fn ffi_to_const(
                    obj: crate::transport::transport_request::Identifier,
                ) -> *const Identifier {
                    ferment_interfaces::boxed(Identifier(obj.0))
                }
                unsafe fn destroy(ffi: *mut Identifier) {
                    ferment_interfaces::unbox_any(ffi);
                }
            }
            impl Drop for Identifier {
                fn drop(&mut self) {
                    unsafe {
                        let ffi_ref = self;
                    }
                }
            }
            #[doc = r" # Safety"]
            #[allow(non_snake_case)]
            #[no_mangle]
            pub unsafe extern "C" fn Identifier_ctor(o_0: u32) -> *mut Identifier {
                ferment_interfaces::boxed(Identifier(o_0))
            }
            #[doc = r" # Safety"]
            #[allow(non_snake_case)]
            #[no_mangle]
            pub unsafe extern "C" fn Identifier_destroy(ffi: *mut Identifier) {
                ferment_interfaces::unbox_any(ffi);
            }
        }
    }
    pub mod nested {
        #[doc = "FFI-representation of the ProtocolError"]
        #[repr(C)]
        #[allow(non_camel_case_types)]
        #[derive(Clone)]
        pub enum ProtocolError {
            IdentifierError(*mut std::os::raw::c_char),
        }
        impl ferment_interfaces::FFIConversion<crate::nested::ProtocolError> for ProtocolError {
            unsafe fn ffi_from_const(ffi: *const ProtocolError) -> crate::nested::ProtocolError {
                let ffi_ref = &*ffi;
                match ffi_ref {
                    ProtocolError::IdentifierError(o_0) => {
                        crate::nested::ProtocolError::IdentifierError(
                            ferment_interfaces::FFIConversion::ffi_from(*o_0),
                        )
                    }
                }
            }
            unsafe fn ffi_to_const(obj: crate::nested::ProtocolError) -> *const ProtocolError {
                ferment_interfaces::boxed(match obj {
                    crate::nested::ProtocolError::IdentifierError(o_0) => {
                        ProtocolError::IdentifierError(ferment_interfaces::FFIConversion::ffi_to(
                            o_0,
                        ))
                    }
                })
            }
            unsafe fn destroy(ffi: *mut ProtocolError) {
                ferment_interfaces::unbox_any(ffi);
            }
        }
        impl Drop for ProtocolError {
            fn drop(&mut self) {
                unsafe {
                    match self {
                        ProtocolError::IdentifierError(o_0) => {
                            < std :: os :: raw :: c_char as ferment_interfaces :: FFIConversion < String >> :: destroy (* o_0) ;
                        }
                    }
                }
            }
        }
        #[doc = r" # Safety"]
        #[allow(non_snake_case)]
        #[no_mangle]
        pub unsafe extern "C" fn ProtocolError_IdentifierError_ctor(
            o_0: *mut std::os::raw::c_char,
        ) -> *mut ProtocolError {
            ferment_interfaces::boxed(ProtocolError::IdentifierError(o_0))
        }
        #[doc = r" # Safety"]
        #[allow(non_snake_case)]
        #[no_mangle]
        pub unsafe extern "C" fn ProtocolError_destroy(ffi: *mut ProtocolError) {
            ferment_interfaces::unbox_any(ffi);
        }
    }
}
#[allow(
    clippy::let_and_return,
    clippy::suspicious_else_formatting,
    clippy::redundant_field_names,
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    redundant_semicolons,
    unused_braces,
    unused_imports,
    unused_unsafe,
    unused_variables,
    unused_qualifications
)]
pub mod generics {}
