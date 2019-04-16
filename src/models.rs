//! Definisi struct untuk model-model yang ada di dalam database.

use chrono::NaiveDateTime;
use serde::Serialize;

use std::fmt;

use crate::schema_op::ID;

/// Bentuk model akun di dalam database.
#[derive(Queryable, Clone, Serialize, PartialEq)]
pub struct $param.service_name_camel_case$ {
    /// ID dari akun.
    pub id: i64,

    /// Nama lengkap akun.
    pub full_name: String,

    /// Alamat email dari akun.
    pub email: String,

    /// Nomor telepon.
    pub phone_num: String,

    /// Penanda apakah akun aktif atau tidak,
    /// apabila tidak aktif maka akun tidak diperkenankan untuk beroperasi.
    pub active: bool,

    /// Waktu kapan akun ini didaftarkan.
    pub register_time: NaiveDateTime,
}

/// Bentuk model dari alamat untuk akun.
#[derive(Queryable)]
pub struct Address {
    /// ID dari record ini.
    pub id: i64,

    /// ID dari akun yang memiliki alamat ini.
    pub $param.service_name_snake_case$_id: i64,

    /// Jenis alamat, 0: Domisili, 1: Kelahiran
    pub kind: i64,

    /// Alamat
    pub address: String,

    /// Kabupaten
    pub regency: String,

    /// Provinsi
    pub province: String,

    /// Negara
    pub country: String,

    /// Nomor telepon yang bisa dihubungi.
    pub phone_num: String,

    /// Penanda apakah alamat ini masih aktif atau tidak.
    pub active: bool,

    /// Catatan tentang alamat ini.
    pub notes: String,
}

#[doc(hidden)]
#[derive(Queryable)]
pub struct Register$param.service_name_camel_case$ {
    // pub id: i64,
    pub token: String,
    pub full_name: String,
    pub email: String,
    pub phone_num: String,
    pub register_time: NaiveDateTime,
    pub code: String,
}

#[doc(hidden)]
#[derive(Queryable, Serialize, PartialEq, Debug)]
pub struct AccessToken {
    pub token: String,
    pub $param.service_name_snake_case$_id: i64,
    pub created: NaiveDateTime,
    pub valid_thru: NaiveDateTime,
}

#[doc(hidden)]
#[derive(Queryable)]
pub struct $param.service_name_camel_case$Pashash {
    pub $param.service_name_snake_case$_id: i64,
    pub passhash: String,
    pub deperecated: bool,
    pub ver: i32,
    pub created: NaiveDateTime,
}

#[doc(hidden)]
#[derive(Queryable, Serialize, Deserialize)]
pub struct Invoice {
    pub id: ID,
    pub id_ref: String,
    pub issuer_$param.service_name_snake_case$: ID,
    pub to_$param.service_name_snake_case$: ID,
    pub discount: f64,
    pub amount: f64,
    pub notes: String,
    pub created: NaiveDateTime,
    pub paid: bool,
    pub paid_by: ID,
    pub paid_at: Option<NaiveDateTime>,
}

#[doc(hidden)]
#[derive(Queryable)]
pub struct InvoiceItem {
    pub id: ID,
    pub invoice_id: String,
    pub name: String,
    pub price: f64,
}

#[doc(hidden)]
#[derive(Queryable)]
pub struct PaymentHistory {
    pub id: ID,
    pub invoice_id: ID,
    pub payer: ID,
    pub via: String,
    pub ts: NaiveDateTime,
}

#[doc(hidden)]
#[derive(Queryable)]
pub struct $param.service_name_camel_case$Key {
    pub id: ID,
    pub $param.service_name_snake_case$_id: ID,
    pub pub_key: String,
    pub secret_key: String,
    pub created: NaiveDateTime,
    pub active: bool,
}

#[doc(hidden)]
#[derive(Queryable, Serialize)]
pub struct Transaction {
    pub id: ID,
    pub dbcr_flag: i32,
    pub ttype: i32,
    // pub subtype: i32,
    pub amount: f64,
    pub status: i32,
    pub created: NaiveDateTime,
    pub last_updated: NaiveDateTime,
    pub invoice_id: Option<ID>,
    pub from_$param.service_name_snake_case$: Option<ID>,
    pub to_$param.service_name_snake_case$: Option<ID>,
    pub merchant_id: Option<ID>,
    pub notes: Option<String>,
}

impl fmt::Display for $param.service_name_camel_case$ {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "$param.service_name_camel_case$({}, {})", self.id, self.full_name)
    }
}

impl fmt::Display for $param.service_name_camel_case$Key {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Key({})", &self.pub_key[..8])
    }
}
