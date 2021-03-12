use crate::{Error, mock::*, RawEvent};
use frame_support::{assert_ok, assert_noop};

const ALICE: u64 = 100;
const BOB: u64 = 101;
const DOMAIN_ID: u64 = 0;

#[test]
fn it_creates_a_domain() {
	new_test_ext().execute_with(|| {
		// Create domain ZOD "Zodiac"
		assert_ok!(NftModule::create_domain(Origin::signed(100), "ZOD".as_bytes().to_vec(), "Zodiac".as_bytes().to_vec()));
		// Event for domain creation with owner and domain id
		assert_eq!(last_event(), Event::nft(RawEvent::DomainCreated(100, 0)));
		assert_ok!(NftModule::create_domain(Origin::signed(100), "ZOD".as_bytes().to_vec(), "Zodiac".as_bytes().to_vec()));
		assert_eq!(last_event(), Event::nft(RawEvent::DomainCreated(100, 1)));
	});
}

#[test]
fn it_creates_a_token() {
	new_test_ext().execute_with(|| {
		// Create domain ZOD "Zodiac"
		assert_ok!(NftModule::create_domain(Origin::signed(ALICE), "ZOD".as_bytes().to_vec(), "Zodiac".as_bytes().to_vec()));
		// Event for domain creation with owner and domain id
		assert_eq!(last_event(), Event::nft(RawEvent::DomainCreated(ALICE, 0)));
		// Create token for new domain id, creator being BOB with asset url and mint to BOB 32 tokens
		assert_ok!(NftModule::create_token(Origin::signed(ALICE), DOMAIN_ID, BOB, 32, "https://barkingmad.io/assets".as_bytes().to_vec()));
		// Event for token creation with id 1 minted 32 for BOB
		assert_eq!(last_event(), Event::nft(RawEvent::TokenCreated(BOB, DOMAIN_ID, 1, 32)));
		// Check BOB's balance for Domain id and token id 1, it should be 32
		assert_eq!(NftModule::balances(BOB, (DOMAIN_ID, 1)), 32);
	});
}

#[test]
fn it_burns_tokens() {
	new_test_ext().execute_with(|| {
		// Create domain ZOD "Zodiac"
		assert_ok!(NftModule::create_domain(Origin::signed(ALICE), "ZOD".as_bytes().to_vec(), "Zodiac".as_bytes().to_vec()));
		// Event for domain creation with owner and domain id
		assert_eq!(last_event(), Event::nft(RawEvent::DomainCreated(ALICE, DOMAIN_ID)));
		// Create token for new domain id, creator being BOB with asset url and mint to BOB 32 tokens
		assert_ok!(NftModule::create_token(Origin::signed(ALICE), DOMAIN_ID, BOB, 32, "https://barkingmad.io/assets".as_bytes().to_vec()));
		// Event for token creation with id 1 minted 32 for BOB
		assert_eq!(last_event(), Event::nft(RawEvent::TokenCreated(BOB, DOMAIN_ID, 1, 32)));
		// Burn 16 tokens from BOB by BOB
		assert_ok!(NftModule::burn_tokens(Origin::signed(ALICE), DOMAIN_ID, 1, BOB, 16));
		// Event for token burning 
		assert_eq!(last_event(), Event::nft(RawEvent::TokensBurnt(BOB, DOMAIN_ID, 1, 16)));
	});
}
