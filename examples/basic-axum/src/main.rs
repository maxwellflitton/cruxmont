mod api;
mod dal;

use axum::{routing::get, Router};
use tokio::main;
use tokio::net::TcpListener;

// Handler for the root endpoint
async fn hello_world() -> &'static str {
    "Hello, World!"
}

#[main]
async fn main() {

    // run migrations with live PG pool
    // set env for `DATABASE_URL` and `DB_MAX_CONNECTIONS`
    dal::basic_migrations::run_migrations(
        &*cruxmont::dal::connections::sqlx_postgres::SQLX_POSTGRES_POOL
    ).await.expect("run migrations");

    // Create the Axum router with a single route
    let app = Router::new()
        .route("/", get(hello_world));

    let app = api::counter_factory(app);

    // Start the server
    let listener = TcpListener::bind("0.0.0.0:8001").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}