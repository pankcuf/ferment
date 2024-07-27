// use quote::quote;
// use crate::composable::FieldComposer;
// use crate::composer::{FieldTypeLocalContext, FieldTypesContext, LocalConversionContext, LocalFieldsOwnerContext, OwnedItemsPunctuated, OwnerAspectWithItems};
// use crate::ext::ConversionTrait;
// use crate::presentable::{Expression, OwnedItemPresentableContext};
// use crate::presentation::Name;
//
// pub type FieldTypeIterator<T, RES> = dyn Fn((T, dyn Fn(&FieldTypeLocalContext) -> Expression)) -> RES + 'static;
// pub type FieldTypeIterator2<T, RES> = dyn Fn((T, dyn Fn(&FieldComposer) -> OwnedItemPresentableContext)) -> RES + 'static;
// pub type OwnedIteratorPostProcessor<SEP> = FieldTypeIterator<LocalConversionContext, OwnerAspectWithItems<SEP>>;
// pub type OwnedIteratorPostProcessor2<SEP> = FieldTypeIterator2<LocalConversionContext, OwnerAspectWithItems<SEP>>;
//
// fn compose_fields_conversions<F, Out, It>(field_types: FieldTypesContext, mapper: F) -> It
//     where F: Fn(&FieldComposer) -> Out, It: FromIterator<Out> {
//     field_types.iter().map(mapper).collect()
// }
//
// fn fields_composer_iterator_root<CTX, Item, OUT>()
//     -> impl Fn((LocalFieldsOwnerContext<CTX>, Box<dyn Fn(&FieldComposer) -> Item>)) -> (CTX, OUT)
//     where OUT: FromIterator<Item> {
//     |(((aspect, field_types), _generics), composer)|
//         (aspect, compose_fields_conversions(field_types, composer))
// }
//
//
// const fn struct_composer_drop_iterator_post_processor<SEP, Out, Map, It>()
//     -> impl Fn((FieldTypesContext, Map)) -> OwnedItemsPunctuated<SEP>
//     where SEP: Default,
//           Map: Fn(&FieldTypeLocalContext) -> Expression,
//           It: FromIterator<Out> {
//     |(field_types, presenter)| {
//         let composer = |composer: &FieldComposer|
//             OwnedItemPresentableContext::Expression(
//                 presenter(&(Name::Empty, composer.conversion_destroy(Expression::FfiRefWithName(composer.name.clone())))),
//                 composer.to_attrs());
//         compose_fields_conversions(field_types, composer)
//     }
// }
