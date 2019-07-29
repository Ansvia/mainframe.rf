#![allow(unused_mut, unused_variables)]

extern crate $name_snake_case$_testkit;
extern crate env_logger;
extern crate log;
#[macro_use]
extern crate serde_json;

use serde_json::Value as JsonValue;

use $name_snake_case$_testkit::ApiKind;

use $name_snake_case$::api::ErrorCode;

mod common;

use crate::common::create_testkit;

#[test]
fn test_get_info() {
    let testkit = create_testkit();
    let api = testkit.api();

    assert_eq!(
        api.public(ApiKind::System).get::<JsonValue>("v1/info").unwrap(),
        json!({ "version": env!("CARGO_PKG_VERSION"),
                "build": env!("BUILD_INFO"), "git": env!("GIT_REV") })
    );
}


#[test]
fn test_register_$param.service_name_snake_case$() {
    let testkit = create_testkit();
    let h = testkit.helper();
    let ah = testkit.api_helper();

    let rv = ah.register_$param.service_name_snake_case$("Akmal", "akmal@gmail.com", "+62857898122");
    assert!(rv.code == ErrorCode::NoError as i32);
    let token = rv.result.unwrap();
    h.cleanup_registered_$param.service_name_snake_case$(&token);
    assert!(token.len() > 0);
}
