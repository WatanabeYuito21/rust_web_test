use argon2::Argon2;
use askama::Template;
use askama_web::WebTemplate;
use axum::{Form, response::Redirect};
use password_hash::{PasswordHash, PasswordVerifier};
use serde::Deserialize;
use tower_sessions::Session;

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
    session: Session,
    Form(form): Form<LoginForm>,
) -> Result<Redirect, LoginTemplate> {
    let username = std::env::var("ADMIN_USERNAME").unwrap_or_else(|_| "admin".into());
    let password_hash = std::env::var("ADMIN_PASSWORD_HASH").unwrap_or_default();

    let is_valid = form.username == username
        && PasswordHash::new(&password_hash)
            .ok()
            .map(|hash| {
                Argon2::default()
                    .verify_password(form.password.as_bytes(), &hash)
                    .is_ok()
            })
            .unwrap_or(false);

    if is_valid {
        session
            .insert(SESSION_USER_KEY, &form.username)
            .await
            .unwrap();
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
