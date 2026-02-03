use askama::Template;
use askama_web::WebTemplate;
use axum::response::sse::{Event, Sse};
use axum::response::{IntoResponse, Redirect};
use axum::extract::State;
use futures::stream;
use serde::Serialize;
use std::time::Duration;
use sysinfo::System;
use tokio_stream::StreamExt as _;
use tower_sessions::Session;
use crate::{AppState, db};
use crate::routes::auth;

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

pub async fn index(
    State(state): State<AppState>,
    session: Session,
) -> Result<SysInfoTemplate, Redirect> {
    // 現在のユーザーを取得
    let current_user = match auth::get_current_user(&session, &state.db).await {
        Some(user) => user,
        None => return Err(Redirect::to("/login")),
    };

    // User以上の権限をチェック
    if !current_user.role().can_access_sysinfo() {
        // 権限エラーを監査ログに記録
        let _ = db::create_audit_log(
            &state.db,
            Some(current_user.id),
            &current_user.username,
            "access_denied",
            Some("/sysinfo"),
            Some("Attempted to access system info without permission"),
            None,
            None,
        ).await;

        return Err(Redirect::to("/"));
    }

    let mut sys = System::new_all();
    sys.refresh_all();

    let total_memory = sys.total_memory() as f64;
    let used_memory = sys.used_memory() as f64;
    let total_gb = total_memory / 1024.0 / 1024.0 / 1024.0;
    let used_gb = used_memory / 1024.0 / 1024.0 / 1024.0;
    let percent = used_memory / total_memory * 100.0;

    Ok(SysInfoTemplate {
        hostname: System::host_name().unwrap_or_else(|| "Unknown".to_string()),
        os: System::name().unwrap_or_else(|| "Unknown".to_string()),
        kernel: System::kernel_version().unwrap_or_else(|| "Unknown".to_string()),
        cpu_count: sys.cpus().len(),
        memory_display: format!("{:.1} / {:.1} GB ({:.0}%)", used_gb, total_gb, percent),
    })
}

pub async fn live(
    State(state): State<AppState>,
    session: Session,
) -> Result<impl IntoResponse, Redirect> {
    // 現在のユーザーを取得
    let current_user = match auth::get_current_user(&session, &state.db).await {
        Some(user) => user,
        None => return Err(Redirect::to("/login")),
    };

    // User以上の権限をチェック
    if !current_user.role().can_access_sysinfo() {
        // 権限エラーを監査ログに記録
        let _ = db::create_audit_log(
            &state.db,
            Some(current_user.id),
            &current_user.username,
            "access_denied",
            Some("/sysinfo/live"),
            Some("Attempted to access system info live stream without permission"),
            None,
            None,
        ).await;

        return Err(Redirect::to("/"));
    }

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

    Ok(Sse::new(stream).keep_alive(
        axum::response::sse::KeepAlive::new()
            .interval(Duration::from_secs(30))
            .text("keep-alive-text"),
    ))
}
