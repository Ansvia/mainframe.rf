//! Schema operation a.k.a DAO, digunakan untuk melakukan operasi seperti
//! membuat akun baru, update, dan delete.

use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;
use failure;

use crate::{
    crypto::{self, PublicKey, SecretKey},
    error::Error as $name_camel_case$Error,
    error::ErrorCode,
    models::*,
    result::Result,
    schema::*,
    token,
    ID
};

use std::sync::Arc;

#[derive(Insertable)]
#[table_name = "register_$param.service_name_snake_case$s"]
#[doc(hidden)]
pub struct NewRegister$param.service_name_camel_case$<'a> {
    pub token: &'a str,
    pub full_name: &'a str,
    pub email: &'a str,
    pub phone_num: &'a str,
    pub register_time: NaiveDateTime,
    pub code: &'a str,
}

#[derive(Insertable)]
#[table_name = "$param.service_name_snake_case$s"]
#[doc(hidden)]
pub struct New$param.service_name_camel_case$<'a> {
    pub full_name: &'a str,
    pub email: &'a str,
    pub phone_num: &'a str,
    pub active: bool,
    pub register_time: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "$param.service_name_snake_case$_passhash"]
#[doc(hidden)]
pub struct New$param.service_name_camel_case$Passhash<'a> {
    pub $param.service_name_snake_case$_id: ID,
    pub passhash: &'a str,
    pub deprecated: bool,
    pub ver: i32
}

#[derive(Insertable)]
#[table_name = "$param.service_name_snake_case$_keys"]
#[doc(hidden)]
pub struct New$param.service_name_camel_case$Key {
    pub $param.service_name_snake_case$_id: ID,
    pub pub_key: String,
    pub secret_key: String,
    pub active: bool,
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

    /// Mendapatkan akun berdasarkan emailnya.
    pub fn get_$param.service_name_snake_case$_by_email(&self, email: &str) -> Result<$param.service_name_camel_case$> {
        use crate::schema::$param.service_name_snake_case$s::{self, dsl};
        dsl::$param.service_name_snake_case$s
            .filter(dsl::email.eq(email))
            .first(self.db)
            .map_err(From::from)
    }

    /// Mendapatkan akun berdasarkan nomor telp-nya.
    pub fn get_$param.service_name_snake_case$_by_phone_num(&self, phone: &str) -> Result<$param.service_name_camel_case$> {
        use crate::schema::$param.service_name_snake_case$s::{self, dsl};
        dsl::$param.service_name_snake_case$s
            .filter(dsl::phone_num.eq(phone))
            .first(self.db)
            .map_err(From::from)
    }

    /// Get $param.service_name_snake_case$ by ID.
    pub fn get_$param.service_name_snake_case$(&self, $param.service_name_snake_case$_id: ID) -> Result<$param.service_name_camel_case$> {
        use crate::schema::$param.service_name_snake_case$s::dsl::$param.service_name_snake_case$s;
        $param.service_name_snake_case$s.find($param.service_name_snake_case$_id).first(self.db).map_err(From::from)
    }

    /// Setting $param.service_name_snake_case$'s password
    pub fn set_password(&self, $param.service_name_snake_case$_id: ID, password: &str) -> Result<()> {
        use crate::schema::$param.service_name_snake_case$_passhash::{self, dsl};

        let _ = self.get_$param.service_name_snake_case$($param.service_name_snake_case$_id)?;

        self.db.build_transaction().read_write().run(|| {
            let passhash = &crate::crypto::get_passhash(password);

            // dipresiasi password lama
            diesel::update(
                dsl::$param.service_name_snake_case$_passhash.filter(dsl::$param.service_name_snake_case$_id.eq($param.service_name_snake_case$_id).and(dsl::deprecated.eq(false))),
            )
            .set(dsl::deprecated.eq(true))
            .execute(self.db)?;
            // .map_err(From::from)?;

            // tambahkan password baru
            diesel::insert_into($param.service_name_snake_case$_passhash::table)
                .values(&New$param.service_name_camel_case$Passhash {
                    $param.service_name_snake_case$_id,
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
    /// Mengembalikan ID dari registered $param.service_name_snake_case$ (bukan [$param.service_name_camel_case$]: $name_snake_case$::models::$param.service_name_camel_case$)
    /// karena user belum aktif, untuk mengaktifkannya perlu memanggil
    /// perintah [Schema::activate_registered_$param.service_name_snake_case$].
    pub fn register_$param.service_name_snake_case$(&self, full_name: &str, email: &str, phone_num: &str) -> Result<String> {
        use crate::schema::{
            $param.service_name_snake_case$s::dsl as dsl_$param.service_name_snake_case$,
            register_$param.service_name_snake_case$s::{self, dsl as dsl_ra},
        };

        if full_name == "" {
            Err($name_camel_case$Error::InvalidParameter(
                "full name cannot be empty".to_string(),
            ))?
        }
        if email == "" {
            Err($name_camel_case$Error::InvalidParameter(
                "email cannot be empty".to_string(),
            ))?
        }
        // @TODO(robin): lakukan validasi format nomor telp
        if phone_num == "" {
            Err($name_camel_case$Error::InvalidParameter(
                "phone_num cannot be empty".to_string(),
            ))?
        }

        // tolak akun dengan nama-nama tertentu
        // @TODO(robin): buat konfigurable
        if full_name == "nobody" {
            warn!("Name exception to register: `{}`", full_name);
            Err($name_camel_case$Error::Unauthorized)?
        }

        // apabila sudah exists di registered_$param.service_name_snake_case$s table
        // kembalikan token-nya aja
        if let Ok(ra) = dsl_ra::register_$param.service_name_snake_case$s
            .filter(dsl_ra::email.eq(email).or(dsl_ra::phone_num.eq(phone_num)))
            .first::<Register$param.service_name_camel_case$>(self.db)
        {
            return Ok(ra.token);
        }

        // check apakah akun dengan email/phone sama sudah ada
        let exists = dsl_$param.service_name_snake_case$::$param.service_name_snake_case$s
            .filter(
                dsl_$param.service_name_snake_case$::email
                    .eq(email)
                    .or(dsl_$param.service_name_snake_case$::phone_num.eq(phone_num)),
            )
            .select(dsl_$param.service_name_snake_case$::id)
            .first::<ID>(self.db)
            .is_ok();

        if exists {
            Err($name_camel_case$Error::AlreadyExists)?
        }

        let new_reg_$param.service_name_snake_case$ = NewRegister$param.service_name_camel_case$ {
            token: &token::generate_token(),
            full_name,
            email,
            phone_num,
            register_time: Utc::now().naive_utc(),
            code: &token::generate_activation_code(),
        };

        diesel::insert_into(register_$param.service_name_snake_case$s::table)
            .values(&new_reg_$param.service_name_snake_case$)
            .returning(dsl_ra::token)
            .get_result(self.db)
            .map_err(From::from)
    }

    /// Mengaktifkan akun yang telah melakukan registrasi tapi belum aktif.
    pub fn activate_registered_$param.service_name_snake_case$(&self, token: String) -> Result<$param.service_name_camel_case$> {
        use crate::schema::$param.service_name_snake_case$_keys::{self, dsl as ak_dsl};
        use crate::schema::{$param.service_name_snake_case$s, register_$param.service_name_snake_case$s};

        self.db.build_transaction().read_write().run(|| {
            let reg_acc: Register$param.service_name_camel_case$ = register_$param.service_name_snake_case$s::dsl::register_$param.service_name_snake_case$s
                .find(token.clone())
                .first(self.db)?;
            // .map_err(From::from)?;

            let new_$param.service_name_snake_case$ = New$param.service_name_camel_case$ {
                full_name: &reg_acc.full_name,
                email: &reg_acc.email,
                phone_num: &reg_acc.phone_num,
                active: true,
                register_time: Utc::now().naive_utc(),
            };

            let $param.service_name_snake_case$ = diesel::insert_into($param.service_name_snake_case$s::table)
                .values(&new_$param.service_name_snake_case$)
                .get_result::<$param.service_name_camel_case$>(self.db)?;
            // .map_err(From::from)?;

            // Buatkan key pair untuk akun yang baru saja dibuat.
            let (pub_key, secret_key) = crypto::gen_keypair();

            diesel::insert_into($param.service_name_snake_case$_keys::table)
                .values(&New$param.service_name_camel_case$Key {
                    $param.service_name_snake_case$_id: $param.service_name_snake_case$.id,
                    pub_key: pub_key.to_hex(),
                    secret_key: secret_key.to_hex(),
                    active: true,
                })
                .execute(self.db)?;

            // delete reference in registered $param.service_name_snake_case$s table
            diesel::delete(register_$param.service_name_snake_case$s::dsl::register_$param.service_name_snake_case$s.find(token)).execute(self.db)?;

            Ok($param.service_name_snake_case$)
        })
    }

    /// Mendapatkan informasi key untuk akun.
    pub fn get_$param.service_name_snake_case$_key(&self, $param.service_name_snake_case$_id: ID) -> Result<$param.service_name_camel_case$Key> {
        use crate::schema::$param.service_name_snake_case$_keys::{self, dsl as ak_dsl};
        use crate::schema::$param.service_name_snake_case$s;

        ak_dsl::$param.service_name_snake_case$_keys
            .filter(ak_dsl::$param.service_name_snake_case$_id.eq($param.service_name_snake_case$_id))
            .first(self.db)
            .map_err(From::from)
    }

    /// Buat akun baru secara langsung.
    pub fn create_$param.service_name_snake_case$(&self, new_$param.service_name_snake_case$: &New$param.service_name_camel_case$) -> Result<($param.service_name_camel_case$, (PublicKey, SecretKey))> {
        use crate::schema::$param.service_name_snake_case$_keys::{self, dsl as ak_dsl};
        use crate::schema::$param.service_name_snake_case$s;

        self.db.build_transaction().read_write().run(|| {
            let $param.service_name_snake_case$ = diesel::insert_into($param.service_name_snake_case$s::table)
                .values(new_$param.service_name_snake_case$)
                .get_result::<$param.service_name_camel_case$>(self.db)?;

            // Buatkan key pair untuk akun yang baru saja dibuat.
            let keypair = crypto::gen_keypair();

            diesel::insert_into($param.service_name_snake_case$_keys::table)
                .values(&New$param.service_name_camel_case$Key {
                    $param.service_name_snake_case$_id: $param.service_name_snake_case$.id,
                    pub_key: keypair.0.to_hex(),
                    secret_key: keypair.1.to_hex(),
                    active: true,
                })
                .execute(self.db)?;

            Ok(($param.service_name_snake_case$, keypair))
        })
    }

    /// Clean up registered $param.service_name_snake_case$ by token
    pub fn cleanup_registered_$param.service_name_snake_case$(&self, token: &str) -> Result<usize> {
        use crate::schema::register_$param.service_name_snake_case$s;
        use crate::schema::register_$param.service_name_snake_case$s::dsl;

        diesel::delete(dsl::register_$param.service_name_snake_case$s.filter(dsl::token.eq(token)))
            .execute(self.db)
            .map_err(From::from)
    }

    /// Get multiple $param.service_name_snake_case$s
    pub fn get_$param.service_name_snake_case$s(&self, offset: i64, limit: i64) -> Result<Vec<$param.service_name_camel_case$>> {
        use crate::schema::$param.service_name_snake_case$s;
        use crate::schema::$param.service_name_snake_case$s::dsl;

        dsl::$param.service_name_snake_case$s
            .filter(dsl::id.ne(0))
            .offset(offset)
            .limit(limit)
            .load(self.db)
            .map_err(From::from)
    }

    /// Mendapatkan jumlah akun keseluruhan di dalam database.
    pub fn get_$param.service_name_snake_case$_count(&self) -> Result<i64> {
        use crate::schema::$param.service_name_snake_case$s;
        use crate::schema::$param.service_name_snake_case$s::dsl;

        dsl::$param.service_name_snake_case$s
            .select(diesel::dsl::count(dsl::id))
            .first(self.db)
            .map_err(From::from)
    }

    /// Mencari akun berdasarkan kata kunci
    /// Ini mengembalikan tidak hanya daftar akun tetapi juga jumlah
    /// akun yang ada sesuai kata kunci tersebut.
    pub fn search_$param.service_name_snake_case$s(&self, keyword: &str, offset: i64, limit: i64) -> Result<(Vec<$param.service_name_camel_case$>, i64)> {
        use crate::schema::$param.service_name_snake_case$s;
        use crate::schema::$param.service_name_snake_case$s::dsl;

        let like_clause = format!("%{}%", keyword);

        let filterer = dsl::id.ne(0).and(
            dsl::full_name
                .like(&like_clause)
                .or(dsl::email.like(&like_clause)),
        );

        let entries = dsl::$param.service_name_snake_case$s
            .filter(filterer)
            .offset(offset)
            .limit(limit)
            .load(self.db)?;

        let count = dsl::$param.service_name_snake_case$s
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
    pub fn cleanup_$param.service_name_snake_case$s(&self, $param.service_name_snake_case$_ids: Vec<ID>) {
        use crate::schema::$param.service_name_snake_case$_passhash::dsl as acp_dsl;
        use crate::schema::$param.service_name_snake_case$s;
        use crate::schema::$param.service_name_snake_case$s::dsl;

        let _ = self
            .db
            .build_transaction()
            .read_write()
            .run::<(), diesel::result::Error, _>(|| {
                for id in $param.service_name_snake_case$_ids {
                    diesel::delete(acp_dsl::$param.service_name_snake_case$_passhash.filter(acp_dsl::$param.service_name_snake_case$_id.eq(id)))
                        .execute(self.db)?;
                    diesel::delete(dsl::$param.service_name_snake_case$s.filter(dsl::id.eq(id))).execute(self.db)?;
                }
                Ok(())
            });
    }

    /// Hapus akun berdasarkan id
    pub fn delete_$param.service_name_snake_case$_by_id(&self, id: ID) -> Result<usize> {
        use crate::schema::$param.service_name_snake_case$s;
        use crate::schema::$param.service_name_snake_case$s::dsl;
        diesel::delete(dsl::$param.service_name_snake_case$s.find(id))
            .execute(self.db)
            .map_err(From::from)
    }
}
