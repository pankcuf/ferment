use std::rc::Rc;
use quote::ToTokens;
use syn::{Attribute, Lifetime, Type};
use syn::token::Comma;
use ferment_macro::ComposerBase;
use crate::ast::{CommaPunctuated, Depunctuated, ParenWrapped, SemiPunctuated};
use crate::composable::{AttrsModel, FieldComposer, FieldTypeKind, GenericBoundsModel, GenModel, LifetimesModel};
use crate::composer::{AspectPresentable, AttrComposable, BasicComposer, BasicComposerOwner, SourceComposable, ComposerLink, GenericComposerInfo, BasicComposerLink, FFIAspect};
use crate::context::{ScopeContext, ScopeContextLink};
use crate::conversion::GenericArgPresentation;
use crate::ext::{LifetimeProcessor, Mangle, Primitive, Resolve, ToType};
use crate::lang::{RustSpecification, Specification};
use crate::presentable::{Aspect, ConversionExpressionKind, Expression, ScopeContextPresentable};
use crate::presentation::{DictionaryExpr, DocComposer, InterfacePresentation, Name, ToFFIVariable};

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
        let mut from_conversions = CommaPunctuated::new();
        let mut to_conversions = CommaPunctuated::new();
        let mut destroy_conversions = SemiPunctuated::new();
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
                let (kind, destroy_expr) = if ty.is_primitive() {
                    (
                        ConversionExpressionKind::Primitive,
                        Expression::default(),
                    )
                } else {
                    (
                        ConversionExpressionKind::Complex,
                        Expression::dict_expr(DictionaryExpr::self_prop(&name)),
                    )
                };
                let item = GenericArgPresentation::<RustSpecification>::new(
                    ty.to_direct_var(),
                    Expression::ConversionExpr(FFIAspect::Drop, kind, destroy_expr.into()),
                    Expression::ConversionExpr(FFIAspect::From, kind, Expression::ffi_ref_with_name(&name).into()),
                    Expression::Named((name.to_token_stream(), Expression::ConversionExpr(FFIAspect::To, kind, Expression::obj_name(&field_name).into()).into())));
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
        let aspect = Aspect::raw_struct_ident(ffi_name);
        Some(GenericComposerInfo::<RustSpecification>::default(aspect, &attrs, field_composers, interfaces))
    }
}
