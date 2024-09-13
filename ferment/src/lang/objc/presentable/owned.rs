use quote::ToTokens;
use syn::{Expr, Type};
use crate::composable::{FieldComposer, FieldTypeKind};
use crate::composer::{Composer, FromConversionFullComposer, VariableComposer};
use crate::context::{ScopeContext, ScopeSearch, ScopeSearchKey};
use crate::ext::{Mangle, Resolve, ToType};
use crate::lang::objc::composers::AttrWrapper;
use crate::lang::objc::ObjCFermentate;
use crate::lang::objc::presentation::ArgPresentation;
use crate::presentable::{OwnedItemPresentableContext, ScopeContextPresentable};
use crate::presentation::FFIVariable;

#[allow(unused)]
fn expr(expr: Expr, attrs: &AttrWrapper) -> ArgPresentation {
    ArgPresentation {
        attr: attrs.clone(),
        objc_ty: Default::default(),
        c_ty: Default::default(),
        name: Default::default(),
    }
}

impl ScopeContextPresentable for OwnedItemPresentableContext<ObjCFermentate, AttrWrapper> {
    type Presentation = ArgPresentation;

    #[allow(unused_variables)]
    fn present(&self, source: &ScopeContext) -> Self::Presentation {
        match self {
            // OwnedItemPresentableContext::PatLitExpr(expr, attrs) => {
            //     // println!("OwnedItemPresentableContext::PatLitExpr({})", expr.to_token_stream());
            //     // attrs.wrap()
            //
            //     ArgPresentation {
            //         attr: attrs.clone(),
            //         objc_ty: Default::default(),
            //         c_ty: Default::default(),
            //         name: Default::default(),
            //     }
            //     // ArgPresentation::Pat(Pat::Lit(PatLit { attrs: attrs.attrs.clone(), expr: Box::new(expr.clone()) }))
            // },
            OwnedItemPresentableContext::Expression(field_type_context, attrs) => {
                // println!("OwnedItemPresentableContext::Expression({})", field_type_context);

                ArgPresentation {
                    attr: attrs.clone(),
                    objc_ty: Default::default(),
                    c_ty: Default::default(),
                    name: Default::default(),
                }

                // Self::PatLitExpr(Expr::Verbatim(field_type_context.present(source)), attrs.clone())
                //     .present(source)
            },

            OwnedItemPresentableContext::SequenceOutput(seq, attrs) => {
                // Self::PatLitExpr(Expr::Verbatim(seq.present(source)), attrs.clone())
                //     .present(source)
                ArgPresentation {
                    attr: attrs.clone(),
                    objc_ty: Default::default(),
                    c_ty: Default::default(),
                    name: Default::default(),
                }
            },
            OwnedItemPresentableContext::DefaultFieldType(field_type, attrs) => {
                // println!("OwnedItemPresentableContext::DefaultFieldType({})", field_type.to_token_stream());

                // Self::PatLitExpr(Expr::Verbatim(<Type as Resolve<FFIVariable>>::resolve(field_type, source).to_token_stream()), attrs.clone())
                //     .present(source)
                ArgPresentation {
                    attr: attrs.clone(),
                    objc_ty: Default::default(),
                    c_ty: Default::default(),
                    name: Default::default(),
                }
            },
            OwnedItemPresentableContext::BindingFieldName(FieldComposer { name, named, attrs, .. }) => {
                // println!("OwnedItemPresentableContext::BindingFieldName({})", name.to_token_stream());
                // Self::PatLitExpr(Expr::Verbatim(named.then(|| name.to_token_stream()).unwrap_or(name.anonymous().to_token_stream())), attrs.clone())
                //     .present(source)
                ArgPresentation {
                    attr: attrs.clone(),
                    objc_ty: Default::default(),
                    c_ty: Default::default(),
                    name: Default::default(),
                }
            },
            OwnedItemPresentableContext::DefaultFieldConversion(FieldComposer { name, attrs, kind, .. }) => {
                let ty = kind.ty();
                // FromConversionFullComposer::new(composer.name.clone(), )
                // FromConversionComposer::new(composer.name.clone(), composer.ty().clone(), None))

                let composer = FromConversionFullComposer::<ObjCFermentate, AttrWrapper>::new(name.clone(), ScopeSearch::KeyInScope(ScopeSearchKey::maybe_from_ref(ty).unwrap(), &source.scope), None);
                // println!("OwnedItemPresentableContext::DefaultFieldConversion.1: {} ({}), {}", name.to_token_stream(), name, composer);
                let from_conversion_expr = composer.compose(source);
                // println!("OwnedItemPresentableContext::DefaultFieldConversion.2: {} ({}), {}", name.to_token_stream(), name, from_conversion_expr);
                let from_conversion_presentation = from_conversion_expr.present(source);
                // println!("OwnedItemPresentableContext::DefaultFieldConversion.3: {} ({}), {}", name.to_token_stream(), name, from_conversion_presentation);
                ArgPresentation {
                    attr: attrs.clone(),
                    objc_ty: Default::default(),
                    c_ty: Default::default(),
                    name: Default::default(),
                }
                // ArgPresentation::Field(Field {
                //     attrs: attrs.attrs.clone(),
                //     vis: Visibility::Inherited,
                //     ident: Some(name.mangle_ident_default()),
                //     colon_token: Some(Default::default()),
                //     ty: Type::Verbatim(from_conversion_presentation),
                // })
            },
            OwnedItemPresentableContext::BindingArg(FieldComposer { name, kind, named, attrs, .. }) => {
                // println!("OwnedItemPresentableContext::BindingArg: {} ({}), {}", name.to_token_stream(), name, kind.ty().to_token_stream());
                let (ident, ty) = match kind {
                    FieldTypeKind::Type(field_type) => (
                        Some((*named).then(|| name.mangle_ident_default()).unwrap_or(name.anonymous())),
                        <Type as Resolve<FFIVariable>>::resolve(field_type, source).to_type()
                    ),
                    FieldTypeKind::Conversion(conversion) => (
                        Some(name.mangle_ident_default()), Type::Verbatim(conversion.clone()))
                };
                ArgPresentation {
                    attr: attrs.clone(),
                    objc_ty: Default::default(),
                    c_ty: Default::default(),
                    name: Default::default(),
                }
                // ArgPresentation::Field(Field {
                //     attrs: attrs.attrs.clone(),
                //     vis: Visibility::Inherited,
                //     ident,
                //     colon_token: Default::default(),
                //     ty
                // })
            },
            OwnedItemPresentableContext::Named(FieldComposer { attrs, name, kind, ..}, visibility) => {
                println!("OwnedItemPresentableContext::Named: {}", kind.ty().to_token_stream());

                // let ty = VarComposer::new(ScopeSearch::KeyInScope(ScopeSearchKey::maybe_from_ref(kind.ty()).unwrap(), &source.scope)).compose(source).to_type();
                let ty = VariableComposer::from(kind.ty())
                    .compose(source)
                    .to_type();
                // println!("OwnedItemPresentableContext::Named::RESULT: {}", ty.to_token_stream());
                // ArgPresentation::Field(Field { attrs: attrs.attrs.clone(), vis: visibility.clone(), ident: Some(name.mangle_ident_default()), colon_token: Some(Default::default()), ty })
                ArgPresentation {
                    attr: attrs.clone(),
                    objc_ty: Default::default(),
                    c_ty: Default::default(),
                    name: Default::default(),
                }
            },
            OwnedItemPresentableContext::Lambda(name, value, attrs) => {
                // println!("OwnedItemPresentableContext::Lambda({}, {})", name, value);
                // ArgPresentation::Arm(Arm {
                //     attrs: attrs.attrs.clone(),
                //     pat: Pat::Verbatim(name.clone()),
                //     guard: None,
                //     fat_arrow_token: Default::default(),
                //     body: Box::new(Expr::Verbatim(value.clone())),
                //     comma: None,
                // })
                ArgPresentation {
                    attr: attrs.clone(),
                    objc_ty: Default::default(),
                    c_ty: Default::default(),
                    name: Default::default(),
                }
            },
            OwnedItemPresentableContext::Exhaustive(attrs) => {
                // println!("OwnedItemPresentableContext::Exhaustive({})", quote!(#(#attrs)*));
                // ArgPresentation::Arm(Arm {
                //     attrs: attrs.attrs.clone(),
                //     pat: Pat::Wild(PatWild { attrs: vec![], underscore_token: Default::default() }),
                //     guard: None,
                //     fat_arrow_token: Default::default(),
                //     body: Box::new(Expr::Verbatim(quote!(unreachable!("This is unreachable")))),
                //     comma: None,
                // })
                ArgPresentation {
                    attr: attrs.clone(),
                    objc_ty: Default::default(),
                    c_ty: Default::default(),
                    name: Default::default(),
                }
            },
            OwnedItemPresentableContext::CallbackArg(composer) => {
                // ArgPresentation::Field(Field {
                //     attrs: attrs.attrs.clone(),
                //     vis: Visibility::Inherited,
                //     ident: Some(name.mangle_ident_default()),
                //     colon_token: Default::default(),
                //     ty: bare.clone()
                // })
                ArgPresentation {
                    attr: composer.attrs.clone(),
                    objc_ty: Default::default(),
                    c_ty: Default::default(),
                    name: Default::default(),
                }
            }
        }
    }
}