//! Schema operation a.k.a DAO, digunakan untuk melakukan operasi seperti
//! membuat akun baru, update, dan delete.

use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;
use failure;

use crate::{
    crypto::{self, PublicKey, SecretKey},
    error::Error as PaymentError,
    error::ErrorCode,
    models::*,
    result::Result,
    schema::*,
    token,
};

use std::sync::Arc;

#[derive(Insertable)]
#[table_name = "register_accounts"]
#[doc(hidden)]
pub struct NewRegisterAccount<'a> {
    pub token: &'a str,
    pub full_name: &'a str,
    pub email: &'a str,
    pub phone_num: &'a str,
    pub register_time: NaiveDateTime,
    pub code: &'a str,
}

#[derive(Insertable)]
#[table_name = "accounts"]
#[doc(hidden)]
pub struct NewAccount<'a> {
    pub full_name: &'a str,
    pub email: &'a str,
    pub phone_num: &'a str,
    pub active: bool,
    pub register_time: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "account_passhash"]
#[doc(hidden)]
pub struct NewAccountPasshash<'a> {
    pub account_id: ID,
    pub passhash: &'a str,
    pub deprecated: bool,
    pub ver: i32
}

#[derive(Insertable)]
#[table_name = "account_keys"]
#[doc(hidden)]
pub struct NewAccountKey {
    pub account_id: ID,
    pub pub_key: String,
    pub secret_key: String,
    pub active: bool,
}

/// Type alias for ID in integer
pub type ID = i64;

/// Untuk mengoperasikan skema data di database
pub struct Schema<'a> {
    db: &'a PgConnection,
}

impl<'a> Schema<'a> {
    /// Create new schema instance.
    pub fn new(db: &'a PgConnection) -> Self {
        Self { db }
    }

    /// Mendapatkan akun berdasarkan emailnya.
    pub fn get_account_by_email(&self, email: &str) -> Result<Account> {
        use crate::schema::accounts::{self, dsl};
        dsl::accounts
            .filter(dsl::email.eq(email))
            .first(self.db)
            .map_err(From::from)
    }

    /// Mendapatkan akun berdasarkan nomor telp-nya.
    pub fn get_account_by_phone_num(&self, phone: &str) -> Result<Account> {
        use crate::schema::accounts::{self, dsl};
        dsl::accounts
            .filter(dsl::phone_num.eq(phone))
            .first(self.db)
            .map_err(From::from)
    }

    /// Get account by ID.
    pub fn get_account(&self, account_id: ID) -> Result<Account> {
        use crate::schema::accounts::dsl::accounts;
        accounts.find(account_id).first(self.db).map_err(From::from)
    }

    /// Setting account's password
    pub fn set_password(&self, account_id: ID, password: &str) -> Result<()> {
        use crate::schema::account_passhash::{self, dsl};

        let _ = self.get_account(account_id)?;

        self.db.build_transaction().read_write().run(|| {
            let passhash = &crate::crypto::get_passhash(password);

            // dipresiasi password lama
            diesel::update(
                dsl::account_passhash.filter(dsl::account_id.eq(account_id).and(dsl::deprecated.eq(false))),
            )
            .set(dsl::deprecated.eq(true))
            .execute(self.db)?;
            // .map_err(From::from)?;

            // tambahkan password baru
            diesel::insert_into(account_passhash::table)
                .values(&NewAccountPasshash {
                    account_id,
                    passhash,
                    deprecated: false,
                    ver: 1
                })
                .execute(self.db)?;
            // .map_err(From::from)?;

            Ok(())
        })
    }

    /// Mendaftarkan akun baru.
    /// Mengembalikan ID dari registered account (bukan [Account]: $name_snake_case$::models::Account)
    /// karena user belum aktif, untuk mengaktifkannya perlu memanggil
    /// perintah [Schema::activate_registered_account].
    pub fn register_account(&self, full_name: &str, email: &str, phone_num: &str) -> Result<String> {
        use crate::schema::{
            accounts::dsl as dsl_account,
            register_accounts::{self, dsl as dsl_ra},
        };

        if full_name == "" {
            Err(PaymentError::InvalidParameter(
                "full name cannot be empty".to_string(),
            ))?
        }
        if email == "" {
            Err(PaymentError::InvalidParameter(
                "email cannot be empty".to_string(),
            ))?
        }
        // @TODO(robin): lakukan validasi format nomor telp
        if phone_num == "" {
            Err(PaymentError::InvalidParameter(
                "phone_num cannot be empty".to_string(),
            ))?
        }

        // tolak akun dengan nama-nama tertentu
        // @TODO(robin): buat konfigurable
        if full_name == "nobody" {
            warn!("Name exception to register: `{}`", full_name);
            Err(PaymentError::Unauthorized)?
        }

        // apabila sudah exists di registered_accounts table
        // kembalikan token-nya aja
        if let Ok(ra) = dsl_ra::register_accounts
            .filter(dsl_ra::email.eq(email).or(dsl_ra::phone_num.eq(phone_num)))
            .first::<RegisterAccount>(self.db)
        {
            return Ok(ra.token);
        }

        // check apakah akun dengan email/phone sama sudah ada
        let exists = dsl_account::accounts
            .filter(
                dsl_account::email
                    .eq(email)
                    .or(dsl_account::phone_num.eq(phone_num)),
            )
            .select(dsl_account::id)
            .first::<ID>(self.db)
            .is_ok();

        if exists {
            Err(PaymentError::AlreadyExists)?
        }

        let new_reg_account = NewRegisterAccount {
            token: &token::generate_token(),
            full_name,
            email,
            phone_num,
            register_time: Utc::now().naive_utc(),
            code: &token::generate_activation_code(),
        };

        diesel::insert_into(register_accounts::table)
            .values(&new_reg_account)
            .returning(dsl_ra::token)
            .get_result(self.db)
            .map_err(From::from)
    }

    /// Mengaktifkan akun yang telah melakukan registrasi tapi belum aktif.
    pub fn activate_registered_account(&self, token: String) -> Result<Account> {
        use crate::schema::account_keys::{self, dsl as ak_dsl};
        use crate::schema::{accounts, register_accounts};

        self.db.build_transaction().read_write().run(|| {
            let reg_acc: RegisterAccount = register_accounts::dsl::register_accounts
                .find(token.clone())
                .first(self.db)?;
            // .map_err(From::from)?;

            let new_account = NewAccount {
                full_name: &reg_acc.full_name,
                email: &reg_acc.email,
                phone_num: &reg_acc.phone_num,
                active: true,
                register_time: Utc::now().naive_utc(),
            };

            let account = diesel::insert_into(accounts::table)
                .values(&new_account)
                .get_result::<Account>(self.db)?;
            // .map_err(From::from)?;

            // Buatkan key pair untuk akun yang baru saja dibuat.
            let (pub_key, secret_key) = crypto::gen_keypair();

            diesel::insert_into(account_keys::table)
                .values(&NewAccountKey {
                    account_id: account.id,
                    pub_key: pub_key.to_hex(),
                    secret_key: secret_key.to_hex(),
                    active: true,
                })
                .execute(self.db)?;

            // delete reference in registered accounts table
            diesel::delete(register_accounts::dsl::register_accounts.find(token)).execute(self.db)?;

            Ok(account)
        })
    }

    /// Mendapatkan informasi key untuk akun.
    pub fn get_account_key(&self, account_id: ID) -> Result<AccountKey> {
        use crate::schema::account_keys::{self, dsl as ak_dsl};
        use crate::schema::accounts;

        ak_dsl::account_keys
            .filter(ak_dsl::account_id.eq(account_id))
            .first(self.db)
            .map_err(From::from)
    }

    /// Buat akun baru secara langsung.
    pub fn create_account(&self, new_account: &NewAccount) -> Result<(Account, (PublicKey, SecretKey))> {
        use crate::schema::account_keys::{self, dsl as ak_dsl};
        use crate::schema::accounts;

        self.db.build_transaction().read_write().run(|| {
            let account = diesel::insert_into(accounts::table)
                .values(new_account)
                .get_result::<Account>(self.db)?;

            // Buatkan key pair untuk akun yang baru saja dibuat.
            let keypair = crypto::gen_keypair();

            diesel::insert_into(account_keys::table)
                .values(&NewAccountKey {
                    account_id: account.id,
                    pub_key: keypair.0.to_hex(),
                    secret_key: keypair.1.to_hex(),
                    active: true,
                })
                .execute(self.db)?;

            Ok((account, keypair))
        })
    }

    /// Clean up registered account by token
    pub fn cleanup_registered_account(&self, token: &str) -> Result<usize> {
        use crate::schema::register_accounts;
        use crate::schema::register_accounts::dsl;

        diesel::delete(dsl::register_accounts.filter(dsl::token.eq(token)))
            .execute(self.db)
            .map_err(From::from)
    }

    /// Get multiple accounts
    pub fn get_accounts(&self, offset: i64, limit: i64) -> Result<Vec<Account>> {
        use crate::schema::accounts;
        use crate::schema::accounts::dsl;

        dsl::accounts
            .filter(dsl::id.ne(0))
            .offset(offset)
            .limit(limit)
            .load(self.db)
            .map_err(From::from)
    }


    /// Mendapatkan jumlah akun keseluruhan di dalam database.
    pub fn get_account_count(&self) -> Result<i64> {
        use crate::schema::accounts;
        use crate::schema::accounts::dsl;

        dsl::accounts
            .select(diesel::dsl::count(dsl::id))
            .first(self.db)
            .map_err(From::from)
    }

    /// Mencari akun berdasarkan kata kunci
    /// Ini mengembalikan tidak hanya daftar akun tetapi juga jumlah
    /// akun yang ada sesuai kata kunci tersebut.
    pub fn search_accounts(&self, keyword: &str, offset: i64, limit: i64) -> Result<(Vec<Account>, i64)> {
        use crate::schema::accounts;
        use crate::schema::accounts::dsl;

        let like_clause = format!("%{}%", keyword);

        let filterer = dsl::id.ne(0).and(
            dsl::full_name
                .like(&like_clause)
                .or(dsl::email.like(&like_clause)),
        );

        let entries = dsl::accounts
            .filter(filterer)
            .offset(offset)
            .limit(limit)
            .load(self.db)?;

        let count = dsl::accounts
            .select(diesel::dsl::count(dsl::id))
            .filter(filterer)
            .first(self.db)?;

        Ok((entries, count))
    }
}

/// Schema untuk memudahkan integration testing
#[cfg(feature = "with-test")]
pub struct TestSchema<'a> {
    db: &'a PgConnection,
}

#[cfg(feature = "with-test")]
impl<'a> TestSchema<'a> {
    #[doc(hidden)]
    pub fn new(db: &'a PgConnection) -> Self {
        Self { db }
    }

    /// Menghapus akun secara batch
    pub fn cleanup_accounts(&self, account_ids: Vec<ID>) {
        use crate::schema::account_passhash::dsl as acp_dsl;
        use crate::schema::accounts;
        use crate::schema::accounts::dsl;

        let _ = self
            .db
            .build_transaction()
            .read_write()
            .run::<(), diesel::result::Error, _>(|| {
                for id in account_ids {
                    diesel::delete(acp_dsl::account_passhash.filter(acp_dsl::account_id.eq(id)))
                        .execute(self.db)?;
                    diesel::delete(dsl::accounts.filter(dsl::id.eq(id))).execute(self.db)?;
                }
                Ok(())
            });
    }

    /// Hapus akun berdasarkan id
    pub fn delete_account_by_id(&self, id: ID) -> Result<usize> {
        use crate::schema::accounts;
        use crate::schema::accounts::dsl;
        diesel::delete(dsl::accounts.find(id))
            .execute(self.db)
            .map_err(From::from)
    }
}
