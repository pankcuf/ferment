use quote::ToTokens;
use syn::Type;
use crate::composable::FieldComposer;
use crate::composer::Composer;
use crate::context::ScopeContext;
use crate::ext::ConversionTrait;
use crate::presentable::Expression;
use crate::presentation::Name;

// pub type ExprSource<'a> = (&'a Expression, &'a ScopeContext);

#[derive(Clone, Debug)]
pub struct DestroyConversionComposer {
    pub name: Name,
    pub ty: Type,
    pub expr: Option<Expression>,
}
impl From<&FieldComposer> for DestroyConversionComposer {
    fn from(value: &FieldComposer) -> Self {
        Self { name: value.name.clone(), ty: value.ty().clone(), expr: None }
    }
}
impl DestroyConversionComposer {
    pub fn new(name: Name, ty: Type, expr: Option<Expression>) -> Self {
        Self { name, ty, expr }
    }
}

// impl<'a> Composer<'a> for TypePath {
//     type Source = ExprSource<'a>;
//     type Result = Expression;
//
//     fn compose(&self, source: &'a Self::Source) -> Self::Result {
//         let (expr, source) = source;
//         let last_segment = self.path.segments.last().unwrap();
//         let last_ident = &last_segment.ident;
//         // let is_string = last_ident.is_string();
//         // let is_str = last_ident.is_str();
//         if last_ident.is_primitive() {
//             Expression::Empty
//         } else if last_ident.is_string() {
//             Expression::DestroyString(expr.into(), self.path.to_token_stream())
//         } else if last_ident.is_str() {
//             Expression::DestroyString(expr.into(), quote!(&#self))
//         } else if last_ident.is_optional() {
//             match path_arguments_to_type_conversions(&last_segment.arguments).first() {
//                 Some(TypeConversion::Primitive(_)) =>
//                     Expression::DestroyOptPrimitive(expr.into()),
//                 _ =>
//                     Expression::DestroyOpt(expr.into()),
//             }
//         } else {
//             Expression::UnboxAnyTerminated(expr.into())
//         }
//     }
// }
//
//
impl<'a> Composer<'a> for DestroyConversionComposer {
    type Source = ScopeContext;
    type Result = Expression;

    fn compose(&self, _source: &'a Self::Source) -> Self::Result {
        let Self { name, ty, expr } = self;

        println!("DestroyConversionComposer:: {} -- {}", name, ty.to_token_stream());
        ty.conversion_destroy(expr.clone().unwrap_or(Expression::Name(name.clone())))
        // match ty {
        //     Type::Path(type_path) =>
        //         type_path.compose(source),
        //     Type::Ptr(TypePtr { elem, .. }) => match &*elem {
        //         Type::Ptr(TypePtr { elem, .. }) => match &*elem {
        //             Type::Path(type_path) =>
        //                 type_path.compose(source),
        //             _ => panic!("Can't destroy_ptr: of type: {}", quote!(#self)),
        //         },
        //         Type::Path(type_path) =>
        //             type_path.compose(source),
        //         _ => panic!("Can't destroy_ptr: of type: {}", quote!(#self)),
        //     },
        //     Type::Reference(TypeReference { elem, ..}) => match &*elem {
        //         Type::Path(type_path) =>
        //             type_path.compose(source),
        //         _ => Expression::UnboxAny(expr.into()),
        //     },
        //     Type::Array(..) |
        //     Type::Slice(..) |
        //     Type::TraitObject(..) |
        //     Type::Tuple(..) |
        //     Type::ImplTrait(..) =>
        //         Expression::UnboxAny(expr.into()),
        //     _ => unimplemented!("No conversions for {}", self.to_token_stream())
        // }
        //
    }
}