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

use chrono::Utc;
use config::Config;
use db::*;

use diesel_async::async_connection_wrapper::AsyncConnectionWrapper;
use diesel_async::AsyncMysqlConnection;
use handlers::*;

use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use log::{debug, error, info};
use models::*;
use moka::future::Cache;
use std::{thread, time::Duration};
use utoipa_swagger_ui::SwaggerUi;
use uuid::Uuid;

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

use crate::{app_context::AppState, errors::HttpError, mqtt_instance::mqtt_instancer::MqttDaemon};

#[actix_web::main]
async fn mqtt_listening(mqtt_app: AppState) {
    // !important: enough randomness to avoid being kicked by a malicious client with the same id
    let (mut client, mut eventloop) =
        MqttDaemon::new_daemon(("MQTT_DAEMON".to_string() + &Uuid::new_v4().to_string()).as_str());
    client
        .subscribe("#", rumqttc::QoS::ExactlyOnce)
        .await
        .expect("Subscribe failed!");
    'eventloop: loop {
        let notification = eventloop.poll().await;
        let notification = match notification {
            Err(_) => {
                // May be kicked...
                let (new_client, new_eventloop) = MqttDaemon::new_daemon(
                    ("MQTT_DAEMON".to_string() + &Uuid::new_v4().to_string()).as_str(),
                );
                client = new_client;
                eventloop = new_eventloop;
                client
                    .subscribe("#", rumqttc::QoS::ExactlyOnce)
                    .await
                    .expect("Subscribe failed!");
                continue 'eventloop;
            }
            Ok(event) => event,
        };

        if let Incoming(Packet::Publish(published)) = notification {
            debug!(
                "got topic={} payload={:?}",
                published.topic, published.payload
            );
            let device = mqtt_app.get_device_by_topic(&published.topic).await;
            let device = match device {
                Ok(device) => device,
                Err(e) => {
                    error!(
                        "Unable to find the device registered for the topic: {:?}",
                        e
                    );
                    continue 'eventloop;
                }
            };
            let res = mqtt_app
                .add_device_records(&NewRecord {
                    did: device.id,
                    payload: &published.payload,
                    timestamp: &Utc::now().naive_utc(),
                })
                .await;
            if let Err(e) = res {
                error!("Insert record failed: {:?}", e);
                continue 'eventloop;
            }
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Run-time env building
    env_logger::init();

    let config = Config::init();
    let app_state: AppState = AppState {
        env: config.clone(),
        db: DBClient::new(&config.database_url).await,
        rate_limit: Cache::builder()
            .time_to_idle(Duration::from_secs(60)) // idle, 60s
            .build(),
        one_time_code: Cache::builder()
            .time_to_live(Duration::from_secs(60 * 60 * 12)) // live, 12h
            .build(),
    };

    // Embedded MQTT Listening Daemon
    let mqtt_app = app_state.clone(); // TODO: no need to share all members in the app state, only db
    thread::Builder::new()
        .name("MQTT-Listener".into())
        .spawn(move || {
            info!("Start MQTT thread");
            mqtt_listening(mqtt_app);
        })
        .expect("Failed to create MQTT listener!");

    // Generate OpenAPI docs

    #[derive(OpenApi)]
    #[openapi(
        paths(
            healthchecker,
            //accounts
            user_register,
            user_login,
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
            RegisterForm,
            LoginForm,
            UpdateUserForm,
            NewDeviceForm,
            RecordForm,
            UpdateDeviceForm,
            UpdateTagForm,
            TagDeviceForm,
            NewTagForm,
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
                        "JWT token saved in Cookie (you may need to add the `token` cookie by yourself \
                            via a browser extension like Cookie-Editor, **but not here**)".to_string()
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
            .service(Files::new("/", "./public").index_file("index.html"))
            .default_service(web::route().to(notfound_404))
    })
    .bind(("0.0.0.0", 8888))?
    .run()
    .await
}
