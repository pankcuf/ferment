use std::rc::Rc;
use std::cell::RefCell;
use quote::{quote, ToTokens};
use syn::{Field, FieldsNamed, FieldsUnnamed, parse_quote, Type};
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
    pub fn compose(&self, context: &Rc<RefCell<ScopeContext>>) -> Vec<FieldTypeConversion> {
        let ctx = context.borrow();
        match self {
            Self::Empty => vec![],
            Self::NamedStruct(fields) =>
                fields
                    .named
                    .iter()
                    .map(|Field { ident, ty, .. }| {
                        println!("conversions_composer:NamedStruct {}: {}: ({})", quote!(#ident), quote!(#ty), ctx.full_type_for(ty).to_token_stream());
                        FieldTypeConversion::Named(Name::Optional(ident.clone()), ctx.full_type_for(ty))
                    })
                    .collect(),
            Self::UnnamedEnumVariant(fields) =>
                fields
                    .unnamed
                    .iter()
                    .enumerate()
                    .map(|(index, Field { ty, .. })|
                        FieldTypeConversion::Unnamed(Name::UnamedArg(index), ctx.full_type_for(ty)))
                        // (context.full_type_for(ty), ffi_unnamed_arg_name(index).to_token_stream()))
                    .collect(),
            Self::UnnamedStruct(fields) =>
                fields
                    .unnamed
                    .iter()
                    .enumerate()
                    .map(|(index, Field { ty, .. })|
                        FieldTypeConversion::Unnamed(Name::UnnamedStructFieldsComp(ty.clone(), index), ctx.full_type_for(ty)))
                        // (context.full_type_for(ty), unnamed_struct_fields_comp(ty, index)))
                    .collect(),
            Self::TypeAlias(ty) => {
                vec![FieldTypeConversion::Unnamed(Name::UnnamedStructFieldsComp(parse_quote!(#ty), 0), ctx.full_type_for(ty))]
            }
        }
    }
}
