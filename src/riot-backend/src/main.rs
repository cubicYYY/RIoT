mod app_context;
mod config;
mod db;
mod errors;
mod handlers;
mod middlewares;
mod models;
mod schema;
mod utils;

use db::*;

use diesel_async::async_connection_wrapper::AsyncConnectionWrapper;
use diesel_async::AsyncMysqlConnection;
use handlers::*;

use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use log::{error, info};
use models::*;
use moka::future::Cache;
use std::{io::Write, thread, time::Duration};
use utoipa_swagger_ui::SwaggerUi;

use actix_files::Files;
use actix_web::{
    middleware::Logger,
    web::{self},
    App, HttpServer,
};

use utoipa::{
    openapi::security::{ApiKey, ApiKeyValue, HttpAuthScheme, HttpBuilder, SecurityScheme},
    Modify, OpenApi,
};

use crate::{app_context::AppState, errors::HttpError};
use actix_cors::Cors;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Run-time env building
    env_logger::init();

    let config = &config::CONFIG;
    let mqtt_db_conn = DBClient::new(&DBClient::get_database_url());
    let mqtt_host = config.mqtt.host.as_str();
    let mqtt_port = config.mqtt.port;
    // Embedded MQTT Listening Daemon
    thread::Builder::new()
        .name("MQTT-Listener".into())
        .spawn(move || {
            info!("Start MQTT thread");
            utils::mqtt_instance::mqtt_listening(mqtt_db_conn, mqtt_host, mqtt_port);
        })
        .expect("Failed to create MQTT listener!");
    // System info metrics tracker daemon
    tokio::spawn(SYSINFO_CACHE.new_daemon());

    // Generate OpenAPI docs

    #[derive(OpenApi)]
    #[openapi(
        paths(
            healthchecker,
            //accounts
            user_register,
            user_login,
            user_logout,
            user_info,
            upd_user_info,
            send_verification_email,
            verify_login_by_email,
            //devices
            add_device,
            owned_devices,
            device_info,
            upd_device_info,
            device_records,
            insert_device_records,
            del_device,
            //tags
            owned_tags,
            add_tag,
            tag_info,
            del_tag,
            upd_tag_info,
            tagged_devices,
            tag_device,
            untag_device,
        ),
        components(schemas(
            User,
            Device,
            Tag,
            Record,
            ServerStatistic,
            RegisterForm,
            LoginForm,
            UpdateUserForm,
            NewDeviceForm,
            RecordForm,
            UpdateDeviceForm,
            UpdateTagForm,
            TagDeviceForm,
            NewTagForm,
            Response,
            CachedSysinfo,
        )),
        modifiers(&SecurityJwt)
    )]
    struct ApiDoc;
    struct SecurityJwt;

    impl Modify for SecurityJwt {
        fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
            let components = openapi.components.as_mut().unwrap();
            components.add_security_scheme(
                "jwt_header",
                SecurityScheme::Http(
                    HttpBuilder::new()
                        .scheme(HttpAuthScheme::Bearer)
                        .bearer_format("JWT")
                        .description(Some("JWT token in HTTP `Authorization` header."))
                        .build(),
                ),
            );
            components.add_security_scheme(
                "jwt_cookie",
                SecurityScheme::ApiKey(ApiKey::Cookie(
                    ApiKeyValue::with_description(
                        "token".to_string(),
                        "JWT token saved in Cookie (you may need to add the `token` cookie by yourself \
                            via a browser extension like Cookie-Editor, **but not here**)".to_string()
                    )
                ))
            );
        }
    }
    let openapi = ApiDoc::openapi();
    let doc = ApiDoc::openapi()
        .to_pretty_json()
        .unwrap_or("Generate failed".to_string());
    let mut file = std::fs::File::create("riot_doc.json")?;
    if let Err(e) = file.write_all(doc.as_bytes()) {
        error!("Failed to generate OpenAPI doc file.{e}");
    }
    // SQL init migration

    info!("Start database init...");
    pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");
    /// seconds
    const TRY_CONNECT_INTERVAL: u64 = 5;
    tokio::task::spawn_blocking(move || {
        use diesel::prelude::Connection;
        let conn = 'wait_for_mysql: loop {
            match AsyncConnectionWrapper::<AsyncMysqlConnection>::establish(
                &DBClient::get_database_url(),
            ) {
                Ok(conn) => break 'wait_for_mysql conn,
                Err(e) => {
                    error!("Trying to reconnect MySQL in {TRY_CONNECT_INTERVAL} seconds. ({e:?})");
                    thread::sleep(Duration::from_secs(TRY_CONNECT_INTERVAL));
                    continue 'wait_for_mysql;
                }
            };
        };
        let mut async_wrapper = AsyncConnectionWrapper::<AsyncMysqlConnection>::from(conn);
        async_wrapper.run_pending_migrations(MIGRATIONS).unwrap();
        Ok::<_, Box<dyn std::error::Error + Send + Sync>>(())
    })
    .await?
    .expect("MySQL Database Connection Failed!");
    info!("Database init finished!");

    // Register services (API endpoints and user interfaces routes)
    let app_state = AppState {
        env: config,
        db: DBClient::new(&DBClient::get_database_url()),
        rate_limit: Cache::builder()
            .time_to_idle(Duration::from_secs(60)) // idle, 60s
            .build(),
        one_time_code: Cache::builder()
            .time_to_live(Duration::from_secs(60 * 60 * 24)) // live, 24h
            .build(),
    };
    let is_debug = app_state.env.riot.debug;
    info!("IN DEBUG MODE");
    let host: &str = app_state.env.riot.host.as_str();
    let app_data = web::Data::new(app_state);
    HttpServer::new(move || {
        App::new()
            .wrap(if is_debug {
                Cors::permissive()
            } else {
                Cors::default()
                    .supports_credentials()
                    .allowed_origin(host)
                    .allow_any_method()
                    .allow_any_header()
                    .max_age(600)
            })
            .app_data(web::Data::clone(&app_data))
            // !WARN: Must cloned the data into the server,
            // !otherwise each worker thread will have their own states without sharing!
            .wrap(Logger::default())
            .service(web::redirect("/api-doc", "/api-doc/"))
            .service(
                // Must be register here (the top instead of the bottom of service chain)
                SwaggerUi::new("/api-doc/{_:.*}").url("/api-docs/openapi.json", openapi.clone()),
            )
            .service(
                web::scope("/api")
                    .app_data(
                        web::QueryConfig::default()
                            // use custom error handler
                            .error_handler(|err, _req| {
                                // err.into()
                                HttpError::bad_request(err.to_string()).into()
                            }),
                    )
                    // RIoT tag
                    .service(healthchecker)
                    //users
                    .service(user_register)
                    .service(user_login)
                    .service(user_logout)
                    .service(user_info)
                    .service(upd_user_info)
                    .service(send_verification_email)
                    .service(verify_login_by_email)
                    // Logged-in users only:
                    // devices
                    .service(add_device)
                    .service(owned_devices)
                    .service(device_info)
                    .service(upd_device_info)
                    .service(device_records)
                    .service(insert_device_records)
                    .service(del_device)
                    // tags
                    .service(add_tag)
                    .service(owned_tags)
                    .service(tag_info)
                    .service(upd_tag_info)
                    .service(tagged_devices)
                    .service(tag_device)
                    .service(del_tag),
                // pipes
                // TODO...
                // Admin only:
                // TODO...
            )
            .service(Files::new("/", "./dist").index_file("index.html"))
            .default_service(web::route().to(index))
    })
    .bind(("0.0.0.0", 8888))?
    .run()
    .await
}
