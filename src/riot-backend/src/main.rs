mod db;
mod errors;
mod handlers;
mod jwt_utils;
mod middlewares;
mod models;
mod mqtt_instance;
mod config;
mod schema;

use config::Config;
use db::*;
use diesel::MysqlConnection;
use diesel_async::{pooled_connection::deadpool::Object, AsyncMysqlConnection};
use diesel_async::async_connection_wrapper::AsyncConnectionWrapper;
use handlers::*;
use jwt_utils::*;
use models::*;
use std::thread;
use utoipa_swagger_ui::{SwaggerUi, Url};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use log::{debug, error, log_enabled, info, Level};

use actix_files::Files;
use actix_web::{
    middleware::Logger,
    web::{self},
    App, HttpServer,
};

use rumqttc::Event::Incoming;
use rumqttc::Packet;
use utoipa::OpenApi;

use crate::{middlewares::RequireAuth, mqtt_instance::mqtt_instancer::MqttDaemon};

#[derive(Clone)]
pub struct AppState {
    pub env: Config,
    pub db: DBClient,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Run-time env building

    std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    let config = Config::init();
    let app_state: AppState = AppState {
        env: config.clone(),
        db: DBClient::new(&config.database_url).await
    };

    // MQTT Listening

    thread::spawn(move || {
        let (_, mut connection) = MqttDaemon::new_daemon();
        for notification in connection.iter() {
            if let Incoming(Packet::Publish(published)) = notification.unwrap() {
                println!(
                    "got topic={} payload={:?}",
                    published.topic, published.payload
                )
            }
        }
    });
    
    // Generate OpenAPI docs

    #[derive(OpenApi)]
    #[openapi(
        paths(device_info),
        components(schemas(User), schemas(Device), schemas(Site))
    )]
    struct ApiDoc;

    let openapi = ApiDoc::openapi();

    // SQL init migration

    info!("Start database init...");
    pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");
    tokio::task::spawn_blocking(move || {
        use diesel::prelude::Connection;
        let conn = AsyncConnectionWrapper::<AsyncMysqlConnection>::establish(&config.database_url)?;
        let mut async_wrapper = AsyncConnectionWrapper::<AsyncMysqlConnection>::from(conn);
        async_wrapper.run_pending_migrations(MIGRATIONS).unwrap();
        Ok::<_, Box<dyn std::error::Error + Send + Sync>>(())
    }).await?.unwrap();
    info!("Database init finished!");

    // Register services (API endpoints and user interfaces routes)

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .wrap(Logger::default())
            .service(web::redirect("/api-doc", "/api-doc/"))
            .service(
                // Must be register here (the top instead of the bottom of service chain)
                SwaggerUi::new("/api-doc/{_:.*}").url("/api-docs/openapi.json", openapi.clone()),
            )
            .service(
                web::scope("/api")
                    .wrap(RequireAuth {
                        priv_needed: 1.into(),
                    })
                    //users
                    .service(user_register)
                    .service(user_login)
                    // devices
                    .service(all_devices)
                    .service(device_info)
                    .service(upd_device_info)
                    .service(device_records)
                    .service(upd_device_records)
                    .service(del_device)
                    // sites
                    .service(all_sites)
                    .service(site_info)
                    .service(upd_site_info)
                    .service(site_devices)
                    .service(upd_site_devices)
                    .service(del_site)
                    // pipes
                    .service(ws_socket)
                    .service(mqtt_sub)
                    .service(mqtt_unsub),
            )
            .service(Files::new("/", "./public").index_file("index.html"))
            .default_service(web::route().to(notfound_404))
    })
    .bind(("0.0.0.0", 8888))?
    .run()
    .await
}
