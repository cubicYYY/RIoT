mod app_context;
mod config;
mod db;
mod errors;
mod handlers;
mod middlewares;
mod models;
mod mqtt_instance;
mod schema;
mod utils;

use config::Config;
use db::*;

use diesel_async::async_connection_wrapper::AsyncConnectionWrapper;
use diesel_async::AsyncMysqlConnection;
use handlers::*;

use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use log::info;
use models::*;
use std::thread;
use utoipa_swagger_ui::SwaggerUi;

use actix_files::Files;
use actix_web::{
    middleware::Logger,
    web::{self},
    App, HttpServer,
};

use rumqttc::Event::Incoming;
use rumqttc::Packet;
use utoipa::{
    openapi::security::{ApiKey, ApiKeyValue, HttpAuthScheme, HttpBuilder, SecurityScheme},
    Modify, OpenApi,
};

use crate::{app_context::AppState, mqtt_instance::mqtt_instancer::MqttDaemon};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Run-time env building
    env_logger::init();

    let config = Config::init();
    let app_state: AppState = AppState {
        env: config.clone(),
        db: DBClient::new(&config.database_url).await,
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
        paths(
            user_register,
            user_login,
            whoami,
            healthchecker,
            //devices
            owned_devices,
            device_info,
            upd_device_info,
            del_device,
            //tags
            //records
            device_records,
            insert_device_records,
        ),
        components(schemas(
            User,
            Device,
            Tag,
            Record,
            LoginForm,
            RegisterForm,
            NewDeviceForm,
            RecordForm,
            UpdateDeviceForm,
            Response
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
                        "JWT token saved in Cookie (you may need to add the \
                            `token` cookie by yourself via a browser extension like Cookie-Editor), \
                            also available in HTTP `Authorization` header (Bearer format)".to_string()
                    )
                ))
            );
        }
    }
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
    })
    .await?
    .expect("MySQL Database Connection Failed!");
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
                    // RIoT tag
                    .service(healthchecker)
                    //users
                    .service(user_register)
                    .service(user_login)
                    .service(whoami)
                    // Logged-in users only:
                    // devices
                    .service(owned_devices)
                    .service(device_info)
                    .service(upd_device_info)
                    .service(device_records)
                    .service(insert_device_records)
                    .service(del_device)
                    // tags
                    .service(owned_tags)
                    .service(tag_info)
                    .service(upd_tag_info)
                    .service(tagged_devices)
                    .service(tag_device)
                    .service(del_tag), // pipes
                                       // TODO...
                                       // Admin only:
                                       // TODO...
            )
            .service(Files::new("/", "./public").index_file("index.html"))
            .default_service(web::route().to(notfound_404))
    })
    .bind(("0.0.0.0", 8888))?
    .run()
    .await
}
