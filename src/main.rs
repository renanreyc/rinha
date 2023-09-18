use std::collections::HashMap;

use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use time::{Date, macros::date};
use uuid::Uuid;

pub struct Person {
    pub id: Uuid,
    pub name: String,
    pub nick: String,
    pub birth_date: Date,
    pub stack: Vec<String>,
}

#[tokio::main] //macros
async fn main() {
    
    let mut people: HashMap<Uuid, Person> = HashMap::new();

    // let id = Uuid::now_v7();

    let person = Person {
        id: Uuid::now_v7(),
        name: String::from("Renan Rey"),
        nick: String::from("rey"),
        birth_date: date!(1996 - 07 - 04),
        stack: vec!["Rust".to_string(), "Python".to_string()]
    };

    people.insert(person.id, person);
    
    // build our application with a single route
    let app = Router::new()
        .route("/pessoas", get(search_people))
        .route("/pessoas/:id", get(find_person))
        .route("/pessoas", post(create_person))
        .route("/contagem-pessoas", get(count_people));

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn search_people() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "Busca de Pessoas")

}

async fn find_person() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "Find")

}

async fn create_person() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "Create")

}

async fn count_people() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "Count")

}