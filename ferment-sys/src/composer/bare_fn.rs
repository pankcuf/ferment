use std::cell::RefCell;
use std::rc::Rc;
use quote::{quote, ToTokens};
use syn::{BareFnArg, Generics, Lifetime, Path, ReturnType, Type, TypeBareFn};
use ferment_macro::ComposerBase;
use crate::ast::CommaPunctuated;
use crate::composable::{AttrsModel, GenModel, LifetimesModel};
use crate::composer::{BasicComposer, BasicComposerOwner, BasicComposerLink, ComposerLink, DocComposer, DocsComposable, Linkable, SourceAccessible, SourceComposable, VarComposer, field, CommaPunctuatedArgKinds, SignatureAspect};
use crate::context::{ScopeContext, ScopeContextLink};
use crate::ext::{ExpressionComposable, Mangle, Primitive, Resolve, ToType, WrapIntoCurlyBraces};
use crate::kind::{GenericTypeKind, TypeKind};
use crate::lang::Specification;
use crate::presentable::{ArgKind, Aspect, BindingPresentableContext, Expression, ScopeContextPresentable};
use crate::presentation::{ArgPresentation, DictionaryExpr, DictionaryName, DocPresentation, FFIConversionFromMethodExpr, FFIFullDictionaryPath, FFIFullPath, Name};

#[allow(unused)]
#[derive(ComposerBase)]
pub struct BareFnComposer<SPEC>
where SPEC: Specification + 'static {
    pub base: BasicComposerLink<SPEC, Self>,
}

impl<SPEC> BareFnComposer<SPEC>
where SPEC: Specification {
    #[allow(unused)]
    fn new(
        ty_context: SPEC::TYC,
        generics: Option<Generics>,
        lifetimes: Vec<Lifetime>,
        attrs: AttrsModel,
        context: &ScopeContextLink) -> ComposerLink<Self> {
        let root = Rc::new(RefCell::new(Self {
            base: BasicComposer::from(DocComposer::from(&ty_context), attrs, ty_context, GenModel::new(generics), LifetimesModel::new(lifetimes), Rc::clone(context)),
        }));
        {
            let mut composer = root.borrow_mut();
            composer.base.link(&root);
        }
        root
    }
}

impl<SPEC> DocsComposable for BareFnComposer<SPEC>
where SPEC: Specification {
    fn compose_docs(&self) -> DocPresentation {
        DocPresentation::Direct(self.base.doc.compose(self.context()))
    }
}

pub fn compose_bare_fn<SPEC>(
    full_fn_path: &Path,
    signature_aspect: SignatureAspect<SPEC>,
    aspect: Aspect<SPEC::TYC>,
    type_bare_fn: &TypeBareFn,
    source: &ScopeContext
) -> BindingPresentableContext<SPEC>
where SPEC: Specification<Expr=Expression<SPEC>, Name=Name<SPEC>>,
      SPEC::Expr: ScopeContextPresentable,
      SPEC::Lt: IntoIterator + Extend<<SPEC::Lt as IntoIterator>::Item>,
      SPEC::Name: ToTokens,
      CommaPunctuatedArgKinds<SPEC>: Extend<ArgKind<SPEC>>,
      FFIFullPath<SPEC>: ToType,
      FFIFullDictionaryPath<SPEC>: ToType,
      Type: Resolve<SPEC::Var>,
      VarComposer<SPEC>: SourceComposable<Source=ScopeContext, Output: ToType> {
    let TypeBareFn { inputs, output, .. } = type_bare_fn;
    let ffi_result = DictionaryName::FFiResult;
    let ffi_result_conversion = FFIConversionFromMethodExpr::FfiFrom(ffi_result.to_token_stream());

    let (return_type, ffi_return_type, post_processing) = match output {
        ReturnType::Type(token, field_type) => (
            ReturnType::Type(token.clone(), Box::new(source.full_type_for(field_type))),
            ReturnType::Type(token.clone(), Box::new(<Type as Resolve<SPEC::Var>>::resolve(field_type, source).to_type())),
            match TypeKind::from(field_type) {
                TypeKind::Primitive(_) =>
                    DictionaryExpr::simple(&ffi_result),
                TypeKind::Generic(GenericTypeKind::Optional(ty)) if ty.is_primitive() =>
                    DictionaryExpr::IfThen(quote!((!#ffi_result.is_null())), DictionaryExpr::deref(&ffi_result).to_token_stream()),
                TypeKind::Generic(GenericTypeKind::Optional(_)) =>
                    DictionaryExpr::IfThen(quote!((!#ffi_result.is_null())), DictionaryExpr::callback_dtor(&ffi_result_conversion, &ffi_result).wrap()),
                TypeKind::Generic(GenericTypeKind::TraitBounds(_)) =>
                    unimplemented!("TODO: mixins+traits+generics"),
                TypeKind::Complex(_) |
                TypeKind::Generic(_) =>
                    DictionaryExpr::callback_dtor(&ffi_result_conversion, &ffi_result)
            }
        ),
        ReturnType::Default => (ReturnType::Default, ReturnType::Default, DictionaryExpr::simple(&ffi_result)),
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
            let var_composer = VarComposer::<SPEC>::key_ref_in_composer_scope(ty);
            let var_ty = var_composer.compose(&source);
            let conversion = TypeKind::from(ty);
            let ident_name = Name::<SPEC>::Optional(name.as_ref().map(|(ident, ..)| ident.clone()));
            arg_names.push(ident_name.to_token_stream());
            arg_target_types.push(ArgPresentation::no_attr_tokens(&ty));
            let mut bare_fn_arg_replacement = bare_fn_arg.clone();
            bare_fn_arg_replacement.ty = var_ty.to_type();
            ffi_args.push(bare_fn_arg_replacement);
            arg_to_conversions.push(match &conversion {
                TypeKind::Primitive(..) =>
                    Expression::<SPEC>::simple(&ident_name),
                TypeKind::Generic(GenericTypeKind::Optional(ty)) if ty.is_primitive() =>
                    Expression::ffi_to_primitive_opt_tokens(&ident_name),
                TypeKind::Generic(GenericTypeKind::Optional(_)) =>
                    Expression::ffi_to_complex_opt_tokens(&ident_name),
                _ =>
                    Expression::ffi_to_complex_tokens(&ident_name)
            });
            arg_target_fields.push(ArgPresentation::Field(field::<SPEC>(ident_name, ty, source)));
        });
    BindingPresentableContext::Callback(
        aspect,
        signature_aspect,
        full_fn_path.mangle_ident_default(),
        arg_target_fields,
        return_type,
        arg_to_conversions,
        post_processing,
        ffi_return_type,
        ffi_args
    )
}