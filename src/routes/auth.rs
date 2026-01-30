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
        // セッションにユーザー名を保存
        if let Err(e) = session.insert(SESSION_USER_KEY, &form.username).await {
            eprintln!("セッション保存エラー: {:?}", e);
            return Err(LoginTemplate {
                error: Some("セッションの保存に失敗しました".into()),
            });
        }

        // セッションを確実に保存
        if let Err(e) = session.save().await {
            eprintln!("セッション永続化エラー: {:?}", e);
            return Err(LoginTemplate {
                error: Some("セッションの永続化に失敗しました".into()),
            });
        }

        Ok(Redirect::to("/"))
    } else {
        Err(LoginTemplate {
            error: Some("ユーザー名またはパスワードが間違っています".into()),
        })
    }
}

pub async fn logout(session: Session) -> Redirect {
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
