pub mod snapshot;
pub mod quorum;
pub mod callback;
pub mod tuples;
pub mod many_scopes;
pub mod indexmap;
pub mod feat_dep;

pub use snapshot::LLMQSnapshot;
pub use snapshot::LLMQSnapshotSkipMode;
pub use quorum::QuorumType;

#[ferment_macro::export]
pub enum TestModLevelSnapshot {
    VO(LLMQSnapshot)
}
#[ferment_macro::export]
pub enum TestModLevelVecSnapshot {
    VO(Vec<LLMQSnapshotSkipMode>)
}

#[ferment_macro::export]
pub enum TestModLevelOptSnapshot {
    VO(Option<LLMQSnapshotSkipMode>)
}

#[ferment_macro::export]
#[derive(Clone, Debug)]
pub struct Quorum {
    pub llmq_type: QuorumType
}
#[ferment_macro::export]
pub struct LLMQParams {
    pub r#type: String,
    pub known_confirmed_at_height: Option<u32>,
}


pub mod ferment_example {
    use crate::model::{Quorum, QuorumType};

    #[ferment_macro::export]
    pub fn get_rotated_quorum() -> Quorum {
        Quorum { llmq_type: QuorumType::Rotated }
    }

    #[ferment_macro::export]
    pub fn get_crazy_case() -> Quorum {
        Quorum { llmq_type: QuorumType::Rotated }
    }
}

pub mod some_inner {
    use crate::model::{Quorum, QuorumType};
    #[ferment_macro::export]
    pub fn get_normal_quorum() -> Quorum {
        Quorum { llmq_type: QuorumType::Normal }
    }
}
pub mod some_inner_2 {
    use crate::model::quorum::QuorumType;
    use crate::model::Quorum;
    #[ferment_macro::export]
    pub fn get_normal_quorum() -> Quorum {
        Quorum { llmq_type: QuorumType::Normal }
    }


    // #[ferment_macro::export]
    // pub struct DocumentTypeV2 {
    //     pub name: String,
    //     pub indexes: BTreeMap<String, Option<Vec<u8>>>,
    // }
    // #[ferment_macro::export]
    // pub struct DocumentTypeV3 {
    //     pub name: String,
    //     pub indexes: BTreeMap<String, Option<Vec<String>>>,
    // }
}

// #[derive(Clone, Debug, Default)]
// #[ferment_macro::opaque]
// pub struct IndexPath<T> {
//     pub indexes: Vec<T>,
//     pub hardened: Vec<bool>,
// }
