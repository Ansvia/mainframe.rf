mod common;

use crate::common::prelude::*;

#[test]
fn test_new_$param.service_name_snake_case$_has_keypair() {
    let testkit = create_testkit();
    let helper = testkit.helper();
    let $param.service_name_snake_case$s = helper.generate_$param.service_name_snake_case$s(2);
    assert_eq!($param.service_name_snake_case$s.len(), 2);
    for acc in &$param.service_name_snake_case$s {
        assert!(acc.public_key.to_hex().len() > 0);
        assert!(acc.secret_key.to_hex().len() > 0);
    }
    helper.cleanup_$param.service_name_snake_case$s($param.service_name_snake_case$s.iter().map(|a| a.$param.service_name_snake_case$.id).collect());
}

#[test]
fn test_register_and_activate_$param.service_name_snake_case$() {
    let testkit = create_testkit();
    let helper = testkit.helper();
    let api_helper = testkit.api_helper();
    let name = helper.generate_full_name();
    let reg_token =
        api_helper.register_$param.service_name_snake_case$(&name, &helper.generate_email(), &helper.generate_phone_num());
    assert!(reg_token.code == ErrorCode::NoError as i32);
    let reg_token = reg_token.result.unwrap();
    let $param.service_name_snake_case$ = api_helper.activate_$param.service_name_snake_case$(reg_token, "123");
    assert!($param.service_name_snake_case$.code == ErrorCode::NoError as i32);
    let $param.service_name_snake_case$ = $param.service_name_snake_case$.result.unwrap();
    assert_eq!($param.service_name_snake_case$.full_name, name);
    helper.cleanup_$param.service_name_snake_case$($param.service_name_snake_case$);
}

#[test]
fn test_activate_$param.service_name_snake_case$_invalid_token() {
    let testkit = create_testkit();
    let helper = testkit.helper();
    let api_helper = testkit.api_helper();

    let name = helper.generate_full_name();

    let reg_token = api_helper
        .register_$param.service_name_snake_case$(&name, &helper.generate_email(), &helper.generate_phone_num())
        .result
        .unwrap();

    let rv = api_helper.activate_$param.service_name_snake_case$(reg_token + "invalid", "123");
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
    test_register_$param.service_name_snake_case$_empty_name_param,
    4002,
    "full name cannot be empty",
    ((helper, apih) | apih.register_$param.service_name_snake_case$("", &helper.generate_email(), &helper.generate_phone_num(),))
);

test_register_empty_param!(
    test_register_$param.service_name_snake_case$_empty_email_param,
    4002,
    "email cannot be empty",
    ((helper, apih) | apih.register_$param.service_name_snake_case$(&helper.generate_full_name(), "", &helper.generate_phone_num(),))
);

test_register_empty_param!(
    test_register_$param.service_name_snake_case$_empty_phone_num_param,
    4002,
    "phone_num cannot be empty",
    ((helper, apih) | apih.register_$param.service_name_snake_case$(&helper.generate_full_name(), &helper.generate_email(), "",))
);
