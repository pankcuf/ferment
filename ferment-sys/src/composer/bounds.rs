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
use crate::lang::{RustSpecification, Specification};
use crate::presentable::{Aspect, ConversionExpressionKind, Expression, ScopeContextPresentable, TypeContext};
use crate::presentation::{DictionaryExpr, DocComposer, InterfacePresentation, Name};

#[derive(ComposerBase)]
pub struct BoundsComposer<SPEC>
    where SPEC: Specification + 'static {
    pub model: GenericBoundsModel,
    base: BasicComposerLink<SPEC, Self>,
}

impl<SPEC> BoundsComposer<SPEC>
    where SPEC: Specification {
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

impl SourceComposable for BoundsComposer<RustSpecification> {
    type Source = ScopeContext;
    type Output = Option<GenericComposerInfo<RustSpecification>>;

    fn compose(&self, source: &Self::Source) -> Self::Output {
        if self.model.is_lambda() {
            return Self::Output::default();
        }
        let mut lifetimes = Vec::<Lifetime>::new();
        let ffi_name = self.model.mangle_ident_default();
        let types = (self.present_ffi_aspect(), self.present_target_aspect());
        let attrs = self.compose_attributes();
        let mut from_conversions = CommaPunctuated::<<<RustSpecification as Specification>::Expr as ScopeContextPresentable>::Presentation>::new();
        let mut to_conversions = CommaPunctuated::<<<RustSpecification as Specification>::Expr as ScopeContextPresentable>::Presentation>::new();
        let mut destroy_conversions = SemiPunctuated::<<<RustSpecification as Specification>::Expr as ScopeContextPresentable>::Presentation>::new();
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
                let (kind, destroy_expr, from_expr, to_expr) = match TypeKind::from(&ty) {
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
                let item = GenericArgPresentation::<RustSpecification>::new(
                    <RustSpecification as Specification>::Var::direct(ty.clone()),
                    Expression::ConversionExpr(FFIAspect::Drop, kind, destroy_expr.into()),
                    Expression::ConversionExpr(FFIAspect::From, kind, from_expr.into()),
                    Expression::Named((name.to_token_stream(), Expression::ConversionExpr(FFIAspect::To, kind, to_expr.into()).into())));
                from_conversions.push(item.from_conversion.present(source));
                to_conversions.push(item.to_conversion.present(source));
                destroy_conversions.push(item.destructor.present(source));
                field_composers.push(FieldComposer::unnamed(name, FieldTypeKind::Type(ty)));
            });
        let interfaces = Depunctuated::from_iter([
            InterfacePresentation::conversion_from_root(&attrs, &types, ParenWrapped::<_, Comma>::new(from_conversions), &None, &lifetimes),
            InterfacePresentation::conversion_to_boxed_self_destructured(&attrs, &types, to_conversions, &None, &lifetimes),
            InterfacePresentation::drop(&attrs, ffi_name.to_type(), destroy_conversions)
        ]);
        let aspect = Aspect::RawTarget(TypeContext::Struct { ident: ffi_name, attrs: vec![], generics: Generics::default() });
        Some(GenericComposerInfo::<RustSpecification>::default(aspect, &attrs, field_composers, interfaces))
    }
}
