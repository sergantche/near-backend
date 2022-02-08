mod constants;

use near_contract_standards::non_fungible_token::{Token, TokenId, NonFungibleToken};
use near_contract_standards::non_fungible_token::metadata::{
    NFTContractMetadata, NonFungibleTokenMetadataProvider, TokenMetadata, NFT_METADATA_SPEC,
  };

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{
    env, log, near_bindgen, PanicOnDefault, AccountId, BorshStorageKey, Promise, PromiseResult, PromiseOrValue
};
use near_sdk::collections::{ UnorderedMap, LazyOption, LookupMap};
use near_sdk::json_types::Base64VecU8;
use near_sdk::serde_json::json;

pub use constants::{BASE_URI, DATA_IMAGE_SVG_NEAR_ICON, ONE_NEAR, ONE_YOCTO, SINGLE_CALL_GAS};

#[derive(BorshSerialize, BorshStorageKey)]
enum StorageKey {
    Stars,
    Experience,
    MaximumLevel,
    NonFungibleToken,
    Metadata,
    TokenMetadata,
    Enumeration,
    Approval,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    owner_id: AccountId,
    // NFT implementation
    tokens: NonFungibleToken,
    metadata: LazyOption<NFTContractMetadata>,
    stars: LookupMap<TokenId, u64>,
    experience: LookupMap<TokenId, u64>,
    maximum_level: LookupMap<TokenId, u64>,
}

#[near_bindgen]
impl Contract {
    /// Initializes the contract owned by `owner_id` with
    /// default metadata (for example purposes only).
    #[init]
    pub fn new() -> Self {
        assert!(!env::state_exists(), "Already initialized");
        let owner_id = env::current_account_id(); // Who deployed owns

        let metadata = NFTContractMetadata {
            spec: NFT_METADATA_SPEC.to_string(),
            name: "vSelf NEAR NFT checkins".to_string(),
            symbol: "VSLF".to_string(),
            icon: Some(DATA_IMAGE_SVG_NEAR_ICON.to_string()),
            base_uri: Some(BASE_URI.to_string()),
            reference: None,
            reference_hash: None,
        };
        metadata.assert_valid();        

        Self {
            owner_id: owner_id.clone().into(),
            tokens: NonFungibleToken::new(
                StorageKey::NonFungibleToken,
                owner_id,
                Some(StorageKey::TokenMetadata),
                Some(StorageKey::Enumeration),
                Some(StorageKey::Approval),
            ),
            metadata: LazyOption::new(StorageKey::Metadata, Some(&metadata)),
            stars: LookupMap::new(StorageKey::Stars),
            experience: LookupMap::new(StorageKey::Experience),
            maximum_level: LookupMap::new(StorageKey::MaximumLevel),
        }                
    }

    #[payable]
    pub fn play(&mut self) -> u8 {
        let account_id = env::signer_account_id();
        let deposit = env::attached_deposit();
        
        assert!((deposit > ONE_NEAR), "not enough currency to play");
        
        // Toss the dice (minimal logic for now)
        let rand: u8 = *env::random_seed().get(0).unwrap();

        return rand;
    }

    pub fn update_hero_stats(&mut self) {
        //
    }

    #[payable]
    pub fn craft_new_hero(&mut self) -> TokenId {
        //log!("step 1");
        let timestamp: u64 = env::block_timestamp();
        let rand: u8 = *env::random_seed().get(0).unwrap();
        let token_id: String = format!("HERO:{}:{}", rand, timestamp);
        log!("token id: {}", token_id.clone());

        let contract_id = env::current_account_id();
        let root_id = AccountId::try_from(contract_id).unwrap();

        let media_url: String = format!("{}.png", token_id.clone());
        let media_hash = Base64VecU8(env::sha256(media_url.as_bytes()));
        log!("media url: {}", media_url.clone());

        // Default to common token
        let mut token_metadata = TokenMetadata {
            title: Some("Common".to_string()),
            description: Some("NFT hero token".to_string()),
            media: Some(media_url),
            media_hash: Some(media_hash),
            copies: Some(1u64),
            issued_at: Some(timestamp.to_string()),
            expires_at: None,
            starts_at: None,
            updated_at: None,
            extra: None,
            reference: None,
            reference_hash: None,
        };
        //log!("step 2");

        // Mint NFT   
        self.nft_mint(token_id.clone(), root_id.clone(), token_metadata.clone());

        // Transfer NFT to new owner
        // log!("username: {}", username.clone());
        // let receiver_id = AccountId::try_from(username).unwrap();
        // log!("receiver id: {}", receiver_id.clone());
        // log!("token_id: {}", token_id.clone());
        // env::promise_create(
        //     root_id,
        //     "nft_transfer",
        //     json!({
        //         "token_id": token_id.clone(),
        //         "receiver_id": receiver_id,
        //     })
        //     .to_string()
        //     .as_bytes(),
        //     ONE_YOCTO,
        //     SINGLE_CALL_GAS,
        // );
        // log!("Success! NFT transfering for {}! Token ID = {}", receiver_id.clone(), token_id.clone());
        token_id.clone()
    }

    // Mint a new token with ID=token_id belonging to receiver_id.
    ///
    /// Since this example implements metadata, it also requires per-token metadata to be provided
    /// in this call. self.tokens.mint will also require it to be Some, since
    /// StorageKey::TokenMetadata was provided at initialization.
    ///
    /// self.tokens.mint will enforce predecessor_account_id to equal the owner_id given in
    /// initialization call to new.
    #[payable]
    pub fn nft_mint(
        &mut self,
        token_id: TokenId,
        receiver_id: AccountId,
        token_metadata: TokenMetadata,
    ) -> Token {
        self.tokens.internal_mint(token_id, receiver_id, Some(token_metadata))
    }
}

// Implement NFT standart
near_contract_standards::impl_non_fungible_token_core!(Contract, tokens);
near_contract_standards::impl_non_fungible_token_approval!(Contract, tokens);
near_contract_standards::impl_non_fungible_token_enumeration!(Contract, tokens);

#[near_bindgen]
impl NonFungibleTokenMetadataProvider for Contract {
  fn nft_metadata(&self) -> NFTContractMetadata {
      self.metadata.get().unwrap()
  }
}