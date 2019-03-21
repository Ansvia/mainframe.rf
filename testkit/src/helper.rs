use $name_snake_case$::api::$param.service_name_snake_case$::models::*;
use $name_snake_case$::api::{
    $param.service_name_snake_case$::{ActivateAccount, RegisterAccount, TxQuery},
    ApiResult,
};
use $name_snake_case$::auth;
use $name_snake_case$::crypto::*;
use $name_snake_case$::models;
use $name_snake_case$::prelude::*;
use $name_snake_case$::schema_op::*;
use $name_snake_case$::{
    api::$param.service_name_snake_case$::IdQuery,
    util,
};
use diesel::{connection::Connection, pg::PgConnection};
use serde_json::Value as JsonValue;

use crate::{ApiKind, TestKit, TestKitApi};

use std::{
    env,
    sync::{Arc, Mutex, MutexGuard},
};

pub struct AccountWithKey {
    pub account: Account,
    pub public_key: PublicKey,
    pub secret_key: SecretKey,
}

impl AccountWithKey {
    pub fn new(account: Account, public_key: PublicKey, secret_key: SecretKey) -> Self {
        Self {
            account,
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
                PgConnection::establish(&env::var("DATABASE_URL").unwrap()).expect("Cannot connect to db")
            ));
        }

        PG_CONN_FOR_TEST.lock().unwrap()
    }

    pub fn get_account_by_id(&self, id: ID) -> Result<models::Account> {
        let db = Self::get_db();
        let schema = Schema::new(&db);
        schema.get_account(id)
    }

    /// Menggenerasikan akses token langsung dari database,
    /// Tidak melalui API endpoint `/authorize`.
    pub fn gen_access_token_for(&self, id: ID) -> Result<models::AccessToken> {
        let db = Self::get_db();
        let schema = auth::Schema::new(&db);
        schema.generate_access_token(id).map_err(From::from)
    }

    pub fn cleanup_registered_account(&self, token: &str) {
        let db = Self::get_db();
        let schema = Schema::new(&db);
        let _ = schema.cleanup_registered_account(token);
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
    pub fn generate_accounts(&self, count: usize) -> Vec<AccountWithKey> {
        let db = Self::get_db();
        let schema = Schema::new(&db);
        let mut rv = vec![];
        for _ in 0..count {
            let new_account = NewAccount {
                full_name: &self.generate_full_name(),
                email: &self.generate_email(),
                phone_num: &self.generate_phone_num(),
                active: true,
                register_time: util::now(),
            };
            let (account, (public_key, secret_key)) = schema
                .create_account(&new_account)
                .expect("cannot create account");
            rv.push(AccountWithKey::new(account.into(), public_key, secret_key));
        }
        rv
    }

    /// Menghapus akun berdasarkan ID.
    pub fn cleanup_account_by_id(&self, account_id: ID) {
        let db = Self::get_db();
        let schema = TestSchema::new(&db);
        let _ = schema.delete_account_by_id(account_id);
    }

    /// Menghapus akun
    pub fn cleanup_account(&self, account: Account) {
        self.cleanup_account_by_id(account.id);
    }

    /// Bersihkan data akun berdasarkan list dari ID-nya.
    pub fn cleanup_accounts(&self, account_ids: Vec<ID>) {
        let db = Self::get_db();
        let schema = TestSchema::new(&db);
        schema.cleanup_accounts(account_ids);
    }
}

pub struct ApiHelper<'a> {
    testkit: &'a TestKit,
}

impl<'a> ApiHelper<'a> {
    pub fn new(testkit: &'a TestKit) -> Self {
        Self { testkit }
    }

    /// Register account
    /// Mengembalikan token untuk aktivasi.
    pub fn register_account(&self, account_name: &str, email: &str, phone_number: &str) -> ApiResult<String> {
        let api = self.testkit.api();

        let data = RegisterAccount {
            full_name: account_name.to_owned(),
            email: email.to_owned(),
            phone_num: phone_number.to_owned(),
        };

        api.public(ApiKind::$param.service_name_camel_case$)
            .query(&data)
            .post("v1/account/register")
            .expect("create account")
    }

    /// Aktivasi akun menggunakan token yang telah didapat dari hasil register.
    pub fn activate_account(&self, token: String, password: &str) -> ApiResult<Account> {
        let api = self.testkit.api();

        let data = ActivateAccount {
            token,
            password: password.to_owned(),
        };

        api.public(ApiKind::$param.service_name_camel_case$)
            .query(&data)
            .post::<ApiResult<Account>>("v1/account/activate")
            .expect("activate account")
    }
}
