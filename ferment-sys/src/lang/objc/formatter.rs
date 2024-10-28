use crate::ast::{Depunctuated, SemiPunctuated};
use crate::lang::objc::fermentate::InterfaceImplementation;
use crate::lang::objc::presentable::ArgPresentation;

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
pub fn format_properties(vec: &SemiPunctuated<ArgPresentation>) -> String {
    vec.iter()
        .map(|item| format!("{item};\n"))
        .collect::<Vec<_>>()
        .join("\n\n")
}