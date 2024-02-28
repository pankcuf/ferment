use proc_macro2::TokenStream as TokenStream2;
use syn::Generics;
use crate::composer::{Depunctuated, ParentComposer};
use crate::context::ScopeContext;
use crate::naming::Name;
use crate::presentation::{BindingPresentation, ConversionInterfacePresentation, DocPresentation, DropInterfacePresentation, Expansion, FFIObjectPresentation, FromConversionPresentation, ToConversionPresentation, TraitVTablePresentation};

pub trait IParentComposer {
    fn context(&self) -> &ParentComposer<ScopeContext>;
    fn compose_attributes(&self) -> Depunctuated<TraitVTablePresentation>;
    fn compose_bindings(&self) -> Depunctuated<BindingPresentation>;
    fn compose_docs(&self) -> DocPresentation;
    fn compose_object(&self) -> FFIObjectPresentation;
    fn compose_drop(&self) -> DropInterfacePresentation;

    fn compose_names(&self) -> (Name, Name);
    fn compose_interface_aspects(&self) -> (FromConversionPresentation, ToConversionPresentation, TokenStream2, Option<Generics>);
    fn expand(&self) -> Expansion {
        let (from_presentation, to_presentation, destroy_presentation, generics) = self.compose_interface_aspects();
        let (ffi_type, target_type) = self.compose_names();
        Expansion::Full {
            comment: self.compose_docs(),
            ffi_presentation: self.compose_object(),
            conversion: ConversionInterfacePresentation::Interface {
                ffi_type,
                target_type,
                from_presentation,
                to_presentation,
                destroy_presentation,
                generics
            },
            drop: self.compose_drop(),
            bindings: self.compose_bindings(),
            traits: self.compose_attributes()
        }
    }
}
