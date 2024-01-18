use syn::Path;
use crate::holder::PathHolder;

pub enum MacroType {
    Export,
    Register(PathHolder)
}

impl MacroType {
    pub fn name(&self) -> &str {
        match self {
            Self::Export => "export",
            Self::Register(..) => "register",
        }
    }

    pub fn is(&self, str: &str) -> bool {
        self.name() == str
    }
}

pub struct MacroAttributes {
    pub path: Path,
    pub arguments: Vec<Path>,
}
