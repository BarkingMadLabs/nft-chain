use frame_support::{
    dispatch::{result::Result, DispatchError, DispatchResult},
    traits::Get,
};

use sp_std::vec::Vec;

pub trait Ownable<AccountId> {
    fn owner() -> AccountId;
    fn set_owner(owner: &AccountId) -> AccountId;
}
