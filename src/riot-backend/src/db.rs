// DB
use sqlx::{Pool, mysql::MySql};

#[derive(Debug, Clone)]
pub struct DBClient {
   pool: Pool<MySql>,
}

impl DBClient {
    pub fn new(pool: Pool<MySql>) -> Self {
        DBClient { pool }
    }
}