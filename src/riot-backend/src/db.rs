// DB
use diesel_async::pooled_connection::deadpool::Pool;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::AsyncMysqlConnection;

#[derive(Clone)]
pub struct DBClient {
    pool: Pool<AsyncMysqlConnection>,
}

impl DBClient {
    pub async fn new(db_url: &str) -> Self {
        let config = AsyncDieselConnectionManager::<AsyncMysqlConnection>::new(db_url);
        DBClient {
            pool: Pool::builder(config)
                .build()
                .expect("Cannot build SQL pool."),
        }
    }
}

#[cfg(test)]
mod tests {
    use diesel::{sql_query, deserialize::QueryableByName, sql_types::Text};
    use diesel_async::{pooled_connection::deadpool::Object, AsyncMysqlConnection, RunQueryDsl};
    use futures::future::join_all;

    use super::DBClient;

    #[tokio::test]
    async fn multi_connections() {
        let url = "mysql://root:root@localhost"; // Modify this link on your own
        let pool = DBClient::new(url).await.pool;

        let mut futures = vec![];

        #[derive(QueryableByName, Debug)]
        struct Raw {
            #[sql_type = "Text"]
            rstr: String
        }

        for _ in 0..10 {
            let pool = pool.clone();
            let query_test = async move {
                let conn: &mut Object<AsyncMysqlConnection> = &mut pool.get().await.unwrap();
                let res: Raw = sql_query("SELECT CURRENT_USER() AS rstr;")
                    .get_result(conn)
                    .await
                    .unwrap();
                println!("{:?}", res.rstr);
            };
            
            futures.push(query_test);
        }
        println!("{:?}", tokio::join!(join_all(futures)));
    }
}
