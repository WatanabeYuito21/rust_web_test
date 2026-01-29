use askama::Template;
use askama_web::WebTemplate;
use sysinfo::System;

#[derive(Template, WebTemplate)]
#[template(path = "sysinfo.html")]
pub struct SysInfoTemplate {
    pub hostname: String,
    pub os: String,
    pub kernel: String,
    pub cpu_count: usize,
    pub memory_display: String,
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
