use std::rc::Rc;
use quote::ToTokens;
use syn::{Attribute, Generics, Lifetime, Type};
use syn::token::Comma;
use ferment_macro::ComposerBase;
use crate::ast::{CommaPunctuated, Depunctuated, ParenWrapped, SemiPunctuated};
use crate::composable::{AttrsModel, FieldComposer, FieldTypeKind, GenericBoundsModel, GenModel, LifetimesModel};
use crate::composer::{AspectPresentable, AttrComposable, BasicComposer, BasicComposerOwner, SourceComposable, ComposerLink, GenericComposerInfo, BasicComposerLink, FFIAspect};
use crate::context::{ScopeContext, ScopeContextLink};
use crate::conversion::{GenericArgPresentation, TypeKind};
use crate::ext::{LifetimeProcessor, Mangle, Resolve, ToType};
use crate::lang::{LangFermentable, RustSpecification, Specification};
use crate::presentable::{Aspect, ConversionExpressionKind, Expression, ScopeContextPresentable, TypeContext};
use crate::presentation::{DictionaryExpr, DocComposer, InterfacePresentation, Name, RustFermentate};

#[derive(ComposerBase)]
pub struct BoundsComposer<LANG, SPEC>
    where LANG: LangFermentable + 'static,
          SPEC: Specification<LANG> + 'static {
    pub model: GenericBoundsModel,
    base: BasicComposerLink<LANG, SPEC, Self>,
}

impl<LANG, SPEC> BoundsComposer<LANG, SPEC>
    where LANG: LangFermentable,
          SPEC: Specification<LANG> {
    pub fn new(
        model: &GenericBoundsModel,
        ty_context: SPEC::TYC,
        attrs: Vec<Attribute>,
        scope_context: &ScopeContextLink
    ) -> Self {
        Self {
            model: model.clone(),
            base: BasicComposer::from(DocComposer::new(ty_context.to_token_stream()), AttrsModel::from(&attrs), ty_context, GenModel::default(), LifetimesModel::default(), Rc::clone(scope_context)),
        }
    }
}

impl<SPEC> SourceComposable for BoundsComposer<RustFermentate, SPEC>
    where SPEC: RustSpecification {
    type Source = ScopeContext;
    type Output = Option<GenericComposerInfo<RustFermentate, SPEC>>;

    fn compose(&self, source: &Self::Source) -> Self::Output {
        if self.model.is_lambda() {
            return Self::Output::default();
        }
        let mut lifetimes = Vec::<Lifetime>::new();
        let ffi_name = self.model.mangle_ident_default();
        let types = (self.present_ffi_aspect(), self.present_target_aspect());
        let attrs = self.compose_attributes();
        let mut from_conversions = CommaPunctuated::<<SPEC::Expr as ScopeContextPresentable>::Presentation>::new();
        let mut to_conversions = CommaPunctuated::<<SPEC::Expr as ScopeContextPresentable>::Presentation>::new();
        let mut destroy_conversions = SemiPunctuated::<<SPEC::Expr as ScopeContextPresentable>::Presentation>::new();
        let mut field_composers = Depunctuated::new();
        self.model
            .predicates
            .keys()
            .enumerate()
            .for_each(|(index, predicate_ty)| {
                let name = Name::UnnamedArg(index);

                let ty: Type = predicate_ty.resolve(source);
                let field_name = Name::Index(index);
                lifetimes.extend(predicate_ty.unique_lifetimes());
                //name: Name, field_name: Name, ty: &Type, source: &ScopeContext
                let (kind, destroy_expr,
                    from_expr,
                    to_expr) = match TypeKind::from(&ty) {
                    TypeKind::Primitive(..) => (
                        ConversionExpressionKind::Primitive,
                        Expression::empty(),
                        Expression::ffi_ref_with_name(&name),
                        Expression::obj_name(&field_name),
                    ),
                    _ => (
                        ConversionExpressionKind::Complex,
                        Expression::dict_expr(DictionaryExpr::SelfProp(name.to_token_stream())),
                        Expression::ffi_ref_with_name(&name),
                        Expression::obj_name(&field_name)
                    ),
                };
                let item = GenericArgPresentation::<RustFermentate, SPEC>::new(
                    SPEC::Var::direct(ty.clone()),
                    Expression::ConversionExpr(FFIAspect::Drop, kind, destroy_expr.into()),
                    Expression::ConversionExpr(FFIAspect::From, kind, from_expr.into()),
                    Expression::Named((name.to_token_stream(), Expression::ConversionExpr(FFIAspect::To, kind, to_expr.into()).into())));


                // let (ty, item) = dictionary_generic_arg_pair::<RustFermentate, SPEC>(name.clone(), Name::Index(index), predicate_ty, &source);
                // args.iter().for_each(|item| {
                    from_conversions.push(item.from_conversion.present(source));
                    to_conversions.push(item.to_conversion.present(source));
                    destroy_conversions.push(item.destructor.present(source));
                // });
                field_composers.push(FieldComposer::unnamed(name, FieldTypeKind::Type(ty)));
            });
        let interfaces = Depunctuated::from_iter([
            InterfacePresentation::conversion_from_root(&attrs, &types, ParenWrapped::<_, Comma>::new(from_conversions), &None, &lifetimes),
            InterfacePresentation::conversion_to_boxed_self_destructured(&attrs, &types, to_conversions, &None, &lifetimes),
            // InterfacePresentation::conversion_unbox_any_terminated(&attrs, &types, DictionaryName::Ffi, &None),
            InterfacePresentation::drop(&attrs, ffi_name.to_type(), destroy_conversions)
        ]);
        let aspect = Aspect::RawTarget(TypeContext::Struct { ident: ffi_name, attrs: vec![], generics: Generics::default() });
        Some(GenericComposerInfo::<RustFermentate, SPEC>::default(aspect, &attrs, field_composers, interfaces))
    }
}
