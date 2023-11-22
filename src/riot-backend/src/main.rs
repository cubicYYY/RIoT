mod models;
mod routes;
mod mqtt_instance;

use std::thread;

use models::*;
use routes::*;

use actix_files::Files;
use actix_web::{web::{self}, App, HttpServer};

use rumqttc::Packet;
use rumqttc::Event::Incoming;
use utoipa::OpenApi;

use crate::mqtt_instance::mqtt_instancer::MqttDaemon;
#[derive(OpenApi)]
#[openapi(paths(device_info), components(schemas(User)))]
#[openapi(paths(device_info), components(schemas(Device)))]
#[openapi(paths(device_info), components(schemas(Site)))]
struct ApiDoc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Generate OpenAPI docs
    println!("{}", ApiDoc::openapi().to_pretty_json().unwrap());
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
    // Register services (API endpoints and user interfaces)
    HttpServer::new(|| {
        App::new()
            .service(
                web::scope("/api")
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
                    .service(mqtt_unsub)
            )
            .service(Files::new("/", "./public").index_file("index.html"))
            .default_service(web::route().to(notfound_404))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
