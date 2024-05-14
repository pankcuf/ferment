use std::time::Duration;

#[ferment_macro::export]
pub struct StructUsesDurationTuple {
    pub time: (Duration, Duration)
}