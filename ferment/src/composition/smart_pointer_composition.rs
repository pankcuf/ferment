// use syn::{Path, Type};
// use std::collections::HashMap;
// use std::fmt::{Debug, Display, Formatter};
// use crate::composition::TypeComposition;
// use crate::formatter::{format_path_vec, format_predicates_dict};
//
// // #[derive(Clone)]
// // pub struct SmartPointerComposition {
// //     pub type_composition: TypeComposition,
// //     pub bounds: Vec<Path>,
// //     pub predicates: HashMap<Type, Vec<Path>>
// // }
// //
// // impl Debug for SmartPointerComposition {
// //     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
// //         let str = format!("SmartPointerComposition({}, {}, {})",
// //                           self.type_composition,
// //                           format_path_vec(&self.bounds),
// //                           format_predicates_dict(&self.predicates));
// //         f.write_str(str.as_str())
// //     }
// // }
// //
// // impl Display for SmartPointerComposition {
// //     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
// //         Debug::fmt(self, f)
// //     }
// // }
// //
// //
// // impl SmartPointerComposition {
// //     pub fn is_empty(&self) -> bool {
// //         self.bounds.is_empty()
// //     }
// // }