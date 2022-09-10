/**
* Non Fungible Token NEP-171 Token contract
* Approval NEP-178
* Enumeration NEP-181
* Metadata NEP-177
* Royalties and Payout NEP-199
*
* The aim of the contract is to provide a basic implementation of the improved function NFT standard.
*
* lib.rs is the main entry point.
* nft_core.rs implements NEP-171 standard handles core function regarding nft transfers [Transfers only among users who satisfy dependencies]
* approval.rs implements Approval Management NEP-178 for management of approvals of transfer of NFT and   also implements Marketplace Approval System.
* enumeration.rs implements NEP-181 standard for getter functions to retrieve data off-chain
* mint.rs implements nft_minting functionality
* metadata.rs implements NEP-177 standard for both Contract and NFT-specific metadata.
* indexing.rs extends NEP-297 for better indexing
* events.rs implements the functionality related to events such as issuing NFT passes for an event
* internal.rs contains internal methods.
**/
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LazyOption, LookupMap, UnorderedMap, UnorderedSet};
use near_sdk::json_types::{Base58PublicKey, Base64VecU8, ValidAccountId, U128};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{
    assert_one_yocto, env, ext_contract, near_bindgen, AccountId, Balance, CryptoHash, Gas,
    PanicOnDefault, Promise,
};

use std::collections::HashMap;
use std::mem::size_of;

pub use crate::approval::*;
pub use crate::events::*;
pub use crate::indexing::*;
use crate::internal::*;
pub use crate::metadata::*;
pub use crate::nft_core::NonFungibleTokenCore;
use crate::utils::*;
pub use crate::view::*;
pub use view::*;

mod approval;
mod enumeration;
mod events;
mod indexing;
mod internal;
mod metadata;
mod nft_core;
mod utils;
mod view;

const CATCH_MARKETPLACE_CONTRACT: &str = "marketplace.catchlabs.near";

const CATCH_MARKETPLACE_CONTRACT_LOCAL_NET: &str = "marketplace.catchlabs.test.near";

const CATCH_MARKETPLACE_CONTRACT_TESTNET: &str = "marketplace.catchlabs.testnet";

const BASE_STORAGE_COST: Balance = 10_000_000_000_000_000_000_000; // this is equal to 0.01 NEAR


#[derive(BorshSerialize)]
pub enum StorageKey {
    TokensPerOwner,
    TokenPerOwnerInner { account_id_hash: CryptoHash },
    TokensById,
    ApprovedAccountsPerToken { token_id_hash: CryptoHash },
    TokenMetadataById,
    EventsById,
    ApprovedMarketplaces,
    NFTContractMetadata,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    //contract owner
    pub owner_id: AccountId,

    //keeps track of all the token IDs for a given account
    pub tokens_per_owner: LookupMap<AccountId, UnorderedSet<TokenId>>,

    //keeps track of the token struct for a given token ID
    pub tokens_by_id: LookupMap<TokenId, Token>,

    //keeps track of the token metadata for a given token ID
    pub token_metadata_by_id: UnorderedMap<TokenId, TokenMetadata>,

    //keeps track of events for a given event ID
    pub events_by_id: UnorderedMap<EventId, Event>,

    //keeps track of the approved marketplace contracts
    pub approved_marketplaces: UnorderedSet<AccountId>,

    //keeps track of the metadata for the contract
    pub metadata: LazyOption<NFTContractMetadata>,
}

#[near_bindgen]
impl Contract {
    /// Initialize The Contract
    #[init]
    pub fn new(owner_id: ValidAccountId, metadata: NFTContractMetadata) -> Self {
        metadata.assert_valid_metadata();
        let mut this = Self {
            owner_id: owner_id.into(),

            tokens_per_owner: LookupMap::new(StorageKey::TokensPerOwner.try_to_vec().unwrap()),

            tokens_by_id: LookupMap::new(StorageKey::TokensById.try_to_vec().unwrap()),

            token_metadata_by_id: UnorderedMap::new(
                StorageKey::TokenMetadataById.try_to_vec().unwrap(),
            ),

            events_by_id: UnorderedMap::new(StorageKey::EventsById.try_to_vec().unwrap()),

            approved_marketplaces: UnorderedSet::new(
                StorageKey::ApprovedMarketplaces.try_to_vec().unwrap(),
            ),

            metadata: LazyOption::new(
                StorageKey::NFTContractMetadata.try_to_vec().unwrap(),
                Some(&metadata),
            ),
        };

        let catch_marketplace = AccountId::from(CATCH_MARKETPLACE_CONTRACT_TESTNET);

        this.approved_marketplaces.insert(&catch_marketplace);

        this
    }

    #[init]
    pub fn new_default_meta(owner_id: ValidAccountId) -> Self {
        //calls the other function "new: with some default metadata and the owner_id passed in
        Self::new(
            owner_id,
            NFTContractMetadata {
                spec: "nft-1.0.0".to_string(),
                name: "Catch".to_string(),
                symbol: "CATCH".to_string(),
                icon: Some("data:image/svg+xml;base64,PHN2ZyBpZD0iQ2FwYV8xIiBkYXRhLW5hbWU9IkNhcGEgMSIgeG1sbnM9Imh0dHA6Ly93d3cudzMub3JnLzIwMDAvc3ZnIiB2aWV3Qm94PSIwIDAgMTA3OC41NSAxMDgwIj48ZGVmcz48c3R5bGU+LmNscy0xe2ZpbGw6Izc4NzFmZjt9PC9zdHlsZT48L2RlZnM+PHBhdGggZD0iTTczMSwzNDcuNzJINDI2LjU3YTc4Ljg4LDc4Ljg4LDAsMCwwLTc5LDc5LjA3VjY1My4yNGE3OC44Niw3OC44NiwwLDAsMCw3OSw3OUg3MzFWNjQ5SDQzMC4zMlY0MzEuMDVINzMxWiIvPjxwYXRoIGNsYXNzPSJjbHMtMSIgZD0iTTY2Miw0ODFhNTksNTksMCwwLDAtNTksNTloMGE1OSw1OSwwLDAsMCw1OSw1OWg1LjYzYTU5LDU5LDAsMCwwLDU5LTU5aDBhNTksNTksMCwwLDAtNTktNTlaIi8+PC9zdmc+".to_string()),
                base_uri: "ipfs".to_string(),
                reference: "ipfs://example.com/hash".to_string(),
                reference_hash: Base64VecU8::from([5_u8; 32].to_vec()),
            },
        )
    }

    #[payable]
    pub fn nft_mint(
        &mut self,
        receiver_id: AccountId,
        token_id: TokenId,
        token_metadata: TokenMetadata,
        public_key: Base58PublicKey,
    ) {
        self.assert_owner();

        let initial_storage = env::storage_usage();

        assert_valid_catch_user_account_pattern(&receiver_id);

        Promise::new(receiver_id.clone())
            .create_account()
            .transfer(BASE_STORAGE_COST)
            .add_full_access_key(public_key.into());

        let token = Token {
            token_id: token_id.clone(),
            copies_minted: 1,
            max_copies: 1,
            expires_at: token_metadata.expires_at,
            token_dependency_by_id: vec![],
            event_dependency_by_id: vec![],
            account_approval_info_per_owner: LookupMap::new(
                StorageKey::ApprovedAccountsPerToken {
                    token_id_hash: hash_id(&token_id),
                }
                .try_to_vec()
                .unwrap(),
            ),
        };

        require!(
            self.tokens_by_id.insert(&token_id, &token).is_none(),
            "Token Already exists"
        );

        self.token_metadata_by_id.insert(&token_id, &token_metadata);

        self.internal_add_token_to_owner(&receiver_id, &token_id);

        // refunding deposit
        let storage_used = env::storage_usage() - initial_storage;
        let storage_cost = env::storage_byte_cost() * storage_used as u128;
        let cost = storage_cost + BASE_STORAGE_COST;
        Promise::new(receiver_id).transfer(env::attached_deposit() - cost);
    }
}
