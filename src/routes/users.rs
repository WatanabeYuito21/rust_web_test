use askama::Template;
use axum::{extract::State, response::Html};
use crate::{AppState, db};

#[derive(Template)]
#[template(path = "users.html")]
pub struct UsersTemplate {
    pub users: Vec<UserDisplay>,
}

#[derive(Clone)]
pub struct UserDisplay {
    pub id: i32,
    pub username: String,
    pub created_at: String,
}

pub async fn list_users(State(state): State<AppState>) -> Result<Html<String>, String> {
    let users = db::list_users(&state.db)
        .await
        .map_err(|e| format!("データベースエラー: {}", e))?;

    let users_display: Vec<UserDisplay> = users
        .into_iter()
        .map(|user| UserDisplay {
            id: user.id,
            username: user.username,
            created_at: "N/A".to_string(), // TODO: created_atフィールドを追加する場合はここを修正
        })
        .collect();

    let template = UsersTemplate {
        users: users_display,
    };

    match template.render() {
        Ok(html) => Ok(Html(html)),
        Err(e) => Err(format!("テンプレートレンダリングエラー: {}", e)),
    }
}
