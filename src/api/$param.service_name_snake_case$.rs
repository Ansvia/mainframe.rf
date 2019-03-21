//! Koleksi query yang digunakan untuk operasi pada rest API.
#![allow(missing_docs)]

use actix_web::{HttpRequest, HttpResponse};
use chrono::NaiveDateTime;
use protobuf;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

use crate::crypto::{self, PublicKey, SecretKey, Signature};

use crate::{
    api,
    api::$param.service_name_snake_case$::models::*,
    api::{ApiResult, Error as ApiError, HttpRequest as ApiHttpRequest},
    auth,
    error::{Error, ErrorCode},
    prelude::*,
    schema_op,
};

#[derive(Serialize)]
pub struct EntriesResult<T> {
    pub entries: Vec<T>,
    pub count: i64,
}

#[derive(Deserialize)]
pub struct ListAccount {
    pub query: Option<String>,
    pub page: i64,
    pub limit: i64,
}

#[derive(Deserialize)]
pub struct QueryEntries {
    pub query: Option<String>,
    pub page: i64,
    pub limit: i64,
}

#[derive(Serialize, Deserialize)]
pub struct IdQuery {
    pub id: ID,
}

/// Definisi query untuk mendaftarkan akun baru via rest API.
#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterAccount {
    pub full_name: String,
    pub email: String,
    pub phone_num: String,
    // comment out: mungkin tidak untuk sekarang
    // pub nik: String,
}

/// Definisi query untuk mengaktifkan akun yang telah didaftarkan.
#[derive(Debug, Serialize, Deserialize)]
pub struct ActivateAccount {
    pub token: String,
    pub password: String,
}

/// Setiap query transaksi harus menggunakan wrapper ini,
/// masuk pada `body` dan signature dari `body` disimpan pada field `signature`.
///
/// TxQuery implement `sign` method yang bisa digunakan untuk melakukan signing
/// pada data di `body`.
#[derive(Debug, Serialize, Deserialize)]
pub struct TxQuery<T>
where
    T: Serialize,
{
    pub body: T,
    pub signature: String,
}

impl<T> TxQuery<T>
where
    T: protobuf::Message + Serialize + Clone,
{
    /// Create new tx query instance.
    pub fn new(body: T) -> Self {
        Self {
            body,
            signature: Default::default(),
        }
    }

    /// Lakukan signing pada data di `body`.
    /// Operasi signing dilakukan dengan cara men-serialize data pada `body` ke dalam
    /// bentuk protobuf bytes lalu di-sign menggunakan `secret_key`.
    pub fn sign(&self, secret_key: &SecretKey) -> Self {
        assert!(self.signature.is_empty(), "already signed.");

        // convert ke bytes format protobuf
        let bytes = self.body.write_to_bytes().expect("Cannot write to bytes");
        let signature = crypto::sign(&bytes, &secret_key);
        Self {
            body: self.body.clone(),
            signature: signature.to_hex(),
        }
    }

    /// Untuk memverifikasi signature pada body ini.
    pub fn verify(&self, public_key: &PublicKey, secret_key: &SecretKey) -> Result<()> {
        if self.signature.is_empty() {
            Err(Error::BadRequest(
                ErrorCode::MessageHasNoSign as i32,
                "message has no signature.".to_string(),
            ))?
        }
        let bytes = self.body.write_to_bytes().expect("Cannot write to bytes");
        let signature: Signature = self.signature.parse::<Signature>()?;
        debug!("verify sig `{}` using pubkey: `{}`", &signature, public_key);
        if !crypto::is_verified(bytes.as_slice(), &signature, public_key) {
            let real_signature = crypto::sign(&bytes, &secret_key);
            debug!("  - expected signature: `{}`", real_signature);
            debug!("  - data: `{}`", hex::encode(bytes));
            Err(Error::Unauthorized)?
        }
        Ok(())
    }
}

/// Model untuk keperluan tukar menukar data API
/// bukan yang di database (crate::models).
pub mod models {

    use chrono::NaiveDateTime;

    use crate::{api::ApiResult, models};

    use std::convert::From;

    /// Bentuk model akun di dalam database.
    #[derive(Clone, Serialize, Deserialize, PartialEq)]
    pub struct Account {
        /// ID dari akun.
        pub id: i64,

        /// Nama lengkap akun.
        pub full_name: String,

        /// Alamat email kun.
        pub email: String,

        /// Nomor telpon akun.
        pub phone_num: String,

        /// Waktu kapan akun ini didaftarkan.
        pub register_time: NaiveDateTime,
    }

    impl From<models::Account> for Account {
        fn from(a: models::Account) -> Self {
            Account {
                id: a.id,
                full_name: a.full_name,
                email: a.email,
                phone_num: a.phone_num,
                register_time: a.register_time,
            }
        }
    }

    impl From<models::Account> for ApiResult<Account> {
        fn from(a: models::Account) -> Self {
            ApiResult::success(Account {
                id: a.id,
                full_name: a.full_name,
                email: a.email,
                phone_num: a.phone_num,
                register_time: a.register_time,
            })
        }
    }

}

use crate::models::AccessToken;


/// Holder untuk implementasi API endpoint publik untuk account.
pub struct PublicApi;

#[api_group("Account", "public", base="/$param.service_name_snake_case$/v1")]
impl PublicApi {


    #[inline]
    fn verify_tx<T>(query: &TxQuery<T>, schema: &Schema, current_account: &db::Account) -> api::Result<()>
    where
        T: Serialize + protobuf::Message + Clone,
    {
        // verifikasi digital signature
        let acc_key = schema.get_account_key(current_account.id)?;
        let secret_key = acc_key.secret_key.parse::<SecretKey>()?;
        let public_key = acc_key.pub_key.parse::<PublicKey>()?;

        query.verify(&public_key, &secret_key)?;

        Ok(())
    }


    /// Rest API endpoint untuk mendaftarkan akun baru.
    /// Setelah register akun tidak langsung aktif, perlu melakukan
    /// aktifasi menggunakan endpoint `/account/activate`.
    #[api_endpoint(path = "/account/register", mutable, auth = "none")]
    pub fn register_account(query: RegisterAccount) -> ApiResult<String> {
        let schema = Schema::new(state.db());

        schema
            .register_account(&query.full_name, &query.email, &query.phone_num)
            .map_err(From::from)
            .map(ApiResult::success)
    }

    /// Mengaktifkan user yang telah teregister.
    /// Ini nantinya dijadikan link yang akan dikirimkan ke email pendaftar.
    #[api_endpoint(path = "/account/activate", auth = "none", mutable)]
    pub fn activate_account(query: ActivateAccount) -> ApiResult<Account> {
        let schema = Schema::new(state.db());
        let account = schema.activate_registered_account(query.token)?;
        schema.set_password(account.id, &query.password)?;
        Ok(account.into())
    }

    /// Hanya digunakan untuk testing sahaja.
    #[api_endpoint(path = "/info", auth = "optional")]
    pub fn info(query: ()) -> JsonValue {
        Ok(json!({ "version": env!("CARGO_PKG_VERSION") }))
    }

    /// Mendapatkan informasi current account.
    #[api_endpoint(path = "/me/info", auth = "required")]
    pub fn me_info(state: &AppState, query: (), req: &ApiHttpRequest) -> Account {
        Ok(current_account.into())
    }
}

use crate::models as db;

/// Holder untuk implementasi API endpoint privat.
pub struct PrivateApi;

#[api_group("Account", "private", base = "/$param.service_name_snake_case$/v1")]
impl PrivateApi {

    /// Listing account
    #[api_endpoint(path = "/accounts", auth = "none")]
    pub fn list_account(query: ListAccount) -> ApiResult<EntriesResult<db::Account>> {
        let schema = Schema::new(state.db());

        let offset = query.page * query.limit;

        let entries = schema.get_accounts(offset, query.limit)?;

        let count = schema.get_account_count()?;
        Ok(ApiResult::success(EntriesResult { count, entries }))
    }

    /// Mencari akun berdasarkan kata kunci.
    #[api_endpoint(path = "/account/search", auth = "none")]
    pub fn search_accounts(query: ListAccount) -> ApiResult<EntriesResult<db::Account>> {
        let schema = Schema::new(state.db());

        let offset = query.page * query.limit;

        if query.query.is_none() {
            return Self::list_account(&state, query, req);
        }

        let keyword = query.query.unwrap();

        let (entries, count) = schema.search_accounts(&keyword, offset, query.limit)?;

        Ok(ApiResult::success(EntriesResult { count, entries }))
    }

    /// Mendapatkan jumlah akun secara keseluruhan.
    #[api_endpoint(path = "/account/count")]
    pub fn account_count(state: &AppState, query: ()) -> ApiResult<i64> {
        let schema = Schema::new(state.db());

        schema
            .get_account_count()
            .map(ApiResult::success)
            .map_err(From::from)
    }

    /// Mendapatkan data akun.
    #[api_endpoint(path = "/account/info", auth = "required")]
    pub fn account_info(query: IdQuery) -> ApiResult<db::Account> {
        let schema = Schema::new(state.db());

        schema
            .get_account(query.id)
            .map(ApiResult::success)
            .map_err(From::from)
    }

}
