mod constants;

use near_contract_standards::non_fungible_token::{Token, TokenId, NonFungibleToken};
use near_contract_standards::non_fungible_token::metadata::{
    NFTContractMetadata, NonFungibleTokenMetadataProvider, TokenMetadata, NFT_METADATA_SPEC,
  };

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{
    env, log, near_bindgen, PanicOnDefault, AccountId, BorshStorageKey, Promise, PromiseOrValue
};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::collections::{ LazyOption, LookupMap};
use near_sdk::json_types::Base64VecU8;
use near_sdk::serde_json::json;

pub use constants::{BASE_URI, DATA_IMAGE_SVG_NEAR_ICON, ONE_NEAR, ONE_YOCTO, SINGLE_CALL_GAS};

#[derive(BorshSerialize, BorshStorageKey)]
enum StorageKey {
    Stars,
    Experience,
    MaximumLevel,
    Rarity,
    NonFungibleToken,
    Metadata,
    TokenMetadata,
    Enumeration,
    Approval,
}

// Token rarity
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub enum Rarity {
    Common,
    Rare,
    Epic,
    Ssr
}

// Hero data
pub struct HeroData {
    name: String,
    media_url: String,
    power: u64,
    health: u64,
    rarity: Rarity, 
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
    rarity: LookupMap<TokenId, Rarity>,
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
            name: "Murkwood Tale's hero NFT".to_string(),
            symbol: "CRTHR".to_string(),
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
            rarity: LookupMap::new(StorageKey::Rarity),
        }                
    }

    // We don't use this method in current version
    #[payable]
    pub fn play(&mut self) -> u8 {
        //let account_id = env::signer_account_id();
        let deposit = env::attached_deposit();
        assert!((deposit > ONE_NEAR), "not enough currency to play");
        
        // Toss the dice
        let rand: u8 = *env::random_seed().get(0).unwrap();
        return rand;
    }

    // Update hero statistics
    pub fn update_hero_stats(&mut self, token_id: TokenId , new_stars: u64, new_experience: u64, new_maximum_level: u64) {
        self.stars.insert(&token_id, &new_stars);
        self.experience.insert(&token_id, &new_experience);
        self.maximum_level.insert(&token_id, &new_maximum_level);
    }

    // Get statistics of a hero
    pub fn get_stats(&self, token_id: TokenId) -> (Option<u64>, Option<u64>, Option<u64>, Option<Rarity>) {
        (self.stars.get(&token_id), self.experience.get(&token_id), self.maximum_level.get(&token_id), self.rarity.get(&token_id))
    }

    // Mint nft ans send them to `username` account
    #[payable]
    pub fn craft_new_hero(&mut self, username: String) -> TokenId {
        // Choose rarity
        let rand: u8 = *env::random_seed().get(0).unwrap();
        let rarity = match rand {
            0..=214 => Rarity::Common,
            215..=240 => Rarity::Rare,
            241..=253 => Rarity::Epic,
            254..=255 => Rarity::Ssr,
        };

        let rand: u8 = *env::random_seed().get(1).unwrap();

        // Define hero
        let hero_data = match rarity {
            Rarity::Common => {
                match rand {
                    0..=41 => HeroData {
                        name: String::from("Krong"),
                        media_url: String::from("UndeadArcherDD"),
                        power: 20,
                        health: 50,
                        rarity: rarity,
                    },
                    42..=83 => HeroData {
                        name: String::from("Freya"),
                        media_url: String::from("DemonRangeDD"),
                        power: 75,
                        health: 440,
                        rarity: rarity,
                    },
                    84..=125 => HeroData {
                        name: String::from("Morg"),
                        media_url: String::from("DemonTank"),
                        power: 50,
                        health: 1000,
                        rarity: rarity,
                    },
                    126..=168 => HeroData {
                        name: String::from("Slay"),
                        media_url: String::from("HumanMeleeDD"),
                        power: 10,
                        health: 400,
                        rarity: rarity,
                    },
                    169..=211 => HeroData {
                        name: String::from("Warg"),
                        media_url: String::from("Warg"),
                        power: 54,
                        health: 1200,
                        rarity: rarity,
                    },
                    _ => HeroData {
                        name: String::from("White Wolf"),
                        media_url: String::from("WhiteWolf"),
                        power: 50,
                        health: 1400,
                        rarity: rarity,
                    },
                }
            },
            Rarity::Rare => {
                match rand {
                    0..=31 => HeroData {
                        name: String::from("Helga"),
                        media_url: String::from("UndeadHeal"),
                        power: 20,
                        health: 50,
                        rarity: rarity,
                    },
                    32..=63 => HeroData {
                        name: String::from("Chi-chi"),
                        media_url: String::from("UndeadSpecialist"),
                        power: 10,
                        health: 50,
                        rarity: rarity,
                    },
                    64..=95 => HeroData {
                        name: String::from("Drakara"),
                        media_url: String::from("DemonHeal"),
                        power: 100,
                        health: 500,
                        rarity: rarity,
                    },
                    96..=127 => HeroData {
                        name: String::from("Hardy"),
                        media_url: String::from("DemonMeleeDD"),
                        power: 90,
                        health: 630,
                        rarity: rarity,
                    },
                    128..=159 => HeroData {
                        name: String::from("Bronks"),
                        media_url: String::from("HumanTank"),
                        power: 10,
                        health: 400,
                        rarity: rarity,
                    },
                    160..=191 => HeroData {
                        name: String::from("Timina"),
                        media_url: String::from("HumanSpecialist"),
                        power: 10,
                        health: 400,
                        rarity: rarity,
                    },
                    192..=223=> HeroData {
                        name: String::from("Gray wolf"),
                        media_url: String::from("NeutralWolf1"),
                        power: 75,
                        health: 670,
                        rarity: rarity,
                    },
                    _ => HeroData {
                        name: String::from("Porcupine"),
                        media_url: String::from("Porcupine"),
                        power: 45,
                        health: 485,
                        rarity: rarity,
                    },
                }
            },
            Rarity::Epic => {
                match rand {
                    0..=84 => HeroData {
                        name: String::from("Unknown"),
                        media_url: String::from("UndeadMeleeDD"),
                        power: 20,
                        health: 5000,
                        rarity: rarity,
                    },
                    85..=169 => HeroData {
                        name: String::from("Berenika"),
                        media_url: String::from("HumanHeal"),
                        power: 10,
                        health: 400,
                        rarity: rarity,
                    },
                    _ => HeroData {
                        name: String::from("Black Bear"),
                        media_url: String::from("BlackBear"),
                        power: 55,
                        health: 1050,
                        rarity: rarity,
                    },
                }
            },
            Rarity::Ssr => {
                match rand {
                    0..=41 => HeroData {
                        name: String::from("Dead King"),
                        media_url: String::from("UndeadTank"),
                        power: 20,
                        health: 5000,
                        rarity: rarity,
                    },
                    42..=83 => HeroData {
                        name: String::from("Luciy"),
                        media_url: String::from("DemonSpecialist"),
                        power: 117,
                        health: 500,
                        rarity: rarity,
                    },
                    84..=125 => HeroData {
                        name: String::from("Iona"),
                        media_url: String::from("HumanRangeDD"),
                        power: 10,
                        health: 400,
                        rarity: rarity,
                    },
                    126..=168 => HeroData {
                        name: String::from("Troll"),
                        media_url: String::from("NeutralTroll"),
                        power: 50,
                        health: 1600,
                        rarity: rarity,
                    },
                    169..=211 => HeroData {
                        name: String::from("Wolf Whelp"),
                        media_url: String::from("WolfWhelp"),
                        power: 50,
                        health: 300,
                        rarity: rarity,
                    },
                    _ => HeroData {
                        name: String::from("Red Dragon"),
                        media_url: String::from("DragonBoss"),
                        power: 20,
                        health: 5000,
                        rarity: rarity,
                    },
                }
            },
        };

        // Generate token_id
        let timestamp: u64 = env::block_timestamp();
        let rand: u8 = *env::random_seed().get(2).unwrap();
        let token_id: String = format!("{}:{}:{}", &hero_data.media_url, rand, timestamp);
        log!("token id: {}", token_id.clone());

        let contract_id = env::current_account_id();
        let root_id = AccountId::try_from(contract_id).unwrap();
        let media_url: String = format!("{}.png", &hero_data.media_url);
        let media_hash = Base64VecU8(env::sha256(media_url.as_bytes()));
        log!("media url: {}", media_url.clone());

        // Default to common token
        let token_metadata = TokenMetadata {
            title: Some(format!("{} {}/{}", &hero_data.name, &hero_data.power, &hero_data.health)),
            description: Some(format!("{}/{}", &hero_data.power, &hero_data.health)),
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

        // Mint NFT   
        self.nft_mint(token_id.clone(), root_id.clone(), token_metadata.clone());

        // Transfer NFT to new owner
        log!("username: {}", username.clone());
        let receiver_id = AccountId::try_from(username).unwrap();
        log!("receiver id: {}", receiver_id.clone());
        log!("token_id: {}", token_id.clone());
        env::promise_create(
            root_id,
            "nft_transfer",
            json!({
                "token_id": token_id.clone(),
                "receiver_id": receiver_id,
            })
            .to_string()
            .as_bytes(),
            ONE_YOCTO,
            SINGLE_CALL_GAS,
        );
        log!("Success! NFT transfering for {}! Token ID = {}", receiver_id.clone(), token_id.clone());

        // Init token stats
        self.stars.insert(&token_id, &0);
        self.experience.insert(&token_id, &0);
        self.maximum_level.insert(&token_id, &0);
        self.rarity.insert(&token_id, &hero_data.rarity);

        token_id
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