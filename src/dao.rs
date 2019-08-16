//! Data Access Object for models, digunakan untuk melakukan operasi seperti
//! membuat project baru, update, dll.

use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;
use failure;

use crate::{
    crypto::{self, PublicKey, SecretKey},
    error::Error as RactaError,
    error::ErrorCode,
    models::*,
    result::Result,
    schema::*,
    token,
    ID,
};

use std::sync::Arc;

