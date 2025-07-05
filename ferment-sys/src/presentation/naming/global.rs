use std::marker::PhantomData;
use syn::{parse_quote, Path, Type};
use ferment_macro::Display;
use crate::ext::{ToPath, ToType};
use crate::lang::{RustSpecification, Specification};

#[allow(unused)]
#[derive(Display)]
pub enum GlobalType {
    /// Traits
    Clone, // For types that can be cloned.
    Copy, // For types that can be copied.
    Debug, // For types that can be formatted using {, //?}.
    Default, // For types that have a default value.
    Drop, // For types that need to run code on destruction.
    Eq, // For types that can be compared for equality.
    PartialEq, // For types that can be compared for partial equality.
    Ord, // For types that can be compared for ordering.
    PartialOrd, // For types that can be compared for partial ordering.
    Hash, // For types that can be hashed.
    From, // For types that can be created from another type.
    Into, // For types that can be converted into another type.
    AsRef, // For types that can be referenced as another type.
    AsMut, // For types that can be mutably referenced as another type.
    Borrow, // For types that can be borrowed as another type.
    BorrowMut, // For types that can be mutably borrowed as another type.
    Deref, // For types that can be dereferenced to another type.
    DerefMut, // For types that can be mutably dereferenced to another type.
    Iterator, // For types that can be iterated over.
    DoubleEndedIterator, // For iterators that can be iterated from both ends.
    ExactSizeIterator, // For iterators with a known exact length.
    Fn,
    FnMut,
    FnOnce, // For types that can be called as functions.

    /// Types
    Box, // For heap-allocated values.
    Vec, // For growable arrays.
    String, // For heap-allocated strings.
    Option, // For optional values.
    Result, // For error handling.
}

#[derive(Debug)]
pub enum FFIFullDictionaryPath<SPEC>
    where SPEC: Specification {
    Void,
    CChar,
    Phantom(PhantomData<SPEC>)
}

impl ToType for FFIFullDictionaryPath<RustSpecification> {
    fn to_type(&self) -> Type {
        match self {
            FFIFullDictionaryPath::Void => parse_quote!(std::os::raw::c_void),
            FFIFullDictionaryPath::CChar => parse_quote!(std::os::raw::c_char),
            FFIFullDictionaryPath::Phantom(_) => panic!("")
        }
    }
}
impl<SPEC> ToPath for FFIFullDictionaryPath<SPEC>
    where SPEC: Specification,
          FFIFullDictionaryPath<SPEC>: ToType {
    fn to_path(&self) -> Path {
        self.to_type()
            .to_path()
    }
}
