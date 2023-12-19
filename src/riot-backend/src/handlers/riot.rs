use std::fs;

use actix_web::get;
use actix_web::HttpResponse;
use actix_web::Responder;
use once_cell::sync::Lazy;
use serde::Deserialize;
use serde::Serialize;
use sysinfo::{CpuExt, System, SystemExt};
use tokio::sync::RwLock;
use utoipa::ToSchema;

pub static SYSINFO: Lazy<RwLock<System>> = Lazy::new(|| {
    let mut sysinfo = System::new_all();
    sysinfo.refresh_all();
    RwLock::new(sysinfo)
});

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct ServerStatistic<'a> {
    sys_name: &'a str,
    cpu_name: &'a str,
    /// Unit: Seconds
    uptime: u64,
    /// Unit: Percentage
    cpu_usage: f32,
    /// Unit: Bytes
    mem_total: u64,
    /// Unit: Bytes  
    /// 
    /// Note: is not always equal to "free memory":
    /// `MemTotal = MemUsed + MemFree + Buffers + Cached + SReclaimable`
    mem_available: u64,
    /// Unit: Bytes
    swap_total: u64,
    /// Unit: Bytes
    swap_free: u64,
    /// Average load within 1/5/15 minute.
    /// Unit: Percentage
    load_avg_1_5_15: [f64; 3],
}

// ROUTES
// health checker
#[utoipa::path(
    get,
    context_path = "/api",
    tag = "RIoT",
    path = "/healthchecker",
    responses(
        (status = 200, description = "Server statistics", body = ServerStatistic),
    ),
    params(),
)]
#[get("/healthchecker")]
/// Return statistics of the server
pub(crate) async fn healthchecker() -> impl Responder {
    {
        let mut sysinfo = SYSINFO.write().await;
        sysinfo.refresh_cpu();
        sysinfo.refresh_memory();
        sysinfo.refresh_disks_list();
        sysinfo.refresh_networks();
    }
    {
        let sysinfo = SYSINFO.read().await;
        HttpResponse::Ok().json(ServerStatistic {
            sys_name: sysinfo.name().as_deref().unwrap_or("Unknown"),
            uptime: sysinfo.uptime(),
            cpu_name: sysinfo.global_cpu_info().name(),
            cpu_usage: sysinfo.global_cpu_info().cpu_usage(),
            mem_total: sysinfo.total_memory(),
            mem_available: sysinfo.available_memory(),
            swap_total: sysinfo.total_swap(),
            swap_free: sysinfo.free_swap(),
            load_avg_1_5_15: [
                sysinfo.load_average().one,
                sysinfo.load_average().five,
                sysinfo.load_average().fifteen,
            ],
        })
    }
}

// HTTP Codes
pub(crate) async fn notfound_404() -> HttpResponse {
    let content =
        fs::read_to_string("./public/404.html").unwrap_or_else(|_| "Page not found".to_string());
    HttpResponse::NotFound().body(content)
}
