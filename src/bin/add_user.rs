use argon2::Argon2;
use password_hash::{PasswordHasher, SaltString, rand_core::OsRng};
use std::io::{self, Write};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    // データベース接続
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env file");

    let pool = rust_dashboard::db::create_pool(&database_url)
        .await
        .expect("Failed to connect to database");

    // ユーザー名の入力
    print!("Username: ");
    io::stdout().flush().unwrap();
    let mut username = String::new();
    io::stdin().read_line(&mut username).unwrap();
    let username = username.trim();

    // パスワードの入力
    print!("Password: ");
    io::stdout().flush().unwrap();
    let mut password = String::new();
    io::stdin().read_line(&mut password).unwrap();
    let password = password.trim();

    // パスワードのハッシュ化
    let salt = SaltString::generate(&mut OsRng);
    let hash = Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .unwrap()
        .to_string();

    // ユーザーの作成
    match rust_dashboard::db::create_user(&pool, username, &hash).await {
        Ok(user) => {
            println!("User created successfully:");
            println!("  ID: {}", user.id);
            println!("  Username: {}", user.username);
        }
        Err(e) => {
            eprintln!("Error creating user: {}", e);
            std::process::exit(1);
        }
    }
}
