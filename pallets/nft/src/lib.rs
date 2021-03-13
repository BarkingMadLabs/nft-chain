#![cfg_attr(not(feature = "std"), no_std)]
use frame_support::{Parameter, decl_error, decl_event, decl_module, decl_storage, dispatch::{DispatchError, DispatchResult}, ensure, traits::{Get}};
use frame_system::ensure_signed;
use sp_runtime::traits::{AtLeast32BitUnsigned, Zero, One, MaybeSerializeDeserialize, Member, CheckedAdd, CheckedSub};
use sp_std::result::Result;
use sp_std::fmt::Debug;
use sp_std::vec::Vec;
use codec::{Codec, Encode, Decode};
#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

pub trait Trait: frame_system::Trait {
	type Balance: Parameter + Member + AtLeast32BitUnsigned + Codec + Default + Copy + MaybeSerializeDeserialize + Debug;
	type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;
	type DomainId: Parameter + Member + AtLeast32BitUnsigned + Codec + Default + Copy + MaybeSerializeDeserialize + Debug + CheckedAdd;
	type TokenId: Parameter + Member + AtLeast32BitUnsigned + Codec + Default + Copy + MaybeSerializeDeserialize + Debug + CheckedAdd;
}

#[derive(Encode, Decode, Default, Clone, PartialEq)]
pub struct Domain<TokenId, AccountId> {
	symbol: Vec<u8>,
	name: Vec<u8>,
	next_token_id: TokenId,
	owner: AccountId,
	
}

#[derive(Encode, Decode, Default, Clone, PartialEq)]
pub struct Token<AccountId, Balance> {
	base_uri: Vec<u8>,
	total_supply: Balance,
	creator: AccountId,
}

decl_storage! {
	trait Store for Module<T: Trait> as NFT {

		pub NextDomainId get(fn next_domain_id): T::DomainId;
		
		pub Numbers get(fn numbers):
			double_map
			hasher(blake2_128_concat) T::AccountId,
			hasher(blake2_128_concat) (T::DomainId, T::TokenId) => T::Balance;

		pub Domains get(fn domains): 
			map 
			hasher(blake2_128_concat) T::DomainId => Domain<T::TokenId, T::AccountId>; 
		
		pub Tokens get(fn tokens):
			double_map 
			hasher(blake2_128_concat) T::DomainId,
			hasher(blake2_128_concat) T::TokenId => Token<T::AccountId, T::Balance>;

		pub Balances get(fn balances):
			double_map 
			hasher(blake2_128_concat) T::AccountId,
			hasher(blake2_128_concat) (T::DomainId, T::TokenId) => T::Balance;
	}
}

decl_event!(
	pub enum Event<T> 
	where 
	AccountId = <T as frame_system::Trait>::AccountId,
	DomainId = <T as Trait>::DomainId,
	TokenId = <T as Trait>::TokenId,
	Balance = <T as Trait>::Balance {
		DomainCreated(AccountId, DomainId),
		TokenCreated(AccountId, DomainId, TokenId, Balance),
		TokensBurnt(AccountId, DomainId, TokenId, Balance),
	}
);

decl_error! {
	pub enum Error for Module<T: Trait> {
		NoneValue,
		DomainIdOverflow,
		TokenIdOverflow,
		InvalidDomain,
		NotDomainOwner,
		InvalidSymbol,
		InvalidName,
		InvalidTotalSupply,
		InvalidQuantityToBurn,
		InvalidBaseUri,
		BalanceOverflow,
	}
}

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		type Error = Error<T>;
		fn deposit_event() = default;

		#[weight = 10_000 + T::DbWeight::get().writes(1)]
		pub fn add_number(origin, domain_id: T::DomainId, token_id: T::TokenId, number: T::Balance) {
			let owner = ensure_signed(origin)?;
			Numbers::<T>::insert(owner.clone(), (domain_id, token_id), number);
		}

		#[weight = 10_000 + T::DbWeight::get().writes(1)]
		pub fn create_domain(origin, symbol: Vec<u8>, name: Vec<u8>) {
			ensure!(symbol.len() > 2, Error::<T>::InvalidSymbol);
			ensure!(name.len() > 3, Error::<T>::InvalidName);
			let owner = ensure_signed(origin)?;
			
			let domain = Domain {
				symbol,
				name,
				next_token_id: Zero::zero(),
				owner: owner.clone()
			};

			let next = Self::get_next_domain_id()?;
			Domains::<T>::insert(next, domain);
			Self::deposit_event(RawEvent::DomainCreated(owner, next));
		}

		#[weight = 10_000 + T::DbWeight::get().writes(1)]		
		pub fn create_token(origin, domain_id: T::DomainId, creator: T::AccountId, total_supply: T::Balance, base_uri: Vec<u8>) -> DispatchResult {
			let who = ensure_signed(origin)?;
			ensure!(total_supply > Zero::zero(), Error::<T>::InvalidTotalSupply);
			ensure!(base_uri.len() > 3, Error::<T>::InvalidBaseUri);			

			Domains::<T>::try_mutate(domain_id, |domain| {
				ensure!(domain.owner == who, Error::<T>::NotDomainOwner);			
				let token = Token {
					base_uri,
					total_supply,
					creator: creator.clone(),
				};
				let next_token_id = domain.next_token_id.checked_add(&One::one()).ok_or(Error::<T>::TokenIdOverflow)?;
				domain.next_token_id = next_token_id;
				Tokens::<T>::insert(domain_id, next_token_id, token);
				Self::mint(creator.clone(), domain_id, next_token_id, total_supply)?;
				Self::deposit_event(RawEvent::TokenCreated(creator, domain_id, next_token_id, total_supply));
				Ok(())
			})
		}

		#[weight = 10_000 + T::DbWeight::get().writes(1)]		
		pub fn burn_tokens(origin, domain_id: T::DomainId, token_id: T::TokenId, victim: T::AccountId, quantity: T::Balance) -> DispatchResult {
			let who = ensure_signed(origin)?;
			ensure!(quantity > Zero::zero(), Error::<T>::InvalidQuantityToBurn);	
			let domain = Domains::<T>::get(domain_id);
			ensure!(domain.owner == who, Error::<T>::NotDomainOwner);			
			Self::burn(victim.clone(), domain_id, token_id, quantity)?;
			Self::deposit_event(RawEvent::TokensBurnt(victim, domain_id, token_id, quantity));
			Ok(())
		}
	}
}

impl <T: Trait> Module<T> {
	fn mint(to: T::AccountId, domain_id: T::DomainId, token_id: T::TokenId, quantity: T::Balance) -> Result<T::Balance, DispatchError> {
		ensure!(Domains::<T>::contains_key(domain_id), Error::<T>::InvalidDomain);
		Balances::<T>::try_mutate(to, (domain_id, token_id), |balance| {			
			let new_balance = balance.checked_add(&quantity).ok_or(Error::<T>::BalanceOverflow)?;
			*balance = new_balance; 
			Ok(new_balance)
		})	
	}

	fn burn(from: T::AccountId, domain_id: T::DomainId, token_id: T::TokenId, quantity: T::Balance) -> Result<T::Balance, DispatchError> {
		ensure!(Domains::<T>::contains_key(domain_id), Error::<T>::InvalidDomain);
		Balances::<T>::try_mutate(from, (domain_id, token_id), |balance| {
			let new_balance = balance.checked_sub(&quantity).ok_or(Error::<T>::BalanceOverflow)?;
			*balance = new_balance; 
			Ok(new_balance)
		})	
	}

	fn get_next_domain_id() -> Result<T::DomainId, DispatchError> {
		NextDomainId::<T>::try_mutate(|next_id| -> Result<T::DomainId, DispatchError> {
			let current_id : <T as Trait>::DomainId = *next_id;
			*next_id = next_id.checked_add(&One::one()).ok_or(Error::<T>::DomainIdOverflow)?;
			Ok(current_id)
		})
	}
}