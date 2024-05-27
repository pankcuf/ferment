use crate::composer::CommaPunctuated;
use crate::composition::NestedArgument;

#[allow(unused)]
pub trait NestedArguments {
    fn nested_arguments(&self) -> CommaPunctuated<NestedArgument>;
}

// pub trait FirstNestedType {
//     fn first_nested_type(&self) -> Option<&Type>;
// }
//
// impl FirstNestedType for Type {
//     fn first_nested_type(&self) -> Option<&Type> {
//         match self {
//             Type::Array(type_array) => Some(&type_array.elem),
//             Type::BareFn(_) => {}
//             Type::Group(_) => {}
//             Type::ImplTrait(impl_trait) => impl_trait.bounds.first().map(|ff| match ff {
//                 TypeParamBound::Trait(tr) => tr.path.to
//                 TypeParamBound::Lifetime(_) => {}
//             })
//             Type::Infer(_) => {}
//             Type::Macro(_) => {}
//             Type::Never(_) => {}
//             Type::Paren(_) => {}
//             Type::Path(_) => {}
//             Type::Ptr(_) => {}
//             Type::Reference(_) => {}
//             Type::Slice(_) => {}
//             Type::TraitObject(_) => {}
//             Type::Tuple(_) => {}
//             Type::Verbatim(_) => {}
//             Type::__NonExhaustive => {}
//         }
//     }
// }