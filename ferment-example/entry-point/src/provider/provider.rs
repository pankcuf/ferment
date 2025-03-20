use std::os::raw::c_void;
use std::net::SocketAddr;
use std::sync::Arc;
use dashcore::sml::masternode_list::MasternodeList;
use crate::messages::{PoolMessage, PoolState, PoolStatus};

// #[ferment_macro::opaque]
pub struct CoinJoinProvider {
    pub context: *const c_void,

    pub get_masternode_list: Arc<dyn Fn(*const c_void) -> MasternodeList>,
    pub update_success_block: Arc<dyn Fn(*const c_void)>,
    pub is_waiting_for_new_block: Arc<dyn Fn(*const c_void) -> bool>,
    pub session_lifecycle_listener: Arc<dyn Fn(*const c_void, bool, i32, [u8; 32], u32, PoolState, PoolMessage, PoolStatus, Option<SocketAddr>, bool)>,
    pub mixing_lifecycle_listener: Arc<dyn Fn(*const c_void, bool, bool, Vec<PoolStatus>)>,
}

#[ferment_macro::export]
impl CoinJoinProvider  {
    pub fn new<
        GML: Fn(*const c_void) -> MasternodeList + 'static,
        USB: Fn(*const c_void) + 'static,
        IWFNB: Fn(*const c_void) -> bool + 'static,
        SLL: Fn(*const c_void, bool, i32, [u8; 32], u32, PoolState, PoolMessage, PoolStatus, Option<SocketAddr>, bool) + 'static,
        MLL: Fn(*const c_void, bool, bool, Vec<PoolStatus>) + 'static,
    >(
        get_masternode_list: GML,
        update_success_block: USB,
        is_waiting_for_new_block: IWFNB,
        session_lifecycle_listener: SLL,
        mixing_lifecycle_listener: MLL,
        context: *const c_void,
    ) -> Self {
        Self {
            context,
            get_masternode_list: Arc::new(get_masternode_list),
            update_success_block: Arc::new(update_success_block),
            is_waiting_for_new_block: Arc::new(is_waiting_for_new_block),
            session_lifecycle_listener: Arc::new(session_lifecycle_listener),
            mixing_lifecycle_listener: Arc::new(mixing_lifecycle_listener),
        }
    }
}

impl CoinJoinProvider {
    pub fn queue_session_lifecycle(&self, is_complete: bool, base_session_id: i32, session_id: [u8; 32], session_denom: u32, state: PoolState, message: PoolMessage, status: PoolStatus, addr: Option<SocketAddr>, joined: bool) {
        (self.session_lifecycle_listener)(self.context, is_complete, base_session_id, session_id, session_denom, state, message, status, addr, joined);
    }
    pub fn queue_mixing_lifecycle(&self, is_complete: bool, is_interrupted: bool, statuses: Vec<PoolStatus>) {
        (self.mixing_lifecycle_listener)(self.context, is_complete, is_interrupted, statuses);
    }
    pub fn is_waiting_for_new_block(&self) -> bool {
        (self.is_waiting_for_new_block)(self.context)
    }
    pub fn get_current_masternode_list(&self) -> MasternodeList {
        (self.get_masternode_list)(self.context)
    }
    pub fn update_success_block(&self) {
        (self.update_success_block)(self.context)
    }
}