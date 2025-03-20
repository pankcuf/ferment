// use std::cell::RefCell;
// use std::collections::{HashMap, HashSet};
// use std::net::SocketAddr;
// use std::os::raw::c_void;
// use std::rc::Rc;
// use std::sync::Arc;
// use crate::wallet_provider::WalletProvider;
// //
//
// #[derive(Clone)]
// #[ferment_macro::opaque]
// pub struct WalletEx {
//     provider: Arc<WalletProvider>,
//     options: Rc<RefCell<[u8; 32]>>,
//     pub locked_coins_set: HashSet<[u8; 32]>,
//     anonymizable_tally_cached_non_denom: bool,
//     vec_anonymizable_tally_cached_non_denom: Vec<[u8; 32]>,
//     anonymizable_tally_cached: bool,
//     vec_anonymizable_tally_cached: Vec<[u8; 32]>,
//     map_outpoint_rounds_cache: HashMap<[u8; 32], i32>,
//     unused_keys: HashMap<[u8; 32], Vec<u8>>,
//     // TODO (DashJ): we may not need keyUsage, it is used as a way to audit unusedKeys
//     key_usage: HashMap<[u8; 32], bool>,
//     coinjoin_salt: [u8; 32],
//     loaded_keys: bool,
// }
//
// #[ferment_macro::export]
// impl WalletEx {
//     pub fn new<
//         GetWalletTx: Fn(*const c_void, [u8; 32]) -> Option<[u8; 32]> + 'static,
//         SignTransaction: Fn(*const c_void, [u8; 32], bool) -> Option<[u8; 32]> + 'static,
//         IsMineInput: Fn(*const c_void, [u8; 32]) -> bool + 'static,
//         GetAvailableCoins: Fn(*const c_void, bool, [u8; 32], WalletEx) -> Vec<[u8; 32]> + 'static,
//         SelectCoins: Fn(*const c_void, bool, bool, bool, i32) -> Vec<[u8; 32]> + 'static,
//         InputsWithAmount: Fn(*const c_void, u64) -> u32 + 'static,
//         FreshCJAddr: Fn(*const c_void, bool) -> Vec<u8> + 'static,
//         CommitTx: Fn(*const c_void, Vec<[u8; 32]>, [u8; 32], bool, [u8; 32]) -> bool + 'static,
//         IsSynced: Fn(*const c_void) -> bool + 'static,
//         IsMasternodeOrDisconnectRequested: Fn(*const c_void, String) -> bool + 'static,
//         DisconnectMasternode: Fn(*const c_void, SocketAddr) -> bool + 'static,
//         SendMessage: Fn(*const c_void, String, Vec<u8>, Option<SocketAddr>, bool) -> bool + 'static,
//         AddPendingMasternode: Fn(*const c_void, [u8; 32], [u8; 32]) -> bool + 'static,
//         StartManagerAsync: Fn(*const c_void) + 'static,
//         GetCoinJoinKeys: Fn(*const c_void, bool) -> Vec<Vec<u8>> + 'static,
//     >(
//         context: *const c_void,
//         options: Rc<RefCell<[u8; 32]>>,
//         get_wallet_transaction: GetWalletTx,
//         sign_transaction: SignTransaction,
//         is_mine_input: IsMineInput,
//         available_coins: GetAvailableCoins,
//         select_coins: SelectCoins,
//         inputs_with_amount: InputsWithAmount,
//         fresh_coinjoin_address: FreshCJAddr,
//         commit_transaction: CommitTx,
//         is_synced: IsSynced,
//         is_masternode_or_disconnect_requested: IsMasternodeOrDisconnectRequested,
//         disconnect_masternode: DisconnectMasternode,
//         send_message: SendMessage,
//         add_pending_masternode: AddPendingMasternode,
//         start_manager_async: StartManagerAsync,
//         get_coinjoin_keys: GetCoinJoinKeys,
//     ) -> Self {
//         let provider = Arc::new(WalletProvider::new(
//             get_wallet_transaction,
//             sign_transaction,
//             is_mine_input,
//             available_coins,
//             select_coins,
//             inputs_with_amount,
//             fresh_coinjoin_address,
//             commit_transaction,
//             is_masternode_or_disconnect_requested,
//             disconnect_masternode,
//             is_synced,
//             send_message,
//             add_pending_masternode,
//             start_manager_async,
//             get_coinjoin_keys,
//             context,
//         ));
//         WalletEx {
//             options,
//             provider,
//             locked_coins_set: HashSet::new(),
//             anonymizable_tally_cached_non_denom: false,
//             vec_anonymizable_tally_cached_non_denom: Vec::new(),
//             anonymizable_tally_cached: false,
//             vec_anonymizable_tally_cached: Vec::new(),
//             map_outpoint_rounds_cache: HashMap::new(),
//             coinjoin_salt: [0; 32], // TODO: InitCoinJoinSalt ?
//             loaded_keys: false,
//             unused_keys: HashMap::with_capacity(1024),
//             key_usage: HashMap::new(),
//         }
//     }
// }
//
