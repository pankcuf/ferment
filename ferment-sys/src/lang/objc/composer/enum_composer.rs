use quote::{format_ident, quote, ToTokens};
use syn::PathSegment;
use crate::ast::{CommaPunctuated, Depunctuated, SemiPunctuated};
use crate::composer::{AspectPresentable, AttrComposable, EnumComposer, FFIAspect, GenericsComposable, InterfaceComposable, NameKindComposable, SourceAccessible, SourceFermentable, TypeAspect};
use crate::ext::{Mangle, ToPath};
use crate::lang::objc::{ObjCFermentate, ObjCSpecification};
use crate::lang::objc::fermentate::InterfaceImplementation;
use crate::lang::objc::formatter::format_interface_implementations;
use crate::lang::objc::presentable::{ArgPresentation, TypeContext};
use crate::lang::Specification;
use crate::presentable::{ArgKind, ScopeContextPresentable};
use crate::presentation::{DictionaryExpr, DictionaryName, Name};
fn to_snake_case(input: &str) -> String {
    let mut snake_case = String::new();
    for (i, ch) in input.chars().enumerate() {
        if ch.is_uppercase() {
            if i != 0 {
                snake_case.push('_');
            }
            snake_case.push(ch.to_ascii_lowercase());
        } else {
            snake_case.push(ch);
        }
    }
    snake_case
}

impl InterfaceComposable<<ObjCSpecification as Specification>::Interface> for EnumComposer<ObjCSpecification>
where Self: SourceAccessible
      + NameKindComposable
      + TypeAspect<TypeContext>
      + AttrComposable<<ObjCSpecification as Specification>::Attr>
      + GenericsComposable<<ObjCSpecification as Specification>::Gen> {
    fn compose_interfaces(&self) -> Depunctuated<<ObjCSpecification as Specification>::Interface> {
        let source = self.source_ref();
        let target_type = self.present_target_aspect();
        let ffi_type = self.present_ffi_aspect();
        let objc_name = target_type.to_token_stream();
        let c_name = ffi_type.to_token_stream();

        println!("OBJC:: ENUM FFI ASPECT TYPE: {}", ffi_type.to_token_stream());
        println!("OBJC:: ENUM TARGET ASPECT TYPE: {}", objc_name);

        let property_names = CommaPunctuated::new();
        let mut properties = SemiPunctuated::new();
        let tag_name = Name::<ObjCSpecification>::EnumTag(ffi_type.mangle_ident_default());
        properties.push(ArgPresentation::NonatomicAssign {
            ty: quote!(enum #tag_name),
            name: DictionaryName::Tag.to_token_stream()
        });

        let body_conversions = Depunctuated::<InterfaceImplementation>::new();

        let mut from_conversions = Depunctuated::new();
        let mut to_conversions = Depunctuated::new();
        let mut destroy_conversions = Depunctuated::new();

        self.variant_composers.iter()
            .for_each(|variant_composer| {
                let attrs = variant_composer.compose_attributes();
                //SeqKind::variant_from
                let from = variant_composer.compose_aspect(FFIAspect::From);
                //SeqKind::variant_to
                let to = variant_composer.compose_aspect(FFIAspect::To);
                //SeqKind::variant_drop
                let destroy = variant_composer.compose_aspect(FFIAspect::Drop);

                let from = ArgKind::AttrSequence(from, attrs.clone());
                let to = ArgKind::AttrSequence(to, attrs.clone());
                let destroy = ArgKind::AttrSequence(destroy, attrs.clone());

                println!("VARIANT: FROM: {}", from.present(&source));
                println!("VARIANT: TO: {}", to.present(&source));
                println!("VARIANT: DESTROY: {}", destroy.present(&source));
                from_conversions.push(from);
                to_conversions.push(to);
                destroy_conversions.push(destroy);
            });

        self.variant_presenters.iter()
            .for_each(|(_c, ((aspect, (_attrs, _lifetimes, _generics), _is_round), args))| {

                args.iter().for_each(|arg| {
                    let asp = aspect.present(&source);
                    let path = asp.to_path();
                    if let Some(PathSegment { ident: last_ident, .. }) = &path.segments.last() {
                        let snake_case = to_snake_case(last_ident.to_string().as_str());
                        let presentation = arg.present(&source);
                        // OBJC ENUM VAR ARG: example_simple_errors_context_ContextProviderError :: InvalidDataContract --> NSString *
                        // -> invalid_data_contract
                        println!("OBJC ENUM VAR ARG: {} --> {last_ident} --> {snake_case} -> {}", aspect.present(&source).to_token_stream(), presentation);

                        properties.push(ArgPresentation::NonatomicReadwrite {
                            ty: presentation.to_token_stream(),
                            name: format_ident!("{snake_case}").to_token_stream(),
                        });
                    }
                });
            });


        let from_body = DictionaryExpr::SwitchFields(quote!(ffi_ref->tag), from_conversions.present(&source));
        let to_body = DictionaryExpr::SwitchFields(quote!(obj.tag), to_conversions.present(&source));
        let drop_body = DictionaryExpr::SwitchFields(quote!(ffi_ref->tag), destroy_conversions.present(&source));

        let to_conversions = CommaPunctuated::new();

        let mut interfaces = Depunctuated::from_iter([
            InterfaceImplementation::Default {
                objc_name: objc_name.clone(),
                properties
            },
            InterfaceImplementation::ConversionsDeclaration {
                objc_name: objc_name.clone(),
                c_name: c_name.clone(),
            },
            InterfaceImplementation::BindingsDeclaration {
                objc_name: objc_name.clone(),
                c_name: c_name.clone(),
            },
            InterfaceImplementation::ConversionsImplementation {
                objc_name: objc_name.clone(),
                c_name: c_name.clone(),
                from_conversions_statements: from_body.to_token_stream(),
                to_conversions_statements: to_body.to_token_stream(),
                destroy_body: drop_body.to_token_stream(),
            },
            InterfaceImplementation::BindingsImplementation {
                objc_name,
                c_name,
                to_conversions,
                property_names,
            }
        ]);
        interfaces.extend(body_conversions);
        interfaces
    }
}
impl SourceFermentable<ObjCFermentate> for EnumComposer<ObjCSpecification> {
    fn ferment(&self) -> ObjCFermentate {
        let implementations = self.compose_interfaces();
        println!("OBJC: ENUM FERMENT: \n{}", format_interface_implementations(&implementations));
        ObjCFermentate::Item {
            implementations
        }
    }
}
// enum example_nested_model_quorum_quorum_type_OBJCEnumTest_Tag {
//     example_nested_model_quorum_quorum_type_OBJCEnumTest_Regular,
//     example_nested_model_quorum_quorum_type_OBJCEnumTest_UnnamedEx,
//     example_nested_model_quorum_quorum_type_OBJCEnumTest_NamedEx,
// };
//
// struct example_nested_model_quorum_quorum_type_OBJCEnumTest_NamedEx_Body {
//     char *qtype;
// };
//
// struct example_nested_model_quorum_quorum_type_OBJCEnumTest {
//     enum example_nested_model_quorum_quorum_type_OBJCEnumTest_Tag tag;
//     union {
//     struct {
//     char *unnamed_ex;
//     };
//     struct example_nested_model_quorum_quorum_type_OBJCEnumTest_NamedEx_Body named_ex;
//     };
// };
//
//
// @interface DSexample_nested_model_quorum_quorum_type_OBJCEnumTest_NamedEx_Body : NSObject
// @property (nonatomic, readwrite, nullable) NSString *qtype;
// @end
//
// @interface DSexample_nested_model_quorum_quorum_type_OBJCEnumTest_NamedEx_Body (Conversions)
// + (DSexample_nested_model_quorum_quorum_type_OBJCEnumTest_NamedEx_Body *)ffi_from:(struct example_nested_model_quorum_quorum_type_OBJCEnumTest_NamedEx_Body *)ffi_ref;
// + (DSexample_nested_model_quorum_quorum_type_OBJCEnumTest_NamedEx_Body *_Nullable)ffi_from_opt:(struct example_nested_model_quorum_quorum_type_OBJCEnumTest_NamedEx_Body *)ffi_ref;
// + (struct example_nested_model_quorum_quorum_type_OBJCEnumTest_NamedEx_Body *)ffi_to:(DSexample_nested_model_quorum_quorum_type_OBJCEnumTest_NamedEx_Body *)obj;
// + (struct example_nested_model_quorum_quorum_type_OBJCEnumTest_NamedEx_Body *)ffi_to_opt:(DSexample_nested_model_quorum_quorum_type_OBJCEnumTest_NamedEx_Body *_Nullable)obj;
// + (void)ffi_destroy:(struct example_nested_model_quorum_quorum_type_OBJCEnumTest_NamedEx_Body *)ffi_ref;
// @end
//
// @interface DSexample_nested_model_quorum_quorum_type_OBJCEnumTest_NamedEx_Body (Bindings)
// + (struct example_nested_model_quorum_quorum_type_OBJCEnumTest_NamedEx_Body *)ffi_ctor:(DSexample_nested_model_quorum_quorum_type_OBJCEnumTest_NamedEx_Body *)obj;
// + (void)ffi_dtor:(struct example_nested_model_quorum_quorum_type_OBJCEnumTest_NamedEx_Body *)ffi_ref;
// @end
//
// @implementation DSexample_nested_model_quorum_quorum_type_OBJCEnumTest_NamedEx_Body (Conversions)
// + (DSexample_nested_model_quorum_quorum_type_OBJCEnumTest_NamedEx_Body *)ffi_from:(struct example_nested_model_quorum_quorum_type_OBJCEnumTest_NamedEx_Body *)ffi_ref {
// DSexample_nested_model_quorum_quorum_type_OBJCEnumTest_NamedEx_Body *obj =
// [[self alloc] init];
// return obj;
// }
// + (DSexample_nested_model_quorum_quorum_type_OBJCEnumTest_NamedEx_Body *_Nullable)ffi_from_opt:(struct example_nested_model_quorum_quorum_type_OBJCEnumTest_NamedEx_Body *)ffi_ref {
//      return ffi_ref ? [self ffi_from:ffi_ref] : nil;
// }
// + (struct example_nested_model_quorum_quorum_type_OBJCEnumTest_NamedEx_Body *)ffi_to:(DSexample_nested_model_quorum_quorum_type_OBJCEnumTest_NamedEx_Body *)obj {
// struct example_nested_model_quorum_quorum_type_OBJCEnumTest_NamedEx_Body *ffi_ref = malloc(
// sizeof(struct example_nested_model_quorum_quorum_type_OBJCEnumTest_NamedEx_Body));
// ffi_ref->qtype = [NSString ffi_to: obj.qtype];
// return ffi_ref;
// }
// + (struct example_nested_model_quorum_quorum_type_OBJCEnumTest_NamedEx_Body *)ffi_to_opt:(DSexample_nested_model_quorum_quorum_type_OBJCEnumTest_NamedEx_Body *_Nullable)obj {
// return obj ? [self ffi_to:obj] : nil;
// }
// + (void)ffi_destroy:(struct example_nested_model_quorum_quorum_type_OBJCEnumTest_NamedEx_Body *)ffi_ref {
// if (!ffi_ref) return;
// free(ffi_ref->qtype)
// free(ffi_ref);
// }
// @end
// @implementation DSexample_nested_model_quorum_quorum_type_OBJCEnumTest_NamedEx_Body (Bindings)
// + (struct example_nested_model_quorum_quorum_type_OBJCEnumTest_NamedEx_Body *)ffi_ctor:
// (DSexample_nested_model_quorum_quorum_type_OBJCEnumTest_NamedEx_Body *)obj {
// example_nested_model_quorum_quorum_type_OBJCEnumTest_NamedEx_Body_ctor([NSString ffi_to: obj.qtype]);
//
// }
// + (void)ffi_dtor:(struct example_nested_model_quorum_quorum_type_OBJCEnumTest_NamedEx_Body *)ffi_ref {
// example_nested_model_quorum_quorum_type_OBJCEnumTest_NamedEx_Body_destroy(ffi_ref);
// }
// @end
//
//
//
// @interface DSexample_nested_model_quorum_quorum_type_OBJCEnumTest : NSObject
//
// @property (nonatomic, assign) enum example_nested_model_quorum_quorum_type_OBJCEnumTest_Tag tag;
//
// @property (nonatomic, readwrite) NSString *unnamed_ex;
// @property (nonatomic, readwrite) DSexample_nested_model_quorum_quorum_type_OBJCEnumTest_NamedEx_Body *named_ex;
//
// @end
//
// @interface DSexample_nested_model_quorum_quorum_type_OBJCEnumTest (Conversions)
// + (DSexample_nested_model_quorum_quorum_type_OBJCEnumTest *)ffi_from:(struct example_nested_model_quorum_quorum_type_OBJCEnumTest *)ffi_ref;
// + (DSexample_nested_model_quorum_quorum_type_OBJCEnumTest *_Nullable)ffi_from_opt:(struct example_nested_model_quorum_quorum_type_OBJCEnumTest *)ffi_ref;
// + (struct example_nested_model_quorum_quorum_type_OBJCEnumTest *)ffi_to:(DSexample_nested_model_quorum_quorum_type_OBJCEnumTest *)obj;
// + (struct example_nested_model_quorum_quorum_type_OBJCEnumTest *)ffi_to_opt:(DSexample_nested_model_quorum_quorum_type_OBJCEnumTest *_Nullable)obj;
// + (void)ffi_destroy:(struct example_nested_model_quorum_quorum_type_OBJCEnumTest *)ffi_ref;
// @end
//
// @interface DSexample_nested_model_quorum_quorum_type_OBJCEnumTest (Bindings)
// + (struct example_nested_model_quorum_quorum_type_OBJCEnumTest *)ffi_ctor:(DSexample_nested_model_quorum_quorum_type_OBJCEnumTest *)obj;
// + (void)ffi_dtor:(struct example_nested_model_quorum_quorum_type_OBJCEnumTest *)ffi_ref;
// @end
//
// @implementation DSexample_nested_model_quorum_quorum_type_OBJCEnumTest (Conversions)
// + (DSexample_nested_model_quorum_quorum_type_OBJCEnumTest *)ffi_from:(struct example_nested_model_quorum_quorum_type_OBJCEnumTest *)ffi_ref {
// DSexample_nested_model_quorum_quorum_type_OBJCEnumTest *obj = [[self alloc] init];
//      switch (ffi_ref->tag) {
//          case example_nested_model_quorum_quorum_type_OBJCEnumTest_Regular: {
//              break;
//          }
//          case example_nested_model_quorum_quorum_type_OBJCEnumTest_UnnamedEx: {
//              obj.unnamed_ex = [NSString ffi_from:ffi_ref->unnamed_ex];
//              break;
//          }
//          case example_nested_model_quorum_quorum_type_OBJCEnumTest_NamedEx: {
//              obj.named_ex = [DSexample_nested_model_quorum_quorum_type_OBJCEnumTest_NamedEx_Body ffi_from:ffi_ref->named_ex];
//              break;
//          }
//      }
//      obj.tag = ffi_ref->tag;
//      return obj;
// }
// + (DSexample_nested_model_quorum_quorum_type_OBJCEnumTest *_Nullable)ffi_from_opt:(struct example_nested_model_quorum_quorum_type_OBJCEnumTest *)ffi_ref {
//      return ffi_ref ? [self ffi_from:ffi_ref] : nil;
// }
// + (struct example_nested_model_quorum_quorum_type_OBJCEnumTest *)ffi_to:(DSexample_nested_model_quorum_quorum_type_OBJCEnumTest *)obj {
//      struct example_nested_model_quorum_quorum_type_OBJCEnumTest *ffi_ref = malloc(sizeof(struct example_nested_model_quorum_quorum_type_OBJCEnumTest));
//      ffi_ref->tag = obj.tag;
//      switch (obj.tag) {
//          case example_nested_model_quorum_quorum_type_OBJCEnumTest_Regular: {
//              break;
//          }
//          case example_nested_model_quorum_quorum_type_OBJCEnumTest_UnnamedEx: {
//              ffi_ref->unnamed_ex = [NSString ffi_to:obj.unnamed_ex];
//              break;
//          }
//          case example_nested_model_quorum_quorum_type_OBJCEnumTest_NamedEx: {
//              ffi_ref->named_ex = [DSexample_nested_model_quorum_quorum_type_OBJCEnumTest_NamedEx_Body ffi_to:obj.named_ex];
//              break;
//          }
//      }
//      return ffi_ref;
// }
// + (struct example_nested_model_quorum_quorum_type_OBJCEnumTest *)ffi_to_opt:(DSexample_nested_model_quorum_quorum_type_OBJCEnumTest *_Nullable)obj {
//      return obj ? [self ffi_to:obj] : nil;
// }
// + (void)ffi_destroy:(struct example_nested_model_quorum_quorum_type_OBJCEnumTest *)ffi_ref {
//      if (!ffi_ref) return;
//      switch (ffi_ref->tag) {
//          case example_nested_model_quorum_quorum_type_OBJCEnumTest_Regular: {
//              break;
//          }
//          case example_nested_model_quorum_quorum_type_OBJCEnumTest_UnnamedEx: {
//              [NSString ffi_destroy:ffi_ref->named_ex];
//              break;
//          }
//          case example_nested_model_quorum_quorum_type_OBJCEnumTest_NamedEx: {
//              [DSexample_nested_model_quorum_quorum_type_OBJCEnumTest_NamedEx_Body ffi_destroy:ffi_ref->named_ex];
//              break;
//          }
//      }
//      free(ffi_ref);
//  }
// @end
//
// @implementation DSexample_nested_model_quorum_quorum_type_OBJCEnumTest (Bindings)
// + (struct example_nested_model_quorum_quorum_type_OBJCEnumTest *)ffi_ctor:
// (DSexample_nested_model_quorum_quorum_type_OBJCEnumTest *)obj {
//      switch (obj.tag) {
//          case example_nested_model_quorum_quorum_type_OBJCEnumTest_Regular:
//              return example_nested_model_quorum_quorum_type_OBJCEnumTest_Regular_ctor();
//          case example_nested_model_quorum_quorum_type_OBJCEnumTest_UnnamedEx:
//              return example_nested_model_quorum_quorum_type_OBJCEnumTest_UnnamedEx_ctor([NSString ffi_to:obj.unnamed_ex]);
//          case example_nested_model_quorum_quorum_type_OBJCEnumTest_NamedEx:
//              return example_nested_model_quorum_quorum_type_OBJCEnumTest_NamedEx_ctor([DSexample_nested_model_quorum_quorum_type_OBJCEnumTest_NamedEx_Body ffi_to:obj.named_ex]);
// }
// }
// + (void)ffi_dtor:(struct example_nested_model_quorum_quorum_type_OBJCEnumTest *)ffi_ref {
//      example_nested_model_quorum_quorum_type_OBJCEnumTest_destroy(ffi_ref);
// }
// @end
