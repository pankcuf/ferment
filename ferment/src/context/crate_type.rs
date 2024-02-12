use std::fmt::Debug;
use quote::quote;
use crate::holder::PathHolder;

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum Crate {
    Current,
    External(PathHolder)
}

impl ToString for Crate {
    fn to_string(&self) -> String {
        match self {
            Crate::Current => format!("Crate::Current({})", quote!(crate)),
            Crate::External(path) => format!("Crate::External({})", quote!(#path)),
        }
    }
}

// impl Debug for Crate {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         f.write_str(self.to_string().as_str())
//     }
// }
//
// impl Display for Crate {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         Debug::fmt(self, f)
//     }
// }
