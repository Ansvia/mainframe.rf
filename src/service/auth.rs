//! Core implementasi untuk Service authentikasi.
#![allow(missing_docs)]

use actix_web::{HttpRequest, HttpResponse};
use chrono::NaiveDateTime;
use serde::Serialize;
use serde_json::Value as JsonValue;

use crate::crypto::{self, SecretKey};
use crate::{
    api::{ApiResult, Error as ApiError, ErrorCode},
    auth, models,
    prelude::*,
    schema_$param.service_name_snake_case$,
};

/// Core basis service untuk authentikasi.
pub struct AuthService {}

impl AuthService {
    #[doc(hidden)]
    pub fn new() -> Box<Self> {
        Box::new(Self {})
    }
}

impl Service for AuthService {
    fn name(&self) -> &'static str {
        "auth"
    }

    fn wire_api(&self, builder: &mut ServiceApiBuilder) {
        builder.public_scope().link(PublicApi::wire);
        builder.private_scope().link(PrivateApi::wire);
    }
}

use crate::models::AccessToken;

#[derive(Debug, Serialize, Deserialize)]
pub struct Authorize {
    pub email: Option<String>,
    pub phone: Option<String>,
    
    // <% if param.password_crypt_algo == "sha256" %>
    pub passhash: String,
    // <% endif %>

    // <% if param.password_crypt_algo == "bcrypt" %>
    pub password: String,
    // <% endif %>
}

#[derive(Serialize, Deserialize)]
pub struct AccessTokenQuery {
    pub token: String,
}

struct PrivateApi;

#[api_group("Authorization", "private", base = "/auth/v1")]
impl PrivateApi {
    /// Menghapus akses token
    #[api_endpoint(path = "/remove_access_token", auth = "required", mutable)]
    pub fn remove_access_token(query: AccessTokenQuery) -> ApiResult<()> {
        unimplemented!();
    }
}

struct PublicApi;

/// API endpoint untuk keperluan otorisasi.
#[api_group("Authorization", "public", base = "/auth/v1")]
impl PublicApi {
    /// Meng-otorisasi akun yang telah teregister
    /// User bisa melakukan otorisasi menggunakan email / nomor telp.
    #[api_endpoint(path = "/authorize", auth = "none", mutable)]
    pub fn authorize(state: &mut AppState, query: Authorize) -> ApiResult<AccessToken> {
        let $param.service_name_snake_case$ = {
            let schema = Schema::new(state.db());
            if let Some(email) = query.email {
                schema.get_$param.service_name_snake_case$_by_email(&email)?
            } else if let Some(phone) = query.phone {
                schema.get_$param.service_name_snake_case$_by_phone_num(&phone)?
            } else {
                Err(ApiError::InvalidParameter(
                    ErrorCode::NoLoginInfo as i32,
                    "No email/phone parameter".to_string(),
                ))?
            }
        };

        let schema = auth::Schema::new(state.db());

        // <% if param.password_crypt_algo == "sha256" %>
        if !schema.valid_passhash($param.service_name_snake_case$.id, &query.passhash) {
            warn!("$param.service_name_snake_case$ `{}` try to authorize using wrong password", &$param.service_name_snake_case$.id);
            Err(ApiError::Unauthorized)?
        }        
        // <% endif %>

        // <% if param.password_crypt_algo == "bcrypt" %>
        let $param.service_name_snake_case$_passhash = schema.get_passhash($param.service_name_snake_case$.id)?;
        if !crypto::password_match(&query.password, &$param.service_name_snake_case$_passhash){
            warn!("$param.service_name_snake_case$ `{}` try to authorize using wrong password", &$param.service_name_snake_case$.id);
            Err(ApiError::Unauthorized)?
        }
        // <% endif %>

        schema
            .generate_access_token($param.service_name_snake_case$.id)
            .map_err(From::from)
            .map(ApiResult::success)

    }

    /// Mendapatkan keypair dari $param.service_name_snake_case$.
    #[api_endpoint(path = "/get_key", auth = "required")]
    fn $param.service_name_snake_case$_get_key(query: ()) -> ApiResult<JsonValue> {
        let schema = Schema::new(state.db());
        let $param.service_name_snake_case$_key = schema.get_$param.service_name_snake_case$_key(current_$param.service_name_snake_case$.id)?;

        Ok(ApiResult::success(
            json!({"pub_key": $param.service_name_snake_case$_key.pub_key, "secret_key": $param.service_name_snake_case$_key.secret_key}),
        ))
    }
}
