use super::*;
use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};

//繁殖kitty成功
#[test]
fn breed_kitties_work() {
    new_test_ext().execute_with(|| {
        run_to_block(10);
        Balances::deposit_creating(&1, 100);
        assert_ok!(KittiesModule::create(Origin::signed(1)));
        assert_ok!(KittiesModule::create(Origin::signed(1)));
        assert_eq!(KittiesModule::lock_id(0), 0); // test lock_id
        assert_eq!(KittiesModule::lock_id(1), 1); // test lock_id
        assert_eq!(KittiesModule::lock_index(), 2); // test lock_index

        assert_ok!(KittiesModule::breed(Origin::signed(1), 0, 1));
        assert_eq!(KittiesModule::kitties_count(), 3); //test count
        assert_eq!(KittiesModule::kitty_owners(2), Some(1)); //test owner
        assert_eq!(Balances::usable_balance(&1), 95); //test lock

        assert_eq!(
            last_event(),
            TestEvent::simple_event(RawEvent::Created(1, 2))
        );
    })
}

//繁殖kitty失败，LockIndexOverflow
#[test]
fn breed_kitties_when_lock_indx_overflow() {
    new_test_ext().execute_with(|| {
        run_to_block(10);
        Balances::deposit_creating(&1, 100);
        assert_ok!(KittiesModule::create(Origin::signed(1)));
        assert_ok!(KittiesModule::create(Origin::signed(1)));
        KittiesModule::set_lock_index(9999999);
        assert_noop!(
            KittiesModule::breed(Origin::signed(1), 0, 1),
            Error::<Test>::LockIndexOverflow
        );
    })
}

//繁殖kitty失败，FreeNotEnough
#[test]
fn breed_kitties_when_balance_not_enough() {
    new_test_ext().execute_with(|| {
        run_to_block(10);
        Balances::deposit_creating(&1, 100);
        assert_ok!(KittiesModule::create(Origin::signed(1)));
        assert_ok!(KittiesModule::create(Origin::signed(1)));
        assert_ok!(Balances::transfer(Origin::signed(1), 2, 1));
        // assert_eq!(KittiesModule::lock_id(0), 0); // test lock_id
        // assert_eq!(KittiesModule::lock_id(1), 1); // test lock_id
        // assert_eq!(KittiesModule::lock_index(), 2); // test lock_index
        // assert_eq!(KittiesModule::next_lock_index(), Ok(2));

        assert_noop!(
            KittiesModule::breed(Origin::signed(2), 0, 1),
            Error::<Test>::FreeNotEnough
        );
        // assert_ok!(KittiesModule::breed(Origin::signed(2), 0, 1));
    })
}

//繁殖kitty失败，InvaildKittyId
#[test]
fn breed_kitties_when_invaild_kitty_id() {
    new_test_ext().execute_with(|| {
        run_to_block(10);
        Balances::deposit_creating(&1, 100);
        assert_ok!(KittiesModule::create(Origin::signed(1)));
        assert_ok!(KittiesModule::create(Origin::signed(1)));
        assert_noop!(
            KittiesModule::breed(Origin::signed(1), 0, 2),
            Error::<Test>::InvaildKittyId
        );
    })
}

//繁殖kitty失败，RequireDifferentParent
#[test]
fn breed_kitties_when_require_different_parent() {
    new_test_ext().execute_with(|| {
        run_to_block(10);
        Balances::deposit_creating(&1, 100);
        assert_ok!(KittiesModule::create(Origin::signed(1)));
        assert_ok!(KittiesModule::create(Origin::signed(1)));
        assert_noop!(
            KittiesModule::breed(Origin::signed(1), 0, 0),
            Error::<Test>::RequireDifferentParent
        );
    })
}
