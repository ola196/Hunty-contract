use crate::NftData;
use soroban_sdk::{symbol_short, Address, Env, Vec};

/// Storage layer for NFTs.
pub struct Storage;

impl Storage {
    const NFT_KEY: soroban_sdk::Symbol = symbol_short!("NFT");
    const NFT_COUNTER_KEY: soroban_sdk::Symbol = symbol_short!("CNTR");
    const OWNER_NFT_COUNT_KEY: soroban_sdk::Symbol = symbol_short!("ONFC");
    const HUNT_NFT_COUNT_KEY: soroban_sdk::Symbol = symbol_short!("HNFC");
    const MAX_SUPPLY_KEY: soroban_sdk::Symbol = symbol_short!("MAXS");
    const INITIALIZED_KEY: soroban_sdk::Symbol = symbol_short!("INIT");
    const ADMIN_KEY: soroban_sdk::Symbol = symbol_short!("ADMIN");
    const MINTER_KEY: soroban_sdk::Symbol = symbol_short!("MNTR");
    const REWARD_MGR_KEY: soroban_sdk::Symbol = symbol_short!("RWDMGR");

    fn nft_key(nft_id: u64) -> (soroban_sdk::Symbol, u64) {
        (Self::NFT_KEY, nft_id)
    }

    fn owner_nft_entry_key(owner: &Address, index: u32) -> (soroban_sdk::Symbol, Address, u32) {
        (symbol_short!("ONFT"), owner.clone(), index)
    }

    fn owner_nft_count_key(owner: &Address) -> (soroban_sdk::Symbol, Address) {
        (Self::OWNER_NFT_COUNT_KEY, owner.clone())
    }

    fn owner_nft_exist_key(owner: &Address, nft_id: u64) -> (soroban_sdk::Symbol, Address, u64) {
        (symbol_short!("ONFX"), owner.clone(), nft_id)
    }

    fn hunt_nft_entry_key(hunt_id: u64, index: u32) -> (soroban_sdk::Symbol, u64, u32) {
        (symbol_short!("HNFT"), hunt_id, index)
    }

    fn hunt_nft_count_key(hunt_id: u64) -> (soroban_sdk::Symbol, u64) {
        (Self::HUNT_NFT_COUNT_KEY, hunt_id)
    }

    fn hunt_nft_exist_key(hunt_id: u64, nft_id: u64) -> (soroban_sdk::Symbol, u64, u64) {
        (symbol_short!("HNFX"), hunt_id, nft_id)
    }

    fn operator_key(owner: &Address, operator: &Address) -> (soroban_sdk::Symbol, Address, Address) {
        (symbol_short!("OPR"), owner.clone(), operator.clone())
    }

    fn minter_key(minter: &Address) -> (soroban_sdk::Symbol, Address) {
        (Self::MINTER_KEY, minter.clone())
    }

    pub fn remove_nft(env: &Env, nft_id: u64) {
        let key = Self::nft_key(nft_id);
        env.storage().persistent().remove(&key);
    }

    pub fn save_admin(env: &Env, admin: &Address) {
        env.storage().instance().set(&Self::ADMIN_KEY, admin);
    }

    pub fn get_admin(env: &Env) -> Option<Address> {
        env.storage().instance().get(&Self::ADMIN_KEY)
    }

    pub fn set_reward_manager(env: &Env, address: &Address) {
        env.storage().instance().set(&Self::REWARD_MGR_KEY, address);
    }

    pub fn save_reward_manager(env: &Env, address: &Address) {
        Self::set_reward_manager(env, address);
    }

    pub fn get_reward_manager(env: &Env) -> Option<Address> {
        env.storage().instance().get(&Self::REWARD_MGR_KEY)
    }

    // --- Minter whitelist (reserved for admin-gated minting) ---

    #[allow(dead_code)]
    pub fn add_minter(env: &Env, minter: &Address) {
        let key = Self::minter_key(minter);
        env.storage().persistent().set(&key, &true);
    }

    #[allow(dead_code)]
    pub fn remove_minter(env: &Env, minter: &Address) {
        let key = Self::minter_key(minter);
        env.storage().persistent().remove(&key);
    }

    #[allow(dead_code)]
    pub fn is_minter(env: &Env, minter: &Address) -> bool {
        let key = Self::minter_key(minter);
        env.storage().persistent().get(&key).unwrap_or(false)
    }

    pub fn save_nft(env: &Env, nft: &NftData) {
        let key = Self::nft_key(nft.nft_id);
        env.storage().persistent().set(&key, nft);
    }

    pub fn get_nft(env: &Env, nft_id: u64) -> Option<NftData> {
        let key = Self::nft_key(nft_id);
        env.storage().persistent().get(&key)
    }

    pub fn next_nft_id(env: &Env) -> u64 {
        let current: u64 = env
            .storage()
            .persistent()
            .get(&Self::NFT_COUNTER_KEY)
            .unwrap_or(0);
        let next = current + 1;
        env.storage()
            .persistent()
            .set(&Self::NFT_COUNTER_KEY, &next);
        next
    }

    pub fn get_nft_counter(env: &Env) -> u64 {
        env.storage()
            .persistent()
            .get(&Self::NFT_COUNTER_KEY)
            .unwrap_or(0)
    }

    pub fn set_max_supply(env: &Env, max_supply: Option<u64>) {
        env.storage()
            .persistent()
            .set(&Self::MAX_SUPPLY_KEY, &max_supply);
        env.storage().persistent().set(&Self::INITIALIZED_KEY, &true);
    }

    pub fn get_max_supply(env: &Env) -> Option<u64> {
        env.storage()
            .persistent()
            .get(&Self::MAX_SUPPLY_KEY)
            .unwrap_or(None)
    }

    pub fn is_initialized(env: &Env) -> bool {
        env.storage()
            .persistent()
            .get(&Self::INITIALIZED_KEY)
            .unwrap_or(false)
    }

    pub fn add_nft_to_owner(env: &Env, owner: &Address, nft_id: u64) {
        let count_key = Self::owner_nft_count_key(owner);
        let count: u32 = env.storage().persistent().get(&count_key).unwrap_or(0);

        let exist_key = Self::owner_nft_exist_key(owner, nft_id);
        if env.storage().persistent().has(&exist_key) {
            return;
        }

        env.storage()
            .persistent()
            .set(&Self::owner_nft_entry_key(owner, count), &nft_id);
        env.storage().persistent().set(&count_key, &(count + 1));
        env.storage().persistent().set(&exist_key, &());
    }

    pub fn add_nft_to_hunt(env: &Env, hunt_id: u64, nft_id: u64) {
        let count_key = Self::hunt_nft_count_key(hunt_id);
        let count: u32 = env.storage().persistent().get(&count_key).unwrap_or(0);

        let exist_key = Self::hunt_nft_exist_key(hunt_id, nft_id);
        if env.storage().persistent().has(&exist_key) {
            return;
        }

        env.storage()
            .persistent()
            .set(&Self::hunt_nft_entry_key(hunt_id, count), &nft_id);
        env.storage().persistent().set(&count_key, &(count + 1));
        env.storage().persistent().set(&exist_key, &());
    }

    pub fn get_hunt_nft_count(env: &Env, hunt_id: u64) -> u32 {
        let count_key = Self::hunt_nft_count_key(hunt_id);
        env.storage().persistent().get(&count_key).unwrap_or(0)
    }

    pub fn get_hunt_nfts(env: &Env, hunt_id: u64, offset: u32, limit: u32) -> Vec<u64> {
        let count_key = Self::hunt_nft_count_key(hunt_id);
        let count: u32 = env.storage().persistent().get(&count_key).unwrap_or(0);
        if offset >= count {
            return Vec::new(env);
        }
        let end = offset.saturating_add(limit).min(count);
        let mut ids = Vec::new(env);
        for i in offset..end {
            let entry_key = Self::hunt_nft_entry_key(hunt_id, i);
            if let Some(id) = env.storage().persistent().get(&entry_key) {
                ids.push_back(id);
            }
        }
        ids
    }

    pub fn remove_nft_from_hunt(env: &Env, hunt_id: u64, nft_id: u64) {
        let count_key = Self::hunt_nft_count_key(hunt_id);
        let count: u32 = env.storage().persistent().get(&count_key).unwrap_or(0);
        let exist_key = Self::hunt_nft_exist_key(hunt_id, nft_id);
        if !env.storage().persistent().has(&exist_key) {
            return;
        }

        for i in 0..count {
            let entry_key = Self::hunt_nft_entry_key(hunt_id, i);
            if let Some(stored_id) = env.storage().persistent().get::<_, u64>(&entry_key) {
                if stored_id == nft_id {
                    let last_idx = count - 1;
                    if i != last_idx {
                        let last_key = Self::hunt_nft_entry_key(hunt_id, last_idx);
                        if let Some(last_id) = env.storage().persistent().get::<_, u64>(&last_key) {
                            env.storage().persistent().set(&entry_key, &last_id);
                        }
                        env.storage().persistent().remove(&last_key);
                    } else {
                        env.storage().persistent().remove(&entry_key);
                    }
                    env.storage().persistent().set(&count_key, &(count - 1));
                    env.storage().persistent().remove(&exist_key);
                    return;
                }
            }
        }
    }

    /// Returns all minted NFT IDs by iterating from 1 to the current counter.
    pub fn get_all_nft_ids(env: &Env) -> Vec<u64> {
        let counter = Self::get_nft_counter(env);
        let mut ids = Vec::new(env);
        for id in 1..=counter {
            if env.storage().persistent().has(&Self::nft_key(id)) {
                ids.push_back(id);
            }
        }
        ids
    }

    pub fn get_owner_nfts(env: &Env, owner: &Address) -> Vec<u64> {
        let count_key = Self::owner_nft_count_key(owner);
        let count: u32 = env.storage().persistent().get(&count_key).unwrap_or(0);
        let mut ids = Vec::new(env);
        for i in 0..count {
            let entry_key = Self::owner_nft_entry_key(owner, i);
            if let Some(id) = env.storage().persistent().get(&entry_key) {
                ids.push_back(id);
            }
        }
        ids
    }

    // --- Operator management ---

    /// Grants operator approval: `operator` can manage all NFTs owned by `owner`.
    pub fn set_operator(env: &Env, owner: &Address, operator: &Address) {
        let key = Self::operator_key(owner, operator);
        env.storage().persistent().set(&key, &true);
    }

    /// Revokes operator approval.
    pub fn remove_operator(env: &Env, owner: &Address, operator: &Address) {
        let key = Self::operator_key(owner, operator);
        env.storage().persistent().remove(&key);
    }

    /// Returns true if `operator` is approved to manage all NFTs of `owner`.
    pub fn is_operator(env: &Env, owner: &Address, operator: &Address) -> bool {
        let key = Self::operator_key(owner, operator);
        env.storage().persistent().get(&key).unwrap_or(false)
    }

    // --- Contract version ---

    pub fn set_contract_version(env: &Env, version: u32) {
        env.storage().instance().set(&symbol_short!("CVER"), &version);
    }

    pub fn get_contract_version(env: &Env) -> Option<u32> {
        env.storage().instance().get(&symbol_short!("CVER"))
    }
}
