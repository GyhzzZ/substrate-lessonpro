use super::*;
use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};
use frame_system::{EventRecord, Phase, RawOrigin};

//转移kitty成功
#[test]
fn transfer_kitties_work() {
    new_test_ext().execute_with(|| {
        run_to_block(10);
        Balances::deposit_creating(&1, 100);
        Balances::deposit_creating(&2, 100);
        assert_ok!(KittiesModule::create(Origin::signed(1)));
        assert_ok!(KittiesModule::transfer(Origin::signed(1), 2, 0));

        assert_eq!(KittiesModule::kitties_count(), 1); //test count
        assert_eq!(KittiesModule::kitty_owners(0), Some(2)); //test owner
        assert_eq!(Balances::usable_balance(&1), 100); //test lock
        assert_eq!(Balances::usable_balance(&2), 95); //test lock

        assert_eq!(
            last_event(),
            TestEvent::simple_event(RawEvent::Transferred(1, 2, 0))
        );
    })
}

//转移kitty失败，BalanceNotEnough
#[test]
fn transfer_kitties_when_balance_not_enough() {
    new_test_ext().execute_with(|| {
        run_to_block(10);
        Balances::deposit_creating(&1, 100);
        assert_ok!(KittiesModule::create(Origin::signed(1)));
        assert_noop!(
            KittiesModule::transfer(Origin::signed(1), 2, 0),
            Error::<Test>::FreeNotEnough
        );
    })
}

//转移kitty失败，InvaildKittyId
#[test]
fn transfer_kitties_when_invaild_kitty_id() {
    new_test_ext().execute_with(|| {
        run_to_block(10);
        Balances::deposit_creating(&1, 100);
        Balances::deposit_creating(&2, 100);
        assert_ok!(KittiesModule::create(Origin::signed(1)));
        assert_noop!(
            KittiesModule::transfer(Origin::signed(1), 2, 1),
            Error::<Test>::InvaildKittyId
        );
    })
}

//转移kitty失败，NotKittyOwner
#[test]
fn transfer_kitties_when_not_kitty_owner() {
    new_test_ext().execute_with(|| {
        run_to_block(10);
        Balances::deposit_creating(&1, 100);
        Balances::deposit_creating(&2, 100);
        Balances::deposit_creating(&3, 100);
        assert_ok!(KittiesModule::create(Origin::signed(1)));
        assert_noop!(
            KittiesModule::transfer(Origin::signed(2), 3, 0),
            Error::<Test>::NotKittyOwner
        );
    })
}
