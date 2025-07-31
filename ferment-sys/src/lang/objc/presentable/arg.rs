use std::fmt::{Display, Formatter};
use quote::{quote, ToTokens};
use syn::__private::TokenStream2;
use crate::composable::{FieldComposer, FieldTypeKind};
use crate::composer::{SourceComposable, ConversionFromComposer, VariableComposer};
use crate::context::ScopeContext;
use crate::ext::{Mangle, Resolve, ToType};
use crate::lang::objc::ObjCSpecification;
use crate::presentable::{ArgKind, ScopeContextPresentable};
use crate::presentation::FFIVariable;


#[derive(Clone, Debug)]
pub enum ArgPresentation {
    NonatomicReadwrite { ty: TokenStream2, name: TokenStream2 },
    NonatomicAssign { ty: TokenStream2, name: TokenStream2 },
    Initializer { field_name: TokenStream2, field_initializer: TokenStream2 },
    AttrConversion { conversion: TokenStream2 }
}

impl ArgPresentation {
    pub fn nonatomic_readwrite<SPEC>(composer: &FieldComposer<ObjCSpecification>) -> Self {
        let FieldComposer { kind, name, .. } = composer;
        ArgPresentation::NonatomicReadwrite {
            ty: kind.to_token_stream(),
            name: name.to_token_stream()
        }
    }
    pub fn nonatomic_assign<SPEC>(composer: &FieldComposer<ObjCSpecification>) -> Self {
        let FieldComposer { kind, name, .. } = composer;
        ArgPresentation::NonatomicAssign {
            ty: kind.to_token_stream(),
            name: name.to_token_stream()
        }
    }
    pub fn field_initializer<SPEC>(composer: &FieldComposer<ObjCSpecification>) -> Self {
        ArgPresentation::Initializer {
            field_name: composer.tokenized_name(),
            field_initializer: composer.to_token_stream()
        }
    }
    pub fn initializer<SPEC>(composer: &FieldComposer<ObjCSpecification>) -> Self {
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
            }
        }.as_str())
    }
}
impl From<&FieldComposer<ObjCSpecification>> for ArgPresentation {
    fn from(value: &FieldComposer<ObjCSpecification>) -> Self {
        ArgPresentation::NonatomicReadwrite {
            ty: value.ty().to_token_stream(),
            name: value.name.to_token_stream()
        }
    }
}

impl ScopeContextPresentable for ArgKind<ObjCSpecification> {
    type Presentation = ArgPresentation;

    #[allow(unused_variables)]
    fn present(&self, source: &ScopeContext) -> Self::Presentation {
        match self {
            ArgKind::AttrExpression(field_type_context, attrs) => {
                let fields = field_type_context.present(source);
                //println!("OBJC ArgKind::AttrExpression: {}", fields);
                ArgPresentation::AttrConversion { conversion: fields }
            },
            ArgKind::AttrName(name, attrs) => {
                //println!("OBJC ArgKind::AttrName: {}", name);
                ArgPresentation::AttrConversion { conversion: name.to_token_stream() }
            },
            ArgKind::AttrSequence(seq, attrs) => {
                let conversion = seq.present(source);
                //println!("OBJC ArgKind::AttrSequence: {}", conversion);
                ArgPresentation::AttrConversion { conversion }
            },
            ArgKind::Unnamed(FieldComposer{ kind, name, attrs, .. }) => {
                //println!("OBJC ArgKind::DefaultFieldType: {} -- {}", kind, name);
                let var = Resolve::<FFIVariable<ObjCSpecification, TokenStream2>>::resolve(&kind.to_type(), source);
                ArgPresentation::AttrConversion {
                    conversion: quote! { #var #name }
                }
            },
            ArgKind::BindingFieldName(FieldComposer { name, named, attrs, .. }) => {
                //println!("OBJC ArgKind::BindingFieldName: {}", name);
                ArgPresentation::AttrConversion { conversion: quote!() }
            },
            ArgKind::DefaultFieldConversion(FieldComposer { name, attrs, kind, .. }) => {
                //println!("OBJC ArgKind::DefaultFieldConversion: {} {}", name, kind);
                let ty = kind.to_type();
                let composer = ConversionFromComposer::<ObjCSpecification>::key_in_scope(name.clone(), &ty, &source.scope);
                let from_conversion_expr = composer.compose(source);
                let from_conversion_presentation = from_conversion_expr.present(source);
                ArgPresentation::AttrConversion { conversion: quote!() }
            },
            ArgKind::BindingArg(FieldComposer { name, kind, named, attrs, .. }) => {
                //println!("OBJC ArgKind::BindingArg: {} {}", name, kind);
                let (ident, ty) = match kind {
                    FieldTypeKind::Type(field_type) => (
                        Some((*named).then(|| name.mangle_ident_default()).unwrap_or_else(|| name.anonymous())),
                        field_type.resolve(source)
                    ),
                    FieldTypeKind::Var(var) => (
                        Some((*named).then(|| name.mangle_ident_default()).unwrap_or_else(|| name.anonymous())),
                            var.clone()
                        ),

                    FieldTypeKind::Conversion(conversion) => (
                        Some(name.mangle_ident_default()), FFIVariable::direct(conversion.clone())),
                };
                ArgPresentation::AttrConversion { conversion: quote!() }
            },
            ArgKind::Named(FieldComposer { attrs, name, kind, ..}, visibility) => {
                //println!("OBJC ArgKind::Named: {} {}", name, kind);
                let ty = VariableComposer::<ObjCSpecification>::new(kind.to_type())
                    .compose(source)
                    .to_token_stream();
                ArgPresentation::AttrConversion {
                    conversion: quote! {
                        #ty
                    }
                }
            },
            ArgKind::NamedReady(FieldComposer { attrs, name, kind, ..}, visibility) => {
                let ty = kind.to_type();
                ArgPresentation::AttrConversion {
                    conversion: quote! {
                        #ty
                    }
                }
            }

            ArgKind::AttrExhaustive(attrs) => {
                ArgPresentation::AttrConversion { conversion: quote!() }
            },
            ArgKind::CallbackArg(composer) => {
                //println!("OBJC ArgKind::CallbackArg: {} {}", composer.name, composer.kind);
                ArgPresentation::AttrConversion { conversion: quote!() }
            }
            ArgKind::AttrExpressionComposer(
                field_composer,
                field_path_resolver,
                expr_composer
            ) => {
                //println!("OBJC ArgKind::AttrExpressionComposer: {} {}", field_composer.name.to_token_stream(), field_composer.kind.to_token_stream());
                let template = field_path_resolver(field_composer);
                let expr = expr_composer(&template);
                let conversion = expr.present(source);
                ArgPresentation::AttrConversion { conversion }

            }
            ArgKind::DefaultFieldByValueConversion(_, _) => panic!("Not supported"),
        }
    }
}