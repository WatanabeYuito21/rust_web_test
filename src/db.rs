use sqlx::{MySqlPool, FromRow};
use serde::Serialize;

#[derive(Debug, Clone, FromRow, Serialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password_hash: String,
}

pub async fn create_pool(database_url: &str) -> Result<MySqlPool, sqlx::Error> {
    MySqlPool::connect(database_url).await
}

pub async fn get_user_by_username(pool: &MySqlPool, username: &str) -> Result<Option<User>, sqlx::Error> {
    let user = sqlx::query_as::<_, User>(
        "SELECT id, username, password_hash FROM users WHERE username = ?"
    )
    .bind(username)
    .fetch_optional(pool)
    .await?;

    Ok(user)
}

pub async fn create_user(pool: &MySqlPool, username: &str, password_hash: &str) -> Result<User, sqlx::Error> {
    let result = sqlx::query(
        "INSERT INTO users (username, password_hash) VALUES (?, ?)"
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
    };

    Ok(user)
}

pub async fn list_users(pool: &MySqlPool) -> Result<Vec<User>, sqlx::Error> {
    let users = sqlx::query_as::<_, User>(
        "SELECT id, username, password_hash FROM users ORDER BY id"
    )
    .fetch_all(pool)
    .await?;

    Ok(users)
}
