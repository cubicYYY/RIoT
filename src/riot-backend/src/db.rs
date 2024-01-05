use std::time::Duration;

use crate::config::CONFIG;
// DB
use crate::models::{
    Device, NewDevice, NewRecord, NewTag, NewUser, Record, Tag, UpdateDevice, UpdateTag,
    UpdateUser, User,
};
use chrono::NaiveDateTime;
use diesel::dsl::exists;
use diesel::mysql::Mysql;
use diesel::result::Error as DieselErr;
use diesel::{debug_query, BoolExpressionMethods, ExpressionMethods, QueryDsl, SelectableHelper};
use diesel_async::pooled_connection::deadpool::Pool;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::AsyncMysqlConnection;
use diesel_async::{AsyncConnection, RunQueryDsl};
use log::debug;

#[derive(Clone)]
pub struct DBClient {
    pub pool: Pool<AsyncMysqlConnection>,
}

impl DBClient {
    pub fn get_database_url() -> String {
        format!(
            "mysql://{uname}:{pwd}@{host}:{port}/{database}",
            uname = CONFIG.mysql.username,
            pwd = CONFIG.mysql.password,
            host = CONFIG.mysql.host,
            port = CONFIG.mysql.port,
            database = CONFIG.mysql.database
        )
    }
    pub fn new(db_url: &str) -> Self {
        let config = AsyncDieselConnectionManager::<AsyncMysqlConnection>::new(db_url);
        DBClient {
            pool: Pool::builder(config)
                .build()
                .expect("Cannot build SQL pool."),
        }
    }
    /// Register a user, return Ok(id) if successful
    pub async fn register_user<'a>(&self, form: &NewUser<'a>) -> Result<u64, DieselErr> {
        use crate::schema::user;
        // TODO: Corner case: email conflicts with another's username
        // Currently we avoid this situation by restrict the username format in the route handler
        let mut conn = self.pool.get().await.unwrap();
        let query = diesel::insert_into(user::table).values(form);
        debug!("{}", debug_query::<Mysql, _>(&query).to_string());
        query.execute(&mut conn).await?;
        diesel::sql_function!(fn last_insert_id() -> Unsigned<BigInt>);
        // ! To get the correct `id``, must be in a single connection
        let id: u64 = diesel::select(last_insert_id()).first(&mut conn).await?;
        Ok(id)
    }
    pub async fn get_user_by_username_or_email(&self, keyword: &str) -> Result<User, DieselErr> {
        use crate::schema::user::dsl::*;
        let mut conn = self.pool.get().await.unwrap();
        let query = user
            .select(User::as_select())
            .filter(email.eq(keyword).or(username.eq(keyword)));
        debug!("{}", debug_query::<Mysql, _>(&query).to_string());
        query.first(&mut conn).await
    }
    pub async fn get_user_by_id(&self, id_: u64) -> Result<User, DieselErr> {
        use crate::schema::user::dsl::*;
        let mut conn = self.pool.get().await.unwrap();
        user.select(User::as_select())
            .filter(id.eq(id_))
            .first(&mut conn)
            .await
    }
    pub async fn get_user_by_api_key(&self, api_key_: &str) -> Result<User, DieselErr> {
        use crate::schema::user::dsl::*;
        let mut conn = self.pool.get().await.unwrap();
        user.select(User::as_select())
            .filter(api_key.eq(api_key_))
            .first(&mut conn)
            .await
    }
    pub async fn update_user<'a>(&self, form: &UpdateUser<'a>) -> Result<usize, DieselErr> {
        let mut conn = self.pool.get().await.unwrap();
        let query = diesel::update(form).set(form);
        debug!("{}", debug_query::<Mysql, _>(&query).to_string());
        query.execute(&mut conn).await
    }
    pub async fn get_device_by_id(&self, id_: u64) -> Result<Device, DieselErr> {
        use crate::schema::device::dsl::*;
        let mut conn = self.pool.get().await.unwrap();
        device
            .select(Device::as_select())
            .filter(id.eq(id_))
            .first(&mut conn)
            .await
    }
    pub async fn get_device_by_topic(&self, topic_: &str) -> Result<Device, DieselErr> {
        use crate::schema::device::dsl::*;
        let mut conn = self.pool.get().await.unwrap();
        device
            .select(Device::as_select())
            .filter(topic.eq(topic_))
            .first(&mut conn)
            .await
    }
    // Plural form of `get_device_by_id`
    pub async fn get_device_by_ids(&self, ids: &[u64]) -> Result<Vec<Device>, DieselErr> {
        use crate::schema::device::dsl::*;
        let mut conn = self.pool.get().await.unwrap();
        device
            .select(Device::as_select())
            .filter(id.eq_any(ids))
            .get_results(&mut conn)
            .await
    }
    pub async fn get_device_cnt(&self) -> Result<i64, DieselErr> {
        use crate::schema::device::dsl::*;
        let mut conn = self.pool.get().await.unwrap();
        device.count().get_result(&mut conn).await
    }
    pub async fn get_online_device_cnt(&self) -> Result<i64, DieselErr> {
        use crate::schema::device::dsl::*;
        let mut conn = self.pool.get().await.unwrap();
        let idle_timestamp: NaiveDateTime =
            (chrono::Utc::now() - Duration::from_secs(60 * 10)).naive_utc(); // 10 min

        device
            .filter(last_update.ge(idle_timestamp))
            .count()
            .get_result(&mut conn)
            .await
    }
    pub async fn get_owned_devices(&self, uid_: u64) -> Result<Vec<Device>, DieselErr> {
        use crate::schema::device::dsl::*;
        let mut conn = self.pool.get().await.unwrap();
        device
            .select(Device::as_select())
            .filter(uid.eq(uid_))
            .get_results(&mut conn)
            .await
    }
    /// Add a new device, return Ok(id) if successful
    pub async fn add_device<'a>(&self, form: &NewDevice<'a>) -> Result<u64, DieselErr> {
        use crate::schema::device;
        let mut conn = self.pool.get().await.unwrap();
        let query = diesel::insert_into(device::table).values(form);
        debug!("{}", debug_query::<Mysql, _>(&query).to_string());
        query.execute(&mut conn).await?;
        diesel::sql_function!(fn last_insert_id() -> Unsigned<BigInt>);
        // ! To get the correct `id``, must be in a single connection
        let id: u64 = diesel::select(last_insert_id()).first(&mut conn).await?;
        Ok(id)
    }
    pub async fn update_device<'a>(
        &self,
        form: &UpdateDevice<'a>,
        only_for: Option<u64>,
    ) -> Result<usize, DieselErr> {
        use crate::schema::device::dsl::*;
        let mut conn = self.pool.get().await.unwrap();
        let query = diesel::update(form);
        if let Some(uid_) = only_for {
            query
                .filter(uid.eq(uid_))
                .set(form)
                .execute(&mut conn)
                .await
        } else {
            query.set(form).execute(&mut conn).await
        }
    }
    // Add a new tag, return Ok(id) if successful
    pub async fn add_tag<'a>(&self, form: &NewTag<'a>) -> Result<u64, DieselErr> {
        use crate::schema::tag;
        let mut conn = self.pool.get().await.unwrap();
        let query = diesel::insert_into(tag::table).values(form);
        debug!("{}", debug_query::<Mysql, _>(&query).to_string());
        query.execute(&mut conn).await?;
        diesel::sql_function!(fn last_insert_id() -> Unsigned<BigInt>);
        // ! To get the correct `id``, must be in a single connection
        let id: u64 = diesel::select(last_insert_id()).first(&mut conn).await?;
        Ok(id)
    }
    pub async fn get_tag_by_id(&self, id_: u64) -> Result<Tag, DieselErr> {
        use crate::schema::tag::dsl::*;
        let mut conn = self.pool.get().await.unwrap();
        tag.select(Tag::as_select())
            .filter(id.eq(id_))
            .first(&mut conn)
            .await
    }
    pub async fn get_owned_tags(&self, uid_: u64) -> Result<Vec<Tag>, DieselErr> {
        use crate::schema::tag::dsl::*;
        let mut conn = self.pool.get().await.unwrap();
        tag.select(Tag::as_select())
            .filter(uid.eq(uid_))
            .get_results(&mut conn)
            .await
    }
    /// return: rows affected
    pub async fn update_tag<'a>(
        &self,
        form: &UpdateTag<'a>,
        only_for: Option<u64>,
    ) -> Result<usize, DieselErr> {
        use crate::schema::tag::dsl::*;
        let mut conn = self.pool.get().await.unwrap();
        let query = diesel::update(form);
        if let Some(uid_) = only_for {
            query
                .filter(uid.eq(uid_))
                .set(form)
                .execute(&mut conn)
                .await
        } else {
            query.set(form).execute(&mut conn).await
        }
    }
    pub async fn device_belongs_to(&self, did_: u64, uid_: u64) -> Result<bool, DieselErr> {
        use crate::schema::device::dsl::*;
        let mut conn = self.pool.get().await.unwrap();
        diesel::select(exists(device.filter(id.eq(did_).and(uid.eq(uid_)))))
            .get_result(&mut conn)
            .await
    }
    pub async fn get_device_records(&self, did_: u64) -> Result<Vec<Record>, DieselErr> {
        use crate::schema::record::dsl::*;
        let mut conn = self.pool.get().await.unwrap();
        record
            .select(Record::as_select())
            .filter(did.eq(did_))
            .get_results(&mut conn)
            .await
    }
    pub async fn add_device_records<'a>(&self, form: &NewRecord<'a>) -> Result<(), DieselErr> {
        use crate::schema::{device, record};
        use diesel_async::scoped_futures::ScopedFutureExt;
        let mut conn = self.pool.get().await.unwrap();
        conn.transaction(|conn| {
            async move {
                // Insert the record
                diesel::insert_into(record::table)
                    .values(form)
                    .execute(conn)
                    .await?;
                // Update last update
                diesel::update(device::table.filter(device::id.eq(form.did)))
                    .set(device::last_update.eq(form.timestamp))
                    .execute(conn)
                    .await?;
                diesel::result::QueryResult::Ok(())
            }
            .scope_boxed()
        })
        .await
    }
    pub async fn tag_belongs_to(&self, tid_: u64, uid_: u64) -> Result<bool, DieselErr> {
        use crate::schema::tag::dsl::*;
        let mut conn = self.pool.get().await.unwrap();
        diesel::select(exists(tag.filter(id.eq(tid_).and(uid.eq(uid_)))))
            .get_result(&mut conn)
            .await
    }
    /// get device IDs
    pub async fn get_dids_under_tag(&self, tid_: u64) -> Result<Vec<u64>, DieselErr> {
        use crate::schema::owns::dsl::*;
        let mut conn = self.pool.get().await.unwrap();
        owns.select(did)
            .filter(tid.eq(tid_))
            .get_results(&mut conn)
            .await
    }
    pub async fn tag_device(&self, tid_: u64, did_: u64) -> Result<usize, DieselErr> {
        use crate::schema::owns::dsl::*;
        let mut conn = self.pool.get().await.unwrap();
        diesel::insert_into(owns)
            .values((tid.eq(tid_), did.eq(did_)))
            .execute(&mut conn)
            .await
    }
    pub async fn untag_device(&self, tid_: u64, did_: u64) -> Result<usize, DieselErr> {
        use crate::schema::owns::dsl::*;
        let mut conn = self.pool.get().await.unwrap();
        diesel::delete(owns)
            .filter(tid.eq(tid_).and(did.eq(did_)))
            .execute(&mut conn)
            .await
    }
}

#[cfg(test)]
mod tests {
    use diesel::{deserialize::QueryableByName, sql_query, sql_types::Text};
    use diesel_async::{pooled_connection::deadpool::Object, AsyncMysqlConnection, RunQueryDsl};
    use futures::future::join_all;

    use super::DBClient;

    #[tokio::test]
    async fn multi_connections_with_raw_query() {
        let url = "mysql://root:root@localhost"; // Modify this link on your own
        let pool = DBClient::new(url).pool;

        let mut futures = vec![];

        #[derive(QueryableByName, Debug)]
        struct Raw {
            #[diesel(sql_type = Text)]
            rstr: String,
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
    use chrono::NaiveDateTime;
    use moka::future::Cache;
    use uuid::Uuid;

    use crate::{
        app_context::AppState,
        config::CONFIG,
        models::{
            NewDevice, NewRecord, NewTag, NewUser, UpdateDevice, UpdateTag, UpdateUser,
            UserPrivilege,
        },
        utils::password::get_pwd_hash,
    };

    #[tokio::test]
    async fn full_db_raw() {
        let config = &CONFIG;
        let app: AppState = AppState {
            env: config,
            db: DBClient::new(&DBClient::get_database_url()),
            rate_limit: Cache::new(1024),
            one_time_code: Cache::new(1024),
        };

        let uid = app
            .db
            .register_user(&NewUser {
                username: &format!("test{}", Uuid::new_v4()),
                email: &format!("kisa{}ma@mail.com", Uuid::new_v4()),
                hashed_password: &get_pwd_hash(
                    &app.env.riot.password_salt.as_bytes(),
                    "Aaa123,????".as_bytes(),
                ),
                privilege: UserPrivilege::Normal as u32,
                api_key: None,
            })
            .await
            .expect("Register failed");
        let new_email = format!("Modified{}@email.com", Uuid::new_v4());
        app.db
            .update_user(&UpdateUser {
                id: uid,
                username: None,
                email: Some(&new_email),
                hashed_password: None,
                privilege: None,
                activated: Some(true),
                api_key: None,
            })
            .await
            .expect("Modify user failed");
        let modified_user = app
            .db
            .get_user_by_username_or_email(&new_email)
            .await
            .expect("Get user failed");
        println!("{:?}", modified_user);
        assert_eq!(modified_user.activated, true);
        assert_eq!(modified_user.privilege, 4);

        // Add a device
        let topic = format!("api-key-for-me/yyy/test{}", Uuid::new_v4());
        let dvc = NewDevice {
            uid: modified_user.id,
            name: "NewDeviceTest",
            desc: Some("Balalala"),
            dtype: 1,
            latitude: None,
            longitude: Some(12.3456),
            topic: &topic,
        };
        let did = app
            .db
            .add_device(&dvc)
            .await
            .expect("Create new device failed");
        app.db
            .update_device(
                &UpdateDevice {
                    id: did,
                    name: Some("Modified!"),
                    desc: Some(Some("Ok...")),
                    dtype: None,
                    latitude: None,
                    longitude: None,
                    last_update: None,
                    activated: None,
                    topic: None,
                },
                Some(modified_user.id),
            )
            .await
            .expect("Create new device failed");
        let modified_device = app
            .db
            .get_device_by_id(did)
            .await
            .expect("Get device failed");
        println!("{:?}", modified_device);
        assert_eq!(modified_device.name, "Modified!");
        assert_eq!(modified_device.desc, Some("Ok...".into()));
        // records
        app.db
            .add_device_records(&NewRecord {
                did,
                payload: &[1, 2, 3],
                timestamp: &NaiveDateTime::from_timestamp_millis(1662921288000).unwrap(),
            })
            .await
            .expect("Add record failed!");
        let records = app
            .db
            .get_device_records(did)
            .await
            .expect("Get records failed");
        println!("{:?}", records);
        assert!(!records.is_empty());

        // tags
        let tid = app
            .db
            .add_tag(&NewTag {
                uid,
                name: &format!("tag_{}", Uuid::new_v4()),
                desc: None,
                activated: true,
            })
            .await
            .expect("Create new tag failed");

        app.db
            .update_tag(
                &UpdateTag {
                    id: tid,
                    name: None,
                    desc: Some(Some("Modified!!!")),
                    activated: None,
                },
                Some(uid),
            )
            .await
            .expect("Update tag error!");

        let modified_tag = app.db.get_tag_by_id(tid).await.expect("Get tag failed");
        assert_eq!(modified_tag.desc, Some("Modified!!!".to_string()));
        println!("{:?}", modified_tag);

        // tag device
        app.db
            .tag_device(modified_tag.id, modified_device.id)
            .await
            .expect("Tagging failed!");
        let res = app
            .db
            .get_dids_under_tag(modified_tag.id)
            .await
            .expect("Get dids under the tag failed");
        println!("{:?}", res);
        assert!(!res.is_empty());
    }
    #[tokio::test]
    async fn racing() {
        let config = &CONFIG;
        let app: AppState = AppState {
            env: config,
            db: DBClient::new(&DBClient::get_database_url()),
            rate_limit: Cache::new(1024),
            one_time_code: Cache::new(1024),
        };
        let mut conn = app.db.pool.get().await.unwrap();

        let new_user = async move {
            for i in 0..10 {
                println!("reg");
                app.db
                    .register_user(&NewUser {
                        username: &format!("racing{}{}", i, Uuid::new_v4()),
                        email: &format!("kisa{}ma@mail.com", Uuid::new_v4()),
                        hashed_password: &get_pwd_hash(
                            &app.env.riot.password_salt.as_bytes(),
                            "Aaa123,????".as_bytes(),
                        ),
                        privilege: UserPrivilege::Normal as u32,
                        api_key: None,
                    })
                    .await
                    .expect("Register failed");
            }
        };
        let modify_user = async move {
            for _ in 0..10 {
                diesel::sql_function!(fn last_insert_id() -> Unsigned<BigInt>);
                let id: u64 = diesel::select(last_insert_id())
                    .first(&mut conn)
                    .await
                    .unwrap();
                println!("upd lii={}", id);
                assert_eq!(id, 0);
            }
        };
        tokio::join!(new_user, modify_user);
    }
}
