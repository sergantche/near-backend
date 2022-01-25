use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{
    env, log, near_bindgen, PanicOnDefault, AccountId, BorshStorageKey, Promise, PromiseResult, PromiseOrValue
};
use std::convert::TryInto;

const ONE_NEAR:u128 = 1_000_000_000_000_000_000_000_000;
const MICRO_NEAR:u128 =   1_000_000_000_000_000_000_000;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct LootboxGame {
}

#[near_bindgen]
impl LootboxGame {
    #[payable]
    pub fn play(&mut self) -> u8 {
        let account_id = env::signer_account_id();
        let deposit = env::attached_deposit();        
        
        assert!(deposit > (ONE_NEAR - MICRO_NEAR), "not enough currency to play");
        
        // Toss the dice (minimal logic for now)
        let rand: u8 = *env::random_seed().get(0).unwrap();

        return rand;
    }
}

