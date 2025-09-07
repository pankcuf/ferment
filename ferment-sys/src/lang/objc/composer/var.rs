use quote::{format_ident, quote, quote_spanned, ToTokens};
use syn::__private::TokenStream2;
use syn::{parse_quote, AngleBracketedGenericArguments, GenericArgument, Path, PathArguments, PathSegment, TraitBound, Type, TypeArray, TypeImplTrait, TypeParamBound, TypePath, TypePtr, TypeReference, TypeSlice, TypeTraitObject};
use syn::spanned::Spanned;
use crate::composable::TypeModel;
use crate::composer::{SourceComposable, VarComposer};
use crate::context::{ScopeContext, ScopeSearchKey};
use crate::kind::{DictFermentableModelKind, DictTypeModelKind, GenericTypeKind, GroupModelKind, ObjectKind, ScopeItemKind, SmartPointerModelKind, SpecialType, TypeKind, TypeModelKind};
use crate::ext::{AsType, DictionaryType, FFISpecialTypeResolve, GenericNestedArg, Mangle, Resolve, ToPath, ToType};
use crate::lang::objc::ObjCSpecification;
use crate::presentation::{FFIFullDictionaryPath, FFIFullPath, FFIVariable};



impl SourceComposable for VarComposer<ObjCSpecification> {
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
                Some(ObjectKind::Item(_, ScopeItemKind::Fn(..))) =>
                    ptr_composer(source.maybe_to_fn_type()
                        .unwrap_or_else(|| search_key.to_type())
                        .to_token_stream()),
                Some(ObjectKind::Type(TypeModelKind::Bounds(bounds))) =>
                    FFIVariable::mut_ptr(bounds.mangle_tokens_default()),
                _ => ptr_composer(special.to_token_stream())
            }
            None => match maybe_obj {
                Some(ObjectKind::Item(_, ScopeItemKind::Fn(..))) =>
                    ptr_composer(source.maybe_to_trait_fn_type::<ObjCSpecification>()
                        .map_or(search_key.to_token_stream(), ToTokens::into_token_stream)),
                Some(ObjectKind::Type(ref ty_model_kind)) |
                Some(ObjectKind::Item(ref ty_model_kind, ..)) => {
                    let conversion = ty_model_kind.maybe_trait_object_maybe_model_kind_or_same(source);
                    match conversion {
                        TypeModelKind::Dictionary(DictTypeModelKind::NonPrimitiveFermentable(DictFermentableModelKind::SmartPointer(SmartPointerModelKind::Box(model)))) => {
                            if let Some(full_nested_ty) = model.as_type().maybe_first_nested_type_ref() {
                                match Resolve::<SpecialType<ObjCSpecification>>::maybe_resolve(full_nested_ty, source) {
                                    Some(special) =>
                                        ptr_composer(special.to_token_stream()),
                                    None => {
                                        let object = source.maybe_object_by_value(full_nested_ty);
                                        let var_ty = object.and_then(|object_kind| object_kind.maybe_fn_or_trait_or_same_kind2(source))
                                            .unwrap_or_else(|| TypeModelKind::unknown_type_ref(full_nested_ty));
                                        let var_c_type = var_ty.to_type();
                                        let ffi_path: Option<FFIFullPath<ObjCSpecification>> = var_c_type.maybe_resolve(source);
                                        let var_ty = ffi_path.map(|p| p.to_type())
                                            .unwrap_or_else(|| var_c_type);
                                        resolve_type_variable(var_ty, source)
                                    }
                                }
                            } else {
                                ptr_composer(model.ty.to_token_stream())
                            }
                        },
                        TypeModelKind::Unknown(TypeModel { ty, .. }) =>
                            FFIVariable::mut_ptr(ty.to_token_stream()),
                        TypeModelKind::Dictionary(DictTypeModelKind::Primitive(TypeModel { ty, .. })) => {
                            FFIVariable::direct(objc_primitive(&ty))
                        },
                        TypeModelKind::Dictionary(DictTypeModelKind::NonPrimitiveFermentable(DictFermentableModelKind::I128(..))) =>
                            FFIVariable::mut_ptr(parse_quote!(NSData)),
                        TypeModelKind::Dictionary(DictTypeModelKind::NonPrimitiveFermentable(DictFermentableModelKind::U128(..))) =>
                            FFIVariable::mut_ptr(parse_quote!(NSData)),
                        TypeModelKind::FnPointer(TypeModel { ty, .. }, ..) =>
                            FFIVariable::direct(Resolve::<SpecialType<ObjCSpecification>>::maybe_resolve(&ty, source)
                                .map(|special| special.to_token_stream())
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
                                DictFermentableModelKind::Other(TypeModel { ty, .. }) |
                                DictFermentableModelKind::Str(TypeModel { ty, .. }) |
                                DictFermentableModelKind::String(TypeModel { ty, .. }))) => {
                            let maybe_ffi_full_path: Option<FFIFullPath<ObjCSpecification>> = ty.maybe_resolve(source);
                            resolve_type_variable(maybe_ffi_full_path.map(|path| path.to_type()).unwrap_or_else(|| parse_quote!(#ty)), source)
                        },
                        TypeModelKind::Dictionary(DictTypeModelKind::NonPrimitiveOpaque(..)) =>
                            Resolve::<FFIVariable<ObjCSpecification, TokenStream2>>::resolve(&conversion, source),
                        TypeModelKind::Bounds(bounds) => {
                            bounds.resolve(source)
                        },
                        ref cnv=> {
                            let var_ty = match maybe_obj {
                                Some(ObjectKind::Item(.., ScopeItemKind::Fn(..))) => match &source.scope.parent_object() {
                                    Some(ObjectKind::Type(ref ty_conversion) | ObjectKind::Item(ref ty_conversion, ..)) =>
                                        ty_conversion.maybe_trait_model_kind_or_same(source),
                                    _ => None
                                },
                                Some(ObjectKind::Type(..) |
                                     ObjectKind::Item(..)) =>
                                    cnv.maybe_trait_model_kind_or_same(source),
                                _ => None,
                            }.unwrap_or_else(|| cnv.clone());
                            let var_c_type = var_ty.to_type();
                            let ffi_path: Option<FFIFullPath<ObjCSpecification>> = var_c_type.maybe_resolve(source);
                            let var_ty = ffi_path.map(|p| p.to_type()).unwrap_or_else(|| parse_quote!(#var_c_type));
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
impl Resolve<FFIVariable<ObjCSpecification, TokenStream2>> for Path {
    fn maybe_resolve(&self, source: &ScopeContext) -> Option<FFIVariable<ObjCSpecification, TokenStream2>> {
        Some(self.resolve(source))
    }
    fn resolve(&self, source: &ScopeContext) -> FFIVariable<ObjCSpecification, TokenStream2> {
        // println!("Path::<FFIVariable>::resolve({})", self.to_token_stream());
        match (self.segments.first(), self.segments.last()) {
            (Some(PathSegment { ident: first_ident, .. }), Some(PathSegment { ident: last_ident, arguments })) => {
                if last_ident.is_primitive() {
                    FFIVariable::direct(objc_primitive_from_path(self))
                } else if last_ident.is_optional() {
                    let result = match arguments {
                        PathArguments::AngleBracketed(AngleBracketedGenericArguments { args, .. }) =>
                            args.iter().find_map(|arg| match arg {
                                GenericArgument::Type(ty) => match TypeKind::from(ty) {
                                    TypeKind::Primitive(ty) => Some(FFIVariable::mut_ptr(ty.to_token_stream())),
                                    TypeKind::Generic(generic_ty) => Some(FFIVariable::mut_ptr(Resolve::<FFIFullPath<ObjCSpecification>>::resolve(&generic_ty, source).to_token_stream())),
                                    TypeKind::Complex(Type::Path(TypePath { path, .. })) => Some(Resolve::<FFIVariable<ObjCSpecification, TokenStream2>>::resolve(&path, source)),
                                    _ => None
                                },
                                _ => None
                            }),
                        _ => None,
                    };
                    result.unwrap_or_else(|| FFIVariable::mut_ptr(self.to_token_stream()))
                } else if last_ident.is_special_generic() || (last_ident.is_result() /*&& path.segments.len() == 1*/) || (last_ident.eq("Map") && first_ident.eq("serde_json")) {
                    FFIVariable::mut_ptr(source.scope_type_for_path(self).map_or(self.to_token_stream(), |full_type| full_type.mangle_tokens_default()))
                } else {
                    FFIVariable::mut_ptr(self.to_token_stream())
                }
            },
            _ => FFIVariable::mut_ptr(self.to_token_stream())
        }
    }
}
// impl SourceComposable for VariableComposer<ObjCSpecification> {
//     type Source = ScopeContext;
//     type Output = FFIVariable<ObjCSpecification, TokenStream2>;
//
//     fn compose(&self, source: &Self::Source) -> Self::Output {
//         let is_const_ptr = match self.ty {
//             Type::Ptr(TypePtr { const_token, .. }) => const_token.is_some(),
//             _ => false
//         };
//
//         let full_ty: Type = Resolve::resolve(&self.ty, source);
//         // println!("VariableComposer (compose): {} ({}) in {}", self.ty.to_token_stream(), full_ty.to_token_stream(), source.scope.fmt_short());
//
//         let maybe_obj = source.maybe_object_ref_by_key_in_scope(ScopeSearchKey::Type(self.ty.clone(), None), &source.scope);
//         let maybe_special: Option<SpecialType<ObjCSpecification>> = full_ty.maybe_resolve(source);
//         let result = match maybe_special {
//             Some(special) => match maybe_obj {
//                 Some(ObjectKind::Item(_fn_ty_conversion, ScopeItemKind::Fn(..))) => {
//                     // println!("VariableComposer (Special Function): {} in {}", fn_ty_conversion.to_token_stream(), source.scope.fmt_short());
//                     let ty = match &source.scope.parent_object().unwrap() {
//                         ObjectKind::Type(ref ty_model_kind) |
//                         ObjectKind::Item(ref ty_model_kind, ..) => {
//                             let parent_scope = source.scope.parent_scope().unwrap();
//                             // println!("VariableComposer (Special Function Parent TYC): {} in {}", ty_model_kind, parent_scope.fmt_short());
//                             let context = source.context.read().unwrap();
//                             context.maybe_scope_ref_obj_first(parent_scope.self_path())
//                                 .and_then(|parent_obj_scope| {
//                                     // println!("VariableComposer (Special Function Parent OBJ SCOPE): {}", parent_obj_scope.fmt_short());
//                                     context.maybe_object_ref_by_tree_key(ty_model_kind.as_type(), parent_obj_scope)
//                                         .and_then(|o| {
//                                             // println!("VariableComposer (Special Function Parent OBJ FULL): {} in {}", o, o.maybe_type().to_token_stream());
//                                             o.maybe_type()
//                                         })
//                                 }).unwrap_or_else(|| parent_scope.to_type())
//                         },
//                         _ => {
//                             // println!("VariableComposer (Special Function Unknown TYC): {} in {}", self.ty.to_token_stream(), source.scope.fmt_short());
//                             self.ty.clone()
//                         }
//                     };
//                     if is_const_ptr {
//                         FFIVariable::const_ptr(ty.to_token_stream())
//                     } else {
//                         FFIVariable::mut_ptr(ty.to_token_stream())
//                     }
//                 }
//                 Some(ObjectKind::Item(TypeModelKind::FnPointer(..), ..) |
//                      ObjectKind::Type(TypeModelKind::FnPointer(..), ..)) => {
//                     // println!("VariableComposer (Special FnPointer): {}", special.to_token_stream());
//                     FFIVariable::direct(special.to_token_stream())
//                 }
//                 Some(ObjectKind::Item(TypeModelKind::Trait(..), ..) |
//                      ObjectKind::Type(TypeModelKind::TraitType(..), ..) |
//                      ObjectKind::Type(TypeModelKind::Dictionary(DictTypeModelKind::LambdaFn(..)), ..)) => {
//                     // println!("VariableComposer (Special Trait): {}", special.to_token_stream());
//                     let ty = special.to_token_stream();
//                     // let ty = parse_quote!(dyn #ty);
//                     if is_const_ptr {
//                         FFIVariable::const_ptr(ty)
//                     } else {
//                         FFIVariable::mut_ptr(ty)
//                     }
//
//                 },
//                 Some(ObjectKind::Type(TypeModelKind::Bounds(bounds))) => {
//                     // println!("VariableComposer (Bounds): {}", bounds);
//                     bounds.resolve(source)
//                 },
//                 _ => {
//                     // println!("VariableComposer (Special MutPtr): {}", special.to_token_stream());
//                     let ty = special.to_token_stream();
//                     if is_const_ptr {
//                         FFIVariable::const_ptr(ty)
//                     } else {
//                         FFIVariable::mut_ptr(ty)
//                     }
//
//                 }
//             }
//             None => {
//                 // println!("VariableComposer (NonSpecial): {} in {}", full_ty.to_token_stream(), source.scope.fmt_short());
//                 match maybe_obj {
//                     Some(ObjectKind::Item(_fn_ty_conversion, ScopeItemKind::Fn(..))) => {
//                         // println!("VariableComposer (Function): {} in {}", fn_ty_conversion.to_token_stream(), source.scope.fmt_short());
//                         let ty = match &source.scope.parent_object().unwrap() {
//                             ObjectKind::Type(ref ty_conversion) |
//                             ObjectKind::Item(ref ty_conversion, ..) => {
//                                 let full_parent_ty: Type = Resolve::resolve(ty_conversion.as_type(), source);
//                                 // println!("VariableComposer (Function Parent): {} ({}) in {}", ty_conversion.to_token_stream(), full_parent_ty.to_token_stream(), source.scope.fmt_short());
//                                 match Resolve::<SpecialType<ObjCSpecification>>::maybe_resolve(&full_parent_ty, source) {
//                                     Some(special) => special.to_type(),
//                                     None => {
//                                         match ty_conversion {
//                                             TypeModelKind::Trait(model) =>
//                                                 model.as_type()
//                                                     .maybe_trait_object(source)
//                                                     .and_then(|oc| oc.maybe_type_model_kind_ref().map(|c| c.to_type()))
//                                                     .unwrap_or_else(|| ty_conversion.to_type()),
//                                             _ => ty_conversion.to_type()
//                                         }
//                                     }
//                                 }
//                             },
//                             _ => self.ty.clone()
//                         };
//                         if is_const_ptr {
//                             FFIVariable::const_ptr(ty.to_token_stream())
//                         } else {
//                             FFIVariable::mut_ptr(ty.to_token_stream())
//                         }
//                     },
//                     Some(ObjectKind::Type(ref ty_model_kind)) |
//                     Some(ObjectKind::Item(ref ty_model_kind, ..)) => {
//                         let conversion = ty_model_kind.maybe_trait_object_maybe_model_kind_or_same(source);
//                         match conversion {
//                             TypeModelKind::Dictionary(DictTypeModelKind::NonPrimitiveFermentable(DictFermentableModelKind::SmartPointer(SmartPointerModelKind::Box(model)))) => {
//                                 // println!("VariableComposer (Boxed kind): {}", kind);
//                                 // let nested_ty = ty.first_nested_type().unwrap();
//                                 let ty = model.as_type();
//                                 let nested_ty = self.ty.maybe_first_nested_type_ref().unwrap();
//                                 let full_nested_ty = ty.maybe_first_nested_type_ref().unwrap();
//                                 match Resolve::<SpecialType<ObjCSpecification>>::maybe_resolve(full_nested_ty, source) {
//                                     Some(special) => {
//                                         // println!("VariableComposer (Special Boxed kind): Nested Type: {}", special.to_token_stream());
//                                         match source.maybe_object_by_key(nested_ty) {
//                                             Some(ObjectKind::Item(TypeModelKind::FnPointer(..), ..) |
//                                                  ObjectKind::Type(TypeModelKind::FnPointer(..), ..)) => {
//                                                 // println!("VariableComposer (Special Boxed kind): Nested Special FnPointer: {}", nested_ty.to_token_stream());
//                                                 FFIVariable::direct(special.to_token_stream())
//                                             }
//                                             Some(ObjectKind::Item(TypeModelKind::Trait(..), ..) |
//                                                  ObjectKind::Type(TypeModelKind::TraitType(..), ..) |
//                                                  ObjectKind::Type(TypeModelKind::Dictionary(DictTypeModelKind::LambdaFn(..)), ..)) => {
//                                                 // println!("VariableComposer (Special Boxed kind): Nested Special Trait: {}", nested_ty.to_token_stream());
//                                                 let ty = special.to_type();
//                                                 // let ty = parse_quote!(dyn #ty);
//                                                 if is_const_ptr {
//                                                     FFIVariable::const_ptr(ty.to_token_stream())
//                                                 } else {
//                                                     FFIVariable::mut_ptr(ty.to_token_stream())
//                                                 }
//
//                                             },
//                                             _ => {
//                                                 // println!("VariableComposer (Boxed kind): Nested Special MutPtr: {}", nested_ty.to_token_stream());
//                                                 let ty = special.to_type();
//                                                 if is_const_ptr {
//                                                     FFIVariable::const_ptr(ty.to_token_stream())
//                                                 } else {
//                                                     FFIVariable::mut_ptr(ty.to_token_stream())
//                                                 }
//
//                                             }
//                                         }
//                                     }
//                                     None => {
//                                         let object = Resolve::<ObjectKind>::maybe_resolve(nested_ty, source);
//                                         // println!("VariableComposer (Nested Boxed Type Conversion (Object?)): {}", object.as_ref().map_or("None".to_string(), |o| format!("{}", o)));
//                                         let var_ty = object.and_then(|object_kind| object_kind.maybe_fn_or_trait_or_same_kind2(source))
//                                             .unwrap_or_else(|| TypeModelKind::unknown_type_ref(nested_ty));
//                                         let var_c_type = var_ty.to_type();
//                                         let ffi_path: Option<FFIFullPath<ObjCSpecification>> = var_c_type.maybe_resolve(source);
//                                         let var_ty = ffi_path.map(|p| p.to_type()).unwrap_or_else(|| parse_quote!(#var_c_type));
//                                         let result = resolve_type_variable(var_ty, source);
//
//                                         // let result = resolve_type_variable(var_ty.to_type(), source);
//
//                                         result
//                                     }
//                                 }
//                             }
//                             TypeModelKind::Unknown(TypeModel { ty, .. }) => {
//                                 // println!("VariableComposer (Unknown): {}", ty.to_token_stream());
//                                 FFIVariable::mut_ptr(ty.to_token_stream())
//                             },
//                             TypeModelKind::Dictionary(DictTypeModelKind::Primitive(TypeModel { ty, .. })) => {
//                                 // println!("VariableComposer (Dictionary Primitive): {}", ty.to_token_stream());
//
//                                 FFIVariable::direct(objc_primitive(&ty))
//                             },
//                             TypeModelKind::FnPointer(TypeModel { ty, .. }, ..) => {
//                                 // println!("VariableComposer (FnPointer Conversion): {}", ty.to_token_stream());
//                                 let result = FFIVariable::direct(
//                                     Resolve::<SpecialType<ObjCSpecification>>::maybe_resolve(&ty, source)
//                                         .map(|special| special.to_token_stream())
//                                         .unwrap_or_else(|| Resolve::<FFIFullPath<ObjCSpecification>>::resolve(&ty, source)
//                                             .to_token_stream())
//                                 );
//                                 // println!("VariableComposer (FnPointer Variable): {}", result.to_token_stream());
//                                 result
//                             },
//                             TypeModelKind::Dictionary(DictTypeModelKind::LambdaFn(TypeModel { ty, .. }, ..)) => {
//                                 // println!("VariableComposer (LambdaFn Conversion): {}", ty.to_token_stream());
//                                 let result = FFIVariable::mut_ptr(
//                                     Resolve::<FFIFullPath<ObjCSpecification>>::resolve(&ty, source).to_token_stream());
//                                 // println!("VariableComposer (LambdaFn Variable): {}", result.to_token_stream());
//                                 result
//                             },
//                             // TypeModelKind::Dictionary(DictTypeModelKind::NonPrimitiveFermentable(TypeModel { ty, .. })) => {
//                             TypeModelKind::Dictionary(
//                                 DictTypeModelKind::NonPrimitiveFermentable(
//                                     DictFermentableModelKind::SmartPointer(
//                                         SmartPointerModelKind::Arc(TypeModel { ty, .. }) |
//                                         SmartPointerModelKind::Rc(TypeModel { ty, .. }) |
//                                         SmartPointerModelKind::Mutex(TypeModel { ty, .. }) |
//                                         SmartPointerModelKind::OnceLock(TypeModel { ty, .. }) |
//                                         SmartPointerModelKind::RwLock(TypeModel { ty, .. }) |
//                                         SmartPointerModelKind::Cell(TypeModel { ty, .. }) |
//                                         SmartPointerModelKind::RefCell(TypeModel { ty, .. }) |
//                                         SmartPointerModelKind::UnsafeCell(TypeModel { ty, .. }) |
//                                         SmartPointerModelKind::Pin(TypeModel { ty, .. })
//                                     ) |
//                                     DictFermentableModelKind::Group(
//                                         GroupModelKind::BTreeSet(TypeModel { ty, .. }) |
//                                         GroupModelKind::HashSet(TypeModel { ty, .. }) |
//                                         GroupModelKind::Map(TypeModel { ty, .. }) |
//                                         GroupModelKind::Result(TypeModel { ty, .. }) |
//                                         GroupModelKind::Vec(TypeModel { ty, .. }) |
//                                         GroupModelKind::IndexMap(TypeModel { ty, .. }) |
//                                         GroupModelKind::IndexSet(TypeModel { ty, .. })
//                                     ) |
//                                     DictFermentableModelKind::Other(TypeModel { ty, .. }) |
//                                     // DictFermentableModelKind::I128(TypeModel { ty, .. }) |
//                                     // DictFermentableModelKind::U128(TypeModel { ty, .. }) |
//                                     DictFermentableModelKind::Str(TypeModel { ty, .. }) |
//                                     DictFermentableModelKind::String(TypeModel { ty, .. }))) => {
//                                 // Dictionary generics and strings should be fermented
//                                 // Others should be treated as opaque
//                                 // println!("VariableComposer (Dictionary NonPrimitiveFermentable Conversion): {}", ty.to_token_stream());
//                                 let maybe_ffi_full_path: Option<FFIFullPath<ObjCSpecification>> = ty.maybe_resolve(source);
//                                 // println!("VariableComposer (Dictionary NonPrimitiveFermentable Conversion FFIFULLPATH?): {}", maybe_ffi_full_path.to_token_stream());
//                                 let result = resolve_type_variable(maybe_ffi_full_path.map(|path| path.to_type()).unwrap_or_else(|| parse_quote!(#ty)), source);
//                                 // println!("VariableComposer (Dictionary NonPrimitiveFermentable Variable): {}", result.to_token_stream());
//                                 result
//                             },
//                             TypeModelKind::Dictionary(DictTypeModelKind::NonPrimitiveFermentable(DictFermentableModelKind::I128(..))) =>
//                                 FFIVariable::mut_ptr(parse_quote!(NSData)),
//                             TypeModelKind::Dictionary(DictTypeModelKind::NonPrimitiveFermentable(DictFermentableModelKind::U128(..))) =>
//                                 FFIVariable::mut_ptr(parse_quote!(NSData)),
//                             TypeModelKind::Dictionary(DictTypeModelKind::NonPrimitiveOpaque(..)) => {
//                                 // Dictionary generics should be fermented
//                                 // Others should be treated as opaque
//                                 // println!("VariableComposer (Dictionary NonPrimitiveOpaque Conversion): {}", kind.to_token_stream());
//                                 let result = Resolve::<FFIVariable<ObjCSpecification, TokenStream2>>::resolve(&conversion, source);
//                                 // println!("VariableComposer (Dictionary NonPrimitiveOpaque Variable): {}", result.to_token_stream());
//                                 result
//                             },
//                             TypeModelKind::Bounds(bounds) => {
//                                 bounds.resolve(source)
//                             }
//
//                             ref cnv => {
//                                 // println!("VariableComposer (Regular Fermentable Conversion): {}", kind);
//                                 // let result: FFIVariable = kind.resolve(source);
//                                 // let conversion_ty = kind.ty();
//                                 let object = Resolve::<ObjectKind>::maybe_resolve(&self.ty, source);
//                                 // println!("VariableComposer (Regular Fermentable Conversion (Object?)): {}", object.as_ref().map_or("None".to_string(), |o| format!("{}", o)));
//                                 let var_ty = match object {
//                                     Some(ObjectKind::Item(.., ScopeItemKind::Fn(..))) =>
//                                         source.maybe_parent_trait_or_regular_model_kind(),
//                                     Some(ObjectKind::Type(..) |
//                                          ObjectKind::Item(..)) =>
//                                         cnv.maybe_trait_object_maybe_model_kind(source)
//                                             .unwrap_or_else(|| Some(cnv.clone())),
//                                     _ => None,
//                                 }.unwrap_or_else(|| cnv.clone());
//                                 let var_c_type = var_ty.to_type();
//                                 let ffi_path: Option<FFIFullPath<ObjCSpecification>> = var_c_type.maybe_resolve(source);
//                                 let var_ty = ffi_path.map(|p| p.to_type()).unwrap_or_else(|| parse_quote!(#var_c_type));
//                                 let result = resolve_type_variable(var_ty, source);
//
//                                 // println!("VariableComposer (Regular Fermentable Variable): {}", result.to_token_stream());
//                                 result
//                             }
//                         }
//                     }
//                     _ => {
//                         //println!("UNKNOWN TOTALLY: {}", self.ty.to_token_stream());
//                         // FFIVariable::mut_ptr(self.ty.clone())
//                         Resolve::<SpecialType<ObjCSpecification>>::maybe_resolve(&self.ty, source)
//                             .map(FFIFullPath::from)
//                             .or_else(|| source.maybe_ffi_full_path(&self.ty))
//                             .map(|ffi_path| ffi_path.to_type())
//                             .unwrap_or_else(|| self.ty.clone())
//                             .resolve(source)
//
//                     }
//                 }
//
//                 // let kind = Resolve::<TypeModelKind>::resolve(&self.ty, source);
//
//             }
//         };
//         // println!("VariableComposer (compose) RESULT: {}", result.to_token_stream());
//         result
//     }
// }

pub fn resolve_type_variable(ty: Type, source: &ScopeContext) -> FFIVariable<ObjCSpecification, TokenStream2> {
    match ty {
        Type::Path(TypePath { path, .. }) =>
            path.resolve(source),
        Type::Array(TypeArray { elem, len, .. }) => {
            FFIVariable::direct(quote!(#elem (*)[#len]))
        },
        Type::Reference(TypeReference { elem, .. }) |
        Type::Slice(TypeSlice { elem, .. }) =>
            elem.resolve(source),
        Type::Ptr(TypePtr { const_token, mutability, elem, .. }) =>
            match *elem {
                Type::Path(TypePath { path, .. }) => if let Some(PathSegment { ident, .. }) = path.segments.last() {
                    let ty = if ident.is_void() {
                        quote!(void)
                    } else {
                        path.to_token_stream()
                    };
                    if const_token.is_some() {
                        FFIVariable::const_ptr(ty)
                    } else {
                        FFIVariable::mut_ptr(ty)
                    }
                } else {
                    FFIVariable::mut_ptr(path.to_token_stream())
                }
                Type::Ptr(..) =>
                    FFIVariable::mut_ptr(elem.to_token_stream()),
                ty if mutability.is_some() =>
                    FFIVariable::mut_ptr(ty.to_token_stream()),
                ty =>
                    FFIVariable::const_ptr(ty.to_token_stream()),
            },
        Type::TraitObject(TypeTraitObject { bounds, .. }) |
            Type::ImplTrait(TypeImplTrait { bounds, .. }) => {
            let maybe_bound = bounds.iter().find_map(|bound| match bound {
                TypeParamBound::Trait(TraitBound { path, .. }) => Some(path.to_type()),
                _ => None
            });
            maybe_bound.map(|bound| bound.resolve(source))
                .unwrap_or_else(|| FFIVariable::mut_ptr(bounds.to_token_stream()))
        }
        ty => FFIVariable::direct(ty.mangle_tokens_default())
    }
}
#[allow(unused)]
pub fn resolve_target_variable(ty: Type, source: &ScopeContext) -> FFIVariable<ObjCSpecification, TokenStream2> {
    //println!("resolve_type_variable: {}", ty.to_token_stream());
    match ty {
        Type::Path(TypePath { path, .. }) =>
            path.resolve(source),
        Type::Array(TypeArray { elem, len, .. }) => {
            FFIVariable::direct(quote!(#elem (*)[#len]))
        },
        Type::Reference(TypeReference { elem, .. }) |
        Type::Slice(TypeSlice { elem, .. }) =>
            elem.resolve(source),
        Type::Ptr(TypePtr { star_token, const_token, mutability, elem }) =>
            match *elem {
                Type::Path(TypePath { path, .. }) => match path.segments.last() {
                    Some(last_segment) => {
                        let ty = if last_segment.ident.eq("c_void") {
                            quote!(void)
                        } else {
                            path.to_token_stream()
                        };
                        if const_token.is_some() {
                            FFIVariable::const_ptr(ty)
                        } else {
                            FFIVariable::mut_ptr(ty)
                        }
                    },
                    _ => FFIVariable::mut_ptr(path.to_token_stream())
                },
                Type::Ptr(..) =>
                    FFIVariable::mut_ptr(elem.to_token_stream()),
                ty if mutability.is_some() =>
                    FFIVariable::mut_ptr(ty.to_token_stream()),
                ty =>
                    FFIVariable::const_ptr(ty.to_token_stream()),
            },
        Type::TraitObject(TypeTraitObject { dyn_token: _, bounds, .. }) |
        Type::ImplTrait(TypeImplTrait { impl_token: _, bounds, .. }) =>
            bounds.iter().find_map(|bound| match bound {
                TypeParamBound::Trait(TraitBound { path, .. }) => Some(path.to_type()),
                _ => None
            })
                .map(|bound| bound.resolve(source))
                .unwrap_or_else(|| FFIVariable::mut_ptr(bounds.to_token_stream())),
        ty => FFIVariable::direct(ty.mangle_tokens_default())
    }
}

pub fn objc_primitive(ty: &Type) -> TokenStream2 {
    match ty {
        Type::Path(TypePath { ref path , ..}) =>
            objc_primitive_from_path(path),
        ty => ty.to_token_stream()
    }
}

pub fn objc_primitive_from_path(path: &Path) -> TokenStream2 {
    match path.segments.last() {
        None => path.to_token_stream(),
        Some(PathSegment { ident, .. }) => match ident.to_string().as_str() {
            "i8" => quote!(int8_t),
            "u8" => quote!(uint8_t),
            "i16" => quote!(int16_t),
            "u16" => quote!(uint16_t),
            "i32" => quote!(int32_t),
            "u32" => quote!(uint32_t),
            "i64" => quote!(int32_t),
            "u64" => quote!(uint32_t),
            "f64" => quote!(double),
            "isize" => quote!(intptr_t),
            "usize" => quote!(uintptr_t),
            "bool" => quote!(BOOL),
            _ => path.to_token_stream()
        }
    }

}


// impl<SPEC> Resolve<SPEC::Var> for TypeModelKind where SPEC: ObjCSpecification {
//     fn resolve(&self, source: &ScopeContext) -> SPEC::Var {
//         let result = match self  {
//             // TODO: For now we assume that every callback defined as fn pointer is opaque
//             TypeModelKind::FnPointer(TypeModel { ty, .. }, ..) => FFIVariable::Direct {
//                 ty: Resolve::<Option<SpecialType>>::resolve(ty, source)
//                     .map(|special| special.to_token_stream())
//                     .unwrap_or(Resolve::<FFIFullPath<ObjCFermentate, SPEC>>::resolve(ty, source)
//                         .to_token_stream())
//             },
//             TypeModelKind::Dictionary(DictTypeModelKind::LambdaFn(TypeModel { ty, .. }, ..)) => FFIVariable::MutPtr {
//                 ty: Resolve::<FFIFullPath<ObjCFermentate, SPEC>>::resolve(ty, source).to_token_stream()
//             },
//             TypeModelKind::Dictionary(DictTypeModelKind::Primitive(composition)) => FFIVariable::Direct {
//                 ty: composition.to_type().to_token_stream()
//             },
//             TypeModelKind::Dictionary(DictTypeModelKind::NonPrimitiveFermentable(DictFermentableModelKind::SmartPointer(SmartPointerModelKind::Box(TypeModel { ty, .. })))) => {
//                 // println!("TypeModelKind::Boxed: {}", ty.to_token_stream());
//                 match ty.first_nested_type() {
//                     Some(nested_full_ty) => {
//                         // println!("Nested: {}", nested_full_ty.to_token_stream());
//                         resolve_type_variable(match Resolve::<Option<SpecialType>>::resolve(nested_full_ty, source) {
//                             Some(special) => special.to_type(),
//                             None => {
//                                 let kind = Resolve::<TypeModelKind>::resolve(nested_full_ty, source);
//                                 Resolve::<Option<FFIFullPath<ObjCFermentate, SPEC>>>::resolve(&kind.to_type(), source)
//                                     .map(|full_path| full_path.to_type())
//                                     .unwrap_or_else(|| nested_full_ty.clone())
//                             }
//                         }, source)
//                     }
//                     None => panic!("error: Arg kind ({}) not supported", ty.to_token_stream())
//                 }
//             },
//             TypeModelKind::Dictionary(
//                 DictTypeModelKind::NonPrimitiveFermentable(
//                     DictFermentableModelKind::SmartPointer(
//                         SmartPointerModelKind::Arc(TypeModel { ty, .. }) |
//                         SmartPointerModelKind::Mutex(TypeModel { ty, .. }) |
//                         SmartPointerModelKind::Rc(TypeModel { ty, .. }) |
//                         SmartPointerModelKind::RefCell(TypeModel { ty, .. }) |
//                         SmartPointerModelKind::RwLock(TypeModel { ty, .. }) |
//                         SmartPointerModelKind::Pin(TypeModel { ty, .. })
//                     ) |
//                     DictFermentableModelKind::Group(
//                         GroupModelKind::BTreeSet(TypeModel { ty, .. }) |
//                         GroupModelKind::HashSet(TypeModel { ty, .. }) |
//                         GroupModelKind::Map(TypeModel { ty, .. }) |
//                         GroupModelKind::Result(TypeModel { ty, .. }) |
//                         GroupModelKind::Vec(TypeModel { ty, .. }) |
//                         GroupModelKind::IndexMap(TypeModel { ty, .. })
//                     ) |
//                     DictFermentableModelKind::Str(TypeModel { ty, .. }) |
//                     DictFermentableModelKind::String(TypeModel { ty, .. }) |
//                     DictFermentableModelKind::Digit128(TypeModel { ty, .. }) |
//                     DictFermentableModelKind::Other(TypeModel { ty, .. })
//                 ) |
//                 DictTypeModelKind::NonPrimitiveOpaque(TypeModel { ty, .. })
//             ) |
//             TypeModelKind::Trait(TypeModel { ty, .. }, ..) |
//             TypeModelKind::TraitType(TypeModel { ty, .. }) |
//             TypeModelKind::Object(TypeModel { ty, .. }) |
//             TypeModelKind::Optional(TypeModel { ty, .. }) |
//             TypeModelKind::Array(TypeModel { ty, .. }) |
//             TypeModelKind::Slice(TypeModel { ty, .. }) |
//             TypeModelKind::Tuple(TypeModel { ty, .. }) |
//             TypeModelKind::Unknown(TypeModel { ty, .. })  => {
//                 Resolve::<Option<SpecialType>>::resolve(ty, source)
//                     .map(|ty| resolve_type_variable(FFIFullPath::External { path: ty.to_path() }.to_type(), source))
//                     .unwrap_or_else(|| {
//                         resolve_type_variableResolve::<Option<ObjectKind>>::resolve(ty, source)
//                                                   .and_then(|external_type| {
//                                                       match external_type {
//                                                           ObjectKind::Item(.., ScopeItemKind::Fn(..)) => {
//                                                               let parent_object = &source.scope.parent_object().unwrap();
//                                                               match parent_object {
//                                                                   ObjectKind::Type(ref ty_conversion) |
//                                                                   ObjectKind::Item(ref ty_conversion, ..) => {
//                                                                       match ty_conversion {
//                                                                           TypeModelKind::Trait(ty, _decomposition, _super_bounds) => {
//                                                                               ty.as_type().maybe_trait_object_maybe_model_kind(source)
//                                                                           },
//                                                                           _ => {
//                                                                               None
//                                                                           },
//                                                                       }.unwrap_or_else(|| {
//                                                                           parent_object.maybe_type_model_kind_ref().cloned()
//                                                                       })
//                                                                   },
//                                                                   ObjectKind::Empty => {
//                                                                       None
//                                                                   }
//                                                               }
//                                                           },
//                                                           ObjectKind::Type(ref ty_conversion) |
//                                                           ObjectKind::Item(ref ty_conversion, ..) => {
//                                                               match ty_conversion {
//                                                                   TypeModelKind::Trait(ty, _decomposition, _super_bounds) => {
//                                                                       // println!("Type::<TypeModelKind> It's a Trait So --> {}", external_type.type_conversion().to_token_stream());
//                                                                       ty.as_type().maybe_trait_object_maybe_model_kind(source)
//                                                                   },
//                                                                   _ => {
//                                                                       None
//                                                                   },
//                                                               }.unwrap_or_else(|| {
//                                                                   // println!("Type::<TypeModelKind> Not a Trait So --> {}", external_type.type_conversion().to_token_stream());
//                                                                   external_type.maybe_type_model_kind_ref().cloned()
//                                                               })
//                                                           },
//                                                           ObjectKind::Empty => {
//                                                               // println!("Type::<TypeModelKind> Has no object --> {}", external_type.type_conversion().to_token_stream());
//                                                               None
//                                                           }
//                                                       }
//                                                   })
//                                                   .unwrap_or_else(|| {
//                                                       TypeModelKind::Unknown(TypeModel::new(ty.clone(), None, Punctuated::new()))
//                                                   }).to_type(), source)
//                     })
//             },
//             TypeModelKind::Fn(TypeModel { ty, .. }, ..) => {
//                 // ty.to_path().popped()
//                 panic!("error: Arg kind (Fn) ({}) not supported", ty.to_token_stream())
//             },
//
//             TypeModelKind::Bounds(bounds) => {
//                 // println!("TypeModelKind::Bounds: {}", bounds);
//                 bounds.resolve(source)
//             },
//             ty =>
//                 panic!("error: Arg kind ({}) not supported", ty),
//         };
//         // println!("TypeModelKind::<FFIVariable>::resolve.2({}) --> {}", self, result.to_token_stream());
//         result
//     }
// }
// impl<SPEC> Resolve<SPEC::Var> for Path where SPEC: ObjCSpecification {
//     fn resolve(&self, source: &ScopeContext) -> SPEC::Var {
//         // println!("Path::<FFIVariable>::resolve({})", self.to_token_stream());
//         let first_segment = self.segments.first().unwrap();
//         let first_ident = &first_segment.ident;
//         let last_segment = self.segments.last().unwrap();
//         let last_ident = &last_segment.ident;
//         if last_ident.is_primitive() {
//             SPEC::Var::direct(self.to_type())
//         } else if last_ident.is_optional() {
//             match path_arguments_to_type_conversions(&last_segment.arguments).first() {
//                 Some(TypeKind::Primitive(ty)) => SPEC::Var::mut_ptr(ty.clone()),
//                 Some(TypeKind::Generic(generic_ty)) => FFIVariable::MutPtr {
//                     ty: Resolve::<FFIFullPath<ObjCFermentate, SPEC>>::resolve(generic_ty, source).to_token_stream()
//                 },
//                 Some(TypeKind::Complex(Type::Path(TypePath { path, .. }))) =>
//                     path.resolve(source),
//                 _ => unimplemented!("ffi_dictionary_variable_type:: Empty Optional")
//             }
//         } else if last_ident.is_special_generic() || (last_ident.is_result() /*&& path.segments.len() == 1*/) || (last_ident.to_string().eq("Map") && first_ident.to_string().eq("serde_json")) {
//             FFIVariable::MutPtr {
//                 ty: source.scope_type_for_path(self)
//                     .map_or(self.to_token_stream(), |full_type| full_type.mangle_tokens_default())
//             }
//         } else {
//             FFIVariable::MutPtr {
//                 ty: self.to_token_stream()
//             }
//         }
//     }
// }
// impl Resolve<FFIVariable<TokenStream2>> for Type {
//     fn resolve(&self, source: &ScopeContext) -> FFIVariable<TokenStream2> {
//         // println!("Type::<FFIVariable>::resolve.1({})", self.to_token_stream());
//         let full_ty = Resolve::<Type>::resolve(self, source);
//         // println!("Type::<FFIVariable>::resolve.2({})", full_ty.to_token_stream());
//         let maybe_special = Resolve::<Option<SpecialType>>::resolve(&full_ty, source);
//         // println!("Type::<FFIVariable>::resolve.3({})", maybe_special.to_token_stream());
//         let refined = maybe_special
//             .map(|ty| FFIFullPath::External { path: ty.to_path() })
//             .or(Resolve::<TypeModelKind>::resolve(self, source)
//                 .to_type()
//                 .resolve(source))
//             .map(|ffi_path| ffi_path.to_type())
//             .unwrap_or(parse_quote!(#self))
//             .to_type();
//         resolve_type_variable(refined, source)
//     }
// }



impl Resolve<FFIFullPath<ObjCSpecification>> for GenericTypeKind {
    fn maybe_resolve(&self, source: &ScopeContext) -> Option<FFIFullPath<ObjCSpecification>> {
        Some(self.resolve(source))
    }
    fn resolve(&self, source: &ScopeContext) -> FFIFullPath<ObjCSpecification> {
        match self {
            GenericTypeKind::Map(ty) |
            GenericTypeKind::Group(ty) |
            GenericTypeKind::Result(ty) |
            GenericTypeKind::Box(ty) |
            GenericTypeKind::AnyOther(ty) =>
                single_generic_ffi_full_path(ty),
            GenericTypeKind::Array(ty) |
            GenericTypeKind::Slice(ty) =>
                FFIFullPath::generic(ty.mangle_ident_default().to_path()),
            GenericTypeKind::Callback(kind) =>
                FFIFullPath::generic(kind.as_type().mangle_ident_default().to_path()),
            GenericTypeKind::Tuple(Type::Tuple(tuple)) => match tuple.elems.len() {
                0 => FFIFullPath::void(),
                1 => single_generic_ffi_full_path(tuple.elems.first().unwrap()),
                _ => FFIFullPath::generic(tuple.mangle_ident_default().to_path())
            }
            GenericTypeKind::Optional(Type::Path(TypePath { path: Path { segments, .. }, .. })) => match segments.last() {
                Some(PathSegment { arguments: PathArguments::AngleBracketed(AngleBracketedGenericArguments { args, .. }), .. }) => match args.first() {
                    Some(GenericArgument::Type(ty)) => match TypeKind::from(ty) {
                        TypeKind::Generic(gen) => gen.resolve(source),
                        _ => single_generic_ffi_full_path(ty),
                    },
                    _ => panic!("TODO: Non-supported optional type as generic argument (PathArguments::AngleBracketed: Empty): {}", segments.to_token_stream()),
                },
                Some(PathSegment { arguments: PathArguments::Parenthesized(args), .. }) =>
                    FFIFullPath::generic(args.mangle_ident_default().to_path()),
                _ => unimplemented!("TODO: Non-supported optional type as generic argument (Empty last segment): {}", segments.to_token_stream()),
            },
            GenericTypeKind::Optional(Type::Array(TypeArray { elem, .. })) =>
                single_generic_ffi_full_path(elem),
            GenericTypeKind::TraitBounds(bounds) => {
                println!("GenericTypeKind (TraitBounds): {}", bounds.to_token_stream());
                match bounds.len() {
                    1 => if let Some(TypeParamBound::Trait(trait_bound)) = bounds.first() {
                        let ty = trait_bound.path.to_type();
                        let maybe_special: Option<SpecialType<ObjCSpecification>> = ty.maybe_special_type(source);
                        match maybe_special {
                            Some(SpecialType::Opaque(..) | SpecialType::Custom(..)) =>
                                FFIFullPath::external(trait_bound.path.clone()),
                            _ =>
                                FFIFullPath::generic(trait_bound.mangle_ident_default().to_path())
                        }
                    } else {
                        FFIFullPath::generic(bounds.mangle_ident_default().to_path())
                    }
                    _ => FFIFullPath::generic(bounds.mangle_ident_default().to_path())
                }
            },
            gen_ty =>
                unimplemented!("TODO: TraitBounds when generic expansion: {}", gen_ty),
        }
    }
}

fn single_generic_ffi_full_path(ty: &Type) -> FFIFullPath<ObjCSpecification> {
    let path: Path = parse_quote!(#ty);
    let first_segment = path.segments.first().unwrap();
    let mut cloned_segments = path.segments.clone();
    let first_ident = &first_segment.ident;
    let last_segment = cloned_segments.iter_mut().last().unwrap();
    let last_ident = &last_segment.ident;
    if last_ident.is_primitive() {
        FFIFullPath::external(last_ident.to_path())
    } else if last_ident.is_any_string() {
        FFIFullPath::c_char()
    } else if last_ident.is_special_generic() ||
        (last_ident.is_result() && path.segments.len() == 1) ||
        // TODO: avoid this hardcode
        (last_ident.eq("Map") && first_ident.eq("serde_json")) ||
        last_ident.is_smart_ptr() ||
        last_ident.is_lambda_fn() {
        FFIFullPath::generic(path.mangle_ident_default().to_path())
    } else {
        let new_segments = cloned_segments
            .into_iter()
            .map(|segment| quote_spanned! { segment.span() => #segment })
            .collect::<Vec<_>>();
        FFIFullPath::external(parse_quote!(#(#new_segments)::*))
    }
}
impl ToType for FFIFullDictionaryPath<ObjCSpecification> {
    fn to_type(&self) -> Type {
        match self {
            FFIFullDictionaryPath::Void => parse_quote!(void),
            // FFIFullDictionaryPath::CChar => parse_quote!(char),
            FFIFullDictionaryPath::CChar => parse_quote!(NSString),
            FFIFullDictionaryPath::Phantom(_) => panic!("")
        }
    }
}

impl ToType for FFIFullPath<ObjCSpecification> {
    fn to_type(&self) -> Type {
        let prefix = "DS";
        match self {
            FFIFullPath::Type { ffi_name, .. } | FFIFullPath::Generic { ffi_name, .. } | FFIFullPath::External { path: ffi_name, .. } =>
                format_ident!("{prefix}{}", ffi_name.mangle_tokens_default().to_string()).to_type(),
            FFIFullPath::Dictionary { path } =>
                path.to_type(),
        }
    }
}
