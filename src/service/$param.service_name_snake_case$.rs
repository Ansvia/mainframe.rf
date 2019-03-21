//! Core implementasi untuk Service $param.service_name$
#![allow(missing_docs)]

use actix_web::{HttpRequest, HttpResponse};
use chrono::NaiveDateTime;
use serde::Serialize;

use crate::crypto::{self, SecretKey};
use crate::{
    api::{Error as ApiError, HttpRequest as ApiHttpRequest, Result as ApiResult},
    auth, models,
    prelude::*,
    schema_op,
};

use crate::api::$param.service_name_snake_case$::{PrivateApi, PublicApi};

/// Core basis service $name_snake_case$.
pub struct $param.service_name_camel_case$Service {}

impl $param.service_name_camel_case$Service {
    #[doc(hidden)]
    pub fn new() -> Box<Self> {
        Box::new(Self {})
    }
}

impl Service for $param.service_name_camel_case$Service {
    fn name(&self) -> &'static str {
        "$param.service_name_snake_case$"
    }

    fn wire_api(&self, builder: &mut ServiceApiBuilder) {
        builder.public_scope().link(PublicApi::wire);
        builder.private_scope().link(PrivateApi::wire);
    }
}
