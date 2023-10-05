// use std::{sync::Arc, env, net::SocketAddr};

use axum::{
    routing::{get, post},
    Router,
};
use crate::AppState;
use super::handlers::{search_people, find_person, create_person, count_people};

pub fn app_router(app_state: AppState) -> Router {
    Router::new()
        .route("/pessoas", get(search_people))
        .route("/pessoas/:id", get(find_person))
        .route("/pessoas", post(create_person))
        .route("/contagem-pessoas", get(count_people))
        .with_state(app_state)
}