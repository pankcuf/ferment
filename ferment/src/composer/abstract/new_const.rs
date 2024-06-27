use quote::quote;
use crate::composable::FieldTypeComposition;
use crate::composer::{FieldTypeLocalContext, FieldTypesContext, LocalConversionContext, LocalFieldsOwnerContext, OwnedItemsPunctuated, OwnerAspectWithItems};
use crate::ext::Conversion;
use crate::presentable::{Expression, OwnedItemPresentableContext};

pub type FieldTypeIterator<T, RES> = dyn Fn((T, dyn Fn(&FieldTypeLocalContext) -> Expression)) -> RES + 'static;
pub type FieldTypeIterator2<T, RES> = dyn Fn((T, dyn Fn(&FieldTypeComposition) -> OwnedItemPresentableContext)) -> RES + 'static;
pub type OwnedIteratorPostProcessor<SEP> = FieldTypeIterator<LocalConversionContext, OwnerAspectWithItems<SEP>>;
pub type OwnedIteratorPostProcessor2<SEP> = FieldTypeIterator2<LocalConversionContext, OwnerAspectWithItems<SEP>>;

fn compose_fields_conversions<F, Out, It>(field_types: FieldTypesContext, mapper: F) -> It
    where F: Fn(&FieldTypeComposition) -> Out, It: FromIterator<Out> {
    field_types.iter().map(mapper).collect()
}

fn fields_composer_iterator_root<CTX, Item, OUT>()
    -> impl Fn((LocalFieldsOwnerContext<CTX>, Box<dyn Fn(&FieldTypeComposition) -> Item>)) -> (CTX, OUT)
    where OUT: FromIterator<Item> {
    |(((aspect, field_types), _generics), composer)|
        (aspect, compose_fields_conversions(field_types, composer))
}


const fn struct_composer_drop_iterator_post_processor<SEP, Out, Map, It>()
    -> impl Fn((FieldTypesContext, Map)) -> OwnedItemsPunctuated<SEP>
    where SEP: Default,
          Map: Fn(&FieldTypeLocalContext) -> Expression,
          It: FromIterator<Out> {
    |(field_types, presenter)| {
        let composer = |field_type: &FieldTypeComposition|
            OwnedItemPresentableContext::Expression(
                presenter(&(quote!(), field_type.conversion_destroy(Expression::FfiRefWithConversion(field_type.clone())))),
                field_type.attrs.clone());
        compose_fields_conversions(field_types, composer)
    }
}

// const fn enum_variant_composer_drop_sequence_iterator_root<SEP, Out, Map, It>()
//     -> impl Fn((FieldTypesContext, Map)) -> OwnedItemsPunctuated<SEP> + 'static
//     where SEP: Default,
//           Map: Fn(&FieldTypeLocalContext) -> Expression,
//           It: FromIterator<Out> {
//     drop_sequence_iterator_root::<SEP, Out, Map, It, _>(
//         |field_type, presenter: &Map|
//             OwnedItemPresentableContext::Expression(
//                 presenter(&(field_type.name.to_token_stream(), field_type.conversion_destroy(Expression::Deref(field_type.name.to_token_stream())))),
//         field_type.attrs.clone()))
// }
// const fn enum_variant_composer_drop_sequence_iterator_root2<SEP, Out, Map, It>()
//     -> impl Fn((FieldTypesContext, Map)) -> OwnedItemsPunctuated<SEP> + 'static
//     where SEP: Default,
//           Map: Fn(&FieldTypeLocalContext) -> Expression,
//           It: FromIterator<Out> {
//     move |(field_types, presenter)| {
//         let composer = |field_type: &FieldTypeComposition|
//             OwnedItemPresentableContext::Expression(
//                 presenter(&(field_type.name.to_token_stream(), field_type.conversion_destroy(Expression::Deref(field_type.name.to_token_stream())))),
//                 field_type.attrs.clone());
//         compose_fields_conversions(field_types, composer)
//     }
// }
//
// const fn drop_sequence_iterator_root<SEP, Out, Map, It, C>(composer: C)
//     -> impl Fn((FieldTypesContext, Map)) -> OwnedItemsPunctuated<SEP> + 'static
//     where SEP: Default,
//           Map: Fn(&FieldTypeLocalContext) -> Expression,
//           C: Fn(&FieldTypeComposition, &Map) -> OwnedItemPresentableContext + 'static,
//           It: FromIterator<Out> {
//     move |(field_types, presenter)|
//         compose_fields_conversions(field_types, |field_type| composer(field_type, &presenter))
// }
//
// const fn drop_sequence_iterator_root<SEP, Out, Map, C>(
//     composer: C,
// ) -> fn((FieldTypesContext, Map)) -> OwnedItemsPunctuated<SEP>
//     where
//         SEP: Default,
//         Map: Fn(&FieldTypeLocalContext) -> Expression,
//         C: Fn(&FieldTypeComposition, &Map) -> OwnedItemPresentableContext + 'static,
// {
//     fn wrapper<SEP, Map>(
//         (field_types, presenter): (FieldTypesContext, Map),
//         composer: &dyn Fn(&FieldTypeComposition, &Map) -> OwnedItemPresentableContext,
//     ) -> OwnedItemsPunctuated<SEP>
//         where
//             SEP: Default,
//             Map: Fn(&FieldTypeLocalContext) -> Expression,
//     {
//         compose_fields_conversions(field_types, |field_type| composer(field_type, &presenter))
//     }
//
//     // We need a closure that captures `composer` and matches the expected signature
//     move |args: (FieldTypesContext, Map)| wrapper(args, &composer)
// }
//
// const fn enum_variant_composer_drop_sequence_iterator_root<SEP, Out, Map>(
// ) -> fn((FieldTypesContext, Map)) -> OwnedItemsPunctuated<SEP>
//     where
//         SEP: Default,
//         Map: Fn(&FieldTypeLocalContext) -> Expression,
// {
//     drop_sequence_iterator_root::<SEP, Out, Map, _>(
//         |field_type, presenter| {
//             OwnedItemPresentableContext::Expression(
//                 presenter(&(field_type.name.to_token_stream(), field_type.conversion_destroy(Expression::Deref(field_type.name.to_token_stream())))),
//                 field_type.attrs.clone(),
//             )
//         },
//     )
// }
