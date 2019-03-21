#![allow(dead_code, unused_variables)]

use $name_snake_case$_testkit::TestKit;

pub mod prelude {
    pub use super::{create_testkit, setup};
    pub use $name_snake_case$::api::{ApiResult, ErrorCode};
    pub use $name_snake_case$_testkit::{TestHelper, TestKit, TestKitApi};
}

pub fn setup() {
    let _ = env_logger::try_init();
}

pub fn create_testkit() -> TestKit {
    setup();
    TestKit::new()
}
