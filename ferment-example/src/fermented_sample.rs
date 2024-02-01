
pub mod some_mod {
    pub trait CanRetry {
        fn can_retry(&self) -> bool;
    }
    pub enum Status {
        Error,
        Success
    }
    impl CanRetry for Status {
        fn can_retry(&self) -> bool { true }
    }
    unsafe impl Send for Status {}
    pub struct Uri {
        pub(crate) scheme: String,
    }

    pub trait TransportClient: Send + Sized {
        type Error: CanRetry + Send;
        fn with_uri(uri: Uri) -> Self;
    }
    pub struct CoreGrpcClient {
        pub uri: Uri
    }
    impl CoreGrpcClient {
        pub fn new(uri: Uri) -> Self { Self { uri } }
    }

    impl TransportClient for CoreGrpcClient {
        type Error = Status;

        fn with_uri(uri: Uri) -> Self {
            CoreGrpcClient::new(uri)
        }
    }
}

pub mod fermented {
    pub mod generics {}
    pub mod vtable {
        pub mod r#crate {
            pub mod some_mod {
                #[repr(C)]
                pub struct CanRetry {
                    pub can_retry: extern "C" fn(*const ()) -> bool,
                }
                static MY_ERROR_CAN_RETRY_VTABLE: CanRetry = CanRetry {
                    can_retry: my_error_can_retry
                };
                #[repr(C)]
                pub struct SomeOtherTrait {
                    pub some_other_method: extern "C" fn(*const ()),
                }
                static MY_ERROR_SOME_OTHER_TRAIT_VTABLE: SomeOtherTraitVTable = SomeOtherTraitVTable { some_other_method: my_error_some_other_method };
            }
        }

        // pub struct CanRetryVTable {
        //     pub can_retry: unsafe extern "C" fn(*const std::ffi::c_void) -> bool,
        // }
        // pub struct SomeOtherTraitVTable {
        //     pub some_other_method: unsafe extern "C" fn(*const std::ffi::c_void),
        // }
        //
        // pub struct TransportClientVTable {
        //     pub with_uri: unsafe extern "C" fn(uri: crate::fermented::vtable::) -> *const std::ffi::c_void,
        // }
    }
    static MY_ERROR_CAN_RETRY_VTABLE: CanRetryVTable = CanRetryVTable { can_retry: my_error_can_retry };
    static MY_ERROR_SOME_OTHER_TRAIT_VTABLE: SomeOtherTraitVTable = SomeOtherTraitVTable { some_other_method: my_error_some_other_method };

    pub trait CanRetry {
        fn can_retry(&self) -> bool;
    }

    pub enum Status {
        Error,
        Success
    }
    impl CanRetry for Status {
        fn can_retry(&self) -> bool { true }
    }
    unsafe impl Send for Status {}
    pub struct Uri {
        pub(crate) scheme: String,
    }
    pub trait SomeOtherTrait {
        fn some_other_method(&self);
    }

    pub trait TransportClient: Send + Sized {
        type Error: CanRetry + Send + SomeOtherTrait;
        fn with_uri(uri: Uri) -> Self;
    }
    pub struct CoreGrpcClient {
        pub uri: Uri
    }
    impl CoreGrpcClient {
        pub fn new(uri: Uri) -> Self { Self { uri } }
    }

    impl TransportClient for CoreGrpcClient {
        type Error = Status;

        fn with_uri(uri: Uri) -> Self {
            CoreGrpcClient::new(uri)
        }
    }
}