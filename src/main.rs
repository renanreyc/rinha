mod persistence;
mod router;
mod models;
mod handlers;

use std::{env, net::SocketAddr, sync::Arc};
use persistence::PostgresRepository;
use router::app_router;

type AppState = Arc<PostgresRepository>;

#[tokio::main]
async fn main() {
    let port = env::var("PORT")
        .ok()
        .and_then(|port| port.parse::<u16>().ok())
        .unwrap_or(9999);

    let database_url = env::var("DATABASE_URL")
        .unwrap_or(String::from("postgres://rinha:rinha@localhost:5432/rinha"));

    let repo = PostgresRepository::connect(database_url).await;
    let app_state = Arc::new(repo);

    // run it with hyper on localhost:3000
    axum::Server::bind(&SocketAddr::from(([0, 0, 0, 0], port)))
        .serve(app_router(app_state).into_make_service())
        .await
        .unwrap();
}