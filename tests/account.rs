mod common;

use crate::common::prelude::*;

#[test]
fn test_new_account_has_keypair() {
    let testkit = create_testkit();
    let helper = testkit.helper();
    let accounts = helper.generate_accounts(2);
    assert_eq!(accounts.len(), 2);
    for acc in &accounts {
        assert!(acc.public_key.to_hex().len() > 0);
        assert!(acc.secret_key.to_hex().len() > 0);
    }
    helper.cleanup_accounts(accounts.iter().map(|a| a.account.id).collect());
}

#[test]
fn test_register_and_activate_account() {
    let testkit = create_testkit();
    let helper = testkit.helper();
    let api_helper = testkit.api_helper();
    let name = helper.generate_full_name();
    let reg_token =
        api_helper.register_account(&name, &helper.generate_email(), &helper.generate_phone_num());
    assert!(reg_token.code == ErrorCode::NoError as i32);
    let reg_token = reg_token.result.unwrap();
    let account = api_helper.activate_account(reg_token, "123");
    assert!(account.code == ErrorCode::NoError as i32);
    let account = account.result.unwrap();
    assert_eq!(account.full_name, name);
    helper.cleanup_account(account);
}

#[test]
fn test_activate_account_invalid_token() {
    let testkit = create_testkit();
    let helper = testkit.helper();
    let api_helper = testkit.api_helper();

    let name = helper.generate_full_name();

    let reg_token = api_helper
        .register_account(&name, &helper.generate_email(), &helper.generate_phone_num())
        .result
        .unwrap();

    let rv = api_helper.activate_account(reg_token + "invalid", "123");
    assert_eq!(rv.code, ErrorCode::DatabaseRecordNotFoundError as i32)
}

macro_rules! test_register_empty_param {
    ($name:ident, $error_code:tt, $error_msg:tt, (($helper:ident, $api_helper:ident)| $($rega:tt)* )  ) => {
        #[test]
        fn $name() {
            let testkit = create_testkit();
            let $helper = testkit.helper();
            let $api_helper = testkit.api_helper();

            let rv = {$($rega)*};

            assert_eq!(rv.code, $error_code);
            assert_eq!(rv.description, format!("Invalid parameter: {}", $error_msg));
        }
    };
}

test_register_empty_param!(
    test_register_account_empty_name_param,
    4002,
    "full name cannot be empty",
    ((helper, apih) | apih.register_account("", &helper.generate_email(), &helper.generate_phone_num(),))
);

test_register_empty_param!(
    test_register_account_empty_email_param,
    4002,
    "email cannot be empty",
    ((helper, apih) | apih.register_account(&helper.generate_full_name(), "", &helper.generate_phone_num(),))
);

test_register_empty_param!(
    test_register_account_empty_phone_num_param,
    4002,
    "phone_num cannot be empty",
    ((helper, apih) | apih.register_account(&helper.generate_full_name(), &helper.generate_email(), "",))
);
