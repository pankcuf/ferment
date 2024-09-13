use std::cell::RefCell;
use std::rc::Rc;
use proc_macro2::Ident;
use quote::{quote, ToTokens};
use syn::{Attribute, BareFnArg, Field, Generics, ImplItemMethod, ItemFn, parse_quote, Pat, Path, ReturnType, Signature, Type, TypeBareFn, TypePtr, Visibility};
use syn::token::Comma;
use crate::ast::{CommaPunctuated, Depunctuated, ParenWrapped};
use crate::composable::{AttrsModel, CfgAttributes, FnArgComposer, FnReturnTypeComposer, FnSignatureContext, GenModel};
use crate::composer::{BasicComposer, BasicComposerOwner, RustBindingComposer, Composer, constants, DocsComposable, NameContext, Linkable, ComposerLink, SourceAccessible, SourceFermentable2, CommaPunctuatedOwnedItems, ToConversionComposer, VarComposer, NameComposable, AttrComposable, GenericsComposable};
use crate::context::{ScopeChain, ScopeContext, ScopeSearch, ScopeSearchKey};
use crate::conversion::{GenericTypeKind, TypeKind};
use crate::ext::{CrateExtension, FFITypeResolve, ItemExtension, Mangle, Resolve, ToType};
use crate::lang::{LangAttrSpecification, LangGenSpecification};
use crate::presentable::{Aspect, Context, Expression, OwnedItemPresentableContext, ScopeContextPresentable, SequenceOutput};
use crate::presentation::{ArgPresentation, BindingPresentation, DictionaryExpr, DictionaryName, DocPresentation, RustFermentate, FFIConversionFromMethodExpr, FFIConversionToMethodExpr, FFIVariable, InterfacePresentation, InterfacesMethodExpr, Name};

// #[derive(BasicComposerOwner)]
pub struct SigComposer<LANG, SPEC, Gen>
    where LANG: Clone + 'static,
          SPEC: LangAttrSpecification<LANG> + 'static,
          Gen: LangGenSpecification<LANG> + 'static {
    pub base: BasicComposer<ComposerLink<Self>, LANG, SPEC, Gen>,
    #[allow(unused)]
    pub binding_composer: RustBindingComposer<ComposerLink<Self>>
}

impl<LANG, SPEC, Gen> BasicComposerOwner<Context, LANG, SPEC, Gen> for SigComposer<LANG, SPEC, Gen>
    where LANG: Clone,
          SPEC: LangAttrSpecification<LANG>,
          Gen: LangGenSpecification<LANG> {
    fn base(&self) -> &BasicComposer<ComposerLink<Self>, LANG, SPEC, Gen> {
        &self.base
    }
}
impl<LANG, SPEC, Gen> AttrComposable<SPEC> for SigComposer<LANG, SPEC, Gen>
    where LANG: Clone + 'static,
          SPEC: LangAttrSpecification<LANG> + 'static,
          Gen: LangGenSpecification<LANG> + 'static,
          SequenceOutput<LANG, SPEC>: ScopeContextPresentable,
          OwnedItemPresentableContext<LANG, SPEC>: ScopeContextPresentable {
    fn compose_attributes(&self) -> SPEC {
        self.base().compose_attributes()
    }
}
impl<LANG, SPEC, Gen> GenericsComposable<Gen> for SigComposer<LANG, SPEC, Gen>
    where LANG: Clone + 'static,
          SPEC: LangAttrSpecification<LANG> + 'static,
          Gen: LangGenSpecification<LANG> + 'static,
          SequenceOutput<LANG, SPEC>: ScopeContextPresentable,
          OwnedItemPresentableContext<LANG, SPEC>: ScopeContextPresentable {
    fn compose_generics(&self) -> Gen {
        self.base().compose_generics()
    }
}

impl<LANG, SPEC, Gen> NameContext<Context> for SigComposer<LANG, SPEC, Gen>
    where LANG: Clone,
          SPEC: LangAttrSpecification<LANG>,
          Gen: LangGenSpecification<LANG>,
          SequenceOutput<LANG, SPEC>: ScopeContextPresentable,
          OwnedItemPresentableContext<LANG, SPEC>: ScopeContextPresentable {
    fn name_context_ref(&self) -> &Context {
        self.base().name_context_ref()
    }
}

impl<LANG, SPEC, Gen> SourceAccessible for SigComposer<LANG, SPEC, Gen>
    where LANG: Clone,
          SPEC: LangAttrSpecification<LANG>,
          Gen: LangGenSpecification<LANG>,
          SequenceOutput<LANG, SPEC>: ScopeContextPresentable,
          OwnedItemPresentableContext<LANG, SPEC>: ScopeContextPresentable {
    fn context(&self) -> &ComposerLink<ScopeContext> {
        self.base().context()
    }
}



impl<LANG, SPEC, Gen> SigComposer<LANG, SPEC, Gen>
    where LANG: Clone,
          SPEC: LangAttrSpecification<LANG>,
          Gen: LangGenSpecification<LANG>,
          SequenceOutput<LANG, SPEC>: ScopeContextPresentable,
          OwnedItemPresentableContext<LANG, SPEC>: ScopeContextPresentable {

    fn new(
        path: Path,
        sig_context: FnSignatureContext,
        generics: Option<Generics>,
        attrs: AttrsModel,
        binding_composer: RustBindingComposer<ComposerLink<Self>>,
        context: &ComposerLink<ScopeContext>) -> ComposerLink<Self> {
        let ty_context = Context::Fn { path, sig_context, attrs: attrs.cfg_attributes() };
        let root = Rc::new(RefCell::new(Self {
            base: BasicComposer::<ComposerLink<Self>, LANG, SPEC, Gen>::from(attrs, ty_context, GenModel::new(generics), constants::composer_doc(), Rc::clone(context)),
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
        sig_context: FnSignatureContext,
        generics: &Generics,
        attrs: &Vec<Attribute>,
        context: &ComposerLink<ScopeContext>
    ) -> ComposerLink<Self> {
        Self::new(
            path,
            sig_context,
            Some(generics.clone()),
            AttrsModel::from(attrs),
            binding_composer_(),
            context)
    }
    pub fn from_item_fn(item_fn: &ItemFn, scope: &ScopeChain, context: &ComposerLink<ScopeContext>) -> ComposerLink<Self> {
        let ItemFn { attrs, sig: Signature { generics, ..}, .. } = item_fn;
        let source = context.borrow();
        Self::with_context(
            scope.self_path().crate_named(&source.scope.crate_ident_as_path()),
            FnSignatureContext::ModFn(item_fn.clone()),
            generics,
            attrs,
            context)
    }
    pub fn from_type_bare_fn(type_bare_fn: &TypeBareFn, target_name: &Ident, generics: &Generics, attrs: &Vec<Attribute>, scope: &ScopeChain, context: &ComposerLink<ScopeContext>) -> ComposerLink<Self> {
        Self::with_context(
            scope.self_path().crate_named(&scope.crate_ident_as_path()),
            FnSignatureContext::Bare(target_name.clone(), type_bare_fn.clone()),
            generics,
            attrs,
            context
        )
    }

    pub fn from_impl_item_method(impl_item_method: &ImplItemMethod, self_ty: &Type, trait_path: Option<Type>, scope: &ScopeChain, context: &ComposerLink<ScopeContext>) -> ComposerLink<Self> {
        let ImplItemMethod { sig, .. } = impl_item_method;
        Self::with_context(
            scope.joined_path_holder(&sig.ident).0,
            FnSignatureContext::Impl(self_ty.clone(), trait_path, sig.clone()),
            &sig.generics,
            sig.maybe_attrs().unwrap_or(&vec![]),
            context
        )
    }
}

impl<LANG, SPEC, Gen> DocsComposable for SigComposer<LANG, SPEC, Gen>
    where LANG: Clone,
          SPEC: LangAttrSpecification<LANG>,
          Gen: LangGenSpecification<LANG> {
    fn compose_docs(&self) -> DocPresentation {
        DocPresentation::Direct(self.base.doc.compose(&()))
    }
}

fn compose_regular_fn(path: Path, aspect: Aspect<Context>, attrs: Vec<Attribute>, generics: Option<Generics>, sig: &Signature, sig_context: &FnSignatureContext, source: &ScopeContext) -> BindingPresentation {
    // println!("compose_regular_fn: {}", path.to_token_stream());
    let Signature { output, inputs, asyncness, .. } = sig;
    let return_type = match output {
        ReturnType::Default => FnReturnTypeComposer {
            presentation: ReturnType::Default,
            conversion: Expression::LineTermination
        },
        ReturnType::Type(_, ty) => FnReturnTypeComposer {
            presentation: ReturnType::Type(Default::default(), Box::new(VarComposer::new(ScopeSearch::KeyInScope(ScopeSearchKey::maybe_from_ref(ty).unwrap(), &source.scope)).compose(source).to_type())),
            conversion: ToConversionComposer::new(Name::Dictionary(DictionaryName::Obj), *ty.clone(), None).compose(source)
        }
    };

    let (arguments, argument_conversions): (CommaPunctuatedOwnedItems<RustFermentate, Vec<Attribute>>, CommaPunctuatedOwnedItems<RustFermentate, Vec<Attribute>>) = inputs
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


// impl SourceAccessible for SigComposer<RustFermentate> {
//     fn context(&self) -> &ParentComposer<ScopeContext> {
//         self.context()
//     }
// }

impl SourceFermentable2<RustFermentate> for SigComposer<RustFermentate, Vec<Attribute>, Option<Generics>> {
    fn ferment(&self) -> Depunctuated<RustFermentate> {
        let source = self.source_ref();
        let binding = match self.name_context_ref() {
            Context::Fn { path: full_fn_path, sig_context, attrs } => {
                // println!("SigComposer::ferment: Fn: {:?}", sig_context);
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
                                presentation: ReturnType::Type(Default::default(), Box::new(VarComposer::new(ScopeSearch::KeyInScope(ScopeSearchKey::maybe_from_ref(ty).unwrap(), &source.scope)).compose(&source).to_type())),
                                conversion: ToConversionComposer::new(Name::Dictionary(DictionaryName::Obj), (&**ty).clone(), None).compose(&source)
                            },
                        };
                        let argument_comps = inputs
                            .iter()
                            .map(|arg| arg.compose(&(sig_context, &source)));

                        let arguments = CommaPunctuated::from_iter(argument_comps
                            .map(|arg| arg.name_type_original.present(&source)));
                        let presentation = ParenWrapped::<_, Comma>::new(arguments);
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
                                let conversion = TypeKind::from(ty);
                                let ident_name = Name::Optional(name.as_ref().map(|(ident, ..)| ident.clone()));
                                arg_names.push(ident_name.to_token_stream());
                                arg_target_types.push(ArgPresentation::Pat(Pat::Verbatim(ty.to_token_stream())));
                                arg_target_fields.push(ArgPresentation::Field(field(ident_name.clone(), ty, &source)));
                                let mut bare_fn_arg_replacement = bare_fn_arg.clone();
                                bare_fn_arg_replacement.ty = var_ty.to_type();
                                ffi_args.push(bare_fn_arg_replacement);
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
                        // let attrs = BasicContextComposable::<SigParentComposer, Vec<Attribute>, Option<Generics>>::compose_attributes(self)
                        // let attrs = <Self as BasicContextComposable<SigParentComposer, Vec<Attribute>, Option<Generics>>>::compose_attributes(self);
                        let ffi_type = self.compose_ffi_name();
                        // <Self as NameContext<Context>>::ffi_name_aspect(self).present(&source);
                        let conversion = InterfacePresentation::callback(attrs, &ffi_type, arg_target_fields, return_type, arg_to_conversions, post_processing);
                        BindingPresentation::Callback {
                            name: full_fn_path.mangle_ident_default(),
                            attrs: attrs.clone(),
                            ffi_args,
                            result: ffi_return_type,
                            conversion
                        }
                    }
                }

            }
            _ => panic!("Wrong name context for fn")
        };
        Depunctuated::from_iter([RustFermentate::Function { comment: self.compose_docs(), binding }])
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
pub const fn binding_composer_<T, CTX, LANG, SPEC, Gen>()
    -> RustBindingComposer<ComposerLink<T>>
    where T: BasicComposerOwner<CTX, LANG, SPEC, Gen>,
          CTX: Clone,
          LANG: Clone,
          Gen: LangGenSpecification<LANG>,
          SPEC: LangAttrSpecification<LANG>,
          Aspect<CTX>: ScopeContextPresentable {
    |composer| {
        let composer = composer.borrow();
        let _source = composer.base().context.borrow();
        BindingPresentation::Empty
    }
}

