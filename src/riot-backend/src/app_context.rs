use crate::models::{
    Device, NewDevice, NewRecord, NewUser, Owns, Record, RecordForm, Site, UpdateDevice,
    UpdateUser, User,
};
use crate::schema::device::activated;
use crate::utils::jwt::generate_token;
use crate::utils::password::get_pwd_hash;
use crate::{config::Config, db::DBClient};
use actix_web::cookie::{self, Cookie};
use chrono::Utc;
use diesel::dsl::exists;
use diesel::mysql::{self, Mysql};
use diesel::result::Error as DieselErr;
use diesel::sql_types::{BigInt, Unsigned};
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
    // Register a user, return Ok(id) if successful
    pub async fn register_user<'a>(&self, form: &NewUser<'a>) -> Result<u64, DieselErr> {
        use crate::schema::user;
        // TODO: Corner case: email conflicts with another's username
        // Currently we avoid this situation by restrict the username format in the route handler
        let mut conn = self.db.pool.get().await.unwrap();
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
    pub async fn update_user<'a>(&self, form: &UpdateUser<'a>) -> Result<usize, DieselErr> {
        let mut conn = self.db.pool.get().await.unwrap();
        let query = diesel::update(form).set(form);
        debug!("{}", debug_query::<Mysql, _>(&query).to_string());
        query.execute(&mut conn).await
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
    // Add a new device, return Ok(id) if successful
    pub async fn add_device<'a>(&self, form: &NewDevice<'a>) -> Result<u64, DieselErr> {
        use crate::schema::device;
        let mut conn = self.db.pool.get().await.unwrap();
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
        let mut conn = self.db.pool.get().await.unwrap();
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
    pub async fn update_site(
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
    pub async fn add_device_records(&self, form: &NewRecord) -> Result<usize, DieselErr> {
        use crate::schema::record;
        let mut conn = self.db.pool.get().await.unwrap();
        diesel::insert_into(record::table)
            .values(form)
            .execute(&mut conn)
            .await
    }
}

#[cfg(test)]
mod tests {
    use uuid::Uuid;

    use crate::{
        app_context::AppState,
        config::Config,
        db::DBClient,
        models::{NewDevice, NewUser, UpdateDevice, UpdateUser, UserPrivilege},
        utils::password::get_pwd_hash,
    };

    #[tokio::test]
    async fn full_db_raw() {
        let config = Config::init();
        let app: AppState = AppState {
            env: config.clone(),
            db: DBClient::new(&config.database_url).await,
        };

        let id = app
            .register_user(&NewUser {
                username: &format!("test{}", Uuid::new_v4()),
                email: &format!("kisa{}ma@mail.com", Uuid::new_v4()),
                hashed_password: &get_pwd_hash(&app.env.password_salt, "Aaa123,????".as_bytes()),
                privilege: UserPrivilege::Normal as u32,
            })
            .await
            .expect("Register failed");
        let new_email = format!("Modified{}@email.com", Uuid::new_v4());
        app.update_user(&UpdateUser {
            id,
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
            .get_user_by_username_or_email(&new_email)
            .await
            .expect("Get user failed");
        println!("{:?}", modified_user);
        assert_eq!(modified_user.activated, true);
        assert_eq!(modified_user.privilege, 4);

        // Add a device
        let dvc = NewDevice {
            uid: modified_user.id,
            name: "NewDeviceTest",
            desc: Some("Balalala"),
            dtype: 1,
            latitude: None,
            longitude: Some(12.3456),
        };
        let did = app
            .add_device(&dvc)
            .await
            .expect("Create new device failed");
        app.update_device(
            &UpdateDevice {
                id: did,
                name: Some("Modified!"),
                desc: Some(Some("Ok...")),
                latitude: None,
                longitude: None,
                last_update: None,
                activated: None,
            },
            Some(modified_user.id),
        )
        .await
        .expect("Create new device failed");
        let device = app.get_device_by_id(did).await.expect("Get device failed");
        println!("{:?}", device);
    }
}
