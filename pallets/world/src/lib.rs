#![cfg_attr(not(feature = "std"), no_std)]
use std::fmt::Debug;
use frame_support::{decl_module, decl_storage, decl_event, decl_error, Parameter, dispatch::{DispatchError, DispatchResult}, traits::Get};
use frame_system::ensure_signed;
use sp_runtime::traits::{AtLeast32BitUnsigned, MaybeSerializeDeserialize, Member, CheckedAdd};
use sp_std::result::Result;
use codec::{Codec, Encode, Decode};
#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

pub trait Trait: frame_system::Trait {
	type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;
	type Identifier: Parameter + Member + AtLeast32BitUnsigned + Codec + Default + Copy + MaybeSerializeDeserialize + Debug + CheckedAdd;
}

#[derive(Encode, Decode, Default, Clone, PartialEq)]
pub struct Contract<Identifier> {
	symbol: Vec<u8>,
	name: Vec<u8>,
	counter: Identifier,
	tokens: Option<Vec<Identifier>>,
}

#[derive(Encode, Decode, Default, Clone, PartialEq)]
pub struct Token<Identifier> {
	id: Identifier,
	base_uri: Vec<u8>,
	total_supply: Identifier,
}

decl_storage! {
	trait Store for Module<T: Trait> as Worlds {
		pub NextContractId get(fn next_contract_id): T::Identifier;
		pub Contracts get(fn contracts): map hasher(blake2_128_concat) T::Identifier => Contract<T::Identifier>; 
		pub Owners get(fn owners): map hasher(blake2_128_concat) T::Identifier => T::AccountId;
	}
}

decl_event!(
	pub enum Event<T> 
	where 
	AccountId = <T as frame_system::Trait>::AccountId,
	Identifier = <T as Trait>::Identifier {
		ContractCreated(AccountId, Identifier),
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

			let contract = Contract::<T::Identifier> {
				symbol,
				name,
				counter: 0u32.into(),
				tokens: None,
			};

			let next = Self::next_contract_id();
			Contracts::<T>::insert(next, contract);
			Owners::<T>::insert(next, who);
		}
	}
}

impl <T: Trait> Module<T> {
	fn get_next_contract_id() -> Result<T::Identifier, DispatchError> {
		NextContractId::<T>::try_mutate(|next_id| -> Result<T::Identifier, DispatchError> {
			let current_id : <T as Trait>::Identifier = *next_id;
			*next_id = next_id.checked_add(&0u32.into()).ok_or(Error::<T>::ContractIdOverflow)?;
			Ok(current_id)
		})
	}
}