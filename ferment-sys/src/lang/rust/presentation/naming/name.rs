use quote::{format_ident, quote, ToTokens};
use syn::__private::TokenStream2;
use syn::Type;
use crate::ext::{usize_to_tokenstream, Mangle, MangleDefault};
use crate::lang::RustSpecification;
use crate::presentation::{DictionaryName, Name};

impl ToTokens for Name<RustSpecification> {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        match self {
            Name::_Phantom(..) |
            Name::Empty => quote!(),
            Name::Index(index) => usize_to_tokenstream(*index),
            Name::UnnamedArg(..) => self.mangle_tokens_default(),
            Name::Constructor(ident) => {
                format_ident!("{}_ctor", ident.mangle_ident_default()).to_token_stream()
            }
            Name::Destructor(ident) => {
                format_ident!("{}_destroy", ident.mangle_ident_default()).to_token_stream()
            }
            Name::Read(ident) => {
                format_ident!("{}_read", ident.mangle_ident_default()).to_token_stream()
            }
            Name::Write(ident) => {
                format_ident!("{}_write", ident.mangle_ident_default()).to_token_stream()
            }
            Name::Dictionary(dict_field_name) => dict_field_name.to_token_stream(),
            Name::DictionaryExpr(dict_field_expr) => dict_field_expr.to_token_stream(),
            Name::Vtable(trait_name) => format_ident!("{}_VTable", trait_name).to_token_stream(),
            Name::TraitObj(ident) => ident.to_token_stream(),
            Name::ModFn(path) => path.mangle_tokens_default(),
            Name::TraitFn(item_name, trait_name) => {
                format_ident!("{}_as_{}", item_name.mangle_string_default(), trait_name.mangle_string_default()).to_token_stream()
            }
            Name::TraitDestructor(item_name, trait_name) => {
                format_ident!("{}_as_{}_destroy", item_name.mangle_string_default(), trait_name.mangle_string_default()).to_token_stream()
            }
            Name::UnnamedStructFieldsComp(ty, index) => match ty {
                Type::Ptr(_) => DictionaryName::Obj.to_token_stream(),
                _ => usize_to_tokenstream(* index)
            },
            Name::TraitImplVtable(item_name, trait_vtable_ident) => {
                format_ident!("{}_{}", item_name, trait_vtable_ident).to_token_stream()
            }
            Name::TraitImplVtableFn(item_name, trait_vtable_ident) => {
                format_ident!("{}_{}", item_name, trait_vtable_ident).to_token_stream()
            }
            Name::VTableInnerFn(ident) => ident.to_token_stream(),

            Name::Getter(obj_type, field_name) => format_ident!(
                "{}_get_{}",
                obj_type.mangle_ident_default(),
                field_name.to_string()
            )
                .to_token_stream(),
            Name::Setter(obj_type, field_name) => format_ident!(
                "{}_set_{}",
                obj_type.mangle_ident_default(),
                field_name.to_string()
            )
                .to_token_stream(),
            Name::Ident(variant) => quote!(#variant),
            Name::Optional(ident) => quote!(#ident),
            Name::Pat(pat) => pat.to_token_stream(),
            Name::Underscore => quote!(_),
            Name::EnumTag(ident) =>
                format_ident!("{ident}_Tag")
                    .to_token_stream(),
            Name::EnumVariantBody(ident) =>
                format_ident!("{ident}_Body")
                    .to_token_stream(),
            Name::Expr(expr) =>
                expr.to_token_stream()
        }
            .to_tokens(tokens)
    }
}

impl Mangle<MangleDefault> for Name<RustSpecification> {
    fn mangle_string(&self, context: MangleDefault) -> String {
        match self {
            Name::_Phantom(..) |
            Name::Empty => String::new(),
            Name::Index(index) => index.to_string(),
            Name::UnnamedArg(index) => format!("o_{}", index),
            Name::UnnamedStructFieldsComp(ty, index) => match ty {
                Type::Path(..) => usize_to_tokenstream(*index).to_string(),
                Type::Array(_type_array) => usize_to_tokenstream(*index).to_string(),
                Type::Ptr(_type_ptr) => DictionaryName::Obj.to_string(),
                _ => unimplemented!(
                    "Name::UnnamedStructFieldsComp :: to_mangled_string: unsupported type {}",
                    quote!(#ty)
                ),
            },
            Name::Constructor(ident) => format!("{}_ctor", ident.mangle_ident_default()),
            Name::Destructor(ident) => format!("{}_destroy", ident.mangle_ident_default()),
            Name::Read(ident) => format!("{}_read", ident.mangle_ident_default()),
            Name::Write(ident) => format!("{}_write", ident.mangle_ident_default()),
            Name::Dictionary(dict_field_name) => dict_field_name.to_token_stream().to_string(),
            Name::DictionaryExpr(dict_field_expr) => dict_field_expr.to_token_stream().to_string(),
            Name::ModFn(name) => name.mangle_string(context).to_string().replace("r#", ""),
            Name::TraitObj(ident) => ident.to_string(),
            Name::TraitImplVtable(item_name, trait_vtable_ident) => {
                format!("{}_{}", item_name, trait_vtable_ident)
            }
            Name::TraitImplVtableFn(item_name, trait_vtable_ident) => {
                format!("{}_{}", item_name, trait_vtable_ident)
            }
            Name::TraitFn(item_name, trait_name) => format!("{}_as_{}", item_name.mangle_ident_default(), trait_name.mangle_ident_default()),
            Name::TraitDestructor(item_name, trait_name) => {
                format!("{}_as_{}_destroy", item_name.mangle_ident_default(), trait_name.mangle_ident_default())
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
            Name::Ident(variant) => variant.to_string(),
            Name::Optional(ident) => quote!(#ident).to_string(),
            Name::Pat(pat) => pat.to_token_stream().to_string().replace("r#", ""),
            Name::VTableInnerFn(ident) => ident.to_token_stream().to_string(),
            Name::Underscore => quote!(_).to_string(),
            Name::EnumTag(ident) => format!("{ident}_Tag").to_string(),
            Name::EnumVariantBody(ident) => format!("{ident}_Body").to_string(),
            Name::Expr(expr) => expr.to_token_stream().to_string(),
        }
    }
}
