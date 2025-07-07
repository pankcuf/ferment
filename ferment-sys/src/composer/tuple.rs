use std::rc::Rc;
use quote::ToTokens;
use syn::{Attribute, Lifetime, Type, TypeTuple};
use syn::token::Comma;
use ferment_macro::ComposerBase;
use crate::ast::{CommaPunctuated, Depunctuated, ParenWrapped, SemiPunctuated};
use crate::composable::{AttrsModel, FieldComposer, FieldTypeKind, GenModel, LifetimesModel};
use crate::composer::{AspectPresentable, AttrComposable, BasicComposer, BasicComposerOwner, SourceComposable, ComposerLink, GenericComposerInfo, BasicComposerLink, FFIAspect};
use crate::context::{ScopeContext, ScopeContextLink};
use crate::conversion::{GenericArgPresentation, GenericTypeKind, TypeKind};
use crate::ext::{LifetimeProcessor, Mangle, Resolve};
use crate::lang::{NameComposable, RustSpecification, Specification};
use crate::presentable::{Aspect, ConversionExpressionKind, Expression, ScopeContextPresentable};
use crate::presentation::{DictionaryExpr, DocComposer, InterfacePresentation, ToFFIVariable};

#[derive(ComposerBase)]
pub struct TupleComposer<SPEC>
    where SPEC: Specification + 'static {
    pub type_tuple: TypeTuple,
    base: BasicComposerLink<SPEC, Self>,
}

impl<SPEC> TupleComposer<SPEC>
    where SPEC: Specification {
    pub fn new(type_tuple: &TypeTuple, ty_context: SPEC::TYC, attrs: Vec<Attribute>, scope_context: &ScopeContextLink) -> Self {
        Self {
            base: BasicComposer::from(DocComposer::new(ty_context.to_token_stream()), AttrsModel::from(&attrs), ty_context, GenModel::default(), LifetimesModel::default(), Rc::clone(scope_context)),
            type_tuple: type_tuple.clone(),
        }
    }
}

impl SourceComposable for TupleComposer<RustSpecification> {
    type Source = ScopeContext;
    type Output = Option<GenericComposerInfo<RustSpecification>>;

    fn compose(&self, source: &Self::Source) -> Self::Output {
        let mut lifetimes = Vec::<Lifetime>::new();
        let ffi_type = self.present_ffi_aspect();
        let types = (ffi_type.clone(), self.present_target_aspect());
        let mut from_conversions = CommaPunctuated::new();
        let mut to_conversions = CommaPunctuated::new();
        let mut destroy_conversions = SemiPunctuated::new();
        let mut field_composers = Depunctuated::new();
        self.type_tuple
            .elems
            .iter()
            .enumerate()
            .for_each(|(index, ty)| {
                lifetimes.extend(ty.unique_lifetimes());
                let name = <RustSpecification as Specification>::Name::unnamed_arg(index);
                let field_name = <RustSpecification as Specification>::Name::index(index);
                let ty: Type = ty.resolve(source);
                let (kind, destroy_expr) = match TypeKind::from(&ty) {
                    TypeKind::Primitive(..) => (
                        ConversionExpressionKind::Primitive,
                        Expression::default(),
                    ),
                    TypeKind::Generic(GenericTypeKind::Optional(..)) => (
                        ConversionExpressionKind::ComplexOpt,
                        Expression::dict_expr(DictionaryExpr::self_prop(&name)),
                    ),
                    _ => (
                        ConversionExpressionKind::Complex,
                        Expression::dict_expr(DictionaryExpr::self_prop(&name)),
                    ),
                };
                let from_expr = Expression::ffi_ref_with_name(&name);
                let to_expr = Expression::obj_name(&field_name);
                let item = GenericArgPresentation::<RustSpecification>::new(
                    ty.to_direct_var(),
                    Expression::ConversionExpr(FFIAspect::Drop, kind, destroy_expr.into()),
                    Expression::ConversionExpr(FFIAspect::From, kind, from_expr.into()),
                    Expression::Named((name.to_token_stream(), Expression::ConversionExpr(FFIAspect::To, kind, to_expr.into()).into())));
                from_conversions.push(item.from_conversion.present(source));
                to_conversions.push(item.to_conversion.present(source));
                destroy_conversions.push(item.destructor.present(source));
                field_composers.push(FieldComposer::unnamed(name, FieldTypeKind::Type(ty)));
            });
        let attrs = self.compose_attributes();
        let interfaces = Depunctuated::from_iter([
            InterfacePresentation::conversion_from_root(&attrs, &types, ParenWrapped::<_, Comma>::new(from_conversions), &None, &lifetimes),
            InterfacePresentation::conversion_to_boxed_self_destructured(&attrs, &types, to_conversions, &None, &lifetimes),
            InterfacePresentation::drop(&attrs, ffi_type, destroy_conversions)
        ]);
        let aspect = Aspect::raw_struct_ident(self.type_tuple.mangle_ident_default());
        Some(GenericComposerInfo::<RustSpecification>::default(aspect, &attrs, field_composers, interfaces))
    }
}
