use askama::Template;
use axum::{extract::State, response::{Html, Redirect, IntoResponse}};
use tower_sessions::Session;
use crate::{AppState, db};
use crate::routes::auth;

#[derive(Template)]
#[template(path = "users.html")]
pub struct UsersTemplate {
    pub users: Vec<UserDisplay>,
}

#[derive(Clone)]
pub struct UserDisplay {
    pub id: i32,
    pub username: String,
    pub role: String,
    pub created_at: String,
}

pub async fn list_users(
    State(state): State<AppState>,
    session: Session,
) -> Result<impl IntoResponse, impl IntoResponse> {
    // 現在のユーザーを取得
    let current_user = match auth::get_current_user(&session, &state.db).await {
        Some(user) => user,
        None => return Err(Redirect::to("/login")),
    };

    // Admin権限をチェック
    if !current_user.role().can_access_users() {
        // 権限エラーを監査ログに記録
        let _ = db::create_audit_log(
            &state.db,
            Some(current_user.id),
            &current_user.username,
            "access_denied",
            Some("/users"),
            Some("Attempted to access users page without permission"),
            None,
            None,
        ).await;

        return Err(Redirect::to("/"));
    }

    let users = db::list_users(&state.db)
        .await
        .map_err(|_| Redirect::to("/"))?;

    let users_display: Vec<UserDisplay> = users
        .into_iter()
        .map(|user| UserDisplay {
            id: user.id,
            username: user.username.clone(),
            role: user.role().as_str().to_string(),
            created_at: "N/A".to_string(),
        })
        .collect();

    let template = UsersTemplate {
        users: users_display,
    };

    match template.render() {
        Ok(html) => Ok(Html(html)),
        Err(_) => Err(Redirect::to("/")),
    }
}
