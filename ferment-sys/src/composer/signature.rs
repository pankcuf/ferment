use std::cell::RefCell;
use std::rc::Rc;
use quote::{quote, ToTokens};
use syn::{parse_quote, Attribute, BareFnArg, Field, FnArg, Generics, ImplItemMethod, ItemFn, Lifetime, Pat, PatType, Path, Receiver, ReturnType, Signature, TraitItemMethod, Type, TypeBareFn, TypePtr, Visibility};
use syn::token::Semi;
use ferment_macro::ComposerBase;
use crate::ast::{CommaPunctuated, CommaPunctuatedTokens};
use crate::composable::{AttrsModel, CfgAttributes, FieldComposer, FieldTypeKind, FnSignatureContext, GenModel, LifetimesModel};
use crate::composer::{AspectPresentable, BasicComposer, BasicComposerLink, BasicComposerOwner, CommaPunctuatedArgKinds, ComposerLink, DocsComposable, FromConversionFullComposer, LifetimesComposable, Linkable, NameKind, SemiPunctuatedArgKinds, SourceAccessible, SourceComposable, SourceFermentable, ToConversionFullComposer, TypeAspect};
use crate::composer::target_var::TargetVarComposer;
use crate::composer::var::VarComposer;
use crate::context::{ScopeContext, ScopeContextLink};
use crate::conversion::{GenericTypeKind, TypeKind};
use crate::ext::{FFITypeResolve, ItemExtension, LifetimeProcessor, Mangle, Resolve, ToType};
use crate::lang::{FromDictionary, LangFermentable, RustSpecification, Specification};
use crate::presentable::{ArgKind, Aspect, BindingPresentableContext, Expression, ScopeContextPresentable, SeqKind, TypeContext};
use crate::presentation::{ArgPresentation, BindingPresentation, DictionaryExpr, DictionaryName, DocComposer, DocPresentation, FFIConversionFromMethodExpr, InterfacePresentation, Name, RustFermentate};

#[derive(ComposerBase)]
pub struct SigComposer<LANG, SPEC>
    where LANG: LangFermentable + 'static,
          SPEC: Specification<LANG> + 'static {
    pub base: BasicComposerLink<LANG, SPEC, Self>,
}

impl<LANG, SPEC> SigComposer<LANG, SPEC>
    where LANG: LangFermentable,
          SPEC: Specification<LANG> {

    fn new(
        ty_context: SPEC::TYC,
        generics: Option<Generics>,
        lifetimes: Vec<Lifetime>,
        attrs: AttrsModel,
        context: &ScopeContextLink) -> ComposerLink<Self> {
        let root = Rc::new(RefCell::new(Self {
            base: BasicComposer::from(DocComposer::new(ty_context.to_token_stream()), attrs, ty_context, GenModel::new(generics), LifetimesModel::new(lifetimes), Rc::clone(context)),
        }));
        {
            let mut composer = root.borrow_mut();
            composer.base.link(&root);
        }
        root
    }
    pub fn with_context(
        ty_context: SPEC::TYC,
        generics: &Generics,
        lifetimes: &Vec<Lifetime>,
        attrs: &Vec<Attribute>,
        context: &ScopeContextLink
    ) -> ComposerLink<Self> {
        Self::new(
            ty_context,
            Some(generics.clone()),
            lifetimes.clone(),
            AttrsModel::from(attrs),
            context)
    }
    pub fn from_item_fn(
        item_fn: &ItemFn,
        ty_context: SPEC::TYC,
        context: &ScopeContextLink
    ) -> ComposerLink<Self> {
        let ItemFn { attrs, sig: Signature { generics, ..}, .. } = item_fn;
        Self::with_context(
            ty_context,
            generics,
            &vec![],
            attrs,
            context)
    }
    pub fn from_type_bare_fn(
        ty_context: SPEC::TYC,
        generics: &Generics,
        lifetimes: &Vec<Lifetime>,
        attrs: &Vec<Attribute>,
        context: &ScopeContextLink
    ) -> ComposerLink<Self> {
        Self::with_context(
            ty_context,
            generics,
            lifetimes,
            attrs,
            context
        )
    }

    pub fn from_impl_item_method(
        impl_item_method: &ImplItemMethod,
        ty_context: SPEC::TYC,
        context: &ScopeContextLink
    ) -> ComposerLink<Self> {
        let ImplItemMethod { sig, .. } = impl_item_method;
        Self::with_context(
            ty_context,
            &sig.generics,
            &vec![],
            sig.maybe_attrs().unwrap_or(&vec![]),
            context
        )
    }
    pub fn from_trait_item_method(
        trait_item_method: &TraitItemMethod,
        ty_context: SPEC::TYC,
        context: &ScopeContextLink
    ) -> ComposerLink<Self> {
        let TraitItemMethod { sig, attrs, .. } = trait_item_method;
        Self::with_context(ty_context, &sig.generics, &vec![], attrs, context)
    }
}

impl<LANG, SPEC> DocsComposable for SigComposer<LANG, SPEC>
    where LANG: LangFermentable,
          SPEC: Specification<LANG> {
    fn compose_docs(&self) -> DocPresentation {
        DocPresentation::Direct(self.base.doc.compose(self.context()))
    }
}

fn compose_regular_fn<SPEC>(
    path: Path,
    aspect: Aspect<SPEC::TYC>,
    attrs: SPEC::Attr,
    generics: SPEC::Gen,
    sig: &Signature,
    sig_context: &FnSignatureContext,
    source: &ScopeContext
) -> BindingPresentableContext<RustFermentate, SPEC>
    where SPEC: RustSpecification,
          CommaPunctuatedArgKinds<RustFermentate, SPEC>: Extend<ArgKind<RustFermentate, SPEC>> {
    let mut used_lifetimes = Vec::<Lifetime>::new();
    let Signature { output, inputs, asyncness, .. } = sig;
    let (return_type_presentation, return_type_conversion) = match output {
        ReturnType::Default => (ReturnType::Default, SPEC::Expr::Simple(Semi::default().to_token_stream())),
        ReturnType::Type(_, ty) => (
            ReturnType::Type(Default::default(), Box::new(VarComposer::<RustFermentate, SPEC>::key_in_scope(ty, &source.scope).compose(source).to_type())),
            ToConversionFullComposer::<RustFermentate, SPEC>::key(SPEC::Name::dictionary_name(DictionaryName::Obj), ty, &source.scope).compose(source)
        )
    };

    let mut arguments = CommaPunctuatedArgKinds::<RustFermentate, SPEC>::new();
    let mut argument_names = CommaPunctuatedTokens::new();
    let mut argument_conversions = CommaPunctuatedArgKinds::<RustFermentate, SPEC>::new();
    let mut argument_conversions2 = SemiPunctuatedArgKinds::<RustFermentate, SPEC>::new();
    for arg in inputs {
        match arg {
            FnArg::Receiver(Receiver { mutability, reference, attrs, .. }) => {
                if let Some((_, Some(lt))) = reference {
                    used_lifetimes.push(lt.clone());
                }

                let (ty, name_type_conversion) = match sig_context {
                    FnSignatureContext::Impl(self_ty, maybe_trait_ty, _) |
                    FnSignatureContext::TraitInner(self_ty, maybe_trait_ty, _) => match maybe_trait_ty {
                        Some(trait_ty) => {
                            let ty: Type = self_ty.resolve(source);
                            let expr = Expression::dict_expr(DictionaryExpr::SelfAsTrait(ty.to_token_stream(), if mutability.is_some() { quote!(mut) } else { quote!(const) }));
                            let expr_composer = match (mutability, reference) {
                                (Some(..), _) => |expr: SPEC::Expr| SPEC::Expr::AsMutRef(expr.into()),
                                (_, Some(..)) => |expr: SPEC::Expr| SPEC::Expr::AsRef(expr.into()),
                                (..) => |expr: SPEC::Expr| expr.into(),
                            };
                            (
                                trait_ty,
                                expr_composer(expr)
                            )
                        },
                        None => {
                            println!("Receiver: {} in {}", self_ty.to_token_stream(), source.scope.fmt_mid());
                            let qualified_ty = match (mutability, reference) {
                                (Some(..), _) => parse_quote!(&mut #self_ty),
                                (_, Some(..)) => parse_quote!(&#self_ty),
                                (..) => self_ty.clone(),
                            };
                            (
                                self_ty,
                                FromConversionFullComposer::<RustFermentate, SPEC>::key_in_scope(SPEC::Name::dictionary_name(DictionaryName::Self_), &qualified_ty, &source.scope).compose(source)
                            )
                        }
                    },
                    FnSignatureContext::TraitAsType(self_ty, _, _) => {
                        let qualified_ty = match (mutability, reference) {
                            (Some(..), _) => parse_quote!(&mut #self_ty),
                            (_, Some(..)) => parse_quote!(&#self_ty),
                            (..) => self_ty.clone(),
                        };
                        (
                            self_ty,
                            FromConversionFullComposer::<RustFermentate, SPEC>::key_in_scope(SPEC::Name::dictionary_name(DictionaryName::Self_), &qualified_ty, &source.scope).compose(source)
                        )
                    },
                    _ => panic!("Receiver in regular fn")
                };
                let name = SPEC::Name::dictionary_name(DictionaryName::Self_);
                argument_names.push(name.to_token_stream());
                let ty = VarComposer::<RustFermentate, SPEC>::key_in_scope(ty, &source.scope).compose(source).to_type();
                arguments.push(ArgKind::Named(
                    FieldComposer::new(name.clone(), FieldTypeKind::Type(ty), true, attrs.cfg_attributes()),
                    Visibility::Inherited
                ));
                argument_conversions.push(ArgKind::AttrExpression(
                    name_type_conversion.clone(),
                    SPEC::Attr::default()
                ));
                argument_conversions2.push(ArgKind::AttrExpression(
                    Expression::DictionaryExpr(DictionaryExpr::LetExpr(name.to_token_stream(), name_type_conversion.present(source))),
                    SPEC::Attr::default()
                ));
            },
            FnArg::Typed(PatType { ty, attrs, pat, .. }) => {
                used_lifetimes.extend(ty.unique_lifetimes());
                let name = Name::Pat(*pat.clone());
                argument_names.push(name.to_token_stream());
                arguments.push(ArgKind::Named(FieldComposer::typed(name.clone(), ty, true, attrs), Visibility::Inherited));
                let from_conversion = FromConversionFullComposer::<RustFermentate, SPEC>::key_in_scope(name.clone(), ty, &source.scope).compose(source);
                argument_conversions.push(ArgKind::AttrExpression(
                    from_conversion.clone(),
                    SPEC::Attr::default()
                ));
                let target_ty = TargetVarComposer::<RustFermentate, SPEC>::key_in_scope(ty, &source.scope).compose(source);
                argument_conversions2.push(ArgKind::AttrExpression(
                    Expression::DictionaryExpr(DictionaryExpr::LetExpr(quote! { #name: #target_ty }, from_conversion.present(source))),
                    SPEC::Attr::default()
                ));

            }
        }
    }
    let aspect_with_args = (aspect, attrs.clone(), generics.clone(), NameKind::Named);
    let input_conversions = SeqKind::FromUnnamedFields((aspect_with_args, argument_conversions));
    BindingPresentableContext::RegFn(
        path,
        asyncness.is_some(),
        arguments,
        return_type_presentation,
        input_conversions,
        return_type_conversion,
        attrs,
        generics,
        used_lifetimes
    )
    // BindingPresentableContext::RegFn2(
    //     path,
    //     asyncness.is_some(),
    //     argument_names,
    //     arguments,
    //     return_type_presentation,
    //     aspect.present(source),
    //     argument_conversions2,
    //     // input_conversions,
    //     return_type_conversion,
    //     attrs,
    //     generics,
    //     used_lifetimes
    // )
}


impl<SPEC> SourceFermentable<RustFermentate> for SigComposer<RustFermentate, SPEC>
    where SPEC: RustSpecification {
    fn ferment(&self) -> RustFermentate {
        let source = self.source_ref();
        let binding = match self.type_context_ref() {
            TypeContext::Fn { parent: _, path: full_fn_path, sig_context, attrs } => {
                match &sig_context {
                    FnSignatureContext::ModFn(ItemFn { sig, .. }) =>
                        compose_regular_fn::<SPEC>(
                            full_fn_path.clone(),
                            self.target_type_aspect(),
                            attrs.clone(),
                            None,
                            sig,
                            sig_context,
                            &source
                        ).present(&source),
                    FnSignatureContext::Impl(_, maybe_trait, sig) => {
                        if let Some(trait_) = maybe_trait {
                            let mut path = full_fn_path.clone();
                            let last = path.segments.pop().unwrap();
                            let last_segment = last.value();
                            let path = parse_quote!(#path<#trait_>::#last_segment);
                            compose_regular_fn::<SPEC>(
                                path,
                                self.ffi_type_aspect(),
                                attrs.clone(),
                                None,
                                sig,
                                sig_context,
                                &source
                            )
                        } else {
                            compose_regular_fn::<SPEC>(
                                full_fn_path.clone(),
                                self.ffi_type_aspect(),
                                attrs.clone(),
                                None,
                                sig,
                                sig_context,
                                &source
                            )
                        }.present(&source)
                    },
                    FnSignatureContext::TraitAsType(_, _, sig) => compose_regular_fn::<SPEC>(
                        full_fn_path.clone(),
                        self.ffi_type_aspect(),
                        attrs.clone(),
                        None,
                        sig,
                        sig_context,
                        &source
                        ).present(&source),
                    FnSignatureContext::TraitInner(_, _, sig) => {
                        let Signature { output, inputs, .. } = sig;
                        let compose_var = |ty: &Type| VarComposer::<RustFermentate, SPEC>::key_in_scope(ty, &source.scope).compose(&source).to_type();
                        let return_type = match output {
                            ReturnType::Default => ReturnType::Default,
                            ReturnType::Type(_, ty) => ReturnType::Type(Default::default(), Box::new(compose_var(ty)))
                        };
                        let arguments = CommaPunctuatedArgKinds::from_iter(inputs
                            .iter()
                            .map(|arg| {
                                ArgKind::<RustFermentate, SPEC>::Named(match arg {
                                    FnArg::Receiver(Receiver { mutability: _, reference: _, attrs, .. }) =>
                                        FieldComposer::self_typed(compose_var(sig_context.receiver_ty()), attrs),
                                    FnArg::Typed(PatType { ty, attrs, pat, .. }) =>
                                        FieldComposer::typed(Name::Pat(*pat.clone()), ty, true, attrs)
                                }, Visibility::Inherited)
                            })).present(&source);
                        BindingPresentation::TraitVTableInnerFn {
                            name: Name::<RustFermentate, SPEC>::VTableInnerFn(sig.ident.clone()).mangle_tokens_default(),
                            name_and_args: quote!(unsafe extern "C" fn (#arguments)),
                            output_expression: return_type
                        }
                    },
                    FnSignatureContext::Bare(_target_name, type_bare_fn) => {
                        let TypeBareFn { inputs, output, .. } = type_bare_fn;
                        let ffi_result = DictionaryName::FFiResult;
                        let opt_conversion = |conversion| DictionaryExpr::Simple(quote!((!ffi_result.is_null()).then(|| { #conversion })));

                        let ffi_result_conversion = FFIConversionFromMethodExpr::FfiFrom(ffi_result.to_token_stream());
                        let from_complex_result = || DictionaryExpr::CallbackDestructor(quote!(#ffi_result_conversion), quote!(#ffi_result));
                        let from_primitive_result = || DictionaryExpr::Simple(quote!(#ffi_result));
                        let from_opt_primitive_result = || DictionaryExpr::Deref(quote!(#ffi_result));
                        let (return_type, ffi_return_type, post_processing) = match output {
                            ReturnType::Type(token, field_type) => (
                                ReturnType::Type(token.clone(), Box::new(field_type.resolve(&source))),
                                ReturnType::Type(token.clone(), Box::new(<Type as Resolve<SPEC::Var>>::resolve(field_type, &source).to_type())),
                                match TypeKind::from(field_type) {
                                    TypeKind::Primitive(_) => from_primitive_result(),
                                    TypeKind::Complex(_) =>  from_complex_result(),
                                    TypeKind::Generic(generic_ty) => match generic_ty {
                                        GenericTypeKind::TraitBounds(_) => unimplemented!("TODO: mixins+traits+generics"),
                                        GenericTypeKind::Optional(ty) => {
                                            opt_conversion(match TypeKind::from(ty) {
                                                TypeKind::Primitive(_) => from_opt_primitive_result(),
                                                TypeKind::Complex(_) |
                                                TypeKind::Generic(_) => from_complex_result(),
                                            })
                                        }
                                        _ => from_complex_result()
                                    }
                                }
                            ),
                            ReturnType::Default => (ReturnType::Default, ReturnType::Default, from_primitive_result()),
                        };
                        let mut arg_names = CommaPunctuated::new();
                        let mut ffi_args = CommaPunctuated::new();
                        let mut arg_target_types = CommaPunctuated::new();
                        let mut arg_target_fields = CommaPunctuated::new();
                        let mut arg_to_conversions = CommaPunctuated::new();
                        inputs
                            .iter()
                            .for_each(|bare_fn_arg| {
                                let BareFnArg { ty, name, .. } = bare_fn_arg;
                                let var_composer = VarComposer::<RustFermentate, SPEC>::key_in_scope(ty, &source.scope);
                                let var_ty = var_composer.compose(&source);
                                let conversion = TypeKind::from(ty);
                                let ident_name = Name::<RustFermentate, SPEC>::Optional(name.as_ref().map(|(ident, ..)| ident.clone()));
                                arg_names.push(ident_name.to_token_stream());
                                arg_target_types.push(ArgPresentation::Pat(Pat::Verbatim(ty.to_token_stream())));
                                arg_target_fields.push(ArgPresentation::Field(field::<RustFermentate, SPEC>(ident_name.clone(), ty, &source)));
                                let mut bare_fn_arg_replacement = bare_fn_arg.clone();
                                bare_fn_arg_replacement.ty = var_ty.to_type();
                                ffi_args.push(bare_fn_arg_replacement);
                                arg_to_conversions.push(match &conversion {
                                    TypeKind::Primitive(..) =>
                                        Expression::<RustFermentate, SPEC>::Simple(ident_name.to_token_stream()),
                                    TypeKind::Generic(GenericTypeKind::Optional(ty)) => match TypeKind::from(ty) {
                                        TypeKind::Primitive(_) =>
                                            Expression::ffi_to_primitive_opt_tokens(ident_name),
                                        TypeKind::Complex(_) |
                                        TypeKind::Generic(_) =>
                                            Expression::ffi_to_complex_opt_tokens(ident_name),
                                    },
                                    _ => Expression::ffi_to_complex_tokens(ident_name)
                                });
                            });
                        let ffi_type = self.present_ffi_aspect();
                        let lifetimes = self.compose_lifetimes();
                        let args_to = arg_to_conversions.present(&source);
                        let conversion = InterfacePresentation::callback(attrs, &ffi_type, arg_target_fields, return_type, &lifetimes, args_to, post_processing);
                        BindingPresentation::Callback {
                            name: full_fn_path.mangle_ident_default(),
                            attrs: attrs.clone(),
                            ffi_args,
                            result: ffi_return_type,
                            conversion
                        }
                    }
                }

            }
            _ => panic!("Wrong name context for fn")
        };
        RustFermentate::Function { comment: self.compose_docs(), binding }
    }
}
fn field<LANG, SPEC>(name: SPEC::Name , ty: &Type, source: &ScopeContext) -> Field
    where LANG: LangFermentable,
          SPEC: Specification<LANG> {
    Field {
        attrs: vec![],
        vis: Visibility::Inherited,
        ident: Some(name.mangle_ident_default()),
        colon_token: None,
        ty: match ty {
            Type::Ptr(TypePtr { const_token, mutability, .. }) => {
                let ty = ty.full_type(source);
                if const_token.is_some() {
                    parse_quote!(*const #ty)
                } else if mutability.is_some() {
                    parse_quote!(*mut #ty)
                } else {
                    ty
                }
            },
            _ => ty.full_type(source)
        },
    }
}
