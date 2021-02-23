use crate::{Error, mock::*, RawEvent};
use frame_support::{assert_ok, assert_noop};

#[test]
fn it_creates_a_domain() {
	new_test_ext().execute_with(|| {
		assert_ok!(NftModule::create_domain(Origin::signed(100), "ZOD".as_bytes().to_vec(), "Zodiac".as_bytes().to_vec()));
		assert_eq!(last_event(), Event::nft(RawEvent::DomainCreated(100, 0)));
		assert_ok!(NftModule::create_domain(Origin::signed(100), "ZOD".as_bytes().to_vec(), "Zodiac".as_bytes().to_vec()));
		assert_eq!(last_event(), Event::nft(RawEvent::DomainCreated(100, 1)));
	});
}

#[test]
fn it_creates_a_token() {
	new_test_ext().execute_with(|| {
		assert_ok!(NftModule::create_domain(Origin::signed(100), "ZOD".as_bytes().to_vec(), "Zodiac".as_bytes().to_vec()));
		assert_eq!(last_event(), Event::nft(RawEvent::DomainCreated(100, 0)));
		assert_ok!(NftModule::create_token(Origin::signed(100), 0, 101, 32, "https://barkingmad.io/assets".as_bytes().to_vec()));
	});
}
