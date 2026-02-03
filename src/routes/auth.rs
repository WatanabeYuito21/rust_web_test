use argon2::Argon2;
use askama::Template;
use askama_web::WebTemplate;
use axum::{Form, response::Redirect, extract::State};
use password_hash::{PasswordHash, PasswordVerifier};
use serde::Deserialize;
use tower_sessions::Session;
use crate::AppState;
use crate::db;

const SESSION_USER_KEY: &str = "user";

#[derive(Template, WebTemplate)]
#[template(path = "login.html")]
pub struct LoginTemplate {
    pub error: Option<String>,
}

#[derive(Deserialize)]
pub struct LoginForm {
    username: String,
    password: String,
}

pub async fn login_page() -> LoginTemplate {
    LoginTemplate { error: None }
}

pub async fn login(
    State(state): State<AppState>,
    session: Session,
    Form(form): Form<LoginForm>,
) -> Result<Redirect, LoginTemplate> {
    // データベースからユーザーを取得
    let user = match db::get_user_by_username(&state.db, &form.username).await {
        Ok(Some(user)) => user,
        Ok(None) => {
            return Err(LoginTemplate {
                error: Some("ユーザー名またはパスワードが間違っています".into()),
            });
        }
        Err(_) => {
            return Err(LoginTemplate {
                error: Some("データベースエラーが発生しました".into()),
            });
        }
    };

    // パスワードを検証
    let is_valid = PasswordHash::new(&user.password_hash)
        .ok()
        .map(|hash| {
            Argon2::default()
                .verify_password(form.password.as_bytes(), &hash)
                .is_ok()
        })
        .unwrap_or(false);

    if is_valid {
        // セッションに保存
        if let Err(e) = session.insert(SESSION_USER_KEY, &form.username).await {
            eprintln!("セッション保存エラー: {:?}", e);
            return Err(LoginTemplate {
                error: Some("セッションエラーが発生しました".into()),
            });
        }

        eprintln!("ログイン成功: {}", form.username);

        // 監査ログに記録
        let _ = db::create_audit_log(
            &state.db,
            Some(user.id),
            &form.username,
            "login",
            None,
            Some("User logged in successfully"),
            None,
            None,
        ).await;

        Ok(Redirect::to("/"))
    } else {
        eprintln!("ログイン失敗: {} (パスワード不一致)", form.username);
        // ログイン失敗も記録
        let _ = db::create_audit_log(
            &state.db,
            None,
            &form.username,
            "login_failed",
            None,
            Some("Failed login attempt"),
            None,
            None,
        ).await;

        Err(LoginTemplate {
            error: Some("ユーザー名またはパスワードが間違っています".into()),
        })
    }
}

pub async fn logout(State(state): State<AppState>, session: Session) -> Redirect {
    // ログアウト前にユーザー名を取得
    if let Some(username) = get_username(&session).await {
        if let Ok(Some(user)) = db::get_user_by_username(&state.db, &username).await {
            // 監査ログに記録
            let _ = db::create_audit_log(
                &state.db,
                Some(user.id),
                &username,
                "logout",
                None,
                Some("User logged out"),
                None,
                None,
            ).await;
        }
    }

    session.delete().await.unwrap();
    Redirect::to("/login")
}

pub async fn is_authenticated(session: &Session) -> bool {
    session
        .get::<String>(SESSION_USER_KEY)
        .await
        .unwrap_or(None)
        .is_some()
}

pub async fn get_username(session: &Session) -> Option<String> {
    session
        .get::<String>(SESSION_USER_KEY)
        .await
        .unwrap_or(None)
}

pub async fn get_current_user(session: &Session, db: &sqlx::MySqlPool) -> Option<crate::db::User> {
    let username = get_username(session).await?;
    db::get_user_by_username(db, &username).await.ok().flatten()
}
