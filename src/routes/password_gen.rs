use askama::Template;
use axum::{
    extract::Form,
    response::{Html, IntoResponse},
};
use rand::{rng, Rng};
use serde::Deserialize;

#[derive(Template)]
#[template(path = "password_gen.html")]
struct PasswordGenTemplate {
    username: String,
    generated_password: String,
    show_password: bool,
}

#[derive(Deserialize)]
pub struct PasswordGenForm {
    length: Option<usize>,
    include_uppercase: Option<String>,
    include_lowercase: Option<String>,
    include_numbers: Option<String>,
    include_symbols: Option<String>,
}

pub async fn show_password_gen(
    axum::extract::Extension(username): axum::extract::Extension<String>,
) -> impl IntoResponse {
    let template = PasswordGenTemplate {
        username,
        generated_password: String::new(),
        show_password: false,
    };
    Html(template.render().unwrap())
}

pub async fn generate_password(
    axum::extract::Extension(username): axum::extract::Extension<String>,
    Form(form): Form<PasswordGenForm>,
) -> impl IntoResponse {
    let length = form.length.unwrap_or(16).max(4).min(128);
    let include_uppercase = form.include_uppercase.is_some();
    let include_lowercase = form.include_lowercase.is_some();
    let include_numbers = form.include_numbers.is_some();
    let include_symbols = form.include_symbols.is_some();

    let mut charset = String::new();
    if include_uppercase {
        charset.push_str("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
    }
    if include_lowercase {
        charset.push_str("abcdefghijklmnopqrstuvwxyz");
    }
    if include_numbers {
        charset.push_str("0123456789");
    }
    if include_symbols {
        charset.push_str("!@#$%^&*()-_=+[]{}|;:,.<>?");
    }

    // デフォルトでは小文字と数字を含める
    if charset.is_empty() {
        charset.push_str("abcdefghijklmnopqrstuvwxyz0123456789");
    }

    let charset_bytes: Vec<u8> = charset.bytes().collect();
    let mut rng = rng();
    let password: String = (0..length)
        .map(|_| {
            let idx = rng.random_range(0..charset_bytes.len());
            charset_bytes[idx] as char
        })
        .collect();

    let template = PasswordGenTemplate {
        username,
        generated_password: password,
        show_password: true,
    };
    Html(template.render().unwrap())
}
