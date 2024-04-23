#[derive(ferment_macro::CompositionContext)]
#[derive(Debug)]
pub enum FnSignatureCompositionContext {
    FFIObject,
    FFIObjectCallback,
    TraitVTableInner,
    // StaticVTableInner,
    // StaticVTable(TraitDecompositionPart2)
}
