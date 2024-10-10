use crate::ast::{Depunctuated, SemiPunctuated};
use crate::lang::objc::fermentate::InterfaceImplementation;
use crate::lang::objc::presentation::Property;

#[allow(unused)]
pub fn format_interface_implementations(vec: &Depunctuated<InterfaceImplementation>) -> String {
    vec.iter()
        .map(|item| {
            format!("{item}\n")
            // item.to_token_stream().to_string()
        })
        .collect::<Vec<_>>()
        .join("\n\n")
}
#[allow(unused)]
pub fn format_properties(vec: &SemiPunctuated<Property>) -> String {
    vec.iter()
        .map(|item| format!("{item};\n"))
        .collect::<Vec<_>>()
        .join("\n\n")
}