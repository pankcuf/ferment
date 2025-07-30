use quote::{quote, ToTokens};
use syn::__private::TokenStream2;
use syn::parse_quote;
use crate::composable::TypeModel;
use crate::composer::{SourceComposable, TargetVarComposer};
use crate::context::{ScopeContext, ScopeSearchKey};
use crate::conversion::{DictFermentableModelKind, DictTypeModelKind, GroupModelKind, ObjectKind, ScopeItemKind, SmartPointerModelKind, TypeModelKind};
use crate::ext::{AsType, GenericNestedArg, Mangle, Resolve, SpecialType, ToType};
use crate::lang::objc::ObjCSpecification;
use crate::lang::objc::composer::var::{objc_primitive, resolve_type_variable};
use crate::presentation::{FFIFullPath, FFIVariable};

impl<'a> SourceComposable for TargetVarComposer<'a, ObjCSpecification> {
    type Source = ScopeContext;
    type Output = FFIVariable<ObjCSpecification, TokenStream2>;

    fn compose(&self, source: &Self::Source) -> Self::Output {
        let search_key = self.search.search_key();
        let ptr_composer = search_key.ptr_composer();
        let maybe_obj = source.maybe_object_by_predicate_ref(&self.search);
        let full_ty = maybe_obj.as_ref().and_then(ObjectKind::maybe_type).unwrap_or_else(|| search_key.to_type());
        let maybe_special: Option<SpecialType<ObjCSpecification>> = full_ty.maybe_resolve(source);
        match maybe_special {
            Some(special) => match maybe_obj {
                Some(ObjectKind::Item(.., ScopeItemKind::Fn(..))) =>
                    ptr_composer(source.maybe_to_fn_type()
                        .unwrap_or_else(|| search_key.to_type())
                        .to_token_stream()),
                Some(ObjectKind::Type(TypeModelKind::Bounds(bounds))) =>
                    FFIVariable::mut_ptr(bounds.mangle_tokens_default()),
                _ =>
                    ptr_composer(special.to_token_stream())
            }
            None => match maybe_obj {
                Some(ObjectKind::Item(_, ScopeItemKind::Fn(..))) =>
                    ptr_composer(source.maybe_to_trait_fn_type::<ObjCSpecification>()
                        .map(ToTokens::into_token_stream)
                        .unwrap_or_else(|| search_key.to_token_stream())),
                Some(ObjectKind::Type(ref ty_model_kind)) |
                Some(ObjectKind::Item(ref ty_model_kind, ..)) => {
                    let conversion = ty_model_kind.maybe_trait_model_kind_or_same(source).unwrap();
                    match conversion {
                        TypeModelKind::Dictionary(
                            DictTypeModelKind::NonPrimitiveFermentable(
                                DictFermentableModelKind::SmartPointer(
                                    SmartPointerModelKind::Box(model)))) => {
                            let ty = model.as_type();
                            let full_nested_ty = ty.maybe_first_nested_type_ref().unwrap();
                            match Resolve::<SpecialType<ObjCSpecification>>::maybe_resolve(full_nested_ty, source) {
                                Some(special) =>
                                    ptr_composer(special.to_token_stream()),
                                None => {
                                    let var_ty = match source.maybe_object_by_value(full_nested_ty) {
                                        Some(ObjectKind::Item(.., ScopeItemKind::Fn(..))) =>
                                            source.maybe_trait_or_regular_model_kind(),
                                        Some(ObjectKind::Type(ref kind) |
                                             ObjectKind::Item(ref kind, ..)) =>
                                            kind.maybe_trait_model_kind_or_same(source),
                                        _ => None,
                                    }.unwrap_or_else(|| TypeModelKind::unknown_type_ref(full_nested_ty));
                                    let var_c_type = var_ty.to_type();
                                    let ffi_path: Option<FFIFullPath<ObjCSpecification>> = var_c_type.maybe_resolve(source);
                                    let var_ty = ffi_path.map(|p| p.to_type())
                                        .unwrap_or_else(|| parse_quote!(#var_c_type));
                                    let result = resolve_type_variable(var_ty, source);
                                    result
                                }
                            }
                        },
                        TypeModelKind::Unknown(TypeModel { ty, .. }) =>
                            FFIVariable::mut_ptr(ty.to_token_stream()),
                        TypeModelKind::Dictionary(DictTypeModelKind::Primitive(TypeModel { ty, .. })) =>
                            FFIVariable::direct(objc_primitive(&ty)),
                        TypeModelKind::Dictionary(DictTypeModelKind::NonPrimitiveFermentable(DictFermentableModelKind::I128(..))) =>
                            FFIVariable::mut_ptr(parse_quote!(uint8_t (*)[16])),
                        TypeModelKind::Dictionary(DictTypeModelKind::NonPrimitiveFermentable(DictFermentableModelKind::U128(..))) =>
                            FFIVariable::mut_ptr(parse_quote!(uint8_t (*)[16])),
                        TypeModelKind::FnPointer(TypeModel { ty, .. }, ..) =>
                            FFIVariable::direct(Resolve::<SpecialType<ObjCSpecification>>::maybe_resolve(&ty, source)
                                .map(ToTokens::into_token_stream)
                                .unwrap_or_else(|| Resolve::<FFIFullPath<ObjCSpecification>>::resolve(&ty, source)
                                    .to_token_stream())),
                        TypeModelKind::Dictionary(DictTypeModelKind::LambdaFn(TypeModel { ty, .. }, ..)) =>
                            FFIVariable::mut_ptr(Resolve::<FFIFullPath<ObjCSpecification>>::resolve(&ty, source)
                                .to_token_stream()),
                        TypeModelKind::Dictionary(
                            DictTypeModelKind::NonPrimitiveFermentable(
                                DictFermentableModelKind::SmartPointer(
                                    SmartPointerModelKind::Arc(TypeModel { ty, .. }) |
                                    SmartPointerModelKind::Rc(TypeModel { ty, .. }) |
                                    SmartPointerModelKind::Mutex(TypeModel { ty, .. }) |
                                    SmartPointerModelKind::OnceLock(TypeModel { ty, .. }) |
                                    SmartPointerModelKind::RwLock(TypeModel { ty, .. }) |
                                    SmartPointerModelKind::Cell(TypeModel { ty, .. }) |
                                    SmartPointerModelKind::RefCell(TypeModel { ty, .. }) |
                                    SmartPointerModelKind::UnsafeCell(TypeModel { ty, .. }) |
                                    SmartPointerModelKind::Pin(TypeModel { ty, .. })
                                ) |
                                DictFermentableModelKind::Group(
                                    GroupModelKind::BTreeSet(TypeModel { ty, .. }) |
                                    GroupModelKind::HashSet(TypeModel { ty, .. }) |
                                    GroupModelKind::Map(TypeModel { ty, .. }) |
                                    GroupModelKind::Result(TypeModel { ty, .. }) |
                                    GroupModelKind::Vec(TypeModel { ty, .. }) |
                                    GroupModelKind::IndexMap(TypeModel { ty, .. }) |
                                    GroupModelKind::IndexSet(TypeModel { ty, .. })
                                ) |
                                DictFermentableModelKind::Other(TypeModel { ty, .. }))) => {

                            let maybe_ffi_full_path: Option<FFIFullPath<ObjCSpecification>> = ty.maybe_resolve(source);
                            resolve_type_variable(maybe_ffi_full_path.map(|path| path.to_type()).unwrap_or_else(|| ty.to_type()), source)
                        },
                        TypeModelKind::Dictionary(
                            DictTypeModelKind::NonPrimitiveFermentable(
                                DictFermentableModelKind::Str(..) |
                                DictFermentableModelKind::String(..))) =>
                            FFIVariable::mut_ptr(quote!(char)),

                        TypeModelKind::Dictionary(DictTypeModelKind::NonPrimitiveOpaque(..)) =>
                            Resolve::<FFIVariable<ObjCSpecification, TokenStream2>>::resolve(&conversion, source),
                        TypeModelKind::Bounds(bounds) =>
                            bounds.resolve(source),
                        ref cnv=> {
                            let var_ty = match maybe_obj {
                                Some(ObjectKind::Item(.., ScopeItemKind::Fn(..))) => match &source.scope.parent_object().unwrap() {
                                    ObjectKind::Type(ref ty_conversion) |
                                    ObjectKind::Item(ref ty_conversion, ..) =>
                                        ty_conversion.maybe_trait_model_kind_or_same(source),
                                    ObjectKind::Empty => None,
                                },
                                Some(ObjectKind::Type(..) |
                                     ObjectKind::Item(..)) =>
                                    cnv.maybe_trait_model_kind_or_same(source),
                                _ => None,
                            }.unwrap_or_else(|| cnv.clone());
                            let var_c_type = var_ty.to_type();
                            let ffi_path: Option<FFIFullPath<ObjCSpecification>> = var_c_type.maybe_resolve(source);
                            let var_ty = ffi_path.map(|p| p.to_type())
                                .unwrap_or_else(|| parse_quote!(#var_c_type));
                            let result = resolve_type_variable(var_ty, source);
                            result
                        }
                    }
                },
                _ => {
                    let maybe_special: Option<SpecialType<ObjCSpecification>> = ScopeSearchKey::maybe_resolve(search_key, source);
                    maybe_special
                        .map(FFIFullPath::from)
                        .or_else(|| Resolve::<TypeModelKind>::resolve(search_key, source)
                            .to_type()
                            .maybe_resolve(source))
                        .map(|ffi_path| ffi_path.to_type())
                        .unwrap_or_else(|| search_key.to_type())
                        .resolve(source)
                }
            }
        }

    }
}