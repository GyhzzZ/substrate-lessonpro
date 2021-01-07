use crate::{Error, mock::*};
use frame_support::{assert_ok, assert_noop};

/// 测试移除别人创建的存证
#[test]
fn revoke_claim_not_owner() {
    new_test_ext().execute_with(|| {
        let claim = vec![0, 1];
        assert_ok!(PoeModule::create_claim(Origin::signed(1), claim.clone()));

        assert_noop!(
            PoeModule::revoke_claim(Origin::signed(2), claim.clone()),
            Error::<Test>::NotProofOwner
        );
    })
}

/// 测试移除移除后创建相同的存证
#[test]
fn create_claim_after_revoke_claim() {
    new_test_ext().execute_with(|| {
        let claim = vec![0, 1];
        assert_ok!(PoeModule::create_claim(Origin::signed(1), claim.clone()));
        assert_ok!(PoeModule::revoke_claim(Origin::signed(1), claim.clone()));

        assert_ok!(PoeModule::create_claim(Origin::signed(2), claim.clone()));
    })
}