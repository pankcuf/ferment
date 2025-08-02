use crate::composer::{ModFnComposer, TraitFnImplComposer};
use crate::composer::bare_fn::BareFnComposer;
use crate::lang::Specification;

#[allow(unused)]
pub enum SignatureComposerWrapper<SPEC>
where SPEC: Specification + 'static {
    Mod(ModFnComposer<SPEC>),
    Bare(BareFnComposer<SPEC>),
    TraitImpl(TraitFnImplComposer<SPEC>),
}
