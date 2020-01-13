//! Core implementasi untuk Service authentikasi.
#![allow(missing_docs)]

use actix_web::{HttpRequest, HttpResponse};
use chrono::NaiveDateTime;
use serde::Serialize;
use serde_json::Value as JsonValue;

use crate::crypto::{self, SecretKey};
use crate::{
    api::{types::IdQuery, ApiResult, Error as ApiError, ErrorCode},
    auth::AuthDao,
    models,
    prelude::*,
    $param.service_name_snake_case$_dao::$param.service_name_pascal_case$Dao,
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
        let conn = state.db();
        let dao = AuthDao::new(&conn);
        dao.remove_access_token(&query.token)?;

        Ok(ApiResult::success(()))
    }

    /// Unauthorize user, this will invalidate all valid access tokens.
    #[api_endpoint(path = "/unauthorize", auth = "required", mutable)]
    pub fn unauthorize(query: IdQuery) -> ApiResult<()> {
        let conn = state.db();
        let dao = AuthDao::new(&conn);

        dao.clear_access_token_by_$param.service_name_snake_case$_id(query.id)?;

        Ok(ApiResult::success(()))
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
        let conn = state.db();
        let $param.service_name_snake_case$ = {
            let dao = $param.service_name_pascal_case$Dao::new(&conn);
            if let Some(email) = query.email {
                dao.get_by_email(&email)?
            } else if let Some(phone) = query.phone {
                dao.get_by_phone_num(&phone)?
            } else {
                Err(ApiError::InvalidParameter(
                    ErrorCode::NoLoginInfo as i32,
                    "No email/phone parameter".to_string(),
                ))?
            }
        };
        
        if !$param.service_name_snake_case$.active {
            return Err(ApiError::InvalidParameter(
                ErrorCode::Unauthorized as i32,
                "Account blocked".to_string(),
            ));
        }

        let dao = AuthDao::new(&conn);

        // <% if param.password_crypt_algo == "sha256" %>
        if !dao.valid_passhash($param.service_name_snake_case$.id, &query.passhash) {
            warn!("$param.service_name_snake_case$ `{}` try to authorize using wrong password", &$param.service_name_snake_case$.id);
            Err(ApiError::Unauthorized)?
        }        
        // <% endif %>

        // <% if param.password_crypt_algo == "bcrypt" %>
        let $param.service_name_snake_case$_passhash = dao.get_passhash("$param.service_name_snake_case$", $param.service_name_snake_case$.id)?;
        if !crypto::password_match(&query.password, &$param.service_name_snake_case$_passhash){
            warn!("$param.service_name_snake_case$ `{}` try to authorize using wrong password", &$param.service_name_snake_case$.id);
            Err(ApiError::Unauthorized)?
        }
        // <% endif %>

        dao.generate_access_token($param.service_name_snake_case$.id)
            .map_err(From::from)
            .map(ApiResult::success)

    }

    /// Unauthorize current $param.service_name$ session, this will invalidate all valid access tokens.
    #[api_endpoint(path = "/unauthorize", auth = "optional", mutable)]
    pub fn unauthorize(query: ()) -> ApiResult<()> {
        match current_$param.service_name_snake_case$ {
            Some(current_$param.service_name_snake_case$) => PrivateApi::unauthorize(
                state,
                IdQuery {
                    id: current_$param.service_name_snake_case$.id,
                },
                req,
            ),
            None => Ok(ApiResult::success(())),
        }
    }

    /// Mendapatkan keypair dari $param.service_name_snake_case$.
    #[api_endpoint(path = "/get_key", auth = "required")]
    fn $param.service_name_snake_case$_get_key(query: ()) -> ApiResult<JsonValue> {
        let conn = state.db();
        let dao = $param.service_name_pascal_case$Dao::new(&conn);
        let $param.service_name_snake_case$_key = dao.get_$param.service_name_snake_case$_key(current_$param.service_name_snake_case$.id)?;

        Ok(ApiResult::success(
            json!({"pub_key": $param.service_name_snake_case$_key.pub_key, "secret_key": $param.service_name_snake_case$_key.secret_key}),
        ))
    }
}
