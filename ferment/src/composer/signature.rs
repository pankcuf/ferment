use std::cell::RefCell;
use std::rc::Rc;
use proc_macro2::Ident;
use quote::{quote, ToTokens};
use syn::{Attribute, BareFnArg, Field, Generics, ItemFn, parse_quote, Pat, Path, ReturnType, Signature, Type, TypeBareFn, TypePtr, Visibility};
use syn::token::RArrow;
use ferment_macro::BasicComposerOwner;
use crate::ast::{CommaPunctuated, ParenWrapped};
use crate::composable::{AttrsModel, CfgAttributes, FnArgComposer, FnReturnTypeComposer, FnSignatureContext};
use crate::composer::{BasicComposable, BasicComposer, BasicComposerOwner, BindingComposer, Composer, constants, DocsComposable, NameContext, Linkable, ParentComposer, SigParentComposer, SourceAccessible, SourceExpandable, CommaPunctuatedOwnedItems, ToConversionComposer, VarComposer};
use crate::context::{ScopeChain, ScopeContext, ScopeSearch, ScopeSearchKey};
use crate::conversion::{GenericTypeKind, TypeKind};
use crate::ext::{FFITypeResolve, Mangle, Resolve, ToType};
use crate::presentable::{Aspect, Context, Expression, OwnedItemPresentableContext, ScopeContextPresentable, SequenceOutput};
use crate::presentation::{ArgPresentation, BindingPresentation, DictionaryExpr, DictionaryName, DocPresentation, Expansion, FFIConversionFromMethodExpr, FFIConversionToMethodExpr, FFIVariable, InterfacePresentation, InterfacesMethodExpr, Name};

#[derive(BasicComposerOwner)]
pub struct SigComposer {
    pub base: BasicComposer<ParentComposer<Self>>,
    #[allow(unused)]
    pub binding_composer: BindingComposer<ParentComposer<Self>>
}

impl SigComposer {

    fn new(
        path: Path,
        sig_context: FnSignatureContext,
        generics: Option<Generics>,
        attrs: AttrsModel,
        binding_composer: BindingComposer<SigParentComposer>,
        context: &ParentComposer<ScopeContext>) -> SigParentComposer {
        let ty_context = Context::Fn { path, sig_context, attrs: attrs.cfg_attributes() };
        let root = Rc::new(RefCell::new(Self {
            base: BasicComposer::from(attrs, ty_context, generics, constants::composer_doc(), Rc::clone(context)),
            binding_composer,
        }));
        {
            let mut composer = root.borrow_mut();
            composer.base.link(&root);
        }
        root
    }
    pub fn with_context(
        path: Path,
        target_name: &Ident,
        sig_context: FnSignatureContext,
        generics: &Generics,
        attrs: &Vec<Attribute>,
        scope: &ScopeChain,
        context: &ParentComposer<ScopeContext>
    ) -> SigParentComposer {
        Self::new(
            path,
            sig_context,
            Some(generics.clone()),
            AttrsModel::from(attrs, target_name, scope),
            binding_composer_(),
            context)
    }
}

pub const fn binding_composer_<T>() -> BindingComposer<ParentComposer<T>> where T: BasicComposerOwner {
    |composer| {
        let composer = composer.borrow();
        let _source = composer.base().context.borrow();
        BindingPresentation::Empty
    }
}

impl DocsComposable for SigComposer {
    fn compose_docs(&self) -> DocPresentation {
        DocPresentation::Direct(self.base.doc.compose(&()))
    }
}

fn compose_regular_fn(path: Path, aspect: Aspect, attrs: Vec<Attribute>, generics: Option<Generics>, sig: &Signature, sig_context: &FnSignatureContext, source: &ScopeContext) -> BindingPresentation {
    // println!("compose_regular_fn: {}", path.to_token_stream());
    let Signature { output, inputs, asyncness, .. } = sig;
    let return_type = match output {
        ReturnType::Default => FnReturnTypeComposer {
            presentation: ReturnType::Default,
            conversion: Expression::LineTermination
        },
        ReturnType::Type(_, ty) => FnReturnTypeComposer {
            presentation: {
                ReturnType::Type(RArrow::default(), Box::new(VarComposer::new(ScopeSearch::KeyInScope(ScopeSearchKey::maybe_from_ref(ty).unwrap(), &source.scope)).compose(source).to_type()))
                // ReturnType::Type(RArrow::default(), Box::new(VariableComposer::from(&**ty).compose(source).to_type()))
            },
            conversion: {
                ToConversionComposer::new(Name::Dictionary(DictionaryName::Obj), *ty.clone(), None)
                    .compose(source)
            }
        }
    };

    let (arguments, argument_conversions): (CommaPunctuatedOwnedItems, CommaPunctuatedOwnedItems) = inputs
        .iter()
        .map(|arg| arg.compose(&(sig_context, &source)))
        .map(|FnArgComposer { name_type_original, name_type_conversion, .. }| {

            println!("COMPOSE_REGULAR_FN ARG: {} -- {}", name_type_original.present(&source).to_token_stream(), name_type_conversion.present(source));
            (name_type_original, OwnedItemPresentableContext::Expression(name_type_conversion, Vec::new()))
        })
        .unzip();
    BindingPresentation::RegularFunction {
        attrs,
        is_async: asyncness.is_some(),
        arguments: arguments.present(&source),
        name: Name::ModFn(path),
        input_conversions: SequenceOutput::RoundBracesFields((aspect, argument_conversions)).present(&source),
        return_type: return_type.presentation.clone(),
        generics,
        output_conversions: return_type.conversion.present(&source)
    }
}


impl SourceExpandable for SigComposer {
    fn expand(&self) -> Expansion {
        let source = self.source_ref();
        let binding = match self.name_context_ref() {
            Context::Fn { path: full_fn_path, sig_context, attrs } => {
                // println!("SigComposer::expand: Fn: {:?}", sig_context);
                match sig_context {
                    FnSignatureContext::ModFn(ItemFn { sig, .. }) => compose_regular_fn(
                        full_fn_path.clone(),
                        self.target_name_aspect(),
                        attrs.clone(),
                        None,
                        sig,
                        sig_context,
                        &source
                    ),
                    FnSignatureContext::Impl(_, _, sig) => compose_regular_fn(
                        full_fn_path.clone(),
                        self.ffi_name_aspect(),
                        attrs.clone(),
                        None,
                        sig,
                        sig_context,
                        &source
                    ),
                    FnSignatureContext::TraitInner(_, _, sig) => {
                        let Signature { output, inputs, .. } = sig;
                        let return_type = match output {
                            ReturnType::Default => FnReturnTypeComposer {
                                presentation: ReturnType::Default,
                                conversion: Expression::LineTermination
                            },
                            ReturnType::Type(_, ty) => FnReturnTypeComposer {
                                presentation: ReturnType::Type(RArrow::default(), Box::new(VarComposer::new(ScopeSearch::KeyInScope(ScopeSearchKey::maybe_from_ref(ty).unwrap(), &source.scope)).compose(&source).to_type())),
                                // presentation: ReturnType::Type(RArrow::default(), Box::new(VariableComposer::from(&**ty).compose(&source).to_type())),
                                conversion: ToConversionComposer::new(Name::Dictionary(DictionaryName::Obj), (&**ty).clone(), None).compose(&source)
                            },
                        };
                        let argument_comps = inputs
                            .iter()
                            .map(|arg| arg.compose(&(sig_context, &source)));

                        let arguments = CommaPunctuated::from_iter(argument_comps
                            .map(|arg| arg.name_type_original.clone()));
                        let presentation = ParenWrapped::new(arguments).present(&source);
                        let output_expression = return_type.presentation;



                        BindingPresentation::TraitVTableInnerFn {
                            name: Name::VTableInnerFn(sig.ident.clone()),
                            name_and_args: quote!(unsafe extern "C" fn #presentation),
                            output_expression
                        }
                    },
                    FnSignatureContext::Bare(_target_name, type_bare_fn) => {
                        let TypeBareFn { inputs, output, .. } = type_bare_fn;
                        let ffi_result = DictionaryName::FFiResult;
                        let opt_conversion = |conversion| DictionaryExpr::Simple(quote!((!ffi_result.is_null()).then(|| { #conversion })));

                        let ffi_result_conversion = FFIConversionFromMethodExpr::FfiFrom(ffi_result.to_token_stream());
                        let from_complex_result = || DictionaryExpr::CallbackDestructor(quote!(#ffi_result_conversion), quote!(#ffi_result));
                        let from_primitive_result = || DictionaryExpr::Simple(quote!(#ffi_result));
                        let from_opt_primitive_result = || DictionaryExpr::Deref(quote!(#ffi_result));
                        // println!("FnSignatureContext::Bare: result: {}", output.to_token_stream());
                        let (return_type, ffi_return_type, post_processing) = match output {
                            ReturnType::Type(token, field_type) => (
                                ReturnType::Type(token.clone(), Box::new(field_type.resolve(&source))),
                                ReturnType::Type(token.clone(), Box::new(<Type as Resolve<FFIVariable>>::resolve(field_type, &source).to_type())),
                                match TypeKind::from(field_type) {
                                    TypeKind::Primitive(_) => from_primitive_result(),
                                    TypeKind::Complex(_) =>  from_complex_result(),
                                    TypeKind::Generic(generic_ty) => match generic_ty {
                                        GenericTypeKind::TraitBounds(_) => unimplemented!("TODO: mixins+traits+generics"),
                                        GenericTypeKind::Optional(ty) => {
                                            opt_conversion(match TypeKind::from(ty) {
                                                TypeKind::Primitive(_) => from_opt_primitive_result(),
                                                TypeKind::Complex(_) |
                                                TypeKind::Generic(_) => from_complex_result(),
                                            })
                                        }
                                        _ => from_complex_result()
                                    }
                                }
                            ),
                            ReturnType::Default => (ReturnType::Default, ReturnType::Default, from_primitive_result()),
                        };
                        let mut arg_names = CommaPunctuated::new();
                        let mut ffi_args = CommaPunctuated::new();
                        let mut arg_target_types = CommaPunctuated::new();
                        let mut arg_target_fields = CommaPunctuated::new();
                        let mut arg_to_conversions = CommaPunctuated::new();
                        inputs
                            .iter()
                            .for_each(|bare_fn_arg| {
                                let BareFnArg { ty, name, .. } = bare_fn_arg;
                                let var_composer = VarComposer::new(ScopeSearch::KeyInScope(ScopeSearchKey::maybe_from_ref(ty).unwrap(), &source.scope));
                                let var_ty = var_composer.compose(&source);
                                println!("BAREFNARG: {}", var_ty.to_token_stream());
                                // let var_ty = VarComposer::new(ScopeSearch::KeyInScope(ScopeSearchKey::TypeRef(ty), &source.scope));
                                // let var_ty = VarComposer::new(ScopeSearch::Value(ScopeSearchKey::TypeRef(ty)));
                                // let var_ty = VariableComposer::from(ty);
                                let conversion = TypeKind::from(ty);
                                let ident_name = Name::Optional(name.as_ref().map(|(ident, ..)| ident.clone()));
                                arg_names.push(ident_name.to_token_stream());
                                arg_target_types.push(ArgPresentation::Pat(Pat::Verbatim(ty.to_token_stream())));
                                arg_target_fields.push(ArgPresentation::Field(field(ident_name.clone(), ty, &source)));


                                let mut bare_fn_arg_replacement = bare_fn_arg.clone();
                                bare_fn_arg_replacement.ty = var_ty.to_type();
                                ffi_args.push(bare_fn_arg_replacement);

                                // ffi_args.push(var_ty.compose(&source).to_token_stream());
                                // ffi_args.push(match &conversion {
                                //     TypeKind::Primitive(ty) => ty.clone(),
                                //     TypeKind::Complex(ty) => ty.special_or_to_ffi_full_path_variable_type(&source),
                                //     TypeKind::Generic(generic_ty) => generic_ty.special_or_to_ffi_full_path_variable_type(&source),
                                // }.to_token_stream());
                                arg_to_conversions.push(match &conversion {
                                    TypeKind::Primitive(..) => ident_name.to_token_stream(),
                                    TypeKind::Generic(generic_ty) => match generic_ty {
                                        GenericTypeKind::TraitBounds(_) => unimplemented!("TODO: mixins+traits+generics"),
                                        GenericTypeKind::Optional(ty) => match TypeKind::from(ty) {
                                            TypeKind::Primitive(_) => InterfacesMethodExpr::ToOptPrimitive(ident_name.to_token_stream()).to_token_stream(),
                                            TypeKind::Complex(_) |
                                            TypeKind::Generic(_) => FFIConversionToMethodExpr::FfiToOpt(ident_name.to_token_stream()).to_token_stream(),
                                        }
                                        _ => FFIConversionToMethodExpr::FfiTo(ident_name.to_token_stream()).to_token_stream()
                                    },
                                    TypeKind::Complex(..) => FFIConversionToMethodExpr::FfiTo(ident_name.to_token_stream()).to_token_stream(),
                                });
                            });

                        let attrs = self.compose_attributes();
                        let conversion = InterfacePresentation::CallbackNew {
                            attrs: attrs.clone(),
                            ffi_type: self.ffi_name_aspect().present(&source),
                            inputs: arg_target_fields,
                            output: return_type,
                            body: {
                                DictionaryExpr::CallbackCaller(arg_to_conversions.to_token_stream(), post_processing.to_token_stream()).to_token_stream()

                            //     quote! {
                            //     // |(#arg_names)| {
                            //         let ffi_result = (self.caller)(#arg_to_conversions);
                            //         #post_processing
                            //     // }
                            // }
                            }
                        };
                        BindingPresentation::Callback {
                            name: full_fn_path.mangle_ident_default(),
                            attrs: attrs.clone(),
                            ffi_args: ffi_args,
                            result: ffi_return_type,
                            conversion
                        }
                    }
                }

            }
            _ => panic!("Wrong name context for fn")
        };
        Expansion::Function {
            comment: self.compose_docs(),
            binding
        }
    }
}

fn field(name: Name, ty: &Type, source: &ScopeContext) -> Field {
    Field {
        attrs: vec![],
        vis: Visibility::Inherited,
        ident: Some(name.mangle_ident_default()),
        colon_token: None,
        ty: match ty {
            Type::Ptr(TypePtr { const_token, mutability, .. }) => {
                let ty = ty.full_type(source);
                if const_token.is_some() {
                    parse_quote!(*const #ty)
                } else if mutability.is_some() {
                    parse_quote!(*mut #ty)
                } else {
                    ty
                }
            },
            _ => ty.full_type(source)
        },
    }
}

// impl<'a, Parent> Composer<'a>
// for MethodComposer<Parent, DestructorContext, DestructorContext>
//     where Parent: SharedAccess {
//     type Source = ScopeContext;
//     type Result = BindingPresentation;
//     fn compose(&self, _source: &Self::Source) -> Self::Result {
//         (self.seq_iterator_item)(
//             self.parent.as_ref()
//                 .expect("no parent")
//                 .access(self.context))
//     }
// }
