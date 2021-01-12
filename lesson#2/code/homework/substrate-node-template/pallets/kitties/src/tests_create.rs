use super::*;
use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};
use frame_system::{EventRecord, Phase, RawOrigin};

//创建kitty成功
#[test]
fn owned_kitties_can_append_values() {
    new_test_ext().execute_with(|| {
        run_to_block(10);
        // assert_ok!(Balances::transfer(Origin::signed(10), 20, 1));
        Balances::deposit_creating(&1, 100);
        assert_ok!(KittiesModule::create(Origin::signed(1)));

        assert_eq!(KittiesModule::lock_id(0), 0); // test lock_id
        assert_eq!(KittiesModule::lock_index(), 1); // test lock_index
        assert_eq!(KittiesModule::kitties_count(), 1); // test count
        assert_eq!(KittiesModule::kitty_owners(0), Some(1)); // test owner
        assert_eq!(Balances::usable_balance(&1), 95); // test lock

        // assert_eq!(
        //     System::events(),
        //     vec![EventRecord {
        //         phase: Phase::Initialization,
        //         event: TestEvent::simple_event(RawEvent::Created(1, 0)),
        //         topics: vec![],
        //     }]
        // );
        assert_eq!(
            last_event(),
            TestEvent::simple_event(RawEvent::Created(1, 0))
        );
    })
}

//创建kitty失败，KittiesCountOverflow
#[test]
fn create_kitties_when_index_max() {
    new_test_ext().execute_with(|| {
        run_to_block(10);
        KittiesModule::set_kitties_count(<mock::Test as Trait>::KittyIndex::max_value());
        assert_noop!(
            KittiesModule::create(Origin::signed(1)),
            Error::<Test>::KittiesCountOverflow
        );
    })
}

//创建kitty失败，LockIndexOverflow
#[test]
fn create_kitties_when_lock_max() {
    new_test_ext().execute_with(|| {
        run_to_block(10);
        KittiesModule::set_lock_index(9999999);
        assert_noop!(
            KittiesModule::create(Origin::signed(1)),
            Error::<Test>::LockIndexOverflow
        );
    })
}

//创建kitty失败，BalanceNotEnough
#[test]
fn create_kitties_when_balance_not_enough() {
    new_test_ext().execute_with(|| {
        run_to_block(10);
        assert_noop!(
            KittiesModule::create(Origin::signed(1)),
            Error::<Test>::FreeNotEnough
        );
    })
}
