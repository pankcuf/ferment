use std::cell::Ref;
use syn::{Attribute, Generics, parse_quote, Type};
use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Display, Formatter};
use std::hash::{Hash, Hasher};
use quote::{quote, ToTokens};
use syn::__private::TokenStream2;
use crate::ast::{CommaPunctuated, Depunctuated, ParenWrapped};
use crate::composable::{FieldComposer, FieldTypeKind, TypeModel, TypeModeled};
use crate::composer::{CommaPunctuatedNestedArguments, ParentComposer};
use crate::context::ScopeContext;
use crate::conversion::{compose_generic_presentation, dictionary_generic_arg_pair, expand_attributes, ObjectKind};
use crate::ext::{AsType, Mangle, Terminated, ToType};
use crate::formatter::{format_obj_vec, format_predicates_obj_dict};
use crate::presentable::ScopeContextPresentable;
use crate::presentation::{DictionaryExpr, DictionaryName, InterfacePresentation, InterfacesMethodExpr, Name};

#[derive(Clone)]
pub struct GenericBoundsModel {
    // 'T'
    type_model: TypeModel,
    // 'Fn(u32) -> Result<bool, ProtocolError>' or 'Clone + Debug + Smth'
    pub bounds: Vec<ObjectKind>,
    pub predicates: HashMap<Type, Vec<ObjectKind>>,
    // pub bounds: Vec<Path>,
    // pub predicates: HashMap<Type, Vec<Path>>,
    pub nested_arguments: CommaPunctuatedNestedArguments,
    // pub nested_arguments: HashMap<Path, CommaPunctuated<NestedArgument>>,
}

impl<'a> AsType<'a> for GenericBoundsModel {
    fn as_type(&'a self) -> &'a Type {
        self.type_model.as_type()
    }
}
impl TypeModeled for GenericBoundsModel {
    fn type_model_mut(&mut self) -> &mut TypeModel {
        &mut self.type_model
    }

    fn type_model_ref(&self) -> &TypeModel {
        &self.type_model
    }
}

impl Debug for GenericBoundsModel {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!(
            "GenericBoundsModel(ty: {}, bounds: {}, predicates: {}, nested_args: {})",
            self.type_model,
            format_obj_vec(&self.bounds),
            format_predicates_obj_dict(&self.predicates),
            self.nested_arguments.to_token_stream()
        ).as_str())
    }
}

impl Display for GenericBoundsModel {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}
impl PartialEq for GenericBoundsModel {
    fn eq(&self, other: &Self) -> bool {
        let self_bounds = self.bounds.iter().map(|b| b.to_token_stream());
        let other_bounds = other.bounds.iter().map(|b| b.to_token_stream());
        let self_tokens = [self.as_type().to_token_stream(), quote!(#(#self_bounds),*)];
        let other_tokens = [other.as_type().to_token_stream(), quote!(#(#other_bounds),*)];
        self_tokens.iter()
            .map(|t| t.to_string())
            .zip(other_tokens.iter().map(ToString::to_string))
            .all(|(a, b)| {
                let x = a == b;
                // println!("GGGGG:::({}) {} ==== {}", x, a, b);
                x
            })
    }
}

impl Eq for GenericBoundsModel {}

impl Hash for GenericBoundsModel {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.as_type().to_token_stream().to_string().hash(state);
        self.bounds.iter().for_each(|bound| bound.to_token_stream().to_string().hash(state));
        // self.predicates.iter().for_each(||)
    }
}

impl GenericBoundsModel {
    pub fn new(ty: Type, bounds: Vec<ObjectKind>, predicates: HashMap<Type, Vec<ObjectKind>>, generics: Generics, nested_arguments: CommaPunctuatedNestedArguments) -> Self {
        Self {
            type_model: TypeModel::new(ty, Some(generics), nested_arguments.clone()),
            bounds,
            predicates,
            nested_arguments,
        }
    }

    pub fn ffi_full_dictionary_type_presenter(&self, _source: &ScopeContext) -> Type {
        // unimplemented!("")
        let ffi_name = self.mangle_ident_default();
        println!("GenericBound: ffi_full_dictionary_type_presenter: {} --- {}", ffi_name, self);
        parse_quote!(crate::fermented::generics::#ffi_name)
        // Determine mixin type
        //
    }

}
impl GenericBoundsModel {

    pub fn is_lambda(&self) -> bool {
        self.bounds.iter().find(|b| {
            match b {
                ObjectKind::Type(ty) |
                ObjectKind::Item(ty, _) => ty.is_lambda(),
                ObjectKind::Empty => false
            }
        }).is_some()
    }

    pub fn extend_as_lambda(&self, _attrs: &HashSet<Option<Attribute>>, _source: Ref<ScopeContext>) -> TokenStream2 {
        quote!()
    }
    pub fn expand(&self, attrs: &HashSet<Option<Attribute>>, scope_context: &ParentComposer<ScopeContext>) -> TokenStream2 {
        let source = scope_context.borrow();
        if self.is_lambda() {
            return self.extend_as_lambda(attrs, source);
        }
        // println!("Mixin::Expand: {} ---- {:?}", self, attrs);
        let attrs = expand_attributes(attrs);
        let ffi_name = self.mangle_ident_default();
        let self_ty = self.as_type();
        let ffi_as_type = ffi_name.to_type();
        println!("Mixin::Expand: {} ---- \n\tattrs: {:?}\n\tname: {}", self, attrs, ffi_name);



        let mixin_items = self.predicates.iter()
            .enumerate()
            .map(|(index, (predicate_ty, _bounds))|
                dictionary_generic_arg_pair(
                    Name::UnnamedArg(index),
                    Name::Index(index),
                    predicate_ty,
                    &source))
            .collect::<Depunctuated<_>>();
        compose_generic_presentation(
            ffi_name,
            attrs.clone(),
            Depunctuated::from_iter(
                mixin_items.iter()
                    .enumerate()
                    .map(|(index, (root_path, _))| FieldComposer::unnamed(Name::UnnamedArg(index), FieldTypeKind::Type(root_path.clone())))),
            Depunctuated::from_iter([
                InterfacePresentation::ConversionFrom {
                    attrs: attrs.clone(),
                    types: (ffi_as_type.clone(), parse_quote!(#self_ty)),
                    conversions: (
                        DictionaryExpr::FromRoot(
                            ParenWrapped::new(
                                CommaPunctuated::from_iter(
                                    mixin_items.iter()
                                        .flat_map(|(_, args)| args.iter().map(|item| item.from_conversion.clone()))))
                                .present(&source))
                            .to_token_stream(),
                        None
                    )
                },
                InterfacePresentation::ConversionTo {
                    attrs: attrs.clone(),
                    types: (ffi_as_type.clone(), parse_quote!(#self_ty)),
                    conversions: (
                        InterfacesMethodExpr::Boxed(
                            DictionaryExpr::SelfDestructuring(
                                CommaPunctuated::from_iter(
                                    mixin_items.iter()
                                        .flat_map(|(_, args)| args.iter().map(|item| item.from_conversion.present(&source))))
                                    .to_token_stream())
                                .to_token_stream())
                            .to_token_stream(),
                        None),
                },
                InterfacePresentation::ConversionDestroy {
                    attrs: attrs,
                    types: (ffi_as_type, parse_quote!(#self_ty)),
                    conversions: (InterfacesMethodExpr::UnboxAny(DictionaryName::Ffi.to_token_stream()).to_token_stream().terminated(), None),
                }
            ]),
            Depunctuated::from_iter(mixin_items.iter().flat_map(|(_, args)| args.iter().map(|item| item.destructor.present(&source).terminated()))),
            &source
        ).to_token_stream()
    }
}