use std::marker::PhantomData;
use std::rc::Rc;
use syn::{Attribute, Generics};
use syn::token::Comma;
use crate::ast::{CommaPunctuated, Depunctuated, ParenWrapped, SemiPunctuated};
use crate::composable::{AttrsModel, FieldComposer, FieldTypeKind, GenericBoundsModel, GenModel};
use crate::composer::{Composer, GenericComposerInfo, NameComposable, ComposerLink, BasicComposer, constants, BasicComposerOwner, AttrComposable, NameContext, SourceAccessible};
use crate::context::ScopeContext;
use crate::conversion::dictionary_generic_arg_pair;
use crate::ext::{Mangle, ToType};
use crate::lang::{LangAttrSpecification, LangGenSpecification};
use crate::presentable::{Context, OwnedItemPresentableContext, ScopeContextPresentable, SequenceOutput};
use crate::presentation::{DictionaryName, InterfacePresentation, Name, RustFermentate};

pub struct BoundsComposer<LANG, SPEC, Gen>
    where LANG: Clone + 'static,
          SPEC: LangAttrSpecification<LANG> + 'static,
          Gen: LangGenSpecification<LANG> + 'static,
          SequenceOutput<LANG, SPEC>: ScopeContextPresentable,
          OwnedItemPresentableContext<LANG, SPEC>: ScopeContextPresentable {
    pub model: GenericBoundsModel,
    // pub attrs: SPEC,
    base: BasicComposer<ComposerLink<Self>, LANG, SPEC, Gen>,
    phantom_data: PhantomData<LANG>,
    // pub context: ComposerLink<ScopeContext>,
}

impl<LANG, SPEC, Gen> BoundsComposer<LANG, SPEC, Gen>
    where LANG: Clone,
          SPEC: LangAttrSpecification<LANG>,
          Gen: LangGenSpecification<LANG>,
          SequenceOutput<LANG, SPEC>: ScopeContextPresentable,
          OwnedItemPresentableContext<LANG, SPEC>: ScopeContextPresentable {
    pub fn new(model: &GenericBoundsModel, context: Context, attrs: Vec<Attribute>, scope_context: &ComposerLink<ScopeContext>) -> Self {
        // let attrs = SPEC::from_attrs(attrs.clone());
        Self {
            model: model.clone(),
            base: BasicComposer::<ComposerLink<Self>, LANG, SPEC, Gen>::from(AttrsModel::from(&attrs), context, GenModel::default(), constants::composer_doc(), Rc::clone(scope_context)),
            // attrs: attrs.clone(),
            phantom_data: Default::default() }
    }
}

impl<LANG, SPEC, Gen> BasicComposerOwner<Context, LANG, SPEC, Gen> for BoundsComposer<LANG, SPEC, Gen>
    where LANG: Clone,
          SPEC: LangAttrSpecification<LANG>,
          Gen: LangGenSpecification<LANG>,
          SequenceOutput<LANG, SPEC>: ScopeContextPresentable,
          OwnedItemPresentableContext<LANG, SPEC>: ScopeContextPresentable {
    fn base(&self) -> &BasicComposer<ComposerLink<Self>, LANG, SPEC, Gen> {
        &self.base
    }
}
impl<LANG, SPEC, Gen> NameContext<Context> for BoundsComposer<LANG, SPEC, Gen>
    where LANG: Clone,
          SPEC: LangAttrSpecification<LANG>,
          Gen: LangGenSpecification<LANG>,
          SequenceOutput<LANG, SPEC>: ScopeContextPresentable,
          OwnedItemPresentableContext<LANG, SPEC>: ScopeContextPresentable {
    fn name_context_ref(&self) -> &Context {
        self.base().name_context_ref()
    }
}
impl<LANG, SPEC, Gen> SourceAccessible for BoundsComposer<LANG, SPEC, Gen>
    where LANG: Clone,
          SPEC: LangAttrSpecification<LANG>,
          Gen: LangGenSpecification<LANG>,
          SequenceOutput<LANG, SPEC>: ScopeContextPresentable,
          OwnedItemPresentableContext<LANG, SPEC>: ScopeContextPresentable {
    fn context(&self) -> &ComposerLink<ScopeContext> {
        self.base().context()
    }
}

impl<'a> Composer<'a> for BoundsComposer<RustFermentate, Vec<Attribute>, Option<Generics>> {
    type Source = ScopeContext;
    type Output = Option<GenericComposerInfo<RustFermentate, Vec<Attribute>, Option<Generics>>>;

    fn compose(&self, source: &'a Self::Source) -> Self::Output {
        if self.model.is_lambda() {
            return None;
        }
        let ffi_name = self.model.mangle_ident_default();
        let types = (self.compose_ffi_name(), self.compose_target_name());
        let attrs = self.base.compose_attributes();
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
                let (ty, args) = dictionary_generic_arg_pair(name.clone(), Name::Index(index), predicate_ty, &source);
                args.iter().for_each(|item| {
                    from_conversions.push(item.from_conversion.present(source));
                    to_conversions.push(item.to_conversion.present(source));
                    destroy_conversions.push(item.destructor.present(source));
                });
                field_composers.push(FieldComposer::unnamed(name, FieldTypeKind::Type(ty)));
            });
        let interfaces = Depunctuated::from_iter([
            InterfacePresentation::conversion_from_root(&attrs, &types, ParenWrapped::<_, Comma>::new(from_conversions), &None),
            InterfacePresentation::conversion_to_boxed_self_destructured(&attrs, &types, to_conversions, &None),
            InterfacePresentation::conversion_unbox_any_terminated(&attrs, &types, DictionaryName::Ffi, &None),
            InterfacePresentation::drop(&attrs, ffi_name.to_type(), destroy_conversions)
        ]);
        Some(GenericComposerInfo::<RustFermentate, Vec<Attribute>, Option<Generics>>::default(ffi_name, &attrs, field_composers, interfaces))
    }
}
