use std::rc::Rc;
use std::cell::RefCell;
use syn::{Field, FieldsNamed, FieldsUnnamed, parse_quote, Type};
use crate::composer::FieldTypesContext;
use crate::context::ScopeContext;
use crate::conversion::FieldTypeConversion;
use crate::naming::Name;

pub enum ConversionsComposer<'a> {
    Empty,
    NamedStruct(&'a FieldsNamed),
    UnnamedStruct(&'a FieldsUnnamed),
    UnnamedEnumVariant(&'a FieldsUnnamed),
    TypeAlias(&'a Type),
}


impl<'a> ConversionsComposer<'a> {
    pub fn compose(&self, source: &Rc<RefCell<ScopeContext>>) -> FieldTypesContext {
        let ctx = source.borrow();
        match self {
            Self::Empty => vec![],
            Self::NamedStruct(fields) =>
                fields
                    .named
                    .iter()
                    .map(|Field { ident, ty, .. }| {
                        FieldTypeConversion::Named(Name::Optional(ident.clone()), ctx.full_type_for(ty))
                    })
                    .collect(),
            Self::UnnamedEnumVariant(fields) =>
                fields
                    .unnamed
                    .iter()
                    .enumerate()
                    .map(|(index, Field { ty, .. })|
                        FieldTypeConversion::Unnamed(Name::UnnamedArg(index), ctx.full_type_for(ty)))
                    .collect(),
            Self::UnnamedStruct(fields) =>
                fields
                    .unnamed
                    .iter()
                    .enumerate()
                    .map(|(index, Field { ty, .. })|
                        FieldTypeConversion::Unnamed(Name::UnnamedStructFieldsComp(ty.clone(), index), ctx.full_type_for(ty)))
                    .collect(),
            Self::TypeAlias(ty) => {
                vec![FieldTypeConversion::Unnamed(Name::UnnamedStructFieldsComp(parse_quote!(#ty), 0), ctx.full_type_for(ty))]
            }
        }
    }
}
