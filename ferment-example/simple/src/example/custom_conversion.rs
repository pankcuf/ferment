use std::collections::BTreeMap;
use std::time::Duration;

#[ferment_macro::export]
pub struct StructUsesDurationTuple {
    pub time: (Duration, Duration)
}


#[ferment_macro::export]
pub struct StructUsesGenericWithCustom {
    pub time: BTreeMap<String, Duration>
}