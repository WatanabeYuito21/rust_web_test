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

const SESSION_USER_KEY: &str = "user";

async fn auth_middleware(
    session: tower_sessions::Session,
    mut request: Request,
    next: Next,
) -> Result<Response, Redirect> {
    let path = request.uri().path();

    if path == "/login" || path.starts_with("/static/") {
        return Ok(next.run(request).await);
    }

    if routes::auth::is_authenticated(&session).await {
        // セッションからユーザー名を取得してExtensionに追加
        if let Ok(Some(username)) = session.get::<String>(SESSION_USER_KEY).await {
            request.extensions_mut().insert(username);
        }
        Ok(next.run(request).await)
    } else {
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
    let session_layer = SessionManagerLayer::new(session_store);

    let app = Router::new()
        .route("/", get(routes::home::index))
        .route("/about", get(routes::home::about))
        .route("/time", get(routes::time::time))
        .route("/sysinfo", get(routes::sysinfo::index))
        .route("/sysinfo/live", get(routes::sysinfo::live))
        .route("/users", get(routes::users::list_users))
        .route("/crypto", get(routes::crypto::index))
        .route("/crypto/encrypt", axum::routing::post(routes::crypto::encrypt))
        .route("/crypto/decrypt", axum::routing::post(routes::crypto::decrypt))
        .route("/password-gen", get(routes::password_gen::show_password_gen))
        .route("/password-gen/generate", axum::routing::post(routes::password_gen::generate_password))
        .route("/text-tools", get(routes::text_tools::show_text_tools))
        .route("/text-tools/process", axum::routing::post(routes::text_tools::process_text))
        .route("/profile", get(routes::profile::show_profile))
        .route("/profile/update", axum::routing::post(routes::profile::update_profile))
        .route("/profile/change-password", axum::routing::post(routes::profile::change_password))
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
