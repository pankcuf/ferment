use std::fmt::{Display, Formatter};
use std::marker::PhantomData;
use proc_macro2::Ident;
use quote::{format_ident, quote, ToTokens};
use syn::__private::TokenStream2;
use syn::{parse_quote, Expr, Pat, Path, Type};
use crate::ext::{Mangle, MangleDefault, ToPath, ToType, usize_to_tokenstream};
use crate::lang::{FromDictionary, NameComposable, RustSpecification, Specification};
use crate::presentation::DictionaryName;


#[derive(Clone, Debug)]
pub enum Name<SPEC> where SPEC: Specification {
    Empty,
    Expr(Expr),
    UnnamedArg(usize),
    Index(usize),
    Constructor(Type),
    Destructor(Type),
    Dictionary(DictionaryName),
    Optional(Option<Ident>),
    UnnamedStructFieldsComp(Type, usize),
    TraitObj(Ident),
    TraitImplVtable(Ident, Ident),
    TraitImplVtableFn(Ident, Ident),
    TraitFn(Type, Type),
    TraitDestructor(Type, Type),
    Vtable(Ident),
    ModFn(Path),
    VTableInnerFn(Ident),
    Getter(Path, TokenStream2),
    Setter(Path, TokenStream2),
    Ident(Ident),
    Pat(Pat),
    Underscore,
    EnumTag(Ident),
    EnumVariantBody(Ident),
    _Phantom(PhantomData<SPEC>),
}

impl<SPEC> FromDictionary for Name<SPEC>
    where SPEC: Specification {
    fn dictionary_name(dictionary: DictionaryName) -> Self {
        Name::Dictionary(dictionary)
    }
}

impl<SPEC> Default for Name<SPEC>
    where SPEC: Specification {
    fn default() -> Self {
        Name::Empty
    }
}
impl<SPEC> ToType for Name<SPEC>
    where SPEC: Specification,
          Self: ToTokens {
    fn to_type(&self) -> Type {
        parse_quote!(#self)
    }
}
impl<SPEC> ToPath for Name<SPEC>
    where SPEC: Specification,
          Self: ToTokens {
    fn to_path(&self) -> Path {
        parse_quote!(#self)
    }
}


impl<SPEC> Display for Name<SPEC>
    where SPEC: Specification,
          Self: ToTokens {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("Name({})", self.to_token_stream()).as_str())
    }
}

impl<SPEC> Name<SPEC>
    where SPEC: Specification,
          Self: ToTokens {
    pub fn getter(path: Path, field_name: &TokenStream2) -> Self {
        Self::Getter(path, field_name.clone())
    }
    pub fn setter(path: Path, field_name: &TokenStream2) -> Self {
        Self::Setter(path, field_name.clone())
    }

    pub fn anonymous(&self) -> Ident {
        format_ident!("o_{}", self.to_token_stream().to_string())
    }

}

impl<SPEC> NameComposable<SPEC> for Name<SPEC>
    where SPEC: Specification<Name=Name<SPEC>> {
    fn ident(ident: Ident) -> Self {
        Self::Ident(ident)
    }

    fn index(ident: usize) -> Self {
        Self::Index(ident)
    }

    fn unnamed_arg(index: usize) -> Self {
        Self::UnnamedArg(index)
    }
}

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
            Name::Dictionary(dict_field_name) => dict_field_name.to_token_stream(),
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
            Name::Dictionary(dict_field_name) => dict_field_name.to_token_stream().to_string(),
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
