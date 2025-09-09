use quote::{quote, ToTokens};
use syn::{parse_quote, BareFnArg, Lifetime, ParenthesizedGenericArguments, ReturnType, Type, TypeBareFn};
use syn::token::RArrow;
use crate::ast::{CommaPunctuated, Depunctuated};
use crate::composable::FieldComposer;
use crate::composer::{AspectPresentable, AttrComposable, GenericComposerInfo, SourceComposable, ConversionToComposer, CallbackComposer, VarComposer};
use crate::context::ScopeContext;
use crate::kind::{CallbackKind, FieldTypeKind, GenericTypeKind, SpecialType, TypeKind};
use crate::ext::{Accessory, FFISpecialTypeResolve, FFIVarResolve, GenericNestedArg, LifetimeProcessor, Mangle, MaybeParenthesizedArgs, MaybeTraitBound, PunctuateOne, Resolve, ToType, WrapIntoRoundBraces};
use crate::lang::{FromDictionary, RustSpecification};
use crate::presentable::{Aspect, ScopeContextPresentable};
use crate::presentation::{ArgPresentation, DictionaryExpr, DictionaryName, InterfacePresentation, Name};

impl SourceComposable for CallbackComposer<RustSpecification> {
    type Source = ScopeContext;
    type Output = Option<GenericComposerInfo<RustSpecification>>;

    fn compose(&self, source: &Self::Source) -> Self::Output {
        let Self { kind, .. } = self;
        let mut lifetimes = Vec::<Lifetime>::new();
        let (inputs, output) = match kind {
            CallbackKind::FnOnce(Type::TraitObject(trait_object)) |
            CallbackKind::Fn(Type::TraitObject(trait_object)) |
            CallbackKind::FnMut(Type::TraitObject(trait_object)) =>
                trait_object.bounds.iter()
                    .find_map(MaybeTraitBound::maybe_trait_bound)
                    .and_then(|trait_bound| trait_bound.path.segments.last())
                    .and_then(MaybeParenthesizedArgs::maybe_parenthesized_args)
                    .map(|ParenthesizedGenericArguments { inputs, output, .. }| (inputs.clone(), output.clone()))?,
            CallbackKind::FnPointer(Type::BareFn(TypeBareFn { inputs, output, .. })) =>
                (inputs.iter().map(|b| b.ty.clone()).collect(), output.clone()),
            CallbackKind::FnOnce(Type::Path(path)) |
            CallbackKind::Fn(Type::Path(path)) |
            CallbackKind::FnMut(Type::Path(path)) |
            CallbackKind::FnPointer(Type::Path(path)) => {
                let ParenthesizedGenericArguments { inputs, output, .. } = path.path.segments.last()?.maybe_parenthesized_args()?;
                (inputs.clone(), output.clone())
            }
            _ => panic!("Unsupported callback kind: {kind:?}")
        };
        let ffi_result = DictionaryName::FFiResult;
        let (return_type, from_result_conversion, dtor_arg) = match output {
            ReturnType::Type(token, field_type) => {
                let full_ty: Type = field_type.resolve(source);
                lifetimes.extend(field_type.unique_lifetimes());
                let (ffi_ty, from_result_conversion) = match TypeKind::from(&full_ty) {
                    TypeKind::Primitive(_) => (full_ty.clone(), DictionaryExpr::DictionaryName(ffi_result)),
                    TypeKind::Complex(ty) => {
                        let ffi_ty = FFIVarResolve::<RustSpecification>::special_or_to_ffi_full_path_type(&ty, source);
                        (ffi_ty.joined_mut(), match FFISpecialTypeResolve::<RustSpecification>::maybe_special_type(&ty, source) {
                            Some(SpecialType::Opaque(..)) =>
                                DictionaryExpr::Clone(quote!((&*#ffi_result))),
                            _ =>
                                DictionaryExpr::callback_dtor(DictionaryExpr::casted_ffi_conversion_from(&ffi_ty, &ty, &ffi_result), &ffi_result)
                        })
                    },
                    TypeKind::Generic(generic_ty) => match generic_ty {
                        GenericTypeKind::Optional(ty) => match ty.maybe_first_nested_type_kind().unwrap() {
                            TypeKind::Primitive(ty) => (ty.joined_mut(), DictionaryExpr::IfElse(quote!(#ffi_result.is_null()), quote!(None), quote!(*#ffi_result))),
                            TypeKind::Complex(ty) => {
                                let maybe_special: Option<SpecialType<RustSpecification>> = ty.maybe_special_type(source);
                                let ffi_ty = FFIVarResolve::<RustSpecification>::special_or_to_ffi_full_path_type(&ty, source);
                                (ffi_ty.joined_mut(), DictionaryExpr::IfElse(quote!(#ffi_result.is_null()), quote!(None), match maybe_special {
                                    Some(SpecialType::Opaque(..)) => DictionaryExpr::some(DictionaryExpr::Clone(WrapIntoRoundBraces::wrap(DictionaryExpr::deref_ref(&ffi_result).to_token_stream()))),
                                    _ =>
                                        DictionaryExpr::callback_dtor(DictionaryExpr::casted_ffi_conversion_from_opt(&ffi_ty, &ty, &ffi_result), &ffi_result)
                                }.to_token_stream()))
                            },
                            TypeKind::Generic(ty) => {
                                let ffi_ty = FFIVarResolve::<RustSpecification>::special_or_to_ffi_full_path_type(&ty, source);
                                (ffi_ty.joined_mut(), DictionaryExpr::callback_dtor(DictionaryExpr::casted_ffi_conversion_from_opt(&ffi_ty, ty.ty(), &ffi_result), &ffi_result))
                            },
                        },
                        GenericTypeKind::TraitBounds(_) => unimplemented!("TODO: mixins+traits+generics"),
                        _ => {
                            let ffi_ty = FFIVarResolve::<RustSpecification>::special_or_to_ffi_full_path_type(&full_ty, source);
                            (ffi_ty.joined_mut(), DictionaryExpr::callback_dtor(DictionaryExpr::casted_ffi_conversion_from(&ffi_ty, &generic_ty, &ffi_result), &ffi_result))
                        }
                    }
                };
                (ReturnType::Type(token, Box::new(full_ty)), from_result_conversion, Some(ffi_ty))
            },
            ReturnType::Default => (ReturnType::Default, DictionaryExpr::DictionaryName(ffi_result), None),
        };
        let mut args = CommaPunctuated::new();
        let mut ffi_args = CommaPunctuated::new();
        let mut arg_to_conversions = CommaPunctuated::new();
        inputs
            .iter()
            .enumerate()
            .for_each(|(index, ty)| {
                let name = Name::UnnamedArg(index);
                lifetimes.extend(ty.unique_lifetimes());
                args.push(ArgPresentation::inherited_field(&[], name.mangle_ident_default(), ty.clone()));
                ffi_args.push(bare_fn_arg(VarComposer::<RustSpecification>::value(ty).compose(source).to_type()));
                arg_to_conversions.push(ConversionToComposer::<RustSpecification>::value(name, ty).compose(source).present(source));
            });
        let ffi_type = self.present_ffi_aspect();
        let attrs = self.compose_attributes();
        Some(GenericComposerInfo::<RustSpecification>::callback(
            Aspect::raw_struct_ident(kind.mangle_ident_default()),
            &attrs,
            if let Some(dtor_arg) = dtor_arg {
                Depunctuated::from_iter([
                    FieldComposer::named_no_attrs(Name::dictionary_name(DictionaryName::Caller), FieldTypeKind::Type(bare(ffi_args, ReturnType::Type(RArrow::default(), Box::new(dtor_arg.clone()))))),
                    FieldComposer::named_no_attrs(Name::dictionary_name(DictionaryName::Destructor), FieldTypeKind::Type(bare(bare_fn_arg(dtor_arg).punctuate_one(), ReturnType::Default)))
                ])
            } else {
                Depunctuated::from_iter([
                    FieldComposer::named_no_attrs(Name::dictionary_name(DictionaryName::Caller), FieldTypeKind::Type(bare(ffi_args, ReturnType::Default))),
                ])
            },
            Depunctuated::from_iter([
                InterfacePresentation::send_sync(&attrs, &ffi_type),
                InterfacePresentation::callback(&attrs, &lifetimes, ffi_type, args, return_type, arg_to_conversions, from_result_conversion),
            ])
        ))
    }
}

fn bare_fn_arg(ty: Type) -> BareFnArg {
    BareFnArg { attrs: vec![], name: None, ty }
}

fn bare(inputs: CommaPunctuated<BareFnArg>, output: ReturnType) -> Type {
    Type::BareFn(TypeBareFn {
        abi: Some(parse_quote!(extern "C")),
        inputs,
        output,
        lifetimes: None,
        unsafety: Some(Default::default()),
        fn_token: Default::default(),
        paren_token: Default::default(),
        variadic: None
    })
}

