#[derive(Clone)]
#[ferment_macro::export]
pub enum LLMQSnapshotSkipMode {
    NoSkipping = 0,
    SkipFirst = 1,
    SkipExcept = 2,
    SkipAll = 3,
}

#[derive(Clone)]
#[ferment_macro::export]
pub struct LLMQSnapshot {
    pub member_list: Vec<u8>,
    pub skip_list: Vec<i32>,
    pub skip_list_mode: LLMQSnapshotSkipMode,
}
