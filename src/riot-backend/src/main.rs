mod db;
mod errors;
mod handlers;
mod jwt_utils;
mod middlewares;
mod models;
mod mqtt_instance;
mod config;

use config::Config;
use db::*;
use handlers::*;
use jwt_utils::*;
use models::*;
use std::thread;
use utoipa_swagger_ui::{SwaggerUi, Url};

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

#[derive(Debug, Clone)]
pub struct AppState {
    pub env: Config,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=debug");

    // Generate OpenAPI docs

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

    #[derive(OpenApi)]
    #[openapi(
        paths(device_info),
        components(schemas(User), schemas(Device), schemas(Site))
    )]
    struct ApiDoc;

    let openapi = ApiDoc::openapi();

    let config = Config::init();
    let app_state: AppState = AppState {
        env: config.clone(),
    };

    // Register services (API endpoints and user interfaces)
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .wrap(Logger::default())
            .service(web::redirect("/swagger", "/swagger/"))
            .service(
                // Must be register here (the top instead of the bottom of service chain)
                SwaggerUi::new("/swagger/{_:.*}").url("/api-docs/openapi.json", openapi.clone()),
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
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
