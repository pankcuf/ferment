use syn::{Attribute, Item, Meta, NestedMeta, parse_quote, Path};
use crate::helper::ItemExtension;
use crate::holder::TypeHolder;

pub enum MacroType {
    Export,
    Register(TypeHolder)
}
#[allow(unused)]
pub fn non_cfg_test(attrs: &Vec<Attribute>) -> bool {
    !cfg_test(attrs)
}
#[allow(unused)]
pub fn cfg_test(attrs: &Vec<Attribute>) -> bool {
    let result = attrs.iter().any(|attr| {
        if attr.path.is_ident("cfg") {
            if let Ok(Meta::List(meta_list)) = attr.parse_meta() {
                meta_list.nested
                    .iter()
                    .any(|nested| matches!(nested, NestedMeta::Meta(Meta::Path(path)) if path.is_ident("test")))
            } else {
                false
            }
        } else {
            false
        }
    });
    result
}

impl TryFrom<&Item> for MacroType {
    type Error = ();

    fn try_from(value: &Item) -> Result<Self, Self::Error> {
        match value.maybe_attrs()
            .and_then(|attrs| attrs.iter().find_map(|attr| {
                let path = &attr.path;
                let mut arguments = Vec::<Path>::new();
                if let Ok(Meta::List(meta_list)) = attr.parse_meta() {
                    meta_list.nested.iter().for_each(|meta| {
                        if let NestedMeta::Meta(Meta::Path(path)) = meta {
                            arguments.push(path.clone());
                        }
                    });
                }
                match path.segments.last().unwrap().ident.to_string().as_str() {
                    "export" =>
                        Some(MacroType::Export),
                    "register" => {
                        let first_path = arguments.first().unwrap();
                        Some(MacroType::Register(parse_quote!(#first_path)))
                    },
                    _ =>
                        None
                }
            })) {
                Some(macro_type) => Ok(macro_type),
                None => Err(())
            }
    }
}


pub struct MacroAttributes {
    pub path: Path,
    pub arguments: Vec<Path>,
}

