use std::marker::PhantomData;
use syn::Path;
use crate::ext::{ToPath, ToType};
use crate::lang::Specification;

#[derive(Debug)]
pub enum FFIFullDictionaryPath<SPEC>
where SPEC: Specification {
    Void,
    CChar,
    Phantom(PhantomData<SPEC>)
}
impl<SPEC> ToPath for FFIFullDictionaryPath<SPEC>
where SPEC: Specification,
      FFIFullDictionaryPath<SPEC>: ToType {
    fn to_path(&self) -> Path {
        self.to_type()
            .to_path()
    }
}
