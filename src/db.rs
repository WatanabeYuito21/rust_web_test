use sqlx::{MySqlPool, FromRow};
use serde::Serialize;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub enum Role {
    Admin,
    User,
    Viewer,
}

impl Role {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "admin" => Role::Admin,
            "user" => Role::User,
            "viewer" => Role::Viewer,
            _ => Role::User, // Default to User for unknown values
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            Role::Admin => "admin",
            Role::User => "user",
            Role::Viewer => "viewer",
        }
    }

    pub fn can_access_sysinfo(&self) -> bool {
        matches!(self, Role::Admin | Role::User)
    }

    pub fn can_access_crypto(&self) -> bool {
        matches!(self, Role::Admin | Role::User)
    }

    pub fn can_access_users(&self) -> bool {
        matches!(self, Role::Admin)
    }

    pub fn can_access_audit(&self) -> bool {
        matches!(self, Role::Admin)
    }
}

#[derive(Debug, Clone, FromRow, Serialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password_hash: String,
    #[sqlx(rename = "role")]
    role_str: String,
}

impl User {
    pub fn role(&self) -> Role {
        Role::from_str(&self.role_str)
    }
}

#[derive(Debug, Clone, FromRow, Serialize)]
pub struct AuditLog {
    pub id: i32,
    pub user_id: Option<i32>,
    pub username: String,
    pub action: String,
    pub resource: Option<String>,
    pub details: Option<String>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub created_at: DateTime<Utc>,
}

pub async fn create_pool(database_url: &str) -> Result<MySqlPool, sqlx::Error> {
    MySqlPool::connect(database_url).await
}

pub async fn get_user_by_username(pool: &MySqlPool, username: &str) -> Result<Option<User>, sqlx::Error> {
    let user = sqlx::query_as::<_, User>(
        "SELECT id, username, password_hash, role FROM users WHERE username = ?"
    )
    .bind(username)
    .fetch_optional(pool)
    .await?;

    Ok(user)
}

pub async fn create_user(pool: &MySqlPool, username: &str, password_hash: &str) -> Result<User, sqlx::Error> {
    let result = sqlx::query(
        "INSERT INTO users (username, password_hash, role) VALUES (?, ?, 'user')"
    )
    .bind(username)
    .bind(password_hash)
    .execute(pool)
    .await?;

    let user_id = result.last_insert_id() as i32;

    let user = User {
        id: user_id,
        username: username.to_string(),
        password_hash: password_hash.to_string(),
        role_str: "user".to_string(),
    };

    Ok(user)
}

pub async fn list_users(pool: &MySqlPool) -> Result<Vec<User>, sqlx::Error> {
    let users = sqlx::query_as::<_, User>(
        "SELECT id, username, password_hash, role FROM users ORDER BY id"
    )
    .fetch_all(pool)
    .await?;

    Ok(users)
}

pub async fn create_audit_log(
    pool: &MySqlPool,
    user_id: Option<i32>,
    username: &str,
    action: &str,
    resource: Option<&str>,
    details: Option<&str>,
    ip_address: Option<&str>,
    user_agent: Option<&str>,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT INTO audit_logs (user_id, username, action, resource, details, ip_address, user_agent) VALUES (?, ?, ?, ?, ?, ?, ?)"
    )
    .bind(user_id)
    .bind(username)
    .bind(action)
    .bind(resource)
    .bind(details)
    .bind(ip_address)
    .bind(user_agent)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn list_audit_logs(pool: &MySqlPool, limit: i64) -> Result<Vec<AuditLog>, sqlx::Error> {
    let logs = sqlx::query_as::<_, AuditLog>(
        "SELECT id, user_id, username, action, resource, details, ip_address, user_agent, created_at FROM audit_logs ORDER BY created_at DESC LIMIT ?"
    )
    .bind(limit)
    .fetch_all(pool)
    .await?;

    Ok(logs)
}

pub async fn list_audit_logs_by_user(pool: &MySqlPool, username: &str, limit: i64) -> Result<Vec<AuditLog>, sqlx::Error> {
    let logs = sqlx::query_as::<_, AuditLog>(
        "SELECT id, user_id, username, action, resource, details, ip_address, user_agent, created_at FROM audit_logs WHERE username = ? ORDER BY created_at DESC LIMIT ?"
    )
    .bind(username)
    .bind(limit)
    .fetch_all(pool)
    .await?;

    Ok(logs)
}
