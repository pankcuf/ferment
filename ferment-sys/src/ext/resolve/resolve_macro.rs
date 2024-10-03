use syn::{Attribute, Path};

pub trait ResolveMacro where Self: Sized {
    fn is_labeled_for(&self, macro_type: &str) -> bool;
    fn is_labeled_for_export(&self) -> bool {
        self.is_labeled_for("export")
    }
    fn is_labeled_for_opaque_export(&self) -> bool {
        self.is_labeled_for("opaque")
    }
    #[allow(unused)]
    fn is_labeled_for_register(&self) -> bool {
        self.is_labeled_for("register")
    }
}

impl ResolveMacro for Attribute {
    fn is_labeled_for(&self, macro_type: &str) -> bool {
        self.path.is_labeled_for(macro_type)
    }
}

impl ResolveMacro for Path {
    fn is_labeled_for(&self, macro_type: &str) -> bool {
        self.segments
            .iter()
            .any(|segment| segment.ident.to_string().eq(macro_type))
    }
}