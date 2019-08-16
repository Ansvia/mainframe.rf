//! Definisi struct untuk model-model yang ada di dalam database.

use chrono::NaiveDateTime;
use serde::Serialize;

use std::fmt;

use crate::ID;

/// Bentuk model akun di dalam database.
#[derive(Queryable, Clone, Serialize, PartialEq)]
pub struct $param.service_name_pascal_case$ {
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
pub struct Register$param.service_name_pascal_case$ {
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
pub struct $param.service_name_pascal_case$Pashash {
    pub $param.service_name_snake_case$_id: i64,
    pub passhash: String,
    pub deperecated: bool,
    pub ver: i32,
    pub created: NaiveDateTime,
}

#[doc(hidden)]
#[derive(Queryable)]
pub struct $param.service_name_pascal_case$Key {
    pub id: ID,
    pub $param.service_name_snake_case$_id: ID,
    pub pub_key: String,
    pub secret_key: String,
    pub created: NaiveDateTime,
    pub active: bool,
}

impl fmt::Display for $param.service_name_pascal_case$ {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "$param.service_name_pascal_case$({}, {})", self.id, self.full_name)
    }
}

impl fmt::Display for $param.service_name_pascal_case$Key {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Key({})", &self.pub_key[..8])
    }
}
