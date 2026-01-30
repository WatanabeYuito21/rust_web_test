use argon2::{Argon2, PasswordHasher};
use argon2::password_hash::{rand_core::OsRng, SaltString, PasswordHash, PasswordVerifier};
use askama::Template;
use axum::{
    extract::{Form, State},
    response::{Html, IntoResponse},
};
use serde::Deserialize;
use tower_sessions::Session;
use crate::{AppState, db};

const SESSION_USER_KEY: &str = "user";

#[derive(Template)]
#[template(path = "profile.html")]
struct ProfileTemplate {
    username: String,
    email: String,
    full_name: String,
    success_message: String,
    error_message: String,
    show_success: bool,
    show_error: bool,
}

#[derive(Deserialize)]
pub struct UpdateProfileForm {
    email: String,
    full_name: String,
}

#[derive(Deserialize)]
pub struct ChangePasswordForm {
    current_password: String,
    new_password: String,
    confirm_password: String,
}

pub async fn show_profile(
    State(state): State<AppState>,
    _session: Session,
    axum::extract::Extension(username): axum::extract::Extension<String>,
) -> impl IntoResponse {
    // データベースからユーザー情報を取得
    let user = match db::get_user_by_username(&state.db, &username).await {
        Ok(Some(user)) => user,
        Ok(None) => {
            return Html(
                ProfileTemplate {
                    username: username.clone(),
                    email: String::new(),
                    full_name: String::new(),
                    success_message: String::new(),
                    error_message: "ユーザーが見つかりません".to_string(),
                    show_success: false,
                    show_error: true,
                }
                .render()
                .unwrap(),
            );
        }
        Err(_) => {
            return Html(
                ProfileTemplate {
                    username: username.clone(),
                    email: String::new(),
                    full_name: String::new(),
                    success_message: String::new(),
                    error_message: "データベースエラーが発生しました".to_string(),
                    show_success: false,
                    show_error: true,
                }
                .render()
                .unwrap(),
            );
        }
    };

    let template = ProfileTemplate {
        username: user.username.clone(),
        email: user.email.clone().unwrap_or_default(),
        full_name: user.full_name.clone().unwrap_or_default(),
        success_message: String::new(),
        error_message: String::new(),
        show_success: false,
        show_error: false,
    };

    Html(template.render().unwrap())
}

pub async fn update_profile(
    State(state): State<AppState>,
    axum::extract::Extension(username): axum::extract::Extension<String>,
    Form(form): Form<UpdateProfileForm>,
) -> impl IntoResponse {
    // ユーザーIDを取得
    let user = match db::get_user_by_username(&state.db, &username).await {
        Ok(Some(user)) => user,
        Ok(None) => {
            return Html(
                ProfileTemplate {
                    username: username.clone(),
                    email: String::new(),
                    full_name: String::new(),
                    success_message: String::new(),
                    error_message: "ユーザーが見つかりません".to_string(),
                    show_success: false,
                    show_error: true,
                }
                .render()
                .unwrap(),
            );
        }
        Err(_) => {
            return Html(
                ProfileTemplate {
                    username: username.clone(),
                    email: String::new(),
                    full_name: String::new(),
                    success_message: String::new(),
                    error_message: "データベースエラーが発生しました".to_string(),
                    show_success: false,
                    show_error: true,
                }
                .render()
                .unwrap(),
            );
        }
    };

    // プロフィールを更新
    let email = if form.email.is_empty() {
        None
    } else {
        Some(form.email.as_str())
    };
    let full_name = if form.full_name.is_empty() {
        None
    } else {
        Some(form.full_name.as_str())
    };

    match db::update_user_profile(&state.db, user.id, email, full_name).await {
        Ok(_) => {
            let template = ProfileTemplate {
                username: user.username.clone(),
                email: email.unwrap_or("").to_string(),
                full_name: full_name.unwrap_or("").to_string(),
                success_message: "プロフィールを更新しました".to_string(),
                error_message: String::new(),
                show_success: true,
                show_error: false,
            };
            Html(template.render().unwrap())
        }
        Err(_) => {
            let template = ProfileTemplate {
                username: user.username.clone(),
                email: user.email.clone().unwrap_or_default(),
                full_name: user.full_name.clone().unwrap_or_default(),
                success_message: String::new(),
                error_message: "プロフィールの更新に失敗しました".to_string(),
                show_success: false,
                show_error: true,
            };
            Html(template.render().unwrap())
        }
    }
}

pub async fn change_password(
    State(state): State<AppState>,
    axum::extract::Extension(username): axum::extract::Extension<String>,
    Form(form): Form<ChangePasswordForm>,
) -> impl IntoResponse {
    // ユーザー情報を取得
    let user = match db::get_user_by_username(&state.db, &username).await {
        Ok(Some(user)) => user,
        Ok(None) => {
            return Html(
                ProfileTemplate {
                    username: username.clone(),
                    email: String::new(),
                    full_name: String::new(),
                    success_message: String::new(),
                    error_message: "ユーザーが見つかりません".to_string(),
                    show_success: false,
                    show_error: true,
                }
                .render()
                .unwrap(),
            );
        }
        Err(_) => {
            return Html(
                ProfileTemplate {
                    username: username.clone(),
                    email: String::new(),
                    full_name: String::new(),
                    success_message: String::new(),
                    error_message: "データベースエラーが発生しました".to_string(),
                    show_success: false,
                    show_error: true,
                }
                .render()
                .unwrap(),
            );
        }
    };

    // 現在のパスワードを確認
    let is_valid = PasswordHash::new(&user.password_hash)
        .ok()
        .map(|hash| {
            Argon2::default()
                .verify_password(form.current_password.as_bytes(), &hash)
                .is_ok()
        })
        .unwrap_or(false);

    if !is_valid {
        return Html(
            ProfileTemplate {
                username: user.username.clone(),
                email: user.email.clone().unwrap_or_default(),
                full_name: user.full_name.clone().unwrap_or_default(),
                success_message: String::new(),
                error_message: "現在のパスワードが間違っています".to_string(),
                show_success: false,
                show_error: true,
            }
            .render()
            .unwrap(),
        );
    }

    // 新しいパスワードの確認
    if form.new_password != form.confirm_password {
        return Html(
            ProfileTemplate {
                username: user.username.clone(),
                email: user.email.clone().unwrap_or_default(),
                full_name: user.full_name.clone().unwrap_or_default(),
                success_message: String::new(),
                error_message: "新しいパスワードが一致しません".to_string(),
                show_success: false,
                show_error: true,
            }
            .render()
            .unwrap(),
        );
    }

    // パスワードの長さチェック
    if form.new_password.len() < 8 {
        return Html(
            ProfileTemplate {
                username: user.username.clone(),
                email: user.email.clone().unwrap_or_default(),
                full_name: user.full_name.clone().unwrap_or_default(),
                success_message: String::new(),
                error_message: "新しいパスワードは8文字以上である必要があります".to_string(),
                show_success: false,
                show_error: true,
            }
            .render()
            .unwrap(),
        );
    }

    // 新しいパスワードをハッシュ化
    let salt = SaltString::generate(&mut OsRng);
    let password_hash = match Argon2::default().hash_password(form.new_password.as_bytes(), &salt) {
        Ok(hash) => hash.to_string(),
        Err(_) => {
            return Html(
                ProfileTemplate {
                    username: user.username.clone(),
                    email: user.email.clone().unwrap_or_default(),
                    full_name: user.full_name.clone().unwrap_or_default(),
                    success_message: String::new(),
                    error_message: "パスワードのハッシュ化に失敗しました".to_string(),
                    show_success: false,
                    show_error: true,
                }
                .render()
                .unwrap(),
            );
        }
    };

    // パスワードを更新
    match db::update_user_password(&state.db, user.id, &password_hash).await {
        Ok(_) => {
            let template = ProfileTemplate {
                username: user.username.clone(),
                email: user.email.clone().unwrap_or_default(),
                full_name: user.full_name.clone().unwrap_or_default(),
                success_message: "パスワードを変更しました".to_string(),
                error_message: String::new(),
                show_success: true,
                show_error: false,
            };
            Html(template.render().unwrap())
        }
        Err(_) => {
            let template = ProfileTemplate {
                username: user.username.clone(),
                email: user.email.clone().unwrap_or_default(),
                full_name: user.full_name.clone().unwrap_or_default(),
                success_message: String::new(),
                error_message: "パスワードの変更に失敗しました".to_string(),
                show_success: false,
                show_error: true,
            };
            Html(template.render().unwrap())
        }
    }
}
