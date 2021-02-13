use frame_support::{
    dispatch::{result::Result, DispatchError, DispatchResult},
    traits::Get,
};

use sp_std::vec::Vec;

pub trait World<AccountId> {
    type WorldId;
    type TokenId;
    type MetaData;

    /// Create a world which is owned by this account
    fn create(owner: &AccountId) -> Result<Self::WorldId, DispatchError>;
    /// Mint items for this world, minting 1 would make the item unique
    fn mint(world_id: &Self::WorldId, amount: u64, data: Option<Self::MetaData>) -> Result<Self::TokenId, DispatchError>;
    /// Total minted for a token
    fn total(world_id: &Self::WorldId, token_id: &Self::TokenId) -> u128;
    /// Total burned for a token
    fn burnt(world_id: &Self::WorldId, token_id: &Self::TokenId) -> u128;
    /// Balance of token for user
    fn balance(world_id: &Self::WorldId, token_id: &Self::TokenId, owner: &AccountId) -> u128;
    /// Owner of token
    fn owner_of(world_id: &Self::WorldId, token_id: &Self::TokenId) -> AccountId;
    /// Transfer token from owner to new owner
    fn transfer(world_id: &Self::WorldId, token_id: &Self::TokenId, amount: u64, to: &AccountId) -> Result<(), DispatchError>;
    /// Transfer multiple tokens
    fn transfer_multiple(world_id: &Self::WorldId, token_ids: Vec<Self::TokenId>, amount: Vec<u64>, to: Vec<AccountId>) -> Result<(), DispatchError>;
    /// Approval account for token
    fn approval(world_id: &Self::WorldId, token_id: &Self::TokenId, approved: &AccountId) -> Result<(), DispatchError>;
    /// Approval account for multiple tokena
    fn approval_multiple(world_id: &Self::WorldId, token_ids: Vec<Self::TokenId>, approved: &AccountId) -> Result<(), DispatchError>;
    /// Burn tokens
    fn burn(world_id: &Self::WorldId, token_id: &Self::TokenId, amount: u64) -> Result<(), DispatchError>;
    /// Data for token
    fn data(world_id: &Self::WorldId, token_id: &Self::TokenId) -> Option<MetaData>;
}