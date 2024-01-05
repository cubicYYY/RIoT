use actix_web::get;
use actix_web::HttpResponse;
use actix_web::Responder;
use once_cell::sync::Lazy;
use serde::Deserialize;
use serde::Serialize;
use std::collections::VecDeque;
use std::fs;
use std::time::Duration;
use sysinfo::{CpuExt, System, SystemExt};
use tokio::sync::RwLock;
use utoipa::ToSchema;

use crate::db::DBClient;

pub static SYSINFO: Lazy<RwLock<System>> = Lazy::new(|| {
    let mut sysinfo = System::new_all();
    sysinfo.refresh_all();
    RwLock::new(sysinfo)
});
pub static SYSINFO_CACHE: Lazy<SysinfoCache> = Lazy::new(SysinfoCache::new);

#[derive(Copy, Clone, Serialize, Deserialize, ToSchema, Debug)]
pub struct CachedSysinfo {
    pub cpu_usage: f32,
    pub record_count: u32,
    pub device_count: i64,
    pub device_online: i64,
}
impl Default for CachedSysinfo {
    fn default() -> Self {
        CachedSysinfo {
            cpu_usage: 0f32,
            record_count: 0,
            device_count: 0,
            device_online: 0,
        }
    }
}
pub struct SysinfoCache {
    pub buffer: RwLock<CachedSysinfo>,
    pub cache: RwLock<VecDeque<CachedSysinfo>>,
}
impl SysinfoCache {
    /// Unit: seconds
    const UPDATE_INTERVAL: usize = 12;
    const MAX_CAPACITY: usize = 30 * 60 / Self::UPDATE_INTERVAL;
    pub fn new() -> Self {
        SysinfoCache {
            buffer: RwLock::new(CachedSysinfo::default()),
            cache: RwLock::new(VecDeque::with_capacity(Self::MAX_CAPACITY)),
        }
    }
    pub async fn flush(&self, db: &DBClient) {
        {
            let mut buf = self.buffer.write().await;
            buf.cpu_usage = SYSINFO.read().await.global_cpu_info().cpu_usage();
            buf.device_count = db.get_device_cnt().await.unwrap_or(0);
            buf.device_online = db.get_online_device_cnt().await.unwrap_or(0);
        }
        {
            let buf = self.buffer.read().await;
            {
                let mut deque = self.cache.write().await;
                if deque.len() >= Self::MAX_CAPACITY {
                    deque.pop_front();
                }
                deque.push_back(*buf);
            }
        }
        // Clear buffer
        *self.buffer.write().await = CachedSysinfo::default();
    }
    pub async fn new_daemon(&self) {
        let db = DBClient::new(&DBClient::get_database_url());
        loop {
            tokio::time::sleep(Duration::from_secs(Self::UPDATE_INTERVAL as u64)).await;
            self.flush(&db).await;
        }
    }
}

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct ServerStatistic<'a> {
    sys_name: &'a str,
    cpu_name: &'a str,
    /// Physical, sum of all cores
    cpu_core_count: usize,
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
    #[schema(value_type=Vec<CachedSysinfo>)]
    last_30min: VecDeque<CachedSysinfo>,
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
///
/// TODO: Incremental info
pub(crate) async fn healthchecker() -> impl Responder {
    {
        let mut sysinfo = SYSINFO.write().await;
        sysinfo.refresh_cpu();
        sysinfo.refresh_memory();
        sysinfo.refresh_disks_list();
    }
    {
        let sysinfo = SYSINFO.read().await;
        HttpResponse::Ok().json(ServerStatistic {
            sys_name: sysinfo.name().as_deref().unwrap_or("Unknown"),
            uptime: sysinfo.uptime(),
            cpu_name: sysinfo.global_cpu_info().name(),
            cpu_core_count: sysinfo.physical_core_count().unwrap_or(0),
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
            last_30min: SYSINFO_CACHE.cache.read().await.clone(),
        })
    }
}

/// For frontend router
pub(crate) async fn index() -> HttpResponse {
    let content = fs::read_to_string("./dist/index.html")
        .unwrap_or_else(|_| "index.html not found".to_string());
    HttpResponse::Ok().body(content)
}
// pub(crate) async fn notfound_404() -> HttpResponse {
//     let content =
//         fs::read_to_string("./public/404.html").unwrap_or_else(|_| "Page not found".to_string());
//     HttpResponse::NotFound().body(content)
// }
