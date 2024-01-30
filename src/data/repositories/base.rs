use std::sync::Arc;

use serde::{Deserialize, Serialize};

use crate::data::repositories::query_stuff::{
    DEFAULT_LIMIT, DEFAULT_OFFSET, QueryParams,
};
use crate::infrastructure::databases::postgres::connection::DBConn;

#[derive(Debug, Serialize, Deserialize)]
pub struct GameQueryParams {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    pub title: Option<String>,
}

impl QueryParams for GameQueryParams {
    fn limit(&self) -> i64 {
        self.limit.or(DEFAULT_LIMIT).unwrap_or_default()
    }
    fn offset(&self) -> i64 {
        self.offset.or(DEFAULT_OFFSET).unwrap_or_default()
    }
}

pub struct BaseDieselRepository {
    pub pool: Arc<DBConn>,
}

impl BaseDieselRepository {
    pub fn new(db: Arc<DBConn>) -> Self {
        BaseDieselRepository { pool: db }
    }
}


