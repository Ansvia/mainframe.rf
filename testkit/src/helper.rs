use $name_snake_case$::api::$param.service_name_snake_case$::types;
use $name_snake_case$::api::{
    $param.service_name_snake_case$::{Activate$param.service_name_camel_case$, Register$param.service_name_camel_case$, TxQuery},
    ApiResult,
};
use $name_snake_case$::auth;
use $name_snake_case$::crypto::*;
use $name_snake_case$::models;
use $name_snake_case$::prelude::*;
use $name_snake_case$::schema_$param.service_name_snake_case$::*;
use $name_snake_case$::{
    api::types::IdQuery,
    util,
};
use diesel::{connection::Connection, pg::PgConnection};
use serde_json::Value as JsonValue;

use crate::{ApiKind, ID, TestKit, TestKitApi};

use std::{
    env,
    sync::{Arc, Mutex, MutexGuard},
};

pub struct $param.service_name_camel_case$WithKey {
    pub $param.service_name_snake_case$: types::$param.service_name_camel_case$,
    pub public_key: PublicKey,
    pub secret_key: SecretKey,
}

impl $param.service_name_camel_case$WithKey {
    pub fn new($param.service_name_snake_case$: types::$param.service_name_camel_case$, public_key: PublicKey, secret_key: SecretKey) -> Self {
        Self {
            $param.service_name_snake_case$,
            public_key,
            secret_key,
        }
    }
}

#[allow(dead_code)]
pub struct TestHelper {
    testkit: TestKit,
}

impl TestHelper {
    pub fn new(testkit: &TestKit) -> Self {
        Self {
            testkit: testkit.clone(),
        }
    }

    fn get_db<'a>() -> MutexGuard<'a, PgConnection> {
        lazy_static! {
            static ref PG_CONN_FOR_TEST: Arc<Mutex<PgConnection>> = Arc::new(Mutex::new(
                PgConnection::establish(&env::var("DATABASE_TEST_URL")
                    .expect("No DATABASE_TEST_URL env var")).expect("Cannot connect to db")
            ));
        }

        PG_CONN_FOR_TEST.lock().unwrap()
    }

    pub fn get_$param.service_name_snake_case$_by_id(&self, id: ID) -> Result<models::$param.service_name_camel_case$> {
        let db = Self::get_db();
        let schema = Schema::new(&db);
        schema.get_$param.service_name_snake_case$(id)
    }

    /// Menggenerasikan akses token langsung dari database,
    /// Tidak melalui API endpoint `/authorize`.
    pub fn gen_access_token_for(&self, id: ID) -> Result<models::AccessToken> {
        let db = Self::get_db();
        let schema = auth::Schema::new(&db);
        schema.generate_access_token(id).map_err(From::from)
    }

    pub fn cleanup_registered_$param.service_name_snake_case$(&self, token: &str) {
        let db = Self::get_db();
        let schema = Schema::new(&db);
        let _ = schema.cleanup_registered_$param.service_name_snake_case$(token);
    }

    pub fn generate_full_name(&self) -> String {
        // @TODO(robin): mungkin nantinya gunakan tool seperti ini: https://github.com/fnichol/names ?
        util::random_string(10)
    }

    pub fn generate_amount(&self) -> f64 {
        util::random_number_f64()
    }

    pub fn generate_email(&self) -> String {
        format!("{}@{}.com", util::random_string(10), util::random_string(5)).to_lowercase()
    }

    pub fn generate_phone_num(&self) -> String {
        let nums: String = (0..10).map(|_| util::random_number().to_string()).collect();
        format!("+628{}", nums)
    }

    /// Menggenerasikan beberapa akun sekaligus,
    /// ini tidak via rest API, tapi langsung ke database.
    pub fn generate_$param.service_name_snake_case$s(&self, count: usize) -> Vec<$param.service_name_camel_case$WithKey> {
        let db = Self::get_db();
        let schema = Schema::new(&db);
        let mut rv = vec![];
        for _ in 0..count {
            let new_$param.service_name_snake_case$ = New$param.service_name_camel_case$ {
                full_name: &self.generate_full_name(),
                email: &self.generate_email(),
                phone_num: &self.generate_phone_num(),
                active: true,
                register_time: util::now(),
            };
            let ($param.service_name_snake_case$, (public_key, secret_key)) = schema
                .create_$param.service_name_snake_case$(&new_$param.service_name_snake_case$)
                .expect("cannot create $param.service_name_snake_case$");
            rv.push($param.service_name_camel_case$WithKey::new($param.service_name_snake_case$.into(), public_key, secret_key));
        }
        rv
    }

    /// Menghapus akun berdasarkan ID.
    pub fn cleanup_$param.service_name_snake_case$_by_id(&self, $param.service_name_snake_case$_id: ID) {
        let db = Self::get_db();
        let schema = TestSchema::new(&db);
        let _ = schema.delete_$param.service_name_snake_case$_by_id($param.service_name_snake_case$_id);
    }

    /// Menghapus akun
    pub fn cleanup_$param.service_name_snake_case$(&self, $param.service_name_snake_case$: types::$param.service_name_camel_case$) {
        self.cleanup_$param.service_name_snake_case$_by_id($param.service_name_snake_case$.id);
    }

    /// Bersihkan data akun berdasarkan list dari ID-nya.
    pub fn cleanup_$param.service_name_snake_case$s(&self, $param.service_name_snake_case$_ids: Vec<ID>) {
        let db = Self::get_db();
        let schema = TestSchema::new(&db);
        schema.cleanup_$param.service_name_snake_case$s($param.service_name_snake_case$_ids);
    }
}

pub struct ApiHelper<'a> {
    testkit: &'a TestKit,
}

impl<'a> ApiHelper<'a> {
    pub fn new(testkit: &'a TestKit) -> Self {
        Self { testkit }
    }

    /// Register $param.service_name_snake_case$
    /// Mengembalikan token untuk aktivasi.
    pub fn register_$param.service_name_snake_case$(&self, $param.service_name_snake_case$_name: &str, email: &str, phone_number: &str) -> ApiResult<String> {
        let api = self.testkit.api();

        let data = Register$param.service_name_camel_case$ {
            full_name: $param.service_name_snake_case$_name.to_owned(),
            email: email.to_owned(),
            phone_num: phone_number.to_owned(),
        };

        api.public(ApiKind::$param.service_name_camel_case$)
            .query(&data)
            .post("v1/$param.service_name_snake_case$/register")
            .expect("create $param.service_name_snake_case$")
    }

    /// Aktivasi akun menggunakan token yang telah didapat dari hasil register.
    pub fn activate_$param.service_name_snake_case$(&self, token: String, password: &str) -> ApiResult<types::$param.service_name_camel_case$> {
        let api = self.testkit.api();

        let data = Activate$param.service_name_camel_case$ {
            token,
            password: password.to_owned(),
        };

        api.public(ApiKind::$param.service_name_camel_case$)
            .query(&data)
            .post::<ApiResult<types::$param.service_name_camel_case$>>("v1/$param.service_name_snake_case$/activate")
            .expect("activate $param.service_name_snake_case$")
    }
}
