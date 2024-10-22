use std::fmt::{Display, Formatter};
use quote::{quote, ToTokens};
use syn::Type;
use syn::__private::TokenStream2;
use crate::composable::{FieldComposer, FieldTypeKind};
use crate::composer::{SourceComposable, FromConversionFullComposer, VariableComposer};
use crate::context::ScopeContext;
use crate::ext::{Mangle, Resolve, ToType};
use crate::lang::objc::{ObjCFermentate, ObjCSpecification};
use crate::presentable::{Aspect, PresentableArgument, ScopeContextPresentable};
use crate::presentation::FFIVariable;


#[derive(Clone, Debug)]
pub enum ArgPresentation {
    NonatomicReadwrite { ty: TokenStream2, name: TokenStream2 },
    NonatomicAssign { ty: TokenStream2, name: TokenStream2 },
    Initializer { field_name: TokenStream2, field_initializer: TokenStream2 },
    AttrConversion { conversion: TokenStream2 }
}

impl ArgPresentation {
    pub fn nonatomic_readwrite<SPEC>(composer: &FieldComposer<ObjCFermentate, SPEC>) -> Self
        where SPEC: ObjCSpecification {
        let FieldComposer { kind, name, .. } = composer;
        ArgPresentation::NonatomicReadwrite {
            ty: kind.to_token_stream(),
            name: name.to_token_stream()
        }
    }
    pub fn nonatomic_assign<SPEC>(composer: &FieldComposer<ObjCFermentate, SPEC>) -> Self
        where SPEC: ObjCSpecification {
        let FieldComposer { kind, name, .. } = composer;
        ArgPresentation::NonatomicAssign {
            ty: kind.to_token_stream(),
            name: name.to_token_stream()
        }
    }
    pub fn field_initializer<SPEC>(composer: &FieldComposer<ObjCFermentate, SPEC>) -> Self
        where SPEC: ObjCSpecification {
        ArgPresentation::Initializer {
            field_name: composer.tokenized_name(),
            field_initializer: composer.to_token_stream()
        }
    }
    pub fn initializer<SPEC>(composer: &FieldComposer<ObjCFermentate, SPEC>) -> Self
        where SPEC: ObjCSpecification {
        ArgPresentation::Initializer {
            field_name: composer.tokenized_name(),
            field_initializer: composer.to_token_stream()
        }
    }
}

impl ToTokens for ArgPresentation {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        match self {
            ArgPresentation::NonatomicReadwrite { ty, name } => {
                quote! {
                    @property (nonatomic, readwrite) #ty #name
                }
            }
            ArgPresentation::NonatomicAssign { ty, name } => {
                quote! {
                    @property (nonatomic, assign) #ty #name
                }
            }
            ArgPresentation::Initializer { field_name, field_initializer } => {
                quote! {
                    obj.#field_name = #field_initializer
                }
            }
            ArgPresentation::AttrConversion { conversion } => quote! {
                #conversion
            }
        }.to_tokens(tokens)
    }
}

impl Display for ArgPresentation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            ArgPresentation::NonatomicReadwrite { ty, name } =>
                format!("@property (nonatomic, readwrite) {} {}", ty.to_string(), name.to_string()),
            ArgPresentation::NonatomicAssign { ty, name } =>
                format!("@property (nonatomic, assign) {} {}", ty.to_string(), name.to_string()),
            ArgPresentation::Initializer { field_name, field_initializer } =>
                format!("obj.{} = {}", field_name.to_string(), field_initializer.to_string()),
            ArgPresentation::AttrConversion { conversion } => {
                format!("{}", conversion.to_string())
                // ArgPresentation::AttrConversion { conversion: fields }
            }
        }.as_str())
    }
}
impl<SPEC> From<&FieldComposer<ObjCFermentate, SPEC>> for ArgPresentation
    where SPEC: ObjCSpecification,
          Aspect<SPEC::TYC>: ScopeContextPresentable {
    fn from(value: &FieldComposer<ObjCFermentate, SPEC>) -> Self {
        ArgPresentation::NonatomicReadwrite {
            ty: value.ty().to_token_stream(),
            name: value.name.to_token_stream()
        }
    }
}
// #[derive(Clone, Debug)]
// pub struct ArgPresentation {
//     pub attr: AttrWrapper,
//     pub objc_ty: TokenStream2,
//     pub c_ty: TokenStream2,
//     pub name: TokenStream2,
// }

// impl ToTokens for ArgPresentation {
//     fn to_tokens(&self, _tokens: &mut TokenStream2) {
//         // let Self { attr, objc_ty, c_ty, name } = self;
//         // quote! {
//         //
//         // }
//     }
// }
// #[allow(unused)]
// fn expr(expr: Expr, attrs: &AttrWrapper) -> ArgPresentation {
//     ArgPresentation {
//         attr: attrs.clone(),
//         objc_ty: Default::default(),
//         c_ty: Default::default(),
//         name: Default::default(),
//     }
// }

impl<SPEC> ScopeContextPresentable for PresentableArgument<ObjCFermentate, SPEC>
    where SPEC: ObjCSpecification {
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
            PresentableArgument::AttrExpression(field_type_context, attrs) => {
                let fields = field_type_context.present(source);
                println!("OBJC PresentableArgument::AttrExpression: {}", fields);


                // let ty = ArgPresentation::Initializer {
                //     field_name: composer.tokenized_name(),
                //     field_initializer: composer.to_token_stream()
                // };

                ArgPresentation::AttrConversion { conversion: fields }

                // Self::PatLitExpr(Expr::Verbatim(field_type_context.present(source)), attrs.clone())
                //     .present(source)
            },
            PresentableArgument::AttrName(name, attrs) => {
                println!("OBJC PresentableArgument::AttrName: {}", name);
                ArgPresentation::AttrConversion { conversion: name.to_token_stream() }

                // ArgPresentation {
                //     attr: attrs.clone(),
                //     objc_ty: Default::default(),
                //     c_ty: Default::default(),
                //     name: Default::default(),
                // }
            },
            PresentableArgument::AttrSequence(seq, attrs) => {
                println!("OBJC PresentableArgument::AttrSequence: {}", seq.present(source));
                // Self::PatLitExpr(Expr::Verbatim(seq.present(source)), attrs.clone())
                //     .present(source)
                ArgPresentation::AttrConversion {
                    conversion: seq.present(source),
                }
                // ArgPresentation {
                //     attr: attrs.clone(),
                //     objc_ty: Default::default(),
                //     c_ty: Default::default(),
                //     name: Default::default(),
                // }
            },
            PresentableArgument::DefaultFieldType(FieldComposer{ kind, name, attrs, .. }) => {
                // println!("OwnedItemPresentableContext::DefaultFieldType({})", field_type.to_token_stream());
                println!("OBJC PresentableArgument::DefaultFieldType: {} -- {}", kind, name);
                let var = <Type as Resolve<FFIVariable<TokenStream2, ObjCFermentate, SPEC>>>::resolve(kind.ty(), source);
                ArgPresentation::AttrConversion {
                    conversion: quote! { #var #name }
                }

                // Self::PatLitExpr(Expr::Verbatim(<Type as Resolve<FFIVariable>>::resolve(field_type, source).to_token_stream()), attrs.clone())
                //     .present(source)
                // ArgPresentation {
                //     attr: attrs.clone(),
                //     objc_ty: Default::default(),
                //     c_ty: Default::default(),
                //     name: Default::default(),
                // }
            },
            PresentableArgument::BindingFieldName(FieldComposer { name, named, attrs, .. }) => {
                println!("OBJC PresentableArgument::BindingFieldName: {}", name);
                // println!("OwnedItemPresentableContext::BindingFieldName({})", name.to_token_stream());
                // Self::PatLitExpr(Expr::Verbatim(named.then(|| name.to_token_stream()).unwrap_or(name.anonymous().to_token_stream())), attrs.clone())
                //     .present(source)
                ArgPresentation::AttrConversion { conversion: quote!() }
                // ArgPresentation {
                //     attr: attrs.clone(),
                //     objc_ty: Default::default(),
                //     c_ty: Default::default(),
                //     name: Default::default(),
                // }
            },
            PresentableArgument::DefaultFieldConversion(FieldComposer { name, attrs, kind, .. }) => {
                println!("OBJC PresentableArgument::DefaultFieldConversion: {} {}", name, kind);
                let ty = kind.ty();
                // FromConversionFullComposer::new(composer.name.clone(), )
                // FromConversionComposer::new(composer.name.clone(), composer.ty().clone(), None))

                let composer = FromConversionFullComposer::<ObjCFermentate, SPEC>::key_in_scope(name.clone(), ty, &source.scope);
                // println!("OwnedItemPresentableContext::DefaultFieldConversion.1: {} ({}), {}", name.to_token_stream(), name, composer);
                let from_conversion_expr = composer.compose(source);
                // println!("OwnedItemPresentableContext::DefaultFieldConversion.2: {} ({}), {}", name.to_token_stream(), name, from_conversion_expr);
                let from_conversion_presentation = from_conversion_expr.present(source);
                // println!("OwnedItemPresentableContext::DefaultFieldConversion.3: {} ({}), {}", name.to_token_stream(), name, from_conversion_presentation);
                ArgPresentation::AttrConversion { conversion: quote!() }
                // ArgPresentation {
                //     attr: attrs.clone(),
                //     objc_ty: Default::default(),
                //     c_ty: Default::default(),
                //     name: Default::default(),
                // }
                // ArgPresentation::Field(Field {
                //     attrs: attrs.attrs.clone(),
                //     vis: Visibility::Inherited,
                //     ident: Some(name.mangle_ident_default()),
                //     colon_token: Some(Default::default()),
                //     ty: Type::Verbatim(from_conversion_presentation),
                // })
            },
            PresentableArgument::BindingArg(FieldComposer { name, kind, named, attrs, .. }) => {
                // println!("OwnedItemPresentableContext::BindingArg: {} ({}), {}", name.to_token_stream(), name, kind.ty().to_token_stream());
                println!("OBJC PresentableArgument::BindingArg: {} {}", name, kind);
                let (ident, ty) = match kind {
                    FieldTypeKind::Type(field_type) => (
                        Some((*named).then(|| name.mangle_ident_default()).unwrap_or(name.anonymous())),
                        <Type as Resolve<SPEC::Var>>::resolve(field_type, source)
                    ),
                    FieldTypeKind::Var(var) => (
                        Some((*named).then(|| name.mangle_ident_default()).unwrap_or(name.anonymous())),
                            var.clone()
                        ),

                    FieldTypeKind::Conversion(conversion) => (
                        Some(name.mangle_ident_default()), FFIVariable::direct(conversion.clone())),
                };
                ArgPresentation::AttrConversion { conversion: quote!() }
                // ArgPresentation {
                //     attr: attrs.clone(),
                //     objc_ty: Default::default(),
                //     c_ty: Default::default(),
                //     name: Default::default(),
                // }
                // ArgPresentation::Field(Field {
                //     attrs: attrs.attrs.clone(),
                //     vis: Visibility::Inherited,
                //     ident,
                //     colon_token: Default::default(),
                //     ty
                // })
            },
            PresentableArgument::Named(FieldComposer { attrs, name, kind, ..}, visibility) => {
                // println!("OBJC PresentableArgument::Named: {} {}", name, kind);

                // let ty = VarComposer::new(ScopeSearch::KeyInScope(ScopeSearchKey::maybe_from_ref(kind.ty()).unwrap(), &source.scope)).compose(source).to_type();
                let ty = VariableComposer::<ObjCFermentate, SPEC>::new(kind.to_type())
                    .compose(source)
                    .to_token_stream();
                // println!("OwnedItemPresentableContext::Named::RESULT: {}", ty.to_token_stream());
                // ArgPresentation::Field(Field { attrs: attrs.attrs.clone(), vis: visibility.clone(), ident: Some(name.mangle_ident_default()), colon_token: Some(Default::default()), ty })
                ArgPresentation::AttrConversion { conversion: quote!() }
                // ArgPresentation {
                //     attr: attrs.clone(),
                //     objc_ty: Default::default(),
                //     c_ty: Default::default(),
                //     name: Default::default(),
                // }
            },
            // PresentableArgument::Lambda(name, value, attrs) => {
            //     // println!("OwnedItemPresentableContext::Lambda({}, {})", name, value);
            //     // ArgPresentation::Arm(Arm {
            //     //     attrs: attrs.attrs.clone(),
            //     //     pat: Pat::Verbatim(name.clone()),
            //     //     guard: None,
            //     //     fat_arrow_token: Default::default(),
            //     //     body: Box::new(Expr::Verbatim(value.clone())),
            //     //     comma: None,
            //     // })
            //     ArgPresentation {
            //         attr: attrs.clone(),
            //         objc_ty: Default::default(),
            //         c_ty: Default::default(),
            //         name: Default::default(),
            //     }
            // },
            PresentableArgument::AttrExhaustive(attrs) => {
                // println!("OwnedItemPresentableContext::Exhaustive({})", quote!(#(#attrs)*));
                // ArgPresentation::Arm(Arm {
                //     attrs: attrs.attrs.clone(),
                //     pat: Pat::Wild(PatWild { attrs: vec![], underscore_token: Default::default() }),
                //     guard: None,
                //     fat_arrow_token: Default::default(),
                //     body: Box::new(Expr::Verbatim(quote!(unreachable!("This is unreachable")))),
                //     comma: None,
                // })
                ArgPresentation::AttrConversion { conversion: quote!() }
                // ArgPresentation {
                //     attr: attrs.clone(),
                //     objc_ty: Default::default(),
                //     c_ty: Default::default(),
                //     name: Default::default(),
                // }
            },
            PresentableArgument::CallbackArg(composer) => {
                println!("OBJC PresentableArgument::CallbackArg: {} {}", composer.name, composer.kind);
                // ArgPresentation::Field(Field {
                //     attrs: attrs.attrs.clone(),
                //     vis: Visibility::Inherited,
                //     ident: Some(name.mangle_ident_default()),
                //     colon_token: Default::default(),
                //     ty: bare.clone()
                // })
                ArgPresentation::AttrConversion { conversion: quote!() }
            }
            PresentableArgument::AttrExpressionComposer(
                field_composer,
                field_path_resolver,
                expr_composer
            ) => {
                // println!("OBJC PresentableArgument::AttrExpressionComposer: {} {}", field_composer.name.to_token_stream(), field_composer.kind.to_token_stream());
                let template = field_path_resolver(field_composer);

                let expr = expr_composer(&template);
                let conversion = expr.present(source);
                // println!("OBJC PresentableArgument::AttrExpressionComposer => {}", conversion);
                ArgPresentation::AttrConversion { conversion }

            }
        }
    }
}