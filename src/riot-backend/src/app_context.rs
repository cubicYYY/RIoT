use crate::models::{Device, Owns, Record, RecordFormDb, RecordFormWeb, Site, User};
use crate::schema::device::activated;
use crate::utils::jwt::generate_token;
use crate::utils::password::get_pwd_hash;
use crate::{config::Config, db::DBClient};
use actix_web::cookie::{self, Cookie};
use chrono::Utc;
use diesel::dsl::exists;
use diesel::mysql::Mysql;
use diesel::result::Error as DieselErr;
use diesel::{
    debug_query, BoolExpressionMethods, ExpressionMethods, Insertable, QueryDsl, SelectableHelper,
    Table,
};
use diesel_async::RunQueryDsl;
use log::debug;

#[derive(Clone)]
pub struct AppState {
    pub env: Config,
    pub db: DBClient,
}

// User ops
// TODO: Refactor to group operations
impl AppState {
    pub fn get_jwt_cookie(&self, uid: u64) -> Cookie {
        let jwt_token =
            generate_token(&uid.to_string(), self.env.jwt_secret, self.env.jwt_maxage).unwrap();
        Cookie::build("token", jwt_token)
            .path("/")
            .max_age(cookie::time::Duration::new(self.env.jwt_maxage, 0))
            .http_only(true)
            .finish()
    }
    pub async fn register_user(
        &self,
        username_: &str,
        email_: &str,
        password_: &str,
        privilege_level: u32,
    ) -> Result<usize, DieselErr> {
        use crate::schema::user::{self, dsl::*};
        // TODO: Corner case: email conflicts with another's username
        // TODO: Refactor to pass params in form
        // Currently we avoid this situation by restrict the username format in the route handler
        let mut conn = self.db.pool.get().await.unwrap();
        let query = diesel::insert_into(user::table).values((
            username.eq(username_),
            email.eq(email_),
            password.eq(get_pwd_hash(self.env.password_salt, password_.as_bytes())),
            privilege.eq(privilege_level),
            since.eq(Utc::now().naive_utc()),
        ));
        debug!("{}", debug_query::<Mysql, _>(&query).to_string());
        query.execute(&mut conn).await
    }
    pub async fn get_user_by_username_or_email(&self, keyword: &str) -> Result<User, DieselErr> {
        use crate::schema::user::dsl::*;
        let mut conn = self.db.pool.get().await.unwrap();
        let query = user
            .select(User::as_select())
            .filter(email.eq(keyword).or(username.eq(keyword)));
        debug!("{}", debug_query::<Mysql, _>(&query).to_string());
        query.first(&mut conn).await
    }
    pub async fn get_user_by_id(&self, id_: u64) -> Result<User, DieselErr> {
        use crate::schema::user::dsl::*;
        let mut conn = self.db.pool.get().await.unwrap();
        user.select(User::as_select())
            .filter(id.eq(id_))
            .first(&mut conn)
            .await
    }
    pub async fn get_user_by_api_key(&self, api_key_: &str) -> Result<User, DieselErr> {
        use crate::schema::user::dsl::*;
        let mut conn = self.db.pool.get().await.unwrap();
        user.select(User::as_select())
            .filter(api_key.eq(api_key_))
            .first(&mut conn)
            .await
    }
    /// Ban/activate a user
    pub async fn update_user_status(&self, id_: u64, activated_: bool) -> Result<usize, DieselErr> {
        use crate::schema::user::dsl::*;
        let mut conn = self.db.pool.get().await.unwrap();
        diesel::update(user)
            .filter(id.eq(id_))
            .set(activated.eq(activated_))
            .execute(&mut conn)
            .await
    }
    pub async fn get_device_by_id(&self, id_: u64) -> Result<Device, DieselErr> {
        use crate::schema::device::dsl::*;
        let mut conn = self.db.pool.get().await.unwrap();
        device
            .select(Device::as_select())
            .filter(id.eq(id_))
            .first(&mut conn)
            .await
    }
    pub async fn get_owned_devices(&self, uid_: u64) -> Result<Vec<Device>, DieselErr> {
        use crate::schema::device::dsl::*;
        let mut conn = self.db.pool.get().await.unwrap();
        device
            .select(Device::as_select())
            .filter(uid.eq(uid_))
            .get_results(&mut conn)
            .await
    }
    pub async fn update_device_status(
        &self,
        id_: u64,
        activated_: bool,
        only_for: Option<u64>,
    ) -> Result<usize, DieselErr> {
        use crate::schema::device::dsl::*;
        let mut conn = self.db.pool.get().await.unwrap();
        let query = diesel::update(device).filter(id.eq(id_));
        if let Some(uid_) = only_for {
            query
                .filter(uid.eq(uid_))
                .set(activated.eq(activated_))
                .execute(&mut conn)
                .await
        } else {
            query.set(activated.eq(activated_)).execute(&mut conn).await
        }
    }
    pub async fn get_site_by_id(&self, id_: u64) -> Result<Site, DieselErr> {
        use crate::schema::site::dsl::*;
        let mut conn = self.db.pool.get().await.unwrap();
        site.select(Site::as_select())
            .filter(id.eq(id_))
            .first(&mut conn)
            .await
    }
    pub async fn get_owned_sites(&self, uid_: u64) -> Result<Vec<Site>, DieselErr> {
        use crate::schema::site::dsl::*;
        let mut conn = self.db.pool.get().await.unwrap();
        site.select(Site::as_select())
            .filter(uid.eq(uid_))
            .get_results(&mut conn)
            .await
    }
    pub async fn update_site_status(
        &self,
        id_: u64,
        activated_: bool,
        only_for: Option<u64>,
    ) -> Result<usize, DieselErr> {
        use crate::schema::site::dsl::*;
        let mut conn = self.db.pool.get().await.unwrap();
        let query = diesel::update(site).filter(id.eq(id_));
        if let Some(uid_) = only_for {
            query
                .filter(uid.eq(uid_))
                .set(activated.eq(activated_))
                .execute(&mut conn)
                .await
        } else {
            query.set(activated.eq(activated_)).execute(&mut conn).await
        }
    }
    pub async fn device_belongs_to(&self, did_: u64, uid_: u64) -> Result<bool, DieselErr> {
        use crate::schema::device::dsl::*;
        let mut conn = self.db.pool.get().await.unwrap();
        diesel::select(exists(device.filter(id.eq(did_).and(uid.eq(uid_)))))
            .get_result(&mut conn)
            .await
    }
    pub async fn get_device_records(&self, did_: u64) -> Result<Vec<Record>, DieselErr> {
        use crate::schema::record::dsl::*;
        let mut conn = self.db.pool.get().await.unwrap();
        record
            .select(Record::as_select())
            .filter(did.eq(did_))
            .get_results(&mut conn)
            .await
    }
    pub async fn add_device_records(&self, form: &RecordFormDb) -> Result<usize, DieselErr> {
        use crate::schema::record;
        let mut conn = self.db.pool.get().await.unwrap();
        diesel::insert_into(record::table)
            .values(form)
            .execute(&mut conn)
            .await
    }
}
