mod routes;
mod db;
use axum::{
    Router, extract::Request, middleware, middleware::Next, response::Redirect, response::Response,
    routing::get,
};
use sqlx::MySqlPool;
use tower_http::services::ServeDir;
use tower_sessions::{MemoryStore, SessionManagerLayer};

#[derive(Clone)]
pub struct AppState {
    pub db: MySqlPool,
}

async fn auth_middleware(
    session: tower_sessions::Session,
    request: Request,
    next: Next,
) -> Result<Response, Redirect> {
    let path = request.uri().path();

    if path == "/login" || path.starts_with("/static/") {
        return Ok(next.run(request).await);
    }

    let is_auth = routes::auth::is_authenticated(&session).await;
    eprintln!("認証チェック: path={}, authenticated={}", path, is_auth);

    if is_auth {
        Ok(next.run(request).await)
    } else {
        eprintln!("未認証のため /login にリダイレクト");
        Err(Redirect::to("/login"))
    }
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    // データベース接続プールの作成
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env file");

    let db_pool = db::create_pool(&database_url)
        .await
        .expect("Failed to connect to database");

    let app_state = AppState {
        db: db_pool,
    };

    let session_store = MemoryStore::default();
    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(false)  // HTTP でも動作するように設定（本番環境では true に）
        .with_same_site(tower_sessions::cookie::SameSite::Lax)
        .with_expiry(tower_sessions::Expiry::OnInactivity(
            tower_sessions::cookie::time::Duration::hours(24)
        ));

    let app = Router::new()
        .route("/", get(routes::home::index))
        .route("/about", get(routes::home::about))
        .route("/time", get(routes::time::time))
        .route("/sysinfo", get(routes::sysinfo::index))
        .route("/sysinfo/live", get(routes::sysinfo::live))
        .route("/users", get(routes::users::list_users))
        .route("/audit", get(routes::audit::list_audit_logs))
        .route("/crypto", get(routes::crypto::index))
        .route("/crypto/encrypt", axum::routing::post(routes::crypto::encrypt))
        .route("/crypto/decrypt", axum::routing::post(routes::crypto::decrypt))
        .route(
            "/login",
            get(routes::auth::login_page).post(routes::auth::login),
        )
        .route("/logout", get(routes::auth::logout))
        .nest_service("/static", ServeDir::new("static"))
        .layer(middleware::from_fn(auth_middleware))
        .layer(session_layer)
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
