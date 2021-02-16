#![cfg_attr(not(feature = "std"), no_std)]
use frame_support::{decl_module, decl_storage, decl_event, decl_error, dispatch, traits::Get};
use frame_system::ensure_signed;
pub mod ownable;
pub mod item;
pub mod market;

pub use crate::ownable::Ownable;
pub use crate::market::Market;
pub use crate::item::Domain;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

pub trait Trait: frame_system::Trait {
	type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;
}

pub type ItemId<T> = <T as frame_system::Trait>::Hash;
pub type DomainId<T> = <T as frame_system::Trait>::Hash;

decl_storage! {
	trait Store for Module<T: Trait> as Worlds {
		
		ItemsForDomain get(fn items_for_domain): map hasher(blake2_128_concat) DomainId<T> => Vec<ItemId<T>>;
        DomainsForAccount get(fn domains_for_account): map hasher(blake2_128_concat) T::AccountId => DomainId<T>;
	}
}

decl_event!(
	pub enum Event<T> where AccountId = <T as frame_system::Trait>::AccountId {
		SomethingStored(u32, AccountId),
	}
);

decl_error! {
	pub enum Error for Module<T: Trait> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
	}
}

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		type Error = Error<T>;
		fn deposit_event() = default;
		#[weight = 10_000 + T::DbWeight::get().writes(1)]
		pub fn do_something(origin, something: u32) -> dispatch::DispatchResult {
			let who = ensure_signed(origin)?;
			Ok(())
		}
	}
}

// impl<T: Trait> Domain<T::AccountId> for Module<T> {
//     fn create_domain(owner: &AccountId) -> Result<Self::DomainId, DispatchError> {

// 	}
//     fn mint_item(domain_id: &Self::DomainId, amount: u64, data: Option<Self::MetaData>) -> Result<Self::ItemId, DispatchError> {

// 	}
//     fn total_items_for_domain(domain_id: &Self::DomainId, item_id: &Self::ItemId) -> u128 {

// 	}
//     fn burnt_items_for_domain(domain_id: &Self::DomainId, item_id: &Self::ItemId) -> u128 {

// 	}
//     fn balance_for_user(domain_id: &Self::DomainId, item_id: &Self::ItemId, owner: &AccountId) -> u128 {

// 	}
//     fn owner_of_item(domain_id: &Self::DomainId, item_id: &Self::ItemId) -> AccountId {

// 	}
//     fn transfer_item(domain_id: &Self::DomainId, item_id: &Self::ItemId, amount: u64, to: &AccountId) -> Result<(), DispatchError> {

// 	}
//     fn transfer_multiple_items(domain_id: &Self::DomainId, item_ids: Vec<Self::ItemId>, amount: Vec<u64>, to: Vec<AccountId>) -> Result<(), DispatchError> {

// 	}
//     fn approval_for_item(domain_id: &Self::DomainId, item_id: &Self::ItemId, approved: &AccountId) -> Result<(), DispatchError> {

// 	}
//     fn approval_multiple_for_items(domain_id: &Self::DomainId, item_ids: Vec<Self::ItemId>, approved: &AccountId) -> Result<(), DispatchError> {

// 	}
//     fn burn_items(domain_id: &Self::DomainId, item_id: &Self::ItemId, amount: u64) -> Result<(), DispatchError> {

// 	}
//     fn data_for_item(domain_id: &Self::DomainId, item_id: &Self::ItemId) -> Option<Self::MetaData> {

// 	}
// }
