#[derive(ferment_macro::CompositionContext)]
pub enum FnSignatureCompositionContext {
    FFIObject,
    FFIObjectCallback,
    TraitVTableInner,
    // StaticVTable(TraitDecompositionPart2)
}
