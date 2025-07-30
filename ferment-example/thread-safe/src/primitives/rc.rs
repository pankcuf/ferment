use std::cell::{Cell, RefCell, UnsafeCell};
use std::rc::Rc;
use crate::entry::FFIContext;

#[derive(Copy, Clone)]
#[ferment_macro::opaque]
pub struct CopyableType {
    pub value: u32,
}
#[ferment_macro::export]
pub struct CellExamples {
    pub simple_cell: Cell<u32>,
    pub simple_rc_cell: Rc<Cell<u32>>,
    pub complex_cell: Cell<CopyableType>,
    pub complex_rc_cell: Rc<Cell<CopyableType>>,
}
#[ferment_macro::export]
pub struct RefCellExamples {
    pub simple: Rc<u32>,
    pub simple_refcell: RefCell<u32>,
    pub simple_rc_refcell: Rc<RefCell<u32>>,
    pub complex: Rc<String>,
    pub complex_refcell: RefCell<String>,
    pub complex_rc_refcell: Rc<RefCell<String>>,
    pub opaque: Rc<FFIContext>,
    pub opaque_refcell: RefCell<FFIContext>,
    pub opaque_rc_refcell: Rc<RefCell<FFIContext>>,
}

#[ferment_macro::export]
pub struct UnsafeCellExamples {
    pub simple: Rc<u32>,
    pub simple_refcell: UnsafeCell<u32>,
    pub simple_rc_refcell: Rc<UnsafeCell<u32>>,
    pub complex: Rc<String>,
    pub complex_refcell: UnsafeCell<String>,
    pub complex_rc_refcell: Rc<UnsafeCell<String>>,
    pub opaque: Rc<FFIContext>,
    pub opaque_refcell: UnsafeCell<FFIContext>,
    pub opaque_rc_refcell: Rc<UnsafeCell<FFIContext>>,
}
