use askama::Template;
use askama_web::WebTemplate;
use axum::response::sse::{Event, Sse};
use axum::response::IntoResponse;
use futures::stream;
use serde::Serialize;
use std::time::Duration;
use sysinfo::System;
use tokio_stream::StreamExt as _;

#[derive(Template, WebTemplate)]
#[template(path = "sysinfo.html")]
pub struct SysInfoTemplate {
    pub hostname: String,
    pub os: String,
    pub kernel: String,
    pub cpu_count: usize,
    pub memory_display: String,
}

#[derive(Serialize)]
pub struct SysInfoData {
    pub hostname: String,
    pub os: String,
    pub kernel: String,
    pub cpu_count: usize,
    pub total_memory_gb: f64,
    pub used_memory_gb: f64,
    pub memory_percent: f64,
}

pub async fn index() -> SysInfoTemplate {
    let mut sys = System::new_all();
    sys.refresh_all();

    let total_memory = sys.total_memory() as f64;
    let used_memory = sys.used_memory() as f64;
    let total_gb = total_memory / 1024.0 / 1024.0 / 1024.0;
    let used_gb = used_memory / 1024.0 / 1024.0 / 1024.0;
    let percent = used_memory / total_memory * 100.0;

    SysInfoTemplate {
        hostname: System::host_name().unwrap_or_else(|| "Unknown".to_string()),
        os: System::name().unwrap_or_else(|| "Unknown".to_string()),
        kernel: System::kernel_version().unwrap_or_else(|| "Unknown".to_string()),
        cpu_count: sys.cpus().len(),
        memory_display: format!("{:.1} / {:.1} GB ({:.0}%)", used_gb, total_gb, percent),
    }
}

pub async fn live() -> impl IntoResponse {
    let stream = stream::repeat_with(|| {
        let mut sys = System::new_all();
        sys.refresh_all();

        let total_memory = sys.total_memory() as f64;
        let used_memory = sys.used_memory() as f64;
        let total_gb = total_memory / 1024.0 / 1024.0 / 1024.0;
        let used_gb = used_memory / 1024.0 / 1024.0 / 1024.0;
        let percent = used_memory / total_memory * 100.0;

        SysInfoData {
            hostname: System::host_name().unwrap_or_else(|| "Unknown".to_string()),
            os: System::name().unwrap_or_else(|| "Unknown".to_string()),
            kernel: System::kernel_version().unwrap_or_else(|| "Unknown".to_string()),
            cpu_count: sys.cpus().len(),
            total_memory_gb: total_gb,
            used_memory_gb: used_gb,
            memory_percent: percent,
        }
    })
    .throttle(Duration::from_secs(1))
    .map(|data| {
        Event::default().json_data(data)
    });

    Sse::new(stream).keep_alive(
        axum::response::sse::KeepAlive::new()
            .interval(Duration::from_secs(30))
            .text("keep-alive-text"),
    )
}
