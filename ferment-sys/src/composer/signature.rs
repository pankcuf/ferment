use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::Rc;
use quote::{quote, ToTokens};
use syn::{Attribute, BareFnArg, Field, FnArg, Generics, ImplItemMethod, ItemFn, parse_quote, Pat, Path, PatType, Receiver, ReturnType, Signature, TraitItemMethod, Type, TypeBareFn, TypePtr, Visibility};
use syn::__private::TokenStream2;
use syn::token::Semi;
use ferment_macro::ComposerBase;
use crate::ast::CommaPunctuated;
use crate::composable::{AttrsModel, CfgAttributes, FieldComposer, FieldTypeKind, FnSignatureContext, GenModel};
use crate::composer::{AspectPresentable, BasicComposer, BasicComposerOwner, CommaPunctuatedPresentableArguments, SourceComposable, ComposerLink, constants, DocsComposable, FromConversionComposer, FromConversionFullComposer, Linkable, SourceAccessible, SourceFermentable, ToConversionComposer, TypeAspect, VarComposer, BasicComposerLink, NameKind};
use crate::context::{ScopeContext, ScopeContextLink};
use crate::conversion::{GenericTypeKind, TypeKind};
use crate::ext::{FFITypeResolve, ItemExtension, Mangle, Resolve, ToType};
use crate::lang::{LangFermentable, RustSpecification, Specification};
use crate::presentable::{Aspect, BindingPresentableContext, TypeContext, ArgKind, ScopeContextPresentable, SeqKind, Expression};
use crate::presentation::{ArgPresentation, BindingPresentation, DictionaryExpr, DictionaryName, DocPresentation, FFIConversionFromMethodExpr, FFIFullDictionaryPath, InterfacePresentation, Name, RustFermentate};

#[derive(ComposerBase)]
pub struct SigComposer<LANG, SPEC>
    where LANG: LangFermentable + 'static,
          SPEC: Specification<LANG, Expr: Clone + ScopeContextPresentable> + 'static,
          Aspect<SPEC::TYC>: ScopeContextPresentable {
    pub base: BasicComposerLink<Self, LANG, SPEC>,
    // #[allow(unused)]
    // pub binding_composer: BindingComposer<ComposerLink<Self>, LANG, SPEC>
}

impl<LANG, SPEC> SigComposer<LANG, SPEC>
    where LANG: LangFermentable,
          SPEC: Specification<LANG, Expr: Clone + ScopeContextPresentable>,
          Aspect<SPEC::TYC>: ScopeContextPresentable {

    fn new(
        ty_context: SPEC::TYC,
        generics: Option<Generics>,
        attrs: AttrsModel,
        // binding_composer: BindingComposer<ComposerLink<Self>, LANG, SPEC>,
        context: &ScopeContextLink) -> ComposerLink<Self> {
        let root = Rc::new(RefCell::new(Self {
            base: BasicComposer::from(attrs, ty_context, GenModel::new(generics), constants::composer_doc(), Rc::clone(context)),
            // binding_composer,
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
        attrs: &Vec<Attribute>,
        context: &ScopeContextLink
    ) -> ComposerLink<Self> {
        Self::new(
            ty_context,
            Some(generics.clone()),
            AttrsModel::from(attrs),
            // binding_composer_(),
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
            attrs,
            context)
    }
    pub fn from_type_bare_fn(
        ty_context: SPEC::TYC,
        generics: &Generics,
        attrs: &Vec<Attribute>,
        context: &ScopeContextLink
    ) -> ComposerLink<Self> {
        Self::with_context(
            ty_context,
            generics,
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
        Self::with_context(ty_context, &sig.generics, attrs, context)
    }
}

impl<LANG, SPEC> DocsComposable for SigComposer<LANG, SPEC>
    where LANG: LangFermentable,
          SPEC: Specification<LANG, Expr: Clone + ScopeContextPresentable>,
          Aspect<SPEC::TYC>: ScopeContextPresentable {
    fn compose_docs(&self) -> DocPresentation {
        DocPresentation::Direct(self.base.doc.compose(&()))
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
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          ArgKind<RustFermentate, SPEC>: ScopeContextPresentable,
          CommaPunctuatedPresentableArguments<RustFermentate, SPEC>: Extend<ArgKind<RustFermentate, SPEC>> {
    println!("compose_regular_fn: {}", path.to_token_stream());
    let Signature { output, inputs, asyncness, .. } = sig;
    let (return_type_presentation, return_type_conversion) = match output {
        ReturnType::Default => (ReturnType::Default, SPEC::Expr::Simple(Semi::default().to_token_stream())),
        ReturnType::Type(_, ty) => (
            ReturnType::Type(Default::default(), Box::new(VarComposer::<RustFermentate, SPEC>::key_in_scope(ty, &source.scope).compose(source).to_type())),
            ToConversionComposer::<RustFermentate, SPEC>::new(Name::Dictionary(DictionaryName::Obj), *ty.clone(), None).compose(source)
        )
    };

    let (arguments, argument_conversions): (CommaPunctuatedPresentableArguments<RustFermentate, SPEC>, CommaPunctuatedPresentableArguments<RustFermentate, SPEC>) = inputs
        .iter()
        .map(|arg| {
            match arg {
                FnArg::Receiver(Receiver { mutability, reference, attrs, .. }) => {
                    let expr_composer = match (mutability, reference) {
                        (Some(..), _) => |expr: SPEC::Expr| SPEC::Expr::AsMutRef(expr.into()),
                        (_, Some(..)) => |expr: SPEC::Expr| SPEC::Expr::AsRef(expr.into()),
                        (..) => |expr: SPEC::Expr| expr.into(),
                    };
                    let (ty, name_type_conversion) = match sig_context {
                        FnSignatureContext::Impl(self_ty, maybe_trait_ty, _) |
                        FnSignatureContext::TraitInner(self_ty, maybe_trait_ty, _) => match maybe_trait_ty {
                            Some(trait_ty) => (
                                trait_ty,
                                expr_composer(from_trait_receiver_expr_composer::<RustFermentate, SPEC>(self_ty, if mutability.is_some() { quote!(mut) } else { quote!(const) }, source))
                            ),
                            None => (
                                self_ty,
                                expr_composer(from_receiver_expr_composer::<RustFermentate, SPEC>(self_ty, source))
                            )
                        }
                        _ => panic!("Receiver in regular fn")
                    };
                    let name = Name::Dictionary(DictionaryName::Self_);
                    let ty = VarComposer::<RustFermentate, SPEC>::key_in_scope(ty, &source.scope).compose(source).to_type();
                    (
                        ArgKind::Named(
                            FieldComposer::new(name, FieldTypeKind::Type(ty), true, attrs.cfg_attributes()),
                            Visibility::Inherited
                        ),
                        ArgKind::AttrExpression(
                            name_type_conversion,
                            SPEC::Attr::default()
                        )
                    )
                },
                FnArg::Typed(PatType { ty, attrs, pat, .. }) => {
                    let name = Name::Pat(*pat.clone());
                    (
                        ArgKind::Named(FieldComposer::typed(name.clone(), ty, true, attrs), Visibility::Inherited),
                        ArgKind::AttrExpression(
                            FromConversionFullComposer::<RustFermentate, SPEC>::key_in_scope(name, ty, &source.scope).compose(source),
                            SPEC::Attr::default()
                        )
                    )
                },
            }
        })
        .unzip();
    let input_conversions = SeqKind::UnnamedFields(((aspect, attrs.clone(), generics.clone(), NameKind::Named), argument_conversions));

    BindingPresentableContext::RegFn(
        path,
        asyncness.is_some(),
        arguments,
        return_type_presentation,
        input_conversions,
        return_type_conversion,
        attrs,
        generics
    )
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
                    FnSignatureContext::Impl(_, _, sig) => compose_regular_fn::<SPEC>(
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
                        let compose_ty = |ty: &Type| VarComposer::<RustFermentate, SPEC>::key_in_scope(ty, &source.scope).compose(&source).to_type();
                        let return_type = match output {
                            ReturnType::Default => ReturnType::Default,
                            ReturnType::Type(_, ty) => ReturnType::Type(Default::default(), Box::new(compose_ty(ty)))
                        };
                        let arguments = CommaPunctuatedPresentableArguments::from_iter(inputs
                            .iter()
                            .map(|arg| {
                                ArgKind::<RustFermentate, SPEC>::Named(match arg {
                                    FnArg::Receiver(Receiver { mutability: _, reference: _, attrs, .. }) =>
                                        FieldComposer::self_typed(compose_ty(sig_context.receiver_ty()), attrs),
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
                        let conversion = InterfacePresentation::callback(attrs, &ffi_type, arg_target_fields, return_type, arg_to_conversions.present(&source), post_processing);
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
pub fn from_receiver_expr_composer<LANG, SPEC>(ty: &Type, source: &ScopeContext) -> SPEC::Expr
    where LANG: LangFermentable,
          SPEC: Specification<LANG, Attr: Debug, Expr=Expression<LANG, SPEC>, Name=Name<LANG, SPEC>, Var: ToType>,
          SPEC::Expr: ScopeContextPresentable,
          Name<LANG, SPEC>: ToTokens,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          FFIFullDictionaryPath<LANG, SPEC>: ToType {
    FromConversionComposer::<LANG, SPEC>::new(Name::Dictionary(DictionaryName::Self_), ty.clone(), None)
        .compose(source)

}
pub fn from_trait_receiver_expr_composer<LANG, SPEC>(ty: &Type, acc: TokenStream2, source: &ScopeContext) -> SPEC::Expr
    where LANG: LangFermentable,
          SPEC: Specification<LANG, Attr: Debug, Expr=Expression<LANG, SPEC>, Var: ToType>,
          SPEC::Expr: ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable {
    let ty: Type = ty.resolve(source);
    Expression::dict_expr(DictionaryExpr::SelfAsTrait(ty.to_token_stream(), acc))
}


fn field<LANG, SPEC>(name: SPEC::Name , ty: &Type, source: &ScopeContext) -> Field
    where LANG: LangFermentable,
          SPEC: Specification<LANG>,
          Aspect<SPEC::TYC>: ScopeContextPresentable {
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
// pub const fn binding_composer_<T, CTX, LANG, SPEC>()
//     -> BindingComposer<ComposerLink<T>, LANG, SPEC>
//     where T: BasicComposerOwner<CTX, LANG, SPEC>,
//           CTX: Clone,
//           LANG: LangFermentable,
//           SPEC: Specification<LANG>,
//           Aspect<CTX>: ScopeContextPresentable,
//           OwnedItemPresentableContext<LANG, SPEC>: ScopeContextPresentable {
//     |composer| {
//         let composer = composer.borrow();
//         let _source = composer.base().context.borrow();
//         BindingPresentableContext::RegFn()
//     }
// }
//


