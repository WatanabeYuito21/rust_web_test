use askama::Template;
use axum::{extract::State, response::{Html, Redirect, IntoResponse}};
use chrono::{DateTime, Local};
use tower_sessions::Session;
use crate::{AppState, db};
use crate::routes::auth;

#[derive(Template)]
#[template(path = "audit.html")]
pub struct AuditLogsTemplate {
    pub logs: Vec<AuditLogDisplay>,
}

#[derive(Clone)]
pub struct AuditLogDisplay {
    pub id: i32,
    pub username: String,
    pub action: String,
    pub resource: String,
    pub details: String,
    pub ip_address: String,
    pub created_at: String,
}

impl From<db::AuditLog> for AuditLogDisplay {
    fn from(log: db::AuditLog) -> Self {
        let local_time: DateTime<Local> = DateTime::from(log.created_at);
        Self {
            id: log.id,
            username: log.username,
            action: log.action,
            resource: log.resource.unwrap_or_else(|| "-".to_string()),
            details: log.details.unwrap_or_else(|| "-".to_string()),
            ip_address: log.ip_address.unwrap_or_else(|| "-".to_string()),
            created_at: local_time.format("%Y-%m-%d %H:%M:%S").to_string(),
        }
    }
}

pub async fn list_audit_logs(
    State(state): State<AppState>,
    session: Session,
) -> Result<impl IntoResponse, impl IntoResponse> {
    // 現在のユーザーを取得
    let current_user = match auth::get_current_user(&session, &state.db).await {
        Some(user) => user,
        None => return Err(Redirect::to("/login")),
    };

    // Admin権限をチェック
    if !current_user.role().can_access_audit() {
        // 権限エラーを監査ログに記録
        let _ = db::create_audit_log(
            &state.db,
            Some(current_user.id),
            &current_user.username,
            "access_denied",
            Some("/audit"),
            Some("Attempted to access audit logs without permission"),
            None,
            None,
        ).await;

        return Err(Redirect::to("/"));
    }

    let logs = db::list_audit_logs(&state.db, 100)
        .await
        .map_err(|_| Redirect::to("/"))?;

    let logs_display: Vec<AuditLogDisplay> = logs
        .into_iter()
        .map(|log| log.into())
        .collect();

    let template = AuditLogsTemplate {
        logs: logs_display,
    };

    match template.render() {
        Ok(html) => Ok(Html(html)),
        Err(_) => Err(Redirect::to("/")),
    }
}
