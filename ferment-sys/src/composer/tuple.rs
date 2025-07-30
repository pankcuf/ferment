use std::rc::Rc;
use quote::ToTokens;
use syn::{Attribute, Lifetime, TypeTuple};
use ferment_macro::ComposerBase;
use crate::ast::{CommaParenWrapped, CommaPunctuated, Depunctuated, SemiPunctuated};
use crate::composable::{AttrsModel, FieldComposer, FieldTypeKind, GenModel, LifetimesModel};
use crate::composer::{AspectPresentable, AttrComposable, BasicComposer, BasicComposerOwner, SourceComposable, ComposerLink, GenericComposerInfo, BasicComposerLink, ConversionFromComposer, ConversionToComposer, ConversionDropComposer, VariableComposer};
use crate::context::{ScopeContext, ScopeContextLink};
use crate::ext::{LifetimeProcessor, Mangle};
use crate::lang::{NameComposable, RustSpecification, Specification};
use crate::presentable::{Aspect, Expression, ScopeContextPresentable};
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
                let from_composer = ConversionFromComposer::<RustSpecification>::value_expr(name.clone(), ty, Expression::ffi_ref_with_name(&name));
                let to_composer = ConversionToComposer::<RustSpecification>::value_expr(name.clone(), ty, Expression::ObjName(<RustSpecification as Specification>::Name::index(index)));
                let drop_composer = ConversionDropComposer::<RustSpecification>::value_expr(name.clone(), ty, Expression::dict_expr(DictionaryExpr::self_prop(&name)));
                from_conversions.push(from_composer.compose(source).present(source));
                to_conversions.push(Expression::named(&name, to_composer.compose(source)).present(source));
                if let Some(drop_conversion) = drop_composer.compose(source) {
                    destroy_conversions.push(drop_conversion.present(source));
                }
                field_composers.push(FieldComposer::unnamed(name, FieldTypeKind::Var(VariableComposer::<RustSpecification>::from(ty).compose(source))));
            });
        let attrs = self.compose_attributes();
        let interfaces = Depunctuated::from_iter([
            InterfacePresentation::conversion_from_root(&attrs, &types, CommaParenWrapped::new(from_conversions), &None, &lifetimes),
            InterfacePresentation::conversion_to_boxed_self_destructured(&attrs, &types, to_conversions, &None, &lifetimes),
            InterfacePresentation::drop(&attrs, ffi_type, destroy_conversions)
        ]);
        Some(GenericComposerInfo::<RustSpecification>::default(Aspect::raw_struct_ident(name), &attrs, field_composers, interfaces))
    }
}
