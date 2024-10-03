#[derive(ferment_macro::CompositionContext)]
#[derive(Debug)]
#[allow(unused)]
pub enum FnSignatureCompositionContext {
    FFIObject,
    FFIObjectCallback,
    TraitVTableInner,
    // StaticVTableInner,
    // StaticVTable(TraitDecompositionPart2)
}
