#![cfg_attr(not(feature = "std"), no_std)]
use std::fmt::Debug;
use frame_support::{Parameter, decl_error, decl_event, decl_module, decl_storage, ensure, dispatch::{DispatchError, DispatchResult}, traits::Get};
use frame_system::ensure_signed;
use sp_core::{hexdisplay::AsBytesRef};
use sp_runtime::traits::{AtLeast32BitUnsigned, Zero, MaybeSerializeDeserialize, Member, CheckedAdd};
use sp_std::result::Result;
use codec::{Codec, Encode, Decode};
use sp_runtime::traits::Hash;
#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

pub trait Trait: frame_system::Trait {
	type Balance: Parameter + Member + AtLeast32BitUnsigned + Codec + Default + Copy + MaybeSerializeDeserialize + Debug;
	type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;
	type Identifier: Parameter + Member + AtLeast32BitUnsigned + Codec + Default + Copy + MaybeSerializeDeserialize + Debug + CheckedAdd;
}

#[derive(Encode, Decode, Default, Clone, PartialEq)]
pub struct Domain<Identifier, AccountId> {
	symbol: Vec<u8>,
	name: Vec<u8>,
	next_token_id: Identifier,
	owner: AccountId,
	
}

#[derive(Encode, Decode, Default, Clone, PartialEq)]
pub struct Token<AccountId, Balance> {
	base_uri: Vec<u8>,
	total_supply: Balance,
	creator: AccountId,
}

decl_storage! {
	trait Store for Module<T: Trait> as Worlds {

		pub NextDomainId get(fn next_domain_id): T::Identifier;
		
		pub Domains get(fn domains): 
			map 
			hasher(blake2_128_concat) T::Identifier => Domain<T::Identifier, T::AccountId>; 
		
		pub Tokens get(fn tokens):
			double_map 
			hasher(blake2_128_concat) T::Identifier, 
			hasher(blake2_128_concat) T::Identifier => Token<T::AccountId, T::Balance>;

		/// Not very user friendly as we need to calculate a hash of 
		/// account id and token id to get balance
		pub Balances get(fn balances):
			map
			hasher(blake2_128_concat) T::Hash => T::Balance;
	}
}

decl_event!(
	pub enum Event<T> 
	where 
	AccountId = <T as frame_system::Trait>::AccountId,
	Identifier = <T as Trait>::Identifier {
		DomainCreated(AccountId, Identifier),
	}
);

decl_error! {
	pub enum Error for Module<T: Trait> {
		NoneValue,
		DomainIdOverflow,
		InvalidDomain,
		NotDomainOwner,
		InvalidSymbol,
		InvalidName,
		InvalidTotalSupply,
		InvalidBaseUri,
		BalanceOverflow,
	}
}

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		type Error = Error<T>;
		fn deposit_event() = default;
		#[weight = 10_000 + T::DbWeight::get().writes(1)]
		pub fn create_domain(origin, symbol: Vec<u8>, name: Vec<u8>) {
			ensure!(symbol.len() > 3, Error::<T>::InvalidSymbol);
			ensure!(name.len() > 3, Error::<T>::InvalidName);
			let owner = ensure_signed(origin)?;
			
			let domain = Domain {
				symbol,
				name,
				next_token_id: Zero::zero(),
				owner
			};

			let next = Self::get_next_domain_id()?;
			Domains::<T>::insert(next, domain);
		}

		#[weight = 10_000 + T::DbWeight::get().writes(1)]		
		pub fn create_token(origin, domain_id: T::Identifier, creator: T::AccountId, total_supply: T::Balance, base_uri: Vec<u8>) -> DispatchResult {
			let who = ensure_signed(origin)?;
			ensure!(total_supply > 0.into(), Error::<T>::InvalidTotalSupply);
			ensure!(base_uri.len() > 3, Error::<T>::InvalidBaseUri);			
				
			Domains::<T>::try_mutate(domain_id, |domain| {
				ensure!(domain.owner == who, Error::<T>::NotDomainOwner);			
				let token = Token {
					base_uri,
					total_supply,
					creator: creator.clone(),
				};
				let next = domain.next_token_id.checked_add(&1u32.into()).ok_or(Error::<T>::DomainIdOverflow)?;
				domain.next_token_id = next;
				Tokens::<T>::insert(domain_id, next, token);
				Self::mint(creator, next, total_supply)?;
				Ok(())
			})
		}
	}
}

impl <T: Trait> Module<T> {
	fn mint(to: T::AccountId, token_id: T::Identifier, quantity: T::Balance) -> Result<T::Balance, DispatchError> {
		let mut encoded = to.encode();
		encoded.append(&mut token_id.encode());
		let hash = T::Hashing::hash(encoded.as_bytes_ref());
		Balances::<T>::try_mutate_exists(hash, |maybe_balance| {
			let balance = maybe_balance.take().ok_or(Error::<T>::InvalidDomain)?;
			let new_balance = balance.checked_add(&quantity).ok_or(Error::<T>::BalanceOverflow)?;
			Ok(new_balance)
		})
	}

	fn get_next_domain_id() -> Result<T::Identifier, DispatchError> {
		NextDomainId::<T>::try_mutate(|next_id| -> Result<T::Identifier, DispatchError> {
			let current_id : <T as Trait>::Identifier = *next_id;
			*next_id = next_id.checked_add(&0u32.into()).ok_or(Error::<T>::DomainIdOverflow)?;
			Ok(current_id)
		})
	}
}