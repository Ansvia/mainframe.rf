//! Module untuk mengurus otorisasi
//!
//!

use chrono::{Duration, NaiveDateTime};
use diesel::{pg::PgConnection, prelude::*};

use crate::{
    models::{AccessToken, Account},
    prelude::*,
    schema::access_tokens,
    token, util,
};

// use std::time::Duration;

#[doc(hidden)]
#[derive(Insertable)]
#[table_name = "access_tokens"]
pub struct NewAccessToken<'a> {
    pub token: &'a str,
    pub account_id: ID,
    pub created: NaiveDateTime,
    pub valid_thru: NaiveDateTime,
}

/// Untuk mengoperasikan skema data di database
pub struct Schema<'a> {
    db: &'a PgConnection,
}

impl<'a> Schema<'a> {
    /// Create new schema instance.
    pub fn new(db: &'a PgConnection) -> Self {
        Self { db }
    }

    /// Mendapatkan akun dari akses token.
    pub fn get_account_from_access_token(&self, access_token: &str) -> Result<Account> {
        use crate::schema::accounts::dsl::accounts;

        // @TODO(robin): ini masih bisa diimprove dengan hanya menggunakan sekali call ke DB
        self.get_access_token(access_token)
            .map(|token| accounts.find(token.account_id).first(self.db).map_err(From::from))?
    }

    /// Mendapatkan akses token object dari string token.
    pub fn get_access_token(&self, access_token: &str) -> Result<AccessToken> {
        use crate::schema::access_tokens::dsl::access_tokens;

        access_tokens
            .find(access_token)
            .first(self.db)
            .map_err(From::from)
    }

    /// Generate access token, this write access token into database.
    pub fn generate_access_token(&self, account_id: ID) -> Result<AccessToken> {
        use crate::schema::access_tokens::{self, dsl};

        let now = chrono::Utc::now().naive_utc();

        // hapus token lama kalau ada
        diesel::delete(dsl::access_tokens.filter(dsl::account_id.eq(account_id))).execute(self.db)?;

        let token = NewAccessToken {
            token: &token::generate_access_token(),
            account_id,
            created: now,
            valid_thru: now
                .checked_add_signed(Duration::days(7))
                .expect("cannot assign valid_thru time"),
        };

        diesel::insert_into(access_tokens::table)
            .values(&token)
            .get_result(self.db)
            .map_err(From::from)
    }

    /// Mendapatkan passhash pada akun.
    pub fn get_passhash(&self, account_id: ID) -> Result<String> {
        use crate::schema::account_passhash::dsl;
        dsl::account_passhash
            .filter(dsl::account_id.eq(account_id))
            .select(dsl::passhash)
            .get_result::<String>(self.db)
            .map_err(From::from)
    }

    /// Periksa apakah akun terhubung dengan spesifik passhash.
    /// Mengembalikan true apabila valid (ada).
    pub fn valid_passhash(&self, account_id: ID, passhash: &str) -> bool {
        use crate::schema::account_passhash::dsl;

        dsl::account_passhash
            .filter(dsl::account_id.eq(account_id).and(dsl::passhash.eq(passhash)))
            .select(dsl::account_id)
            .get_result::<i64>(self.db)
            .is_ok()
    }
}
