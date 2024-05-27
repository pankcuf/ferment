use std::collections::HashSet;
use std::fmt::{Debug, Display, Formatter};
use proc_macro2::Ident;
use quote::{quote, quote_spanned, ToTokens};
use syn::{AngleBracketedGenericArguments, Attribute, GenericArgument, parse_quote, Path, PathArguments, PathSegment, spanned::Spanned, Type, TypeArray, TypeParamBound, TypePath, TypeSlice, TypeTuple};
use syn::__private::TokenStream2;
use crate::composer::{AddPunctuated, BraceWrapped, CommaPunctuated, ComposerPresenter, ConstructorPresentableContext, Depunctuated, ParentComposer};
use crate::context::ScopeContext;
use crate::conversion::{FieldTypeConversion, FieldTypeConversionKind, TypeConversion};
use crate::conversion::macro_conversion::merge_attributes;
use crate::ext::{Accessory, DictionaryType, FFITypeResolve, GenericNestedArg, Mangle, Resolve, Terminated, ToPath, ToType};
use crate::helper::usize_to_tokenstream;
use crate::interface::create_struct;
use crate::naming::{DictionaryExpr, DictionaryName, FFIConversionMethod, FFIVecConversionMethodExpr, InterfacesMethodExpr, Name};
use crate::presentation::context::{BindingPresentableContext, FieldContext, IteratorPresentationContext, OwnedItemPresentableContext};
use crate::presentation::{DropInterfacePresentation, FFIObjectPresentation, FromConversionPresentation, InterfacePresentation, ScopeContextPresentable, ToConversionPresentation};
use crate::presentation::destroy_presentation::DestroyPresentation;

pub type InterfacesMethodComposer = ComposerPresenter<TokenStream2, InterfacesMethodExpr>;
pub const FROM_OPT_PRIMITIVE: InterfacesMethodComposer = |expr| InterfacesMethodExpr::FromOptPrimitive(expr);
pub const TO_OPT_PRIMITIVE: InterfacesMethodComposer = |expr| InterfacesMethodExpr::ToOptPrimitive(expr);
pub const FROM_COMPLEX: InterfacesMethodComposer = |expr| InterfacesMethodExpr::FFIConversion(FFIConversionMethod::FfiFrom, expr);
pub const FROM_OPT_COMPLEX: InterfacesMethodComposer = |expr| InterfacesMethodExpr::FFIConversion(FFIConversionMethod::FfiFromOpt, expr);

pub const TO_COMPLEX: InterfacesMethodComposer = |expr| InterfacesMethodExpr::FFIConversion(FFIConversionMethod::FfiTo, expr);
pub const TO_OPT_COMPLEX: InterfacesMethodComposer = |expr| InterfacesMethodExpr::FFIConversion(FFIConversionMethod::FfiToOpt, expr);

pub const FROM_PRIMITIVE_GROUP: InterfacesMethodComposer = |expr| InterfacesMethodExpr::FromPrimitiveGroup(expr);
pub const FROM_COMPLEX_GROUP: InterfacesMethodComposer = |expr| InterfacesMethodExpr::FromComplexGroup(expr);
pub const FROM_OPT_PRIMITIVE_GROUP: InterfacesMethodComposer = |expr| InterfacesMethodExpr::FromOptPrimitiveGroup(expr);
pub const FROM_OPT_COMPLEX_GROUP: InterfacesMethodComposer = |expr| InterfacesMethodExpr::FromOptComplexGroup(expr);

pub const TO_PRIMITIVE_GROUP: InterfacesMethodComposer = |expr| InterfacesMethodExpr::ToPrimitiveGroup(expr);
pub const TO_COMPLEX_GROUP: InterfacesMethodComposer = |expr| InterfacesMethodExpr::ToComplexGroup(expr);
pub const TO_OPT_PRIMITIVE_GROUP: InterfacesMethodComposer = |expr| InterfacesMethodExpr::ToOptPrimitiveGroup(expr);
pub const TO_OPT_COMPLEX_GROUP: InterfacesMethodComposer = |expr| InterfacesMethodExpr::ToOptComplexGroup(expr);

pub const DESTROY_PRIMITIVE_GROUP: InterfacesMethodComposer = |expr| InterfacesMethodExpr::UnboxVecPtr(expr);
pub const DESTROY_COMPLEX_GROUP: InterfacesMethodComposer = |expr| InterfacesMethodExpr::UnboxAnyVecPtr(expr);
pub const DESTROY_COMPLEX: InterfacesMethodComposer = |expr| InterfacesMethodExpr::UnboxAny(expr);

// pub const BOXED_VEC: InterfacesMethodComposer = |expr| InterfacesMethodExpr::BoxedVec(expr);
pub const DESTROY_OPT_PRIMITIVE: InterfacesMethodComposer = |expr| InterfacesMethodExpr::DestroyOptPrimitive(expr);

pub struct ArgComposer {
    // pub ty: Type,
    pub from_composer: InterfacesMethodComposer,
    pub to_composer: InterfacesMethodComposer,
    pub destroy_composer: InterfacesMethodComposer,
}
impl ArgComposer {
    pub fn new(from_composer: InterfacesMethodComposer, to_composer: InterfacesMethodComposer, destroy_composer: InterfacesMethodComposer) -> Self {
        Self { from_composer, to_composer, destroy_composer }
    }
    pub fn from(&self, expr: TokenStream2) -> InterfacesMethodExpr {
        (self.from_composer)(expr)
    }
    pub fn to(&self, expr: TokenStream2) -> InterfacesMethodExpr {
        (self.to_composer)(expr)
    }
    pub fn destroy(&self, expr: TokenStream2) -> InterfacesMethodExpr {
        (self.destroy_composer)(expr)
    }
}

pub type GenericNestedArgComposer = fn(arg_name: &Name, arg_ty: &Type) -> GenericArgPresentation;

#[allow(unused)]
pub trait GenericNamedArgComposer {
    fn compose_with(&self, name: &Name, composer: GenericNestedArgComposer) -> GenericArgPresentation;
}

impl GenericNamedArgComposer for Type {
    fn compose_with(&self, name: &Name, composer: GenericNestedArgComposer) -> GenericArgPresentation {
        composer(name, self)
    }
}

pub struct GenericArgPresentation {
    pub ty: Type,
    pub destructor: FieldContext,
    pub from_conversion: FieldContext,
    pub to_conversion: FieldContext,
}

impl Debug for GenericArgPresentation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("GenericArgPresentation({})", self.ty.to_token_stream()))
    }
}
impl Display for GenericArgPresentation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}

impl GenericArgPresentation {
    pub fn new(ty: Type, destructor: FieldContext, from_conversion: FieldContext, to_conversion: FieldContext) -> Self {
        Self { ty, destructor, from_conversion, to_conversion }
    }
}

#[derive(Clone, PartialEq, Eq)]
pub enum SmartPtr {
    Arc,
    Rc,
    Mutex,
    RwLock,
}

#[derive(Clone, PartialEq, Eq)]
pub enum GenericTypeConversion {
    Map(Type),
    IndexMap(Type),
    SerdeJsonMap(Type),
    Vec(Type),
    BTreeSet(Type),
    HashSet(Type),
    Result(Type),
    Box(Type),
    AnyOther(Type),
    Array(Type),
    Slice(Type),
    Tuple(Type),
    Optional(Type),
    TraitBounds(AddPunctuated<TypeParamBound>)
}
impl Debug for GenericTypeConversion {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("GenericTypeConversion::{}({})", match self {
            GenericTypeConversion::Map(_) => "Map",
            GenericTypeConversion::IndexMap(_) => "IndexMap",
            GenericTypeConversion::SerdeJsonMap(_) => "SerdeJsonMap",
            GenericTypeConversion::Vec(_) => "Vec",
            GenericTypeConversion::BTreeSet(_) => "BTreeSet",
            GenericTypeConversion::HashSet(_) => "HashSet",
            GenericTypeConversion::Result(_) => "Result",
            GenericTypeConversion::Box(_) => "Box",
            GenericTypeConversion::AnyOther(_) => "AnyOther",
            GenericTypeConversion::Array(_) => "Array",
            GenericTypeConversion::Slice(_) => "Slice",
            GenericTypeConversion::Tuple(_) => "Tuple",
            GenericTypeConversion::TraitBounds(_) => "TraitBounds",
            GenericTypeConversion::Optional(_) => "Optional"
        }, self.to_token_stream()))
    }
}
impl Display for GenericTypeConversion {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}

impl ToTokens for GenericTypeConversion {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        match self {
            GenericTypeConversion::Map(ty) |
            GenericTypeConversion::IndexMap(ty) |
            GenericTypeConversion::SerdeJsonMap(ty) |
            GenericTypeConversion::Vec(ty) |
            GenericTypeConversion::BTreeSet(ty) |
            GenericTypeConversion::HashSet(ty) |
            GenericTypeConversion::Result(ty) |
            GenericTypeConversion::Box(ty) |
            GenericTypeConversion::Array(ty) |
            GenericTypeConversion::Slice(ty) |
            GenericTypeConversion::AnyOther(ty) |
            GenericTypeConversion::Optional(ty) |
            GenericTypeConversion::Tuple(ty) => ty.to_tokens(tokens),
            GenericTypeConversion::TraitBounds(bounds) => bounds.to_tokens(tokens),
        }
    }
}
impl GenericTypeConversion {
    pub fn ty(&self) -> Option<&Type> {
        match self {
            GenericTypeConversion::Map(ty) |
            GenericTypeConversion::IndexMap(ty) |
            GenericTypeConversion::SerdeJsonMap(ty) |
            GenericTypeConversion::Vec(ty) |
            GenericTypeConversion::BTreeSet(ty) |
            GenericTypeConversion::HashSet(ty) |
            GenericTypeConversion::Result(ty) |
            GenericTypeConversion::Box(ty) |
            GenericTypeConversion::Array(ty) |
            GenericTypeConversion::Slice(ty) |
            GenericTypeConversion::AnyOther(ty) |
            GenericTypeConversion::Tuple(ty) => Some(ty),
            GenericTypeConversion::Optional(ty) => match ty {
                Type::Path(TypePath { qself: _, path }) => match path.segments.last() {
                    Some(last_segment) => match &last_segment.arguments {
                        PathArguments::AngleBracketed(AngleBracketedGenericArguments { args, .. }) => match args.first() {
                            Some(generic_argument) => match generic_argument {
                                GenericArgument::Type(ty) => Some(ty),
                                _ => panic!("TODO: Non-supported optional type as generic argument (PathArguments::AngleBracketed: Non-Type Generic): {}", ty.to_token_stream()),
                            },
                            _ => panic!("TODO: Non-supported optional type as generic argument (PathArguments::AngleBracketed: Empty): {}", ty.to_token_stream()),
                        }
                        _ => panic!("TODO: Non-supported optional type as generic argument (PathArguments): {}", ty.to_token_stream()),
                    },
                    None => unimplemented!("TODO: Non-supported optional type as generic argument (Empty last segment): {}", ty.to_token_stream()),
                },
                _ => unimplemented!("TODO: Non-supported optional type as generic argument (Type): {}", ty.to_token_stream()),
            }
            GenericTypeConversion::TraitBounds(_) => {
                // TODO: Make mixin here
                None
            }
        }
    }
    pub fn to_ffi_type(&self) -> Type {
        match self {
            GenericTypeConversion::Map(ty) |
            GenericTypeConversion::IndexMap(ty) |
            GenericTypeConversion::SerdeJsonMap(ty) |
            GenericTypeConversion::Vec(ty) |
            GenericTypeConversion::BTreeSet(ty) |
            GenericTypeConversion::HashSet(ty) |
            GenericTypeConversion::Result(ty) |
            GenericTypeConversion::Box(ty) |
            GenericTypeConversion::AnyOther(ty) =>
                single_generic_ffi_type(ty),
            GenericTypeConversion::Array(ty) |
            GenericTypeConversion::Slice(ty) => {
                let ffi_name = ty.mangle_ident_default();
                parse_quote!(crate::fermented::generics::#ffi_name)
            }
            GenericTypeConversion::Tuple(tuple) => {
                let tuple: TypeTuple = parse_quote!(#tuple);
                match tuple.elems.len() {
                    0 => single_generic_ffi_type(tuple.elems.first().unwrap()),
                    _ => {
                        let ffi_name = tuple.mangle_ident_default();
                        parse_quote!(crate::fermented::generics::#ffi_name)
                    }
                }
            }
            GenericTypeConversion::Optional(ty) => {
                match ty {
                    Type::Path(TypePath { qself: _, path }) => match path.segments.last() {
                        Some(last_segment) => {
                            match &last_segment.arguments {
                                PathArguments::None => panic!("Empty optional arguments as generic argument (PathArguments::None): {}", ty.to_token_stream()),
                                PathArguments::AngleBracketed(AngleBracketedGenericArguments { args, .. }) => {
                                    match args.first() {
                                       Some(generic_argument) => match generic_argument {
                                           GenericArgument::Type(ty) => {
                                               let result = match TypeConversion::from(ty) {
                                                   TypeConversion::Primitive(_) => single_generic_ffi_type(ty),
                                                   TypeConversion::Complex(_) => single_generic_ffi_type(ty),
                                                   TypeConversion::Callback(_) => single_generic_ffi_type(ty),
                                                   TypeConversion::Generic(gen) => gen.to_ffi_type(),
                                               };

                                               println!("OPTionalllll: {} --- {}", ty.to_token_stream(), result.to_token_stream());
                                               // single_generic_ffi_type(ty)
                                               result
                                           },
                                           _ => panic!("TODO: Non-supported optional type as generic argument (PathArguments::AngleBracketed: Non-Type Generic): {}", ty.to_token_stream()),
                                       },
                                        _ => panic!("TODO: Non-supported optional type as generic argument (PathArguments::AngleBracketed: Empty): {}", ty.to_token_stream()),
                                    }
                                }
                                PathArguments::Parenthesized(_) => panic!("TODO: Non-supported optional type as generic argument (PathArguments::Parenthesized): {}", ty.to_token_stream()),
                            }
                        },
                        None => unimplemented!("TODO: Non-supported optional type as generic argument (Empty last segment): {}", ty.to_token_stream()),
                    },
                    Type::Array(TypeArray { elem, .. }) => single_generic_ffi_type(elem),
                    _ => unimplemented!("TODO: Non-supported optional type as generic argument (Type): {}", ty.to_token_stream()),
                }
            }
            GenericTypeConversion::TraitBounds(ty) =>
                unimplemented!("TODO: TraitBounds when generic expansion: {}", ty.to_token_stream()),
        }
    }

}

impl GenericTypeConversion {

    pub fn expand(&self, attrs: &HashSet<Option<Attribute>>, scope_context: &ParentComposer<ScopeContext>) -> TokenStream2 {
        let source = scope_context.borrow();
        let attrs = merge_attributes(attrs);
        let attrs = (!attrs.is_empty()).then(|| quote!(#[cfg(#attrs)])).unwrap_or_default();
        println!("GenericTypeConversion::expand.1: {} ---- {}", self, attrs.to_token_stream());

        match self {
            GenericTypeConversion::Result(ty) => {
                let ffi_name = ty.mangle_ident_default();
                let ffi_as_type = ffi_name.to_type();
                let arg_0_name = Name::Dictionary(DictionaryName::Ok);
                let arg_1_name = Name::Dictionary(DictionaryName::Error);
                let compose_arg = |arg_ty: Type, from_expr: FieldContext, to_expr: FieldContext, destroy_expr: FieldContext|
                    GenericArgPresentation::new(
                        arg_ty,
                        destroy_expr,
                        FieldContext::MapExpression(FieldContext::O.into(), from_expr.into()),
                        to_expr);
                let compose = |arg_name: &Name, ty: &Type| match TypeConversion::from(ty) {
                    TypeConversion::Callback(_) => unimplemented!("Callback: {}", ty.to_token_stream()),
                    TypeConversion::Primitive(arg_ty) => {
                        compose_arg(
                            arg_ty.clone(),
                            FieldContext::Deref(DictionaryName::O.to_token_stream()),
                            FieldContext::AsMut_(FieldContext::O.into()),
                            FieldContext::Empty)
                    }
                    TypeConversion::Complex(arg_ty) => {
                        let arg_composer = ArgComposer::new(FROM_COMPLEX, TO_COMPLEX, DESTROY_COMPLEX);
                        compose_arg(
                            arg_ty.to_custom_or_ffi_type(&source),
                            FieldContext::InterfacesExpr(arg_composer.from(DictionaryName::O.to_token_stream())),
                            FieldContext::InterfacesExpr(arg_composer.to(DictionaryName::O.to_token_stream())),
                            FieldContext::InterfacesExpr(arg_composer.destroy(DictionaryExpr::SelfProp(arg_name.to_token_stream()).to_token_stream())))
                    }
                    TypeConversion::Generic(generic_arg_ty) => {
                        let (arg_composer, arg_ty) = if let GenericTypeConversion::Optional(..) = generic_arg_ty {
                            println!("RESULT OPT ARG: {}", generic_arg_ty);
                            match generic_arg_ty.ty() {
                                None => unimplemented!("Mixin inside generic: {}", generic_arg_ty),
                                Some(ty) => match TypeConversion::from(ty) {
                                    TypeConversion::Callback(_) => unimplemented!("Callback inside generic: {}", ty.to_token_stream()),
                                    TypeConversion::Primitive(_) => (ArgComposer::new(FROM_OPT_PRIMITIVE, TO_OPT_PRIMITIVE, DESTROY_OPT_PRIMITIVE), ty.to_custom_or_ffi_type(&source)),
                                    TypeConversion::Generic(nested_nested) => (ArgComposer::new(FROM_OPT_COMPLEX, TO_OPT_COMPLEX, DESTROY_COMPLEX), nested_nested.to_custom_or_ffi_type(&source)),
                                    _ => (ArgComposer::new(FROM_OPT_COMPLEX, TO_OPT_COMPLEX, DESTROY_COMPLEX), ty.to_custom_or_ffi_type(&source)),
                                }
                            }
                        } else { (ArgComposer::new(FROM_COMPLEX, TO_COMPLEX, DESTROY_COMPLEX), generic_arg_ty.to_custom_or_ffi_type(&source)) };
                        compose_arg(
                            arg_ty,
                            FieldContext::InterfacesExpr(arg_composer.from(DictionaryName::O.to_token_stream())),
                            FieldContext::InterfacesExpr(arg_composer.to(DictionaryName::O.to_token_stream())),
                            FieldContext::InterfacesExpr(arg_composer.destroy(DictionaryExpr::SelfProp(arg_name.to_token_stream()).to_token_stream())))
                    }
                };

                let nested_types = ty.nested_types();
                let arg_0_presentation = compose(&arg_0_name, nested_types[0]);
                let arg_1_presentation = compose(&arg_1_name, nested_types[1]);
                compose_generic_presentation(
                    ffi_name,
                    attrs.clone(),
                    Depunctuated::from_iter([
                        FieldTypeConversion::named(arg_0_name, FieldTypeConversionKind::Type(arg_0_presentation.ty.joined_mut())),
                        FieldTypeConversion::named(arg_1_name, FieldTypeConversionKind::Type(arg_1_presentation.ty.joined_mut())),
                    ]),
                    Depunctuated::from_iter([
                        InterfacePresentation::Conversion {
                            attrs,
                            types: (ffi_as_type.clone(), ty.clone()),
                            conversions: (
                                FromConversionPresentation::Result(arg_0_presentation.from_conversion.present(&source), arg_1_presentation.from_conversion.present(&source)),
                                ToConversionPresentation::Result(arg_0_presentation.to_conversion.present(&source), arg_1_presentation.to_conversion.present(&source)),
                                DestroyPresentation::Default,
                                None
                            )
                        }
                    ]),
                    Depunctuated::from_iter([arg_0_presentation.destructor.present(&source).terminated(), arg_1_presentation.destructor.present(&source).terminated()]),
                    &source
                )
            },
            GenericTypeConversion::Map(ty) |
            GenericTypeConversion::IndexMap(ty) |
            GenericTypeConversion::SerdeJsonMap(ty)=> {
                let ffi_name = ty.mangle_ident_default();
                let ffi_as_type = ffi_name.to_type();


                let arg_0_name = Name::Dictionary(DictionaryName::Keys);
                let arg_1_name = Name::Dictionary(DictionaryName::Values);
                let count_name = Name::Dictionary(DictionaryName::Count);

                let arg_context = |arg_name: &Name| quote!(obj.#arg_name().cloned());
                let arg_args = |arg_name: &Name| CommaPunctuated::from_iter([
                    DictionaryExpr::SelfProp(arg_name.to_token_stream()),
                    DictionaryExpr::SelfProp(count_name.to_token_stream())]);

                let compose_arg = |arg_ty: Type, from_expr: FieldContext, to_expr: InterfacesMethodExpr, destroy_expr: InterfacesMethodExpr|
                    GenericArgPresentation::new(
                        arg_ty,
                        FieldContext::InterfacesExpr(destroy_expr),
                        FieldContext::MapExpression(FieldContext::O.into(), from_expr.into()),
                        FieldContext::InterfacesExpr(to_expr));
                let compose = |arg_name: &Name, ty: &Type| match TypeConversion::from(ty) {
                    TypeConversion::Primitive(arg_ty) => {
                        compose_arg(
                            arg_ty.clone(),
                            FieldContext::O,
                            TO_PRIMITIVE_GROUP(arg_context(arg_name)),
                            DESTROY_PRIMITIVE_GROUP(arg_args(arg_name).to_token_stream()))
                    },
                    TypeConversion::Complex(arg_ty) => {
                        let arg_composer = ArgComposer::new(FROM_COMPLEX, TO_COMPLEX_GROUP, DESTROY_COMPLEX_GROUP);
                        compose_arg(
                            arg_ty.to_custom_or_ffi_type_mut_ptr(&source),
                            FieldContext::InterfacesExpr(arg_composer.from(DictionaryName::O.to_token_stream())).into(),
                            arg_composer.to(arg_context(arg_name)),
                            arg_composer.destroy(arg_args(arg_name).to_token_stream())
                        )
                    },
                    TypeConversion::Generic(generic_arg_ty) => {
                        let (arg_composer, arg_ty) = if let GenericTypeConversion::Optional(..) = generic_arg_ty {
                            match generic_arg_ty.ty() {
                                None => unimplemented!("Mixin inside generic: {}", generic_arg_ty),
                                Some(ty) => (match TypeConversion::from(ty) {
                                    TypeConversion::Callback(_) => unimplemented!("Callback inside generic: {}", ty.to_token_stream()),
                                    TypeConversion::Primitive(_) => ArgComposer::new(FROM_OPT_PRIMITIVE, TO_OPT_PRIMITIVE_GROUP, DESTROY_COMPLEX_GROUP),
                                    _ => ArgComposer::new(FROM_OPT_COMPLEX, TO_OPT_COMPLEX_GROUP, DESTROY_COMPLEX_GROUP),
                                }, ty.to_custom_or_ffi_type_mut_ptr(&source))
                            }
                        } else { (ArgComposer::new(FROM_COMPLEX, TO_COMPLEX_GROUP, DESTROY_COMPLEX_GROUP), generic_arg_ty.to_custom_or_ffi_type_mut_ptr(&source)) };
                        compose_arg(
                            arg_ty,
                            FieldContext::InterfacesExpr(arg_composer.from(DictionaryName::O.to_token_stream())),
                            arg_composer.to(arg_context(arg_name)),
                            arg_composer.destroy(arg_args(arg_name).to_token_stream())
                        )
                    },
                    TypeConversion::Callback(_) => unimplemented!("Callback: {}", ty.to_token_stream())
                };

                let nested_types = ty.nested_types();
                let arg_0_presentation = compose(&arg_0_name, nested_types[0]);
                let arg_1_presentation = compose(&arg_1_name, nested_types[1]);
                let GenericArgPresentation { ty: key, from_conversion: from_key_conversion, to_conversion: to_key_conversion, destructor: key_destructor } = arg_0_presentation;
                let GenericArgPresentation { ty: value, from_conversion: from_value_conversion, to_conversion: to_value_conversion, destructor: value_destructor } = arg_1_presentation;
                compose_generic_presentation(
                    ffi_name,
                    attrs.clone(),
                    Depunctuated::from_iter([
                        FieldTypeConversion::named(count_name, FieldTypeConversionKind::Type(parse_quote!(usize))),
                        FieldTypeConversion::named(arg_0_name,FieldTypeConversionKind::Type(key.joined_mut())),
                        FieldTypeConversion::named(arg_1_name, FieldTypeConversionKind::Type(value.joined_mut()))
                    ]),
                    Depunctuated::from_iter([
                        InterfacePresentation::Conversion {
                            attrs: attrs.clone(),
                            types: (ffi_as_type.clone(), ty.clone()),
                            conversions: (
                                FromConversionPresentation::Map(from_key_conversion.present(&source), from_value_conversion.present(&source)),
                                ToConversionPresentation::Map(to_key_conversion.present(&source), to_value_conversion.present(&source)),
                                DestroyPresentation::Default,
                                None
                            )
                        }
                    ]),
                    Depunctuated::from_iter([key_destructor.present(&source).terminated(), value_destructor.present(&source).terminated()]),
                    &source
                )
            },
            GenericTypeConversion::BTreeSet(ty) |
            GenericTypeConversion::HashSet(ty) |
            GenericTypeConversion::Vec(ty) => {
                let nested_ty = ty.first_nested_type().unwrap();
                compose_generic_group(
                    ty,
                    ty.clone(),
                    TypeConversion::from(nested_ty),
                    FromConversionPresentation::Just(FFIVecConversionMethodExpr::Decode(DictionaryExpr::FfiDerefAsRef.to_token_stream()).to_token_stream()),
                    ToConversionPresentation::Simple(FFIVecConversionMethodExpr::Encode(DictionaryName::Obj.to_token_stream()).to_token_stream()),
                    attrs,
                    &source)
            },
            GenericTypeConversion::Array(ty) => {
                let nested_ty = ty.first_nested_type().unwrap();
                compose_generic_group(
                    ty,
                    parse_quote!(Vec<#nested_ty>),
                    TypeConversion::from(nested_ty),
                    FromConversionPresentation::TryInto(FFIVecConversionMethodExpr::Decode(DictionaryExpr::FfiDerefAsRef.to_token_stream()).to_token_stream()),
                    ToConversionPresentation::Simple(FFIVecConversionMethodExpr::Encode(DictionaryExpr::ObjToVec.to_token_stream()).to_token_stream()),
                    attrs,
                    &source)
            },
            GenericTypeConversion::Slice(ty) => {
                let ffi_name = ty.mangle_ident_default();
                let ffi_as_type = ffi_name.to_type();
                let type_slice: TypeSlice = parse_quote!(#ty);
                let elem_type = &type_slice.elem;
                let target_type: Type = parse_quote!(Vec<#elem_type>);
                let arg_0_name = Name::Dictionary(DictionaryName::Values);
                let count_name = Name::Dictionary(DictionaryName::Count);
                let self_props = CommaPunctuated::from_iter([
                    DictionaryExpr::SelfProp(arg_0_name.to_token_stream()),
                    DictionaryExpr::SelfProp(count_name.to_token_stream())]);
                let arg_0_destroy = |composer: InterfacesMethodComposer| FieldContext::InterfacesExpr(composer(self_props.to_token_stream()));
                let arg_0_from = |composer: InterfacesMethodComposer| FieldContext::InterfacesExpr(composer(self_props.to_token_stream()));
                let arg_0_to = |composer: InterfacesMethodComposer|
                    FieldContext::InterfacesExpr(
                        InterfacesMethodExpr::Boxed(
                            DictionaryExpr::NamedStructInit(
                                CommaPunctuated::from_iter([
                                    FieldTypeConversion::named(count_name.clone(), FieldTypeConversionKind::Conversion(DictionaryExpr::ObjLen.to_token_stream())),
                                    FieldTypeConversion::named(arg_0_name.clone(), FieldTypeConversionKind::Conversion(composer(DictionaryExpr::ObjIntoIter.to_token_stream()).to_token_stream()))]))
                                .to_token_stream()));

                let arg_0_presentation = match TypeConversion::from(&type_slice.elem) {
                    TypeConversion::Callback(arg_0_target_ty) =>
                        unimplemented!("Callbacks are not implemented in generics: {}", arg_0_target_ty.to_token_stream()),
                    TypeConversion::Primitive(arg_0_target_path) => {
                        GenericArgPresentation::new(
                            arg_0_target_path.clone(),
                            arg_0_destroy(DESTROY_PRIMITIVE_GROUP),
                            arg_0_from(FROM_PRIMITIVE_GROUP),
                            arg_0_to(TO_PRIMITIVE_GROUP))
                    }
                    TypeConversion::Complex(arg_0_target_ty) => {
                        GenericArgPresentation::new(
                            arg_0_target_ty.to_custom_or_ffi_type_mut_ptr(&source),
                            arg_0_destroy(DESTROY_COMPLEX_GROUP),
                            arg_0_from(FROM_COMPLEX_GROUP),
                            arg_0_to(TO_COMPLEX_GROUP))
                    }
                    TypeConversion::Generic(arg_0_generic_path_conversion) => {
                        GenericArgPresentation::new(
                            arg_0_generic_path_conversion.to_custom_or_ffi_type_mut_ptr(&source),
                            arg_0_destroy(DESTROY_COMPLEX_GROUP),
                            arg_0_from(FROM_COMPLEX_GROUP),
                            arg_0_to(TO_COMPLEX_GROUP))
                    }
                };
                let GenericArgPresentation { ty: value, from_conversion: decode, to_conversion: encode, destructor: value_destructor } = arg_0_presentation;

                compose_generic_presentation(
                    ffi_name,
                    attrs.clone(),
                    Depunctuated::from_iter([
                        FieldTypeConversion::named(count_name, FieldTypeConversionKind::Type(parse_quote!(usize))),
                        FieldTypeConversion::named(arg_0_name, FieldTypeConversionKind::Type(value.joined_mut()))
                    ]),
                    Depunctuated::from_iter([
                        InterfacePresentation::Conversion {
                            attrs: attrs.clone(),
                            types: (ffi_as_type.clone(), target_type.clone()),
                            conversions: (
                                FromConversionPresentation::Just(FFIVecConversionMethodExpr::Decode(DictionaryExpr::FfiDerefAsRef.to_token_stream()).to_token_stream()),
                                ToConversionPresentation::Simple(FFIVecConversionMethodExpr::Encode(DictionaryName::Obj.to_token_stream()).to_token_stream()),
                                DestroyPresentation::Default,
                                None
                            )
                        },
                        InterfacePresentation::VecConversion { attrs: attrs.clone(), types: (ffi_as_type, target_type), decode: decode.present(&source), encode: encode.present(&source) }
                    ]),
                    Depunctuated::from_iter([value_destructor.present(&source).terminated()]),
                    &source
                )
            },
            GenericTypeConversion::Tuple(ty) => {
                let ffi_name = ty.mangle_ident_default();
                let ffi_as_type = ffi_name.to_type();
                let type_tuple: TypeTuple = parse_quote!(#ty);
                let tuple_items = type_tuple.elems.iter()
                    .enumerate()
                    .map(|(index, ty)|
                        dictionary_generic_arg(
                            Name::UnnamedArg(index),
                            usize_to_tokenstream(index),
                            ty,
                            &source))
                    .collect::<Depunctuated<(Type, Depunctuated<GenericArgPresentation>)>>();
                compose_generic_presentation(
                    ffi_name,
                    attrs.clone(),
                    Depunctuated::from_iter(
                        tuple_items.iter()
                            .enumerate()
                            .map(|(index, (root_path, _))| FieldTypeConversion::unnamed(Name::UnnamedArg(index), FieldTypeConversionKind::Type(root_path.clone())))),
                    Depunctuated::from_iter([
                        InterfacePresentation::Conversion {
                            attrs: attrs.clone(),
                            types: (ffi_as_type, parse_quote!(#ty)),
                            conversions: (
                                FromConversionPresentation::Tuple(tuple_items.iter().flat_map(|(_, args)| args.iter().map(|item| item.from_conversion.present(&source))).collect()),
                                ToConversionPresentation::Tuple(tuple_items.iter().flat_map(|(_, args)| args.iter().map(|item| item.to_conversion.present(&source))).collect()),
                                DestroyPresentation::Default,
                                None
                            )
                        }
                    ]),
                    Depunctuated::from_iter(tuple_items.iter().flat_map(|(_, args)| args.iter().map(|item| item.destructor.present(&source).terminated()))),
                    &source
                )
            },
            GenericTypeConversion::AnyOther(ty) => {
                let ffi_name = ty.mangle_ident_default();
                let ffi_type = ffi_name.to_type();
                let arg_0_name = Name::Dictionary(DictionaryName::Obj);

                let mut path = ty.to_path();
                path.segments.last_mut().unwrap().arguments = PathArguments::None;

                // Arc/Rc: primitive arg: to: "*obj"
                // Arc/Rc: complex arg: to: "(*obj).clone()"
                // Mutex/RwLock: primitive/complex arg: to: "obj.into_inner().expect("Err")"
                // RefCell: primitive/complex arg: to: "obj.into_inner()"

                let arg_to_conversion = match &path.segments.last() {
                    Some(PathSegment { ident, .. }) => match ident.to_string().as_str() {
                        "Arc" | "Rc" => match TypeConversion::from(ty) {
                            TypeConversion::Primitive(_) => DictionaryExpr::Deref(arg_0_name.to_token_stream()).to_token_stream(),
                            TypeConversion::Complex(_) => quote!((*#arg_0_name).clone()),
                            TypeConversion::Generic(_) => quote!((*#arg_0_name).clone()),
                            TypeConversion::Callback(_) => panic!("Errror")
                        },
                        "Mutex" | "RwLock" => quote!(#arg_0_name.into_inner().expect("Err")),
                        "RefCell" => quote!(#arg_0_name.into_inner()),
                        "Pin" => quote!(&**#arg_0_name),
                        _ => panic!("Error Generic Expansion (Non Supported AnyOther): {}", ty.to_token_stream())
                    }
                    None => {
                        panic!("Error Generic Expansion (AnyOther): {}", ty.to_token_stream())
                    }
                };

                let compose_arg = |arg_ty: Type, from_expr: FieldContext, to_expr: FieldContext, destroy_expr: FieldContext|
                    GenericArgPresentation::new(
                        arg_ty,
                        destroy_expr,
                        from_expr,
                        to_expr);
                let compose = |arg_name: &Name, ty: &Type| match TypeConversion::from(ty) {
                    TypeConversion::Callback(_) => unimplemented!("Callback: {}", ty.to_token_stream()),
                    TypeConversion::Primitive(arg_ty) => {
                        compose_arg(
                            arg_ty.clone(),
                            FieldContext::FfiRefWithFieldName(FieldContext::DictionaryName(DictionaryName::Obj).into()),
                            FieldContext::InterfacesExpr(
                                InterfacesMethodExpr::Boxed(
                                    DictionaryExpr::NamedStructInit(
                                        CommaPunctuated::from_iter([
                                            FieldTypeConversion::named(arg_0_name.clone(), FieldTypeConversionKind::Conversion(arg_to_conversion.to_token_stream()))
                                        ])).to_token_stream())),
                            FieldContext::Empty)
                    }
                    TypeConversion::Complex(arg_ty) => {
                        let arg_composer = ArgComposer::new(FROM_COMPLEX, TO_COMPLEX, DESTROY_COMPLEX);
                        compose_arg(
                            arg_ty.to_custom_or_ffi_type(&source),
                            FieldContext::InterfacesExpr(arg_composer.from(quote!(ffi_ref.#arg_0_name))),
                            FieldContext::InterfacesExpr(
                                InterfacesMethodExpr::Boxed(
                                    DictionaryExpr::NamedStructInit(
                                        CommaPunctuated::from_iter([
                                            FieldTypeConversion::named(arg_0_name.clone(), FieldTypeConversionKind::Conversion(arg_composer.to(arg_to_conversion).to_token_stream()))
                                        ])).to_token_stream())),
                            FieldContext::InterfacesExpr(arg_composer.destroy(DictionaryExpr::SelfProp(arg_name.to_token_stream()).to_token_stream())))
                    }
                    TypeConversion::Generic(generic_arg_ty) => {
                        let (arg_composer, arg_ty) = if let GenericTypeConversion::Optional(..) = generic_arg_ty {
                            match generic_arg_ty.ty() {
                                None => unimplemented!("Mixin inside generic: {}", generic_arg_ty),
                                Some(ty) => (match TypeConversion::from(ty) {
                                    TypeConversion::Callback(_) => unimplemented!("Callback inside generic: {}", ty.to_token_stream()),
                                    TypeConversion::Primitive(_) => ArgComposer::new(FROM_OPT_PRIMITIVE, TO_OPT_PRIMITIVE, DESTROY_OPT_PRIMITIVE),
                                    _ => ArgComposer::new(FROM_OPT_COMPLEX, TO_OPT_COMPLEX, DESTROY_COMPLEX),
                                }, ty.to_custom_or_ffi_type(&source))
                            }

                        } else { (ArgComposer::new(FROM_COMPLEX, TO_COMPLEX, DESTROY_COMPLEX), generic_arg_ty.to_custom_or_ffi_type(&source)) };
                        compose_arg(
                            arg_ty,
                            FieldContext::InterfacesExpr(arg_composer.from(quote!(ffi_ref.#arg_0_name))),
                            FieldContext::InterfacesExpr(
                                InterfacesMethodExpr::Boxed(
                                    DictionaryExpr::NamedStructInit(
                                        CommaPunctuated::from_iter([
                                            FieldTypeConversion::named(arg_0_name.clone(), FieldTypeConversionKind::Conversion(arg_composer.to(arg_to_conversion).to_token_stream()))
                                        ])).to_token_stream())),
                            FieldContext::InterfacesExpr(arg_composer.destroy(DictionaryExpr::SelfProp(arg_name.to_token_stream()).to_token_stream())))
                    }
                };

                let nested_ty = ty.first_nested_type().unwrap();
                let arg_0_presentation = compose(&arg_0_name, nested_ty);

                compose_generic_presentation(
                    ffi_name,
                    attrs.clone(),
                    Depunctuated::from_iter([
                        FieldTypeConversion::named(arg_0_name, FieldTypeConversionKind::Type(arg_0_presentation.ty))
                    ]),
                    Depunctuated::from_iter([
                        InterfacePresentation::Conversion {
                            attrs: attrs.clone(),
                            types: (ffi_type.clone(), ty.clone()),
                            conversions: (
                                FromConversionPresentation::SmartPointer(path.to_token_stream(), arg_0_presentation.from_conversion.present(&source)),
                                ToConversionPresentation::Simple(arg_0_presentation.to_conversion.present(&source)),
                                DestroyPresentation::Default,
                                None
                            )
                        },
                    ]),
                    Depunctuated::from_iter([arg_0_presentation.destructor.present(&source).terminated()]),
                    &source
                )

            },
            GenericTypeConversion::Optional(_) |
            GenericTypeConversion::Box(_) |
            GenericTypeConversion::TraitBounds(_) => FFIObjectPresentation::Empty,
        }.to_token_stream()
    }
}
fn compose_generic_group(ty: &Type, vec_conversion_type: Type, arg_conversion: TypeConversion, from_conversion_presentation: FromConversionPresentation, to_conversion_presentation: ToConversionPresentation, attrs: TokenStream2, source: &ScopeContext) -> FFIObjectPresentation {
    let ffi_name = ty.mangle_ident_default();
    let ffi_type = ffi_name.to_type();
    let arg_0_name = Name::Dictionary(DictionaryName::Values);
    let count_name = Name::Dictionary(DictionaryName::Count);
    let from_args = CommaPunctuated::from_iter([
        DictionaryExpr::SelfProp(arg_0_name.to_token_stream()),
        DictionaryExpr::SelfProp(count_name.to_token_stream())]);
    let arg_0_from = |composer: InterfacesMethodComposer|
        FieldContext::InterfacesExpr(composer(from_args.to_token_stream()));

    let arg_0_to = |composer: InterfacesMethodComposer|
        FieldContext::InterfacesExpr(
            InterfacesMethodExpr::Boxed(
                DictionaryExpr::NamedStructInit(
                    CommaPunctuated::from_iter([
                        FieldTypeConversion::named(count_name.clone(), FieldTypeConversionKind::Conversion(DictionaryExpr::ObjLen.to_token_stream())),
                        FieldTypeConversion::named(arg_0_name.clone(), FieldTypeConversionKind::Conversion(composer(DictionaryExpr::ObjIntoIter.to_token_stream()).to_token_stream()))]))
                    .to_token_stream()));

    let arg_0_destroy = |composer: InterfacesMethodComposer|
        FieldContext::InterfacesExpr(composer(from_args.to_token_stream()));

    let arg_0_presentation = match arg_conversion {
        TypeConversion::Primitive(arg_0_target_path) => {
            GenericArgPresentation::new(
                arg_0_target_path.clone(),
                arg_0_destroy(DESTROY_PRIMITIVE_GROUP),
                arg_0_from(FROM_PRIMITIVE_GROUP),
                arg_0_to(TO_PRIMITIVE_GROUP)
            )
        }
        TypeConversion::Complex(arg_0_target_ty) => {
            GenericArgPresentation::new(
                arg_0_target_ty.to_custom_or_ffi_type_mut_ptr(source),
                arg_0_destroy(DESTROY_COMPLEX_GROUP),
                arg_0_from(FROM_COMPLEX_GROUP),
                arg_0_to(TO_COMPLEX_GROUP)
            )
        }
        TypeConversion::Generic(arg_0_generic_path_conversion) => {
            // let arg_ty = arg_0_generic_path_conversion.to_custom_or_ffi_type_mut_ptr(source);
            let (arg_0_composer, arg_ty) = {
                if let GenericTypeConversion::Optional(..) = arg_0_generic_path_conversion {
                    match arg_0_generic_path_conversion.ty() {
                        None => unimplemented!("Mixin inside generic: {}", arg_0_generic_path_conversion),
                        Some(ty) => match TypeConversion::from(ty) {
                            TypeConversion::Callback(_) => unimplemented!("Callback inside generic: {}", ty.to_token_stream()),
                            TypeConversion::Primitive(_) => (ArgComposer::new(FROM_OPT_PRIMITIVE_GROUP, TO_OPT_PRIMITIVE_GROUP, DESTROY_COMPLEX_GROUP), ty.to_custom_or_ffi_type_mut_ptr(source)),
                            TypeConversion::Generic(nested_nested) => {
                                (ArgComposer::new(FROM_OPT_COMPLEX_GROUP, TO_OPT_COMPLEX_GROUP, DESTROY_COMPLEX_GROUP), nested_nested.to_custom_or_ffi_type_mut_ptr(source))
                            },
                            _ => (ArgComposer::new(FROM_OPT_COMPLEX_GROUP, TO_OPT_COMPLEX_GROUP, DESTROY_COMPLEX_GROUP), ty.to_custom_or_ffi_type_mut_ptr(source) ),
                        }
                    }
                } else {
                    (ArgComposer::new(FROM_COMPLEX_GROUP, TO_COMPLEX_GROUP, DESTROY_COMPLEX_GROUP), arg_0_generic_path_conversion.to_custom_or_ffi_type_mut_ptr(source))
                }
            };
            GenericArgPresentation::new(
                arg_ty,
                arg_0_destroy(arg_0_composer.destroy_composer),
                arg_0_from(arg_0_composer.from_composer),
                arg_0_to(arg_0_composer.to_composer)
            )
        }
        _ => {
            return FFIObjectPresentation::Empty;
        },
    };
    compose_generic_presentation(
        ffi_name,
        attrs.clone(),
        Depunctuated::from_iter([
            FieldTypeConversion::named(count_name, FieldTypeConversionKind::Type(parse_quote!(usize))),
            FieldTypeConversion::named(arg_0_name, FieldTypeConversionKind::Type(arg_0_presentation.ty.joined_mut()))
        ]),
        Depunctuated::from_iter([
            InterfacePresentation::Conversion {
                attrs: attrs.clone(),
                types: (ffi_type.clone(), ty.clone()),
                conversions: (
                    from_conversion_presentation,
                    to_conversion_presentation,
                    DestroyPresentation::Default,
                    None
                )
            },
            InterfacePresentation::VecConversion { attrs: attrs.clone(), types: (ffi_type, vec_conversion_type), decode: arg_0_presentation.from_conversion.present(source), encode: arg_0_presentation.to_conversion.present(source) }
        ]),
        Depunctuated::from_iter([arg_0_presentation.destructor.present(source).terminated()]),
        source
    )
}
fn compose_generic_presentation(
    ffi_name: Ident,
    attrs: TokenStream2,
    field_conversions: Depunctuated<FieldTypeConversion>,
    interface_presentations: Depunctuated<InterfacePresentation>,
    drop_body: Depunctuated<TokenStream2>,
    source: &ScopeContext) -> FFIObjectPresentation {
    let ffi_as_path: Path = parse_quote!(#ffi_name);
    let ffi_as_type: Type = parse_quote!(#ffi_name);
    let fields = CommaPunctuated::from_iter(field_conversions.iter().map(|field| OwnedItemPresentableContext::Named(field.clone(), true)));
    let body = BraceWrapped::new(fields.present(source));
    let object_presentation = create_struct(&ffi_as_path.segments.last().unwrap().ident, attrs.clone(), body);
    let bindings = compose_bindings(&ffi_as_type, attrs.clone(), field_conversions).present(source);
    let drop_presentation = DropInterfacePresentation::Full { attrs, ty: ffi_as_type, body: drop_body.to_token_stream() };
    FFIObjectPresentation::Generic { object_presentation, interface_presentations, drop_presentation, bindings }
}

fn compose_bindings(ffi_type: &Type, attrs: TokenStream2, conversions: Depunctuated<FieldTypeConversion>) -> Depunctuated<BindingPresentableContext> {
    Depunctuated::from_iter([
        BindingPresentableContext::Constructor(
            ConstructorPresentableContext::Default(ffi_type.clone(), attrs.to_token_stream()),
            conversions.iter().map(|field| OwnedItemPresentableContext::Named(field.clone(), false)).collect(),
            IteratorPresentationContext::Curly(conversions.iter().map(|field| OwnedItemPresentableContext::DefaultField(field.clone())).collect())),
        BindingPresentableContext::Destructor(ffi_type.clone(), attrs.to_token_stream())
    ])
}

fn dictionary_generic_arg(name: Name, field_name: TokenStream2, ty: &Type, source: &ScopeContext) -> (Type, Depunctuated<GenericArgPresentation>) {
    let ty = ty.resolve(source);
    match TypeConversion::from(&ty) {
        TypeConversion::Callback(arg_0_target_ty) =>
            unimplemented!("Callbacks are not implemented in generics: {}", arg_0_target_ty.to_token_stream()),
        TypeConversion::Primitive(arg_ty) => {
            (arg_ty.clone(), Depunctuated::from_iter([GenericArgPresentation::new(
                arg_ty.clone(),
                FieldContext::Empty,
                FieldContext::FfiRefWithConversion(FieldTypeConversion::unnamed(name.clone(), FieldTypeConversionKind::Type(arg_ty))),
                FieldContext::Named((name.to_token_stream(), FieldContext::ObjFieldName(field_name).into())))]))
        }
        TypeConversion::Complex(arg_type) => {
            (arg_type.clone(), Depunctuated::from_iter([GenericArgPresentation::new(
                arg_type.clone(),
                FieldContext::InterfacesExpr(DESTROY_COMPLEX(DictionaryExpr::SelfProp(name.to_token_stream()).to_token_stream())),
                FieldContext::From(FieldContext::FfiRefWithConversion(FieldTypeConversion::unnamed(name.clone(), FieldTypeConversionKind::Type(arg_type))).into()),
                FieldContext::Named((name.to_token_stream(), FieldContext::To(FieldContext::ObjFieldName(field_name).into()).into())))]))
        }

        TypeConversion::Generic(root_path) => {
            let arg_type: Type = parse_quote!(#root_path);
            (arg_type.clone(), Depunctuated::from_iter([GenericArgPresentation::new(
                arg_type.clone(),
                FieldContext::InterfacesExpr(DESTROY_COMPLEX(DictionaryExpr::SelfProp(name.to_token_stream()).to_token_stream())),
                FieldContext::From(FieldContext::FfiRefWithConversion(FieldTypeConversion::unnamed(name.clone(), FieldTypeConversionKind::Type(arg_type))).into()),
                FieldContext::Named((name.to_token_stream(), FieldContext::To(FieldContext::ObjFieldName(field_name).into()).into())))]))
        }
    }
}

pub fn single_generic_ffi_type(ty: &Type) -> Type {
    let path: Path = parse_quote!(#ty);
    let first_segment = path.segments.first().unwrap();
    let mut cloned_segments = path.segments.clone();
    let first_ident = &first_segment.ident;
    let last_segment = cloned_segments.iter_mut().last().unwrap();
    let last_ident = &last_segment.ident;
    if last_ident.is_primitive() {
        parse_quote!(#last_ident)
    } else if last_ident.is_any_string() {
        DictionaryExpr::CChar.to_token_stream().to_type()
    } else if last_ident.is_special_generic() || (last_ident.is_result() && path.segments.len() == 1) || (last_ident.to_string().eq("Map") && first_ident.to_string().eq("serde_json")) {
        let ffi_name = path.mangle_ident_default();
        parse_quote!(crate::fermented::generics::#ffi_name)
    } else if last_ident.is_smart_ptr() {
        let ffi_name = path.mangle_ident_default();
        parse_quote!(crate::fermented::generics::#ffi_name)
    } else {
        let new_segments = cloned_segments
            .into_iter()
            .map(|segment| quote_spanned! { segment.span() => #segment })
            .collect::<Vec<_>>();
        parse_quote!(#(#new_segments)::*)
    }
}

