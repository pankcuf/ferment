use crate::composer::{SourceComposable, GenericComposer, GenericComposerInfo};
use crate::context::ScopeContext;
use crate::lang::objc::{ObjCFermentate, ObjCSpecification};

impl SourceComposable for GenericComposer<ObjCSpecification> {
    type Source = ScopeContext;
    type Output = Option<ObjCFermentate>;

    fn compose(&self, source: &Self::Source) -> Self::Output {
        self.wrapper
            .compose(source)
            .map(|GenericComposerInfo {
                      field_composers: _,
                      field_composer: _,
                      ffi_aspect: _,
                      attrs: _,
                      interfaces: implementations,
                      bindings: _
                  }| {
                // println!("OBJC GEN1");
                // let fields = CommaPunctuated::from_iter(field_composers.iter().map(field_composer));
                // println!("OBJC GEN2");
                // let _implementation = BraceWrapped::new(fields).present(source);
                // println!("OBJC GEN3");
                // let ffi_presentation = FFIObjectPresentation::Full(present_struct(&ffi_name, &attrs, implementation));
                // let ffi_type = ffi_name.to_type();
                // let global = source.context.read().unwrap();
                // let config = global.config.maybe_objc_config().unwrap();

                // let bindings = Depunctuated::from_iter([
                //     crate::composer::constants::struct_composer_ctor_root()((
                //         ((ffi_type.clone(), attrs.clone(), SPEC::Gen::default()) , false),
                //         crate::composer::constants::field_conversions_iterator(field_composers.clone(), binding_composer)
                //     )),
                //     BindingPresentableContext::dtor((ffi_type, attrs.clone(), SPEC::Gen::default()))
                // ]);
                ObjCFermentate::Item { implementations }
            })
    }
}