use std::cell::RefCell;
use std::collections::{HashSet, HashMap};
use std::net::SocketAddr;
use std::os::raw::c_void;
use std::rc::Rc;
use std::sync::Arc;
use crate::provider::wallet_provider::WalletProvider2;


#[derive(Clone)]
#[ferment_macro::opaque]
pub struct WalletEx2 {
    provider: Arc<WalletProvider2>,
    options: Rc<RefCell<[u8; 32]>>,
    pub locked_coins_set: HashSet<[u8; 32]>,
    anonymizable_tally_cached_non_denom: bool,
    vec_anonymizable_tally_cached_non_denom: Vec<[u8; 32]>,
    anonymizable_tally_cached: bool,
    vec_anonymizable_tally_cached: Vec<[u8; 32]>,
    map_outpoint_rounds_cache: HashMap<[u8; 32], i32>,
    unused_keys: HashMap<[u8; 32], Vec<u8>>,
    // TODO (DashJ): we may not need keyUsage, it is used as a way to audit unusedKeys
    key_usage: HashMap<[u8; 32], bool>,
    coinjoin_salt: [u8; 32],
    loaded_keys: bool,
}

#[ferment_macro::export]
impl WalletEx2 {
    pub fn new<
        // GetWalletTx: Fn(c_void, [u8; 32]) -> Option<[u8; 32]> + 'static,
        SignTransaction: Fn(*const WalletEx2, &[u8; 32], bool) -> Option<[u8; 32]> + 'static,
        IsMineInput: Fn(*const c_void, [u8; 32]) -> bool + 'static,
        GetAvailableCoins: Fn(*const c_void, bool, [u8; 32], &WalletEx2) -> Vec<[u8; 32]> + 'static,
        // SelectCoins: Fn(*const c_void, bool, bool, bool, i32) -> Vec<[u8; 32]> + 'static,
        // InputsWithAmount: Fn(*const c_void, u64) -> u32 + 'static,
        // FreshCJAddr: Fn(*const c_void, bool) -> Vec<u8> + 'static,
        // CommitTx: Fn(*const c_void, Vec<[u8; 32]>, [u8; 32], bool, [u8; 32]) -> bool + 'static,
        // IsSynced: Fn(*const c_void) -> bool + 'static,
        // IsMasternodeOrDisconnectRequested: Fn(*const c_void, SocketAddr) -> bool + 'static,
        // DisconnectMasternode: Fn(*const c_void, SocketAddr) -> bool + 'static,
        // SendMessage: Fn(*const c_void, String, Vec<u8>, SocketAddr, bool) -> bool + 'static,
        // AddPendingMasternode: Fn(*const c_void, [u8; 32], [u8; 32]) -> bool + 'static,
        // StartManagerAsync: Fn(*const c_void) + 'static,
        // GetCoinJoinKeys: Fn(*const c_void, bool) -> Vec<Vec<u8>> + 'static,
    >(
        context: *const c_void,
        options: Rc<RefCell<[u8; 32]>>,
        // get_wallet_transaction: GetWalletTx,
        sign_transaction: SignTransaction,
        is_mine_input: IsMineInput,
        available_coins: GetAvailableCoins,
        // select_coins: SelectCoins,
        // inputs_with_amount: InputsWithAmount,
        // fresh_coinjoin_address: FreshCJAddr,
        // commit_transaction: CommitTx,
        // is_synced: IsSynced,
        // is_masternode_or_disconnect_requested: IsMasternodeOrDisconnectRequested,
        // disconnect_masternode: DisconnectMasternode,
        // send_message: SendMessage,
        // add_pending_masternode: AddPendingMasternode,
        // start_manager_async: StartManagerAsync,
        // get_coinjoin_keys: GetCoinJoinKeys,
    ) -> Self {
        let provider = Arc::new(WalletProvider2::new(
            // get_wallet_transaction,
            sign_transaction,
            is_mine_input,
            available_coins,
            // select_coins,
            // inputs_with_amount,
            // fresh_coinjoin_address,
            // commit_transaction,
            // is_masternode_or_disconnect_requested,
            // disconnect_masternode,
            // is_synced,
            // send_message,
            // add_pending_masternode,
            // start_manager_async,
            // get_coinjoin_keys,
            context,
        ));
        WalletEx2 {
            options,
            provider,
            locked_coins_set: HashSet::new(),
            anonymizable_tally_cached_non_denom: false,
            vec_anonymizable_tally_cached_non_denom: Vec::new(),
            anonymizable_tally_cached: false,
            vec_anonymizable_tally_cached: Vec::new(),
            map_outpoint_rounds_cache: HashMap::new(),
            coinjoin_salt: [0; 32], // TODO: InitCoinJoinSalt ?
            loaded_keys: false,
            unused_keys: HashMap::with_capacity(1024),
            key_usage: HashMap::new(),
        }
    }

    pub fn lock_coin(&mut self, outpoint: [u8; 32]) {
        self.locked_coins_set.insert(outpoint);
        self.clear_anonymizable_caches();
    }

    pub fn unlock_coin(&mut self, outpoint: &[u8; 32]) {
        self.locked_coins_set.remove(outpoint);
        self.clear_anonymizable_caches();
    }

    pub fn is_locked_coin(&self, outpoint: &[u8; 32]) -> bool {
        self.locked_coins_set.contains(outpoint)
    }

    pub fn is_fully_mixed(&mut self, outpoint: [u8; 32]) -> bool {
        true
    }

    pub fn get_real_outpoint_coinjoin_rounds(&mut self, outpoint: [u8; 32], rounds: i32) -> i32 {
        0
    }

    pub fn ssssocke(&mut self, addr: SocketAddr) -> i32 {
        0
    }

    pub fn has_collateral_inputs(&self, only_confirmed: bool) -> bool {
        true
    }

    // pub fn available_coins(&self, only_safe: bool, coin_control: [u8; 32]) -> Vec<[u8; 32]> {
    //     self.provider.available_coins(only_safe, coin_control, self.clone())
    // }

    pub fn select_coins_grouped_by_addresses(
        &mut self, 
        skip_denominated: bool, 
        anonymizable: bool, 
        skip_unconfirmed: bool, 
        max_outpoints_per_address: i32
    ) -> Vec<[u8; 32]> {
        vec![]
    }

    pub fn get_anonymizable_balance(&mut self, skip_denominated: bool, skip_unconfirmed: bool) -> u64 {
0    }

    // pub fn get_wallet_transaction(&self, hash: [u8; 32]) -> Option<[u8; 32]> {
    //     self.provider.get_wallet_transaction(hash)
    // }
    //
    // /**
    //  * Count the number of unspent outputs that have a certain value
    //  */
    // pub fn count_inputs_with_amount(&self, value: u64) -> u32 {
    //     self.provider.count_inputs_with_amount(value)
    // }

    pub fn get_unused_key(&mut self, internal: bool) -> Option<[u8; 32]> {
        None
    }

    pub fn add_unused_key(&mut self, destination: Vec<u8>) {
    }

    pub fn remove_unused_key(&mut self, destination: &[u8; 32]) {
    }

    pub fn refresh_unused_keys(&mut self) {
        self.loaded_keys = true;
        
    }

    pub fn process_used_scripts(&mut self, scripts: &Vec<Vec<u8>>) {
    }

    // pub fn commit_transaction(&self, vec_send: Vec<[u8; 32]>, coin_control: [u8; 32], is_denominating: bool, client_session_id: [u8; 32]) -> bool {
    //     self.provider.commit_transaction(vec_send, coin_control, is_denominating, client_session_id)
    // }

    pub fn sign_transaction(&self, tx: &[u8; 32], anyone_can_pay: bool) -> Option<[u8; 32]> {
        self.provider.sign_transaction(tx, anyone_can_pay)
    }
    pub fn select_tx_dsins_by_denomination(&mut self, denom: u32, value_max: u64, vec_tx_dsin_ret: &mut Vec<[u8; 32]>) -> bool {
        true
    }

    pub fn select_denominated_amounts(&self, value_max: u64, set_amounts_ret: &mut HashSet<u64>) -> bool {
        true
    }

    // pub fn is_masternode_or_disconnect_requested(&self, address: SocketAddr) -> bool {
    //     self.provider.is_masternode_or_disconnect_requested(address)
    // }
    //
    // pub fn disconnect_masternode(&self, address: SocketAddr) -> bool {
    //     self.provider.disconnect_masternode(address)
    // }
    // pub fn is_synced(&self) -> bool {
    //     self.provider.is_synced()
    // }
    // pub fn send_message(&self, message: Vec<u8>, msg_type: String, address: SocketAddr, warn: bool) -> bool {
    //     self.provider.send_message(message, msg_type, address, warn)
    // }
    //
    // pub fn add_pending_masternode(&self, pro_tx_hash: [u8; 32], session_id: [u8; 32]) -> bool {
    //     self.provider.add_pending_masternode(pro_tx_hash, session_id)
    // }
    //
    // pub fn start_manager_async(&self) {
    //     self.provider.start_manager_async()
    // }

    pub fn clear_anonymizable_caches(&mut self) {
        self.anonymizable_tally_cached_non_denom = false;
        self.anonymizable_tally_cached = false;
    }

    pub fn fresh_receive_key(&mut self, internal: bool) -> Vec<u8> {
        vec![]
    }

}
