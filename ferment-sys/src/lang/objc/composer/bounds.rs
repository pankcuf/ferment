use crate::composer::{BoundsComposer, Composer, GenericComposerInfo};
use crate::context::ScopeContext;
use crate::ext::{AsType, Mangle, ToType};
use crate::lang::objc::{ObjCFermentate, ObjCSpecification};

impl<'a, SPEC> Composer<'a> for BoundsComposer<ObjCFermentate, SPEC>
    where SPEC: ObjCSpecification {
    type Source = ScopeContext;
    type Output = Option<GenericComposerInfo<ObjCFermentate, SPEC>>;

    #[allow(unused_variables)]
    fn compose(&self, source: &'a Self::Source) -> Self::Output {
        if self.model.is_lambda() {
            return Self::Output::default();
        }
        let ffi_name = self.model.mangle_ident_default();
        let self_ty = self.model.as_type();
        let ffi_as_type = ffi_name.to_type();
        // println!("Mixin::Expand: {} ---- \n\tattrs: {:?}\n\tname: {}", self.model, self.attrs, ffi_name);


        None
    }
}