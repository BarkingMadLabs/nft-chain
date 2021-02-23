use crate::{Error, mock::*, RawEvent};
use frame_support::{assert_ok, assert_noop};

#[test]
fn it_creates_a_domain() {
	new_test_ext().execute_with(|| {
		assert_ok!(WorldModule::create_domain(Origin::signed(100), "ZOD".as_bytes().to_vec(), "Zodiac".as_bytes().to_vec()));
		assert_eq!(last_event(), Event::world(RawEvent::DomainCreated(100, 0)));
		assert_ok!(WorldModule::create_domain(Origin::signed(100), "ZOD".as_bytes().to_vec(), "Zodiac".as_bytes().to_vec()));
		assert_eq!(last_event(), Event::world(RawEvent::DomainCreated(100, 1)));
	});
}

#[test]
fn correct_error_for_none_value() {
	new_test_ext().execute_with(|| {
	});
}
