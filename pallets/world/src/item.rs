use frame_support::{
    dispatch::{result::Result, DispatchError, DispatchResult},
    traits::Get,
};

use sp_std::vec::Vec;

pub trait Domain<AccountId> {
    type DomainId;
    type ItemId;
    type MetaData;

    /// Create a domain which is owned by this account
    fn create_domain(owner: &AccountId) -> Result<Self::DomainId, DispatchError>;
    /// Mint items for this domain, minting 1 would make the item unique
    fn mint_item(domain_id: &Self::DomainId, amount: u64, data: Option<Self::MetaData>) -> Result<Self::ItemId, DispatchError>;
    /// Total minted for a item
    fn total_items_for_domain(domain_id: &Self::DomainId, item_id: &Self::ItemId) -> u128;
    /// Total burned for a item
    fn burnt_items_for_domain(domain_id: &Self::DomainId, item_id: &Self::ItemId) -> u128;
    /// Balance of item for user
    fn balance_for_user(domain_id: &Self::DomainId, item_id: &Self::ItemId, owner: &AccountId) -> u128;
    /// Owner of item
    fn owner_of_item(domain_id: &Self::DomainId, item_id: &Self::ItemId) -> AccountId;
    /// Transfer item from owner to new owner
    fn transfer_item(domain_id: &Self::DomainId, item_id: &Self::ItemId, amount: u64, to: &AccountId) -> Result<(), DispatchError>;
    /// Transfer multiple item
    fn transfer_multiple_items(domain_id: &Self::DomainId, item_ids: Vec<Self::ItemId>, amount: Vec<u64>, to: Vec<AccountId>) -> Result<(), DispatchError>;
    /// Approval account for item
    fn approval_for_item(domain_id: &Self::DomainId, item_id: &Self::ItemId, approved: &AccountId) -> Result<(), DispatchError>;
    /// Approval account for multiple item
    fn approval_multiple_for_items(domain_id: &Self::DomainId, item_ids: Vec<Self::ItemId>, approved: &AccountId) -> Result<(), DispatchError>;
    /// Burn items
    fn burn_items(domain_id: &Self::DomainId, item_id: &Self::ItemId, amount: u64) -> Result<(), DispatchError>;
    /// Data for item
    fn data_for_item(domain_id: &Self::DomainId, item_id: &Self::ItemId) -> Option<Self::MetaData>;
}