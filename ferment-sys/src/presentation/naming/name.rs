use std::fmt::{Display, Formatter};
use std::marker::PhantomData;
use proc_macro2::Ident;
use quote::{format_ident, ToTokens};
use syn::__private::TokenStream2;
use syn::{parse_quote, Expr, Pat, Path, Type};
use crate::composable::FieldComposer;
use crate::ext::{ToPath, ToType};
use crate::kind::FieldTypeKind;
use crate::lang::{FromDictionary, NameComposable, Specification};
use crate::presentation::{DictionaryExpr, DictionaryName};


#[derive(Clone, Debug)]
pub enum Name<SPEC> where SPEC: Specification {
    Empty,
    Expr(Expr),
    UnnamedArg(usize),
    Index(usize),
    Constructor(Type),
    Destructor(Type),
    Read(Type),
    Write(Type),
    GetValueAtIndex(Type),
    SetValueAtIndex(Type),
    GetKeyByValue(Type),
    SetKeyForValue(Type),
    GetValueByKey(Type),
    SetValueForKey(Type),
    Dictionary(DictionaryName),
    DictionaryExpr(DictionaryExpr),
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
    where SPEC: Specification<Name=Self>,
          Self: ToTokens {
    
    
    pub fn o() -> Self {
        Self::Dictionary(DictionaryName::O)
    }
    pub fn obj() -> Self {
        Self::Dictionary(DictionaryName::Obj)
    }
    pub fn self_() -> Self {
        Self::Dictionary(DictionaryName::Self_)
    }
    pub fn keys() -> Self {
        Self::Dictionary(DictionaryName::Keys)
    }
    pub fn values() -> Self {
        Self::Dictionary(DictionaryName::Values)
    }
    pub fn count() -> Self {
        Self::Dictionary(DictionaryName::Count)
    }
    pub fn ctor(ty: &Type) -> Self {
        Self::Constructor(ty.clone())
    }
    pub fn getter(path: Path, field_name: &TokenStream2) -> Self {
        Self::Getter(path, field_name.clone())
    }
    pub fn setter(path: Path, field_name: &TokenStream2) -> Self {
        Self::Setter(path, field_name.clone())
    }

    pub fn pat_tokens<T: ToTokens>(tokens: T) -> Self {
        Self::Pat(Pat::Verbatim(tokens.to_token_stream()))
    }
    pub fn self_prop<T: ToTokens>(field_name: T) -> Self {
        Self::DictionaryExpr(DictionaryExpr::self_prop(field_name))
    }

    pub fn pat(pat: &Pat) -> Self {
        Self::Pat(pat.clone())
    }

    pub fn anonymous(&self) -> Ident {
        format_ident!("o_{}", self.to_token_stream().to_string())
    }

    pub fn field_composer(&self, kind: FieldTypeKind<SPEC>) -> FieldComposer<SPEC> {
        FieldComposer::<SPEC>::named_no_attrs(self.clone(), kind)
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

