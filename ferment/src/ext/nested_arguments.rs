use syn::punctuated::Punctuated;
use syn::token::Comma;
use crate::composition::NestedArgument;

pub trait NestedArguments {
    fn nested_arguments(&self) -> Punctuated<NestedArgument, Comma>;
}

// impl NestedArguments for Item {
//     fn nested_arguments(&self) -> Punctuated<NestedArgument, Comma> {
//         match self {
//             Item::Enum(item) => item.generics.params.iter().filter_map(|p| match p {
//                 GenericParam::Type(TypeParam { attrs, ident, colon_token, bounds, eq_token, default }) => {}
//                 GenericParam::Const(_) => {}
//                 GenericParam::Lifetime(_) => None
//             }),
//             Item::Fn(item) => Some(&item.attrs),
//             Item::ForeignMod(item) => Some(&item.attrs),
//             Item::Impl(item) => Some(&item.attrs),
//             Item::Macro(item) => Some(&item.attrs),
//             Item::Macro2(item) => Some(&item.attrs),
//             Item::Mod(item) => Some(&item.attrs),
//             Item::Static(item) => Some(&item.attrs),
//             Item::Struct(item) => Some(&item.attrs),
//             Item::Trait(item) => Some(&item.attrs),
//             Item::TraitAlias(item) => Some(&item.attrs),
//             Item::Type(item) => Some(&item.attrs),
//             Item::Union(item) => Some(&item.attrs),
//             Item::Use(item) => Some(&item.attrs),
//             _ => Punctuated::new(),
//         }
//     }
// }