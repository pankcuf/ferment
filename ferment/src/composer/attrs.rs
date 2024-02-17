use std::rc::Rc;
use std::cell::RefCell;
use quote::{format_ident, quote, ToTokens};
use syn::{ItemTrait, parse_quote};
use crate::composer::item::ItemComposer;
use crate::composer_impl;
use crate::composition::{AttrsComposition, FnReturnTypeComposition, TraitDecompositionPart2, TraitVTableMethodComposition, TypeComposition};
use crate::conversion::TypeConversion;
use crate::context::{ScopeChain, ScopeContext};
use crate::holder::EMPTY;
use crate::interface::ROUND_BRACES_FIELDS_PRESENTER;
use crate::naming::Name;
use crate::presentation::context::{IteratorPresentationContext, OwnedItemPresenterContext};
use crate::presentation::{BindingPresentation, FFIObjectPresentation, ScopeContextPresentable, TraitVTablePresentation};

pub struct AttrsComposer {
    pub parent: Option<Rc<RefCell<ItemComposer>>>,
    pub attrs: AttrsComposition
}

impl AttrsComposer {
    pub const fn new(attrs: AttrsComposition) -> Self {
        Self { parent: None, attrs }
    }
}

composer_impl!(AttrsComposer, Vec<TraitVTablePresentation>, |itself: &AttrsComposer, context: &ScopeContext| {
    let mut trait_types = context.trait_items_from_attributes(&itself.attrs.attrs);
    trait_types.iter_mut()
        .map(|(composition, trait_scope)| {
            // TODO: move to full
            let conversion = TypeConversion::Object(TypeComposition::new(context.scope.to_type(), Some(composition.item.generics.clone())));
            println!("AttrsComposer: {} {} {}", composition.item.ident, trait_scope, conversion);
            composition.implementors.push(conversion);
            implement_trait_for_item((&composition.item, trait_scope), &itself.attrs, context)
        })
        .collect()
});

pub fn implement_trait_for_item(item_trait: (&ItemTrait, &ScopeChain), attrs_composition: &AttrsComposition, context: &ScopeContext) -> TraitVTablePresentation {
    let (item_trait, trait_scope) = item_trait;
    let AttrsComposition { ident: item_name, scope: item_scope, .. } = attrs_composition;
    let trait_decomposition = TraitDecompositionPart2::from_trait_items(&item_trait.items, Some(parse_quote!(#item_name)), &EMPTY, context);
    let trait_ident = &item_trait.ident;
    let item_full_ty = context.full_type_for(&parse_quote!(#item_name));
    let trait_full_ty = context.full_type_for(&parse_quote!(#trait_ident));
    let methods_compositions: Vec<TraitVTableMethodComposition> = trait_decomposition.methods.into_iter()
        .map(|signature_decomposition| {
            let FnReturnTypeComposition { presentation: output_expression, conversion: output_conversions } = signature_decomposition.return_type;
            let fn_name = signature_decomposition.ident.unwrap();
            let ffi_fn_name = format_ident!("{}_{}", item_name, fn_name);
            let arguments = signature_decomposition.arguments
                .iter()
                .map(|arg| arg.name_type_original.clone())
                .collect::<Vec<_>>();

            let name_and_args = ROUND_BRACES_FIELDS_PRESENTER((quote!(unsafe extern "C" fn #ffi_fn_name), arguments)).present(context);
            let argument_names = IteratorPresentationContext::Round(
                signature_decomposition.arguments
                    .iter()
                    .map(|arg| if arg.name.is_some() {
                        OwnedItemPresenterContext::Conversion(quote!(cast_obj))
                    } else {
                        OwnedItemPresenterContext::Conversion(arg.name_type_conversion.clone())
                    })
                    .collect())
                .present(context);


            TraitVTableMethodComposition {
                fn_name,
                ffi_fn_name,
                item_type: item_full_ty.clone(),
                trait_type: trait_full_ty.clone(),
                name_and_args,
                output_expression,
                output_conversions,
                argument_names
            }
        }).collect();
    let trait_vtable_ident = Name::Vtable(trait_ident.clone());
    let trait_object_ident = Name::TraitObj(trait_ident.clone());
    let is_defined_in_same_scope = item_scope.has_same_parent(&trait_scope);
    let full_trait_type = if is_defined_in_same_scope { quote!(#trait_object_ident) } else { quote!(#trait_scope::#trait_object_ident) };
    TraitVTablePresentation::Full {
        vtable: FFIObjectPresentation::StaticVTable {
            name: Name::TraitImplVtable(item_name.clone(), trait_ident.clone()),
            fq_trait_vtable: if is_defined_in_same_scope { quote!(#trait_vtable_ident) } else { quote!(#trait_scope::#trait_vtable_ident) },
            methods_compositions,
        },
        export: BindingPresentation::ObjAsTrait {
            name: Name::TraitFn(item_name.clone(), trait_ident.clone()),
            item_type: item_full_ty.to_token_stream(),
            trait_type: full_trait_type.to_token_stream(),
            vtable_name: Name::TraitImplVtable(item_name.clone(), trait_ident.clone()),
        },
        destructor: BindingPresentation::ObjAsTraitDestructor {
            name: Name::TraitDestructor(item_name.clone(), trait_ident.clone()),
            item_type: item_full_ty.to_token_stream(),
            trait_type: full_trait_type.to_token_stream(),
        }
    }
}
