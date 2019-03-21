use diesel::{pg::PgConnection, prelude::*};

pub fn connect(db_url: &str) -> PgConnection {
    PgConnection::establish(db_url).unwrap_or_else(|_| panic!("Cannot connect to `{}`", db_url))
}
