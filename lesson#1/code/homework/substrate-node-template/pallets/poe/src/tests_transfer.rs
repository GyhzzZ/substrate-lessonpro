use crate::{Error, mock::*};
use frame_support::{assert_ok, assert_noop};
use super::*;

/// 正常
#[test]
fn transfer_claim_works() {
    new_test_ext().execute_with(|| {
        let claim = vec![0, 1];
        assert_ok!(PoeModule::create_claim(Origin::signed(1), claim.clone()));

        assert_ok!(PoeModule::transfer_claim(Origin::signed(1), claim.clone(), 2));

        assert_eq!(Proofs::<Test>::get(&claim), (2, frame_system::Module::<Test>::block_number()));
    })
}

/// 测试transfer别人创建的存证
#[test]
fn transfer_claim_not_owner() {
    new_test_ext().execute_with(|| {
        let claim = vec![0, 1];
        assert_ok!(PoeModule::create_claim(Origin::signed(1), claim.clone()));

        assert_noop!(
            PoeModule::transfer_claim(Origin::signed(2), claim.clone(), 3),
            Error::<Test>::NotProofOwner
            );
    })
}

/// 测试tranfer不存在的存证
#[test]
fn transfer_claim_when_claim_not_exist() {
    new_test_ext().execute_with(|| {
        let claim = vec![0, 1];
        assert_ok!(PoeModule::create_claim(Origin::signed(1), claim.clone()));

        let claim = vec![0];
        assert_noop!(
            PoeModule::transfer_claim(Origin::signed(1), claim.clone(), 3),
            Error::<Test>::NoSuchProof
            );
    })
}