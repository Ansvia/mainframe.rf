//! Service helper
//! 
//! Here you can implement service as simple write one line
//! this done by using impl_service macro, example:
//! 
//! `impl_service!(UserService, user);`
//!
//! You need to implement api interface for the above service inside `api` module.
//!
#![allow(missing_docs)]

use actix_web::{HttpRequest, HttpResponse};
use chrono::NaiveDateTime;
use serde::Serialize;

use crate::crypto::{self, SecretKey};
use crate::{
    api::{Error as ApiError, HttpRequest as ApiHttpRequest, Result as ApiResult},
    auth, models,
    prelude::*,
};

macro_rules! impl_service {
    ($service_name:ident, $api_name:ident) => {
        mod $api_name {
            use crate::api::$api_name::{PrivateApi, PublicApi};

            /// Core basis service racta.
            pub struct $service_name {}

            impl $service_name {
                #[doc(hidden)]
                pub fn new() -> Box<Self> {
                    Box::new(Self {})
                }
            }

            impl crate::service::Service for $service_name {
                fn name(&self) -> &'static str {
                    stringify!($api_name)
                }

                fn wire_api(&self, builder: &mut crate::api::ServiceApiBuilder) {
                    builder.public_scope().link(PublicApi::wire);
                    builder.private_scope().link(PrivateApi::wire);
                }
            }
        }

        pub use $api_name::$service_name;
    };
}

// Example implementing service using macro:
// impl_service!(UserService, user);
impl_service!(AdminService, admin);
