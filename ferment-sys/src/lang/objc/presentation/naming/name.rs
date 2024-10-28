use quote::{format_ident, quote, ToTokens};
use syn::__private::TokenStream2;
use syn::Type;
use crate::ext::{Mangle, MangleDefault, usize_to_tokenstream};
use crate::lang::objc::{ObjCFermentate, ObjCSpecification};
use crate::presentation::{DictionaryName, Name};

impl<SPEC> Mangle<MangleDefault> for Name<ObjCFermentate, SPEC>
    where SPEC: ObjCSpecification {
    fn mangle_string(&self, context: MangleDefault) -> String {
        match self {
            Name::_Phantom(..) |
            Name::Empty => String::new(),
            Name::Index(index) => index.to_string(),
            Name::UnnamedArg(index) => format!("o_{}", index),
            Name::UnnamedStructFieldsComp(ty, index) => match ty {
                Type::Path(..) | Type::Array(..) => format!("_{}", *index),
                    // usize_to_tokenstream(*index).to_string(),
                // Type::Array(..) => usize_to_tokenstream(*index).to_string(),
                Type::Ptr(..) => DictionaryName::Obj.to_string(),
                _ => unimplemented!(
                    "Name::UnnamedStructFieldsComp :: to_mangled_string: unsupported type {}",
                    quote!(#ty)
                ),
            },
            Name::Constructor(ident) =>
                format!("{}_ctor", ident.mangle_string_default().replace("r#", "")),
            Name::Destructor(ident) =>
                format!("{}_destroy", ident.mangle_string_default().replace("r#", "")),
            Name::Dictionary(dict_field_name) =>
                dict_field_name.to_token_stream().to_string(),
            Name::ModFn(name) =>
                name.mangle_string(context).replace("r#", ""),
            Name::TraitObj(ident) =>
                ident.to_string().replace("r#", ""),
            Name::TraitImplVtable(item_name, trait_vtable_ident) => {
                format!("{}_{}", item_name, trait_vtable_ident)
                    .replace("r#", "")
            }
            Name::TraitImplVtableFn(item_name, trait_vtable_ident) => {
                format!("{}_{}", item_name, trait_vtable_ident)
                    .replace("r#", "")
            }
            Name::TraitFn(item_name, trait_name) =>
                format!("{}_as_{}", item_name.mangle_ident_default(), trait_name.mangle_ident_default())
                    .replace("r#", ""),
            Name::TraitDestructor(item_name, trait_name) => {
                format!("{}_as_{}_destroy", item_name.mangle_string_default(), trait_name.mangle_string_default())
                    .replace("r#", "")
            }
            Name::Vtable(trait_name) => format!("{}_VTable", trait_name),
            Name::Getter(obj_type, field_name) => format!(
                "{}_get_{}",
                obj_type.mangle_ident_default(),
                field_name.to_string().replace("r#", "")
            ),
            Name::Setter(obj_type, field_name) => format!(
                "{}_set_{}",
                obj_type.mangle_ident_default(),
                field_name.to_string().replace("r#", "")
            ),
            Name::Ident(variant) => variant.to_string().replace("r#", ""),
            Name::Optional(ident) => quote!(#ident).to_string().replace("r#", ""),
            Name::Pat(pat) => pat.to_token_stream().to_string().replace("r#", ""),
            Name::VTableInnerFn(ident) => ident.to_token_stream().to_string().replace("r#", ""),
            Name::Underscore => quote!(_).to_string(),
            Name::EnumTag(ident) => format!("{ident}_Tag").replace("r#", ""),
            Name::EnumVariantBody(ident) =>
                format!("{ident}_Body")
                    .replace("r#", ""),
            Name::Expr(expr) =>
                expr.to_token_stream().to_string()
        }
    }
}

impl<SPEC> ToTokens for Name<ObjCFermentate, SPEC>
    where SPEC: ObjCSpecification {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        match self {
            Name::Underscore =>
                quote!(_).to_tokens(tokens),
            Name::Index(index) =>
                usize_to_tokenstream(*index).to_tokens(tokens),
            Name::UnnamedArg(..) => self.mangle_tokens_default().to_tokens(tokens),
            Name::Constructor(ident) =>
                format_ident!("{}_ctor", ident.mangle_ident_default()).to_tokens(tokens),
            Name::Destructor(ident) =>
                format_ident!("{}_destroy", ident.mangle_ident_default()).to_tokens(tokens),
            Name::Dictionary(dict_field_name) => dict_field_name.to_tokens(tokens),
            Name::Vtable(trait_name) => format_ident!("{}_VTable", trait_name).to_tokens(tokens),
            Name::ModFn(path) => path.mangle_tokens_default().to_tokens(tokens),
            Name::TraitFn(item_name, trait_name) =>
                format_ident!("{}_as_{}", item_name.mangle_string_default(), trait_name.mangle_string_default()).to_tokens(tokens),
            Name::TraitDestructor(item_name, trait_name) =>
                format_ident!("{}_as_{}_destroy", item_name.mangle_string_default(), trait_name.mangle_string_default()).to_tokens(tokens),
            Name::UnnamedStructFieldsComp(ty, index) => match ty {
                Type::Ptr(_) => DictionaryName::Obj.to_tokens(tokens),
                _ => format_ident!("_{}", *index).to_tokens(tokens),
            },
            Name::TraitImplVtable(item_name, trait_vtable_ident) =>
                format_ident!("{}_{}", item_name, trait_vtable_ident).to_tokens(tokens),
            Name::TraitImplVtableFn(item_name, trait_vtable_ident) =>
                format_ident!("{}_{}", item_name, trait_vtable_ident).to_tokens(tokens),
            Name::TraitObj(ident) |
            Name::VTableInnerFn(ident) |
            Name::Ident(ident) => ident.to_tokens(tokens),
            Name::Getter(obj_type, field_name) => format_ident!(
                "{}_get_{}",
                obj_type.mangle_ident_default(),
                field_name.to_string()
            ).to_tokens(tokens),
            Name::Setter(obj_type, field_name) => format_ident!(
                "{}_set_{}",
                obj_type.mangle_ident_default(),
                field_name.to_string()
            ).to_tokens(tokens),
            Name::Optional(ident) => ident.to_tokens(tokens),
            Name::Pat(pat) => pat.to_tokens(tokens),
            Name::EnumTag(ident) => format_ident!("{ident}_Tag").to_tokens(tokens),
            Name::EnumVariantBody(ident) =>
                format_ident!("{ident}_Body")
                    .to_tokens(tokens),
            Name::Expr(expr) => expr.to_tokens(tokens),
            _ => {}
        }
    }
}
