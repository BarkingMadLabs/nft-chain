use crate::{Error, mock::*, RawEvent};
use frame_support::{assert_ok, assert_noop};

const ALICE: u64 = 100;
const BOB: u64 = 101;
const DOMAIN_ID: u64 = 0;
const TICKER: &str = "ZOD";
const NAME: &str = "Zodiac";
const BASE_URI: &str = "https://barkingmad.io/assets";
const NUM_TOKENS: u64 = 32;

#[test]
fn it_creates_a_domain() {
	new_test_ext().execute_with(|| {
		// Confirm we can't create a domain without a valid ticker
		assert_noop!(NftModule::create_domain(Origin::signed(100), "".as_bytes().to_vec(), NAME.as_bytes().to_vec()), Error::<Test>::InvalidSymbol);
		// Confirm we can't create a domain without a valid name
		assert_noop!(NftModule::create_domain(Origin::signed(100), TICKER.as_bytes().to_vec(), "".as_bytes().to_vec()), Error::<Test>::InvalidName);
		// Create domain ZOD "Zodiac"
		assert_ok!(NftModule::create_domain(Origin::signed(100), TICKER.as_bytes().to_vec(), NAME.as_bytes().to_vec()));
		// Event for domain creation with owner and domain id
		assert_eq!(last_event(), Event::nft(RawEvent::DomainCreated(100, 0)));
		// Create domain ZOD "Zodiac"
		assert_ok!(NftModule::create_domain(Origin::signed(100), TICKER.as_bytes().to_vec(), NAME.as_bytes().to_vec()));
		// Event for domain creation with owner and domain id + 1
		assert_eq!(last_event(), Event::nft(RawEvent::DomainCreated(100, 1)));
	});
}

#[test]
fn it_creates_a_token() {
	new_test_ext().execute_with(|| {
		// Create domain ZOD "Zodiac"
		assert_ok!(NftModule::create_domain(Origin::signed(ALICE), TICKER.as_bytes().to_vec(), NAME.as_bytes().to_vec()));
		// Event for domain creation with owner and domain id
		assert_eq!(last_event(), Event::nft(RawEvent::DomainCreated(ALICE, 0)));
		// Confirm we can't create a token without an amount
		assert_noop!(NftModule::create_token(Origin::signed(ALICE), DOMAIN_ID, BOB, 0, BASE_URI.as_bytes().to_vec()), Error::<Test>::InvalidTotalSupply);		
		// Confirm we can't create a token without a valid base uri
		assert_noop!(NftModule::create_token(Origin::signed(ALICE), DOMAIN_ID, BOB, NUM_TOKENS, "".as_bytes().to_vec()), Error::<Test>::InvalidBaseUri);		
		// Confirm we can't create a token if not domain owner
		assert_noop!(NftModule::create_token(Origin::signed(BOB), DOMAIN_ID, BOB, NUM_TOKENS, BASE_URI.as_bytes().to_vec()), Error::<Test>::NotDomainOwner);		
		// Confirm we can't create a token if not a valid domain
		assert_noop!(NftModule::create_token(Origin::signed(ALICE), 1, BOB, NUM_TOKENS, BASE_URI.as_bytes().to_vec()), Error::<Test>::NotDomainOwner);		
		// Create token for new domain id, creator being BOB with asset url and mint to BOB 32 tokens
		assert_ok!(NftModule::create_token(Origin::signed(ALICE), DOMAIN_ID, BOB, NUM_TOKENS, BASE_URI.as_bytes().to_vec()));
		// Check storage for token and supply is correct
		assert_eq!(NftModule::tokens(DOMAIN_ID, 1).total_supply, NUM_TOKENS);
		// Check Bob's balance, should be all of them
		assert_eq!(NftModule::balances(BOB, (DOMAIN_ID, 1)), NUM_TOKENS);
		// Event for token creation with id 1 minted NUM_TOKENS for BOB
		assert_eq!(last_event(), Event::nft(RawEvent::TokenCreated(BOB, DOMAIN_ID, 1, NUM_TOKENS)));
		// Check BOB's balance for Domain id and token id 1, it should be 32
		assert_eq!(NftModule::balances(BOB, (DOMAIN_ID, 1)), NUM_TOKENS);
	});
}

#[test]
fn it_burns_tokens() {
	new_test_ext().execute_with(|| {
		// Create domain ZOD "Zodiac"
		assert_ok!(NftModule::create_domain(Origin::signed(ALICE), TICKER.as_bytes().to_vec(), NAME.as_bytes().to_vec()));
		// Event for domain creation with owner and domain id
		assert_eq!(last_event(), Event::nft(RawEvent::DomainCreated(ALICE, DOMAIN_ID)));
		// Create token for new domain id, creator being BOB with asset url and mint to BOB 32 tokens
		assert_ok!(NftModule::create_token(Origin::signed(ALICE), DOMAIN_ID, BOB, NUM_TOKENS, BASE_URI.as_bytes().to_vec()));
		// Event for token creation with id 1 minted 32 for BOB
		assert_eq!(last_event(), Event::nft(RawEvent::TokenCreated(BOB, DOMAIN_ID, 1, NUM_TOKENS)));
		// Burn 16 tokens from BOB by BOB
		assert_ok!(NftModule::burn_tokens(Origin::signed(ALICE), DOMAIN_ID, 1, BOB, NUM_TOKENS / 2));
		// Event for token burning 
		assert_eq!(last_event(), Event::nft(RawEvent::TokensBurnt(BOB, DOMAIN_ID, 1, NUM_TOKENS / 2)));
	});
}
