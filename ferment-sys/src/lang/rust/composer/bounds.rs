use syn::{Lifetime, Type};
use crate::ast::{CommaParenWrapped, CommaPunctuated, Depunctuated, SemiPunctuated};
use crate::composable::FieldComposer;
use crate::composer::{AspectPresentable, AttrComposable, BoundsComposer, GenericComposerInfo, SourceComposable};
use crate::context::ScopeContext;
use crate::ext::{LifetimeProcessor, Mangle, Primitive, Resolve, ToType};
use crate::kind::FieldTypeKind;
use crate::lang::{NameComposable, RustSpecification, Specification};
use crate::presentable::{Aspect, Expression, ScopeContextPresentable};
use crate::presentable::ConversionExpressionKind;
use crate::presentation::{DictionaryExpr, InterfacePresentation};
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
                let name = <RustSpecification as Specification>::Name::unnamed_arg(index);
                let field_name = <RustSpecification as Specification>::Name::index(index);
                let ty: Type = predicate_ty.resolve(source);
                lifetimes.extend(predicate_ty.unique_lifetimes());
                let kind = ConversionExpressionKind::from(&ty);
                from_conversions.push(Expression::expression_from(kind, Expression::<RustSpecification>::ffi_ref_with_name(&name)).present(source));
                to_conversions.push(Expression::named(&name, Expression::expression_to(kind, Expression::<RustSpecification>::obj_name(&field_name))).present(source));
                if !ty.is_primitive() {
                    destroy_conversions.push(Expression::expression_drop(kind, Expression::<RustSpecification>::dict_expr(DictionaryExpr::self_prop(&name))).present(source));
                }
                field_composers.push(FieldComposer::unnamed(name, FieldTypeKind::Type(ty)));
            });
        let interfaces = Depunctuated::from_iter([
            InterfacePresentation::conversion_from_root(&attrs, &types, CommaParenWrapped::new(from_conversions), &None, &lifetimes),
            InterfacePresentation::conversion_to_boxed_self_destructured(&attrs, &types, to_conversions, &None, &lifetimes),
            InterfacePresentation::drop(&attrs, ffi_name.to_type(), destroy_conversions)
        ]);
        Some(GenericComposerInfo::<RustSpecification>::default(Aspect::raw_struct_ident(ffi_name), &attrs, field_composers, interfaces))
    }
}
