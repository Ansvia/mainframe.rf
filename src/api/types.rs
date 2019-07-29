//! API message types
//! 
#![doc(hidden)]

use crate::ID;


#[derive(Serialize)]
pub struct EntriesResult<T> {
    pub entries: Vec<T>,
    pub count: i64,
}

#[derive(Deserialize)]
pub struct QueryEntries {
    pub query: Option<String>,
    pub offset: i64,
    pub limit: i64,
}

#[derive(Serialize, Deserialize)]
pub struct IdQuery {
    pub id: ID,
}



