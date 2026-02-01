use askama::Template;
use axum::{
    extract::{Form, State},
    response::{Html, IntoResponse},
};
use serde::Deserialize;
use tower_sessions::Session;
use crate::{AppState, db};
use super::auth;

#[derive(Template)]
#[template(path = "crypto.html")]
struct CryptoTemplate {
    encrypted_text: String,
    decrypted_text: String,
    error: String,
    show_encrypted: bool,
    show_decrypted: bool,
    show_error: bool,
}

#[derive(Deserialize)]
pub struct EncryptForm {
    plaintext: String,
    password: String,
}

#[derive(Deserialize)]
pub struct DecryptForm {
    ciphertext: String,
    password: String,
}

pub async fn index() -> impl IntoResponse {
    let template = CryptoTemplate {
        encrypted_text: String::new(),
        decrypted_text: String::new(),
        error: String::new(),
        show_encrypted: false,
        show_decrypted: false,
        show_error: false,
    };
    Html(template.render().unwrap())
}

pub async fn encrypt(
    State(state): State<AppState>,
    session: Session,
    Form(form): Form<EncryptForm>,
) -> impl IntoResponse {
    let username = auth::get_username(&session).await.unwrap_or_else(|| "unknown".to_string());
    let user = db::get_user_by_username(&state.db, &username).await.ok().flatten();

    match encrypt_string(&form.plaintext, &form.password) {
        Ok(encrypted) => {
            // 監査ログに記録
            let _ = db::create_audit_log(
                &state.db,
                user.as_ref().map(|u| u.id),
                &username,
                "encrypt",
                Some("/crypto/encrypt"),
                Some(&format!("Encrypted text (length: {})", form.plaintext.len())),
                None,
                None,
            ).await;

            let template = CryptoTemplate {
                encrypted_text: encrypted,
                decrypted_text: String::new(),
                error: String::new(),
                show_encrypted: true,
                show_decrypted: false,
                show_error: false,
            };
            Html(template.render().unwrap())
        }
        Err(e) => {
            // エラーも記録
            let _ = db::create_audit_log(
                &state.db,
                user.as_ref().map(|u| u.id),
                &username,
                "encrypt_failed",
                Some("/crypto/encrypt"),
                Some(&format!("Encryption failed: {}", e)),
                None,
                None,
            ).await;

            let template = CryptoTemplate {
                encrypted_text: String::new(),
                decrypted_text: String::new(),
                error: format!("暗号化エラー: {}", e),
                show_encrypted: false,
                show_decrypted: false,
                show_error: true,
            };
            Html(template.render().unwrap())
        }
    }
}

pub async fn decrypt(
    State(state): State<AppState>,
    session: Session,
    Form(form): Form<DecryptForm>,
) -> impl IntoResponse {
    let username = auth::get_username(&session).await.unwrap_or_else(|| "unknown".to_string());
    let user = db::get_user_by_username(&state.db, &username).await.ok().flatten();

    match decrypt_string(&form.ciphertext, &form.password) {
        Ok(decrypted) => {
            // 監査ログに記録
            let _ = db::create_audit_log(
                &state.db,
                user.as_ref().map(|u| u.id),
                &username,
                "decrypt",
                Some("/crypto/decrypt"),
                Some(&format!("Decrypted text (length: {})", decrypted.len())),
                None,
                None,
            ).await;

            let template = CryptoTemplate {
                encrypted_text: String::new(),
                decrypted_text: decrypted,
                error: String::new(),
                show_encrypted: false,
                show_decrypted: true,
                show_error: false,
            };
            Html(template.render().unwrap())
        }
        Err(e) => {
            // エラーも記録
            let _ = db::create_audit_log(
                &state.db,
                user.as_ref().map(|u| u.id),
                &username,
                "decrypt_failed",
                Some("/crypto/decrypt"),
                Some(&format!("Decryption failed: {}", e)),
                None,
                None,
            ).await;

            let template = CryptoTemplate {
                encrypted_text: String::new(),
                decrypted_text: String::new(),
                error: format!("復号化エラー: {}", e),
                show_encrypted: false,
                show_decrypted: false,
                show_error: true,
            };
            Html(template.render().unwrap())
        }
    }
}

// encript_toolの機能を使用した暗号化関数
fn encrypt_string(text: &str, password: &str) -> anyhow::Result<String> {
    use aes_gcm::{
        Aes256Gcm, Nonce,
        aead::{Aead, KeyInit},
    };
    use rand::RngCore;

    // キー生成（簡易版 - Argon2を使用）
    let key = derive_key_from_password(password)?;

    // ランダムナンス生成
    let mut nonce_bytes = [0u8; 12];
    rand::rng().fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);

    // AES-GCM暗号化
    let cipher = Aes256Gcm::new(&key.into());
    let ciphertext = cipher
        .encrypt(nonce, text.as_bytes())
        .map_err(|e| anyhow::anyhow!("暗号化に失敗: {}", e))?;

    // ナンス + 暗号文を結合してBase64エンコード
    let mut result = nonce_bytes.to_vec();
    result.extend_from_slice(&ciphertext);

    Ok(base64::Engine::encode(&base64::engine::general_purpose::STANDARD, &result))
}

// encript_toolの機能を使用した復号化関数
fn decrypt_string(encrypted_text: &str, password: &str) -> anyhow::Result<String> {
    use aes_gcm::{
        Aes256Gcm, Nonce,
        aead::{Aead, KeyInit},
    };

    // Base64デコード
    let data = base64::Engine::decode(&base64::engine::general_purpose::STANDARD, encrypted_text)
        .map_err(|e| anyhow::anyhow!("Base64デコードに失敗: {}", e))?;

    if data.len() < 12 {
        return Err(anyhow::anyhow!("データが短すぎます"));
    }

    // ナンスと暗号文を分離
    let (nonce_bytes, ciphertext) = data.split_at(12);
    let nonce = Nonce::from_slice(nonce_bytes);

    // キー再生成
    let key = derive_key_from_password(password)?;

    // AES-GCM復号化
    let cipher = Aes256Gcm::new(&key.into());
    let plaintext = cipher
        .decrypt(nonce, ciphertext)
        .map_err(|e| anyhow::anyhow!("復号化に失敗: {}", e))?;

    String::from_utf8(plaintext)
        .map_err(|e| anyhow::anyhow!("UTF-8変換に失敗: {}", e))
}

// パスワードから鍵を生成（Argon2使用）
fn derive_key_from_password(password: &str) -> anyhow::Result<[u8; 32]> {
    use argon2::Argon2;
    use std::hash::{Hash, Hasher};

    // ソルト生成（パスワードから導出）
    let mut salt = [0u8; 16];
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    password.hash(&mut hasher);
    let hash_value = hasher.finish();
    let hash_bytes = hash_value.to_le_bytes();
    salt[..8].copy_from_slice(&hash_bytes);
    salt[8..16].copy_from_slice(&hash_bytes);

    // Argon2でキー導出
    let params = argon2::Params::new(
        65536, // 64MB
        3,     // 3回繰り返し
        4,     // 4並列
        Some(32),
    )
    .map_err(|e| anyhow::anyhow!("Argon2パラメータエラー: {}", e))?;

    let argon2 = Argon2::new(
        argon2::Algorithm::Argon2id,
        argon2::Version::V0x13,
        params,
    );

    let mut key = [0u8; 32];
    argon2
        .hash_password_into(password.as_bytes(), &salt, &mut key)
        .map_err(|e| anyhow::anyhow!("キー導出に失敗: {}", e))?;

    Ok(key)
}
