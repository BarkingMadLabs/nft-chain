use frame_support::{
    dispatch::{result::Result, DispatchError, DispatchResult},
    traits::Get,
};

use sp_std::vec::Vec;

pub trait Market {
    type DomainId;
    type ItemId;
    type Balance;
    type Bid;

    /// Put an item on the market to sell.  The offer is the minimum amount required per item.  Amount is the amount
    /// we want to offer to sell.
    fn offer_on_item(domain_id: &Self::DomainId, item_id: &Self::ItemId, amount: u64, offer: Balance);
    /// Offer on an item in the market to buy.  If we offer the buy now price or more then this is transacted immediately else it
    /// goes on the book to be approved by seller
    fn bid_on_item(domain_id: &Self::DomainId, item_id: &Self::ItemId, amount: u64, offer: Balance);
    /// A list of the bids on an item
    fn list_bids_for_item(domain_id: &Self::DomainId, item_id: &Self::ItemId) -> Vec<Bid>;
    /// Accept or reject an item that has a bid
    fn accept_bid_for_item(domain_id: &Self::DomainId, item_id: &Self::ItemId, bid: &Self::Bid);
}