

#[cfg(feature = "validation")]
#[cfg_attr(feature = "validation", ferment_macro::export)]
pub struct Validatorrr {}
#[ferment_macro::export]
pub struct FeatureDependentField {
    pub field_name: String,
    #[cfg(feature = "validation")]
    pub field_type: Validatorrr,
}