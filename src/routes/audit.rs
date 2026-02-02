use askama::Template;
use axum::{extract::State, response::Html};
use chrono::{DateTime, Local};
use crate::{AppState, db};

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

pub async fn list_audit_logs(State(state): State<AppState>) -> Result<Html<String>, String> {
    let logs = db::list_audit_logs(&state.db, 100)
        .await
        .map_err(|e| format!("データベースエラー: {}", e))?;

    let logs_display: Vec<AuditLogDisplay> = logs
        .into_iter()
        .map(|log| log.into())
        .collect();

    let template = AuditLogsTemplate {
        logs: logs_display,
    };

    match template.render() {
        Ok(html) => Ok(Html(html)),
        Err(e) => Err(format!("テンプレートレンダリングエラー: {}", e)),
    }
}
