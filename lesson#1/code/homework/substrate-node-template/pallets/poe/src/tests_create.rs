use crate::{Error, mock::*};
use frame_support::{assert_ok, assert_noop};
use super::*;

/// 测试不同的用户创建相同的存证
#[test]
fn create_claim_diffuser_sameclaim() {
    new_test_ext().execute_with(|| {
        let claim = vec![0];

        // Origin 1
        assert_ok!(PoeModule::create_claim(Origin::signed(1), claim.clone()));
        assert_eq!(Proofs::<Test>::get(&claim), (1, frame_system::Module::<Test>::block_number()));

        // Origin 2
        assert_noop!(
                PoeModule::create_claim(Origin::signed(2), claim.clone()),
            Error::<Test>::ProofAlreadyClaimed
        );
    })
}

/// 测试合法的存证长度
#[test]
fn create_claim_valid_size_works() {
    new_test_ext().execute_with(|| {
        // size >= 1
        let claim = vec![0];
        assert_ok!(PoeModule::create_claim(Origin::signed(1), claim.clone()));
        assert_eq!(Proofs::<Test>::get(&claim), (1, frame_system::Module::<Test>::block_number()));

        // size <= 10240
        let claim = vec![0;10240];
        assert_ok!(PoeModule::create_claim(Origin::signed(2), claim.clone()));
        assert_eq!(Proofs::<Test>::get(&claim), (2, frame_system::Module::<Test>::block_number()));
    })
}

/// 测试非法的存证长度
#[test]
fn create_claim_invalid_size() {
    new_test_ext().execute_with(|| {
        // size < 1
        let claim = vec![];
        assert_noop!(
            PoeModule::create_claim(Origin::signed(1), claim.clone()),
            Error::<Test>::InvaildProofSize
        );

        // size > 10240
        let claim = vec![0;10241];
        assert_noop!(
            PoeModule::create_claim(Origin::signed(1), claim.clone()),
            Error::<Test>::InvaildProofSize
        );
    })
}