use std::rc::Rc;
use quote::{quote, ToTokens};
use syn::{Attribute, BareFnArg, ParenthesizedGenericArguments, parse_quote, PathSegment, ReturnType, Type, TypeBareFn, TypePath, Visibility};
use syn::__private::TokenStream2;
use ferment_macro::ComposerBase;
use crate::ast::{CommaPunctuated, Depunctuated};
use crate::composable::{AttrsModel, FieldComposer, FieldTypeKind, GenModel};
use crate::composer::{AspectPresentable, AttrComposable, BasicComposer, BasicComposerOwner, SourceComposable, ComposerLink, GenericComposerInfo, ToConversionComposer, VarComposer, BasicComposerLink};
use crate::context::{ScopeContext, ScopeContextLink, ScopeSearch, ScopeSearchKey};
use crate::conversion::{GenericTypeKind, TypeKind};
use crate::ext::{Accessory, FFIVarResolve, GenericNestedArg, Mangle, Resolve, ToType};
use crate::lang::{FromDictionary, LangFermentable, RustSpecification, Specification};
use crate::presentable::{Aspect, ScopeContextPresentable, TypeContext};
use crate::presentation::{ArgPresentation, DictionaryExpr, DictionaryName, DocComposer, InterfacePresentation, Name, RustFermentate};

#[derive(ComposerBase)]
pub struct CallbackComposer<LANG, SPEC>
    where LANG: LangFermentable + 'static,
          SPEC: Specification<LANG> + 'static {
    pub ty: Type,
    base: BasicComposerLink<LANG, SPEC, Self>,
}

impl<LANG, SPEC> CallbackComposer<LANG, SPEC>
    where LANG: LangFermentable,
          SPEC: Specification<LANG> {
    pub fn new(ty: &Type, ty_context: SPEC::TYC, attrs: Vec<Attribute>, scope_context: &ScopeContextLink) -> Self {
        Self {
            base: BasicComposer::from(DocComposer::new(ty_context.to_token_stream()), AttrsModel::from(&attrs), ty_context, GenModel::default(), Rc::clone(scope_context)),
            ty: ty.clone()
        }
    }
}

impl<SPEC> SourceComposable for CallbackComposer<RustFermentate, SPEC>
    where SPEC: RustSpecification {
    type Source = ScopeContext;
    type Output = Option<GenericComposerInfo<RustFermentate, SPEC>>;

    fn compose(&self, source: &Self::Source) -> Self::Output {
        let Self { ty, .. } = self;
        let type_path: TypePath = parse_quote!(#ty);
        let PathSegment { arguments, .. } = type_path.path.segments.last()?;
        let ParenthesizedGenericArguments { inputs, output, .. } = parse_quote!(#arguments);
        let ffi_result = DictionaryName::FFiResult;
        let opt_conversion = |conversion: TokenStream2| quote! {
            if #ffi_result.is_null() {
                None
            } else {
                #conversion
            }
        };

        let from_ = |result_conversion: TokenStream2|
            DictionaryExpr::CallbackDestructor(result_conversion, quote!(#ffi_result));

        let from_complex_result = |ty: TokenStream2, ffi_ty: TokenStream2|
            from_(DictionaryExpr::CastedFFIConversionFrom(ffi_ty, ty, quote!(#ffi_result)).to_token_stream()).to_token_stream();
        let from_opt_complex_result = |ty: TokenStream2, ffi_ty: TokenStream2|
            from_(DictionaryExpr::CastedFFIConversionFromOpt(ffi_ty, ty, quote!(#ffi_result)).to_token_stream()).to_token_stream();

        let from_primitive_result = || quote!(ffi_result);
        let from_opt_primitive_result = || quote!(*#ffi_result);
        let (return_type, from_result_conversion, dtor_arg) = match output {
            ReturnType::Type(token, field_type) => {
                let full_ty: Type = field_type.resolve(source);
                let (ffi_ty, from_result_conversion) = match TypeKind::from(&full_ty) {
                    TypeKind::Primitive(_) => (full_ty.clone(), from_primitive_result()),
                    TypeKind::Complex(ty) => {
                        let ffi_ty = FFIVarResolve::<RustFermentate, SPEC>::special_or_to_ffi_full_path_type(&ty, source);
                        (ffi_ty.joined_mut(), from_complex_result(ty.to_token_stream(), ffi_ty.to_token_stream()))
                    },
                    TypeKind::Generic(generic_ty) => match generic_ty {
                        GenericTypeKind::Optional(ty) => match ty.maybe_first_nested_type_kind().unwrap() {
                            TypeKind::Primitive(ty) => (ty.joined_mut(), opt_conversion(from_opt_primitive_result())),
                            TypeKind::Complex(ty) => {
                                let ffi_ty = FFIVarResolve::<RustFermentate, SPEC>::special_or_to_ffi_full_path_type(&ty, source);
                                (ffi_ty.joined_mut(), opt_conversion(from_opt_complex_result(ty.to_token_stream(), ffi_ty.to_token_stream())))
                            },
                            TypeKind::Generic(ty) => {
                                let ffi_ty = FFIVarResolve::<RustFermentate, SPEC>::special_or_to_ffi_full_path_type(&ty, source);
                                (ffi_ty.joined_mut(), from_opt_complex_result(ty.ty().to_token_stream(), ffi_ty.to_token_stream()))
                            },
                        },
                        GenericTypeKind::TraitBounds(_) => unimplemented!("TODO: mixins+traits+generics"),
                        _ => {
                            let ffi_ty = FFIVarResolve::<RustFermentate, SPEC>::special_or_to_ffi_full_path_type(&full_ty, source);
                            (ffi_ty.joined_mut(), from_complex_result(generic_ty.to_token_stream(), ffi_ty.to_token_stream()))
                        }
                    }
                };
                (ReturnType::Type(token, Box::new(full_ty)), from_result_conversion, Some(ffi_ty))
            },
            ReturnType::Default => (ReturnType::Default, from_primitive_result(), None),
        };
        let mut args = CommaPunctuated::new();
        let mut ffi_args = CommaPunctuated::new();
        let mut arg_to_conversions = CommaPunctuated::new();
        inputs
            .iter()
            .enumerate()
            .for_each(|(index, ty)| {
                let name = Name::UnnamedArg(index);
                args.push(ArgPresentation::field(&vec![], Visibility::Inherited, Some(name.mangle_ident_default()), ty.clone()));
                ffi_args.push(bare_fn_arg(VarComposer::<RustFermentate, SPEC>::new(ScopeSearch::Value(ScopeSearchKey::TypeRef(ty, None))).compose(source).to_type()));
                arg_to_conversions.push(ToConversionComposer::<RustFermentate, SPEC>::new(name, ty.clone(), None).compose(source).present(source));
            });
        let ffi_type = self.present_ffi_aspect();
        let attrs = self.compose_attributes();
        Some(GenericComposerInfo::<RustFermentate, SPEC>::callback(
            Aspect::RawTarget(TypeContext::Struct { ident: ty.mangle_ident_default(), attrs: vec![] }),
            &attrs,
            if let Some(dtor_arg) = dtor_arg {
                Depunctuated::from_iter([
                    FieldComposer::named(SPEC::Name::dictionary_name(DictionaryName::Caller), FieldTypeKind::Type(bare(ffi_args, ReturnType::Type(Default::default(), Box::new(dtor_arg.clone()))))),
                    FieldComposer::named(SPEC::Name::dictionary_name(DictionaryName::Destructor), FieldTypeKind::Type(bare(CommaPunctuated::from_iter([bare_fn_arg(dtor_arg)]), ReturnType::Default)))
                ])
            } else {
                Depunctuated::from_iter([
                    FieldComposer::named(SPEC::Name::dictionary_name(DictionaryName::Caller), FieldTypeKind::Type(bare(ffi_args, ReturnType::Default))),
                ])
            },
            Depunctuated::from_iter([
                InterfacePresentation::callback(&attrs, &ffi_type, args, return_type, arg_to_conversions, from_result_conversion),
                InterfacePresentation::send_sync(&attrs, &ffi_type)
            ])
        ))
    }
}

fn bare_fn_arg(ty: Type) -> BareFnArg {
    BareFnArg { attrs: vec![], name: None, ty }
}

fn bare(inputs: CommaPunctuated<BareFnArg>, output: ReturnType) -> Type {
    Type::BareFn(TypeBareFn { abi: Some(parse_quote!(extern "C")), inputs, output, lifetimes: None, unsafety: Some(Default::default()), fn_token: Default::default(), paren_token: Default::default(), variadic: None, })
}

