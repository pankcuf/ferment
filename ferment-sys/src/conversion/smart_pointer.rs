use std::fmt::{Debug, Display, Formatter};
use syn::Type;
use crate::composable::{TypeModel, TypeModeled};
use crate::ext::AsType;

#[derive(Clone)]
pub enum SmartPointerModelKind {
    Box(TypeModel),
    Arc(TypeModel),
    Rc(TypeModel),
    Mutex(TypeModel),
    RwLock(TypeModel),
    RefCell(TypeModel),
    Pin(TypeModel)
}

impl<'a> AsType<'a> for SmartPointerModelKind {
    fn as_type(&'a self) -> &'a Type {
        self.type_model_ref().as_type()
    }
}

impl TypeModeled for SmartPointerModelKind {
    fn type_model_mut(&mut self) -> &mut TypeModel {
        match self {
            SmartPointerModelKind::Box(model) |
            SmartPointerModelKind::Arc(model) |
            SmartPointerModelKind::Rc(model) |
            SmartPointerModelKind::Mutex(model) |
            SmartPointerModelKind::RwLock(model) |
            SmartPointerModelKind::RefCell(model) |
            SmartPointerModelKind::Pin(model) => model
        }
    }
    fn type_model_ref(&self) -> &TypeModel {
        match self {
            SmartPointerModelKind::Box(model) |
            SmartPointerModelKind::Arc(model) |
            SmartPointerModelKind::Rc(model) |
            SmartPointerModelKind::Mutex(model) |
            SmartPointerModelKind::RwLock(model) |
            SmartPointerModelKind::RefCell(model) |
            SmartPointerModelKind::Pin(model) => model
        }
    }
}

impl Debug for SmartPointerModelKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            SmartPointerModelKind::Arc(model) =>
                format!("Arc({})", model),
            SmartPointerModelKind::Rc(model) =>
                format!("Rc({})", model),
            SmartPointerModelKind::Mutex(model) =>
                format!("Mutex({})", model),
            SmartPointerModelKind::RwLock(model) =>
                format!("RwLock({})", model),
            SmartPointerModelKind::RefCell(model) =>
                format!("RefCell({})", model),
            SmartPointerModelKind::Pin(model) =>
                format!("Pin({})", model),
            SmartPointerModelKind::Box(model) =>
                format!("Box({})", model),
        }.as_str())
    }
}

impl Display for SmartPointerModelKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}