use askama::Template;
use axum::{
    extract::Form,
    response::{Html, IntoResponse},
};
use base64::{Engine as _, engine::general_purpose};
use serde::Deserialize;
use sha2::{Sha256, Sha512, Digest};

#[derive(Template)]
#[template(path = "text_tools.html")]
struct TextToolsTemplate {
    username: String,
    result: String,
    error: String,
    show_result: bool,
    show_error: bool,
}

#[derive(Deserialize)]
pub struct TextToolsForm {
    operation: String,
    input_text: String,
}

pub async fn show_text_tools(
    axum::extract::Extension(username): axum::extract::Extension<String>,
) -> impl IntoResponse {
    let template = TextToolsTemplate {
        username,
        result: String::new(),
        error: String::new(),
        show_result: false,
        show_error: false,
    };
    Html(template.render().unwrap())
}

pub async fn process_text(
    axum::extract::Extension(username): axum::extract::Extension<String>,
    Form(form): Form<TextToolsForm>,
) -> impl IntoResponse {
    let (result, error, show_result, show_error) = match form.operation.as_str() {
        "base64_encode" => {
            let encoded = general_purpose::STANDARD.encode(form.input_text.as_bytes());
            (encoded, String::new(), true, false)
        }
        "base64_decode" => {
            match general_purpose::STANDARD.decode(form.input_text.as_bytes()) {
                Ok(decoded_bytes) => {
                    match String::from_utf8(decoded_bytes) {
                        Ok(decoded_str) => (decoded_str, String::new(), true, false),
                        Err(_) => (String::new(), "デコード結果が有効なUTF-8文字列ではありません".to_string(), false, true),
                    }
                }
                Err(e) => (String::new(), format!("Base64デコードエラー: {}", e), false, true),
            }
        }
        "url_encode" => {
            let encoded = urlencoding::encode(&form.input_text);
            (encoded.to_string(), String::new(), true, false)
        }
        "url_decode" => {
            match urlencoding::decode(&form.input_text) {
                Ok(decoded) => (decoded.to_string(), String::new(), true, false),
                Err(e) => (String::new(), format!("URLデコードエラー: {}", e), false, true),
            }
        }
        "sha256" => {
            let mut hasher = Sha256::new();
            hasher.update(form.input_text.as_bytes());
            let hash_result = hasher.finalize();
            (format!("{:x}", hash_result), String::new(), true, false)
        }
        "sha512" => {
            let mut hasher = Sha512::new();
            hasher.update(form.input_text.as_bytes());
            let hash_result = hasher.finalize();
            (format!("{:x}", hash_result), String::new(), true, false)
        }
        _ => (String::new(), "無効な操作です".to_string(), false, true),
    };

    let template = TextToolsTemplate {
        username,
        result,
        error,
        show_result,
        show_error,
    };
    Html(template.render().unwrap())
}
