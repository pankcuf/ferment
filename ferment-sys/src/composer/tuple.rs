use std::rc::Rc;
use quote::ToTokens;
use syn::{Attribute, Lifetime, Type, TypeTuple};
use syn::token::Comma;
use ferment_macro::ComposerBase;
use crate::ast::{CommaPunctuated, Depunctuated, ParenWrapped, SemiPunctuated};
use crate::composable::{AttrsModel, FieldComposer, FieldTypeKind, GenModel, LifetimesModel};
use crate::composer::{AspectPresentable, AttrComposable, BasicComposer, BasicComposerOwner, SourceComposable, ComposerLink, GenericComposerInfo, BasicComposerLink};
use crate::context::{ScopeContext, ScopeContextLink};
use crate::ext::{LifetimeProcessor, Mangle, Primitive, Resolve};
use crate::lang::{NameComposable, RustSpecification, Specification};
use crate::presentable::{Aspect, ConversionExpressionKind, Expression, ScopeContextPresentable};
use crate::presentation::{DictionaryExpr, DocComposer, InterfacePresentation};

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
        let name = self.type_tuple.mangle_ident_default();
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
                let kind = ConversionExpressionKind::from(&ty);
                from_conversions.push(Expression::expression_from(kind, Expression::ffi_ref_with_name(&name)).present(source));
                to_conversions.push(Expression::Named((name.to_token_stream(), Expression::expression_to(kind, Expression::obj_name(&field_name)).into())).present(source));
                if !ty.is_primitive() {
                    destroy_conversions.push(Expression::expression_drop(kind, Expression::dict_expr(DictionaryExpr::self_prop(&name))).present(source));
                }
                field_composers.push(FieldComposer::unnamed(name, FieldTypeKind::Type(ty)));
            });
        let attrs = self.compose_attributes();
        let interfaces = Depunctuated::from_iter([
            InterfacePresentation::conversion_from_root(&attrs, &types, ParenWrapped::<_, Comma>::new(from_conversions), &None, &lifetimes),
            InterfacePresentation::conversion_to_boxed_self_destructured(&attrs, &types, to_conversions, &None, &lifetimes),
            InterfacePresentation::drop(&attrs, ffi_type, destroy_conversions)
        ]);
        Some(GenericComposerInfo::<RustSpecification>::default(Aspect::raw_struct_ident(name), &attrs, field_composers, interfaces))
    }
}
