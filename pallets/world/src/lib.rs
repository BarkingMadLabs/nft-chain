#![cfg_attr(not(feature = "std"), no_std)]
use std::fmt::Debug;
use frame_support::{Parameter, decl_error, decl_event, decl_module, decl_storage, ensure, dispatch::{DispatchError}, traits::Get};
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
pub struct Contract {
	symbol: Vec<u8>,
	name: Vec<u8>,
}

#[derive(Encode, Decode, Default, Clone, PartialEq)]
pub struct Token<AccountId> {
	base_uri: Vec<u8>,
	total_supply: u128,
	creator: AccountId,
}

decl_storage! {
	trait Store for Module<T: Trait> as Worlds {

		pub NextContractId get(fn next_contract_id): 
			T::Identifier;
		
		pub NextTokenId get(fn next_token_id): 
			map
			hasher(blake2_128_concat) T::Identifier => T::Identifier;

		pub Contracts get(fn contracts): 
			map 
			hasher(blake2_128_concat) T::Identifier => Contract; 
		
		pub Owners get(fn owners): 
			map 
			hasher(blake2_128_concat) T::Identifier => T::AccountId;
		
		pub Tokens get(fn tokens):
			double_map 
			hasher(blake2_128_concat) T::Identifier, 
			hasher(blake2_128_concat) T::Identifier => Token<T::AccountId>;
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
		InvalidContract,
		NotContractOwner,
		InvalidSymbol,
		InvalidName,
	}
}

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		type Error = Error<T>;
		fn deposit_event() = default;
		#[weight = 10_000 + T::DbWeight::get().writes(1)]
		pub fn create_contract(origin, symbol: Vec<u8>, name: Vec<u8>) {
			ensure!(symbol.len() > 3, Error::<T>::InvalidSymbol);
			ensure!(name.len() > 3, Error::<T>::InvalidName);
			let who = ensure_signed(origin)?;
			
			let contract = Contract {
				symbol,
				name,
			};

			let next = Self::get_next_contract_id()?;
			Contracts::<T>::insert(next, contract);
			Owners::<T>::insert(next, who);
		}

		#[weight = 10_000 + T::DbWeight::get().writes(1)]		
		pub fn create_token(origin, contract_id: T::Identifier, creator: T::AccountId, total_supply: u128, base_uri: Vec<u8>) {
			let who = ensure_signed(origin)?;
			ensure!(Contracts::<T>::contains_key(&contract_id), Error::<T>::InvalidContract);
			ensure!(Owners::<T>::get(&contract_id) == who, Error::<T>::NotContractOwner);
			let token = Token {
				base_uri,
				total_supply,
				creator,
			};
			let next = Self::get_next_token_id(&contract_id)?;
			Tokens::<T>::insert(contract_id, next, token);
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
	fn get_next_token_id(contract_id: &T::Identifier) -> Result<T::Identifier, DispatchError> {
		NextTokenId::<T>::try_mutate_exists(contract_id, |maybe_id| {
			let current_id = maybe_id.take().ok_or(Error::<T>::InvalidContract)?;
			let new_id = current_id.checked_add(&1u32.into()).ok_or(Error::<T>::ContractIdOverflow)?;
			Ok(new_id)
		})
	}
}