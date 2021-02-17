#![cfg_attr(not(feature = "std"), no_std)]
use frame_support::{decl_module, decl_storage, decl_event, decl_error, dispatch::{DispatchError, DispatchResult}, traits::Get};
use frame_system::ensure_signed;
use frame_support::codec::{Decode, Encode};
use sp_runtime::traits::AtLeast32BitUnsigned;
use sp_std::result::Result;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

pub trait Trait: frame_system::Trait {
	type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;
	type Index: AtLeast32BitUnsigned;
}

type Identifier = u32;

#[derive(Encode, Decode, Default, Clone, PartialEq)]
pub struct Contract {
	symbol: Vec<u8>,
	name: Vec<u8>,
	counter: Identifier,
	tokens: Option<Vec<Identifier>>,
}

#[derive(Encode, Decode, Default, Clone, PartialEq)]
pub struct Token {
	id: Identifier,
	base_uri: Vec<u8>,
	total_supply: Identifier,
}

decl_storage! {
	trait Store for Module<T: Trait> as Worlds {
		pub NextContractId get(fn next_contract_id): Index;
		pub Contracts get(fn contracts): map hasher(blake2_128_concat) Index => Contract; 
		pub Owners get(fn owners): map hasher(blake2_128_concat) Index => T::AccountId;
	}
}

decl_event!(
	pub enum Event<T> where AccountId = <T as frame_system::Trait>::AccountId {
		ContractCreated(Identifier, AccountId),
	}
);

decl_error! {
	pub enum Error for Module<T: Trait> {
		NoneValue,
		ContractIdOverflow,
	}
}

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		type Error = Error<T>;
		fn deposit_event() = default;
		#[weight = 10_000 + T::DbWeight::get().writes(1)]
		pub fn create_contract(origin, symbol: Vec<u8>, name: Vec<u8>) {
			let who = ensure_signed(origin)?;

			let contract = Contract {
				symbol,
				name,
				counter: 0u32.into(),
				tokens: None,
			};

			let next: Identifier = Self::next_contract_id();
			Contracts::insert(next, contract);
		}
	}
}

impl <T: Trait> Module<T> {
	fn get_next_contract_id() -> Result<Identifier, DispatchError> {
		NextContractId::try_mutate(|next_id| -> Result<Identifier, DispatchError> {
			let current_id = *next_id;
			*next_id = next_id.checked_add(1).ok_or(Error::<T>::ContractIdOverflow)?;
			Ok(current_id)
		})
	}
}