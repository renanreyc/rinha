use std::{sync::Arc, env, net::SocketAddr};

use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Router, extract::{State, Path, Query}, Json,
};
use persistence::PostgresRepository;
use serde::{Serialize, Deserialize};
use time::{Date};
// use tokio::sync::{RwLock};
use uuid::Uuid;

mod persistence;
mod macros_extra;

time::serde::format_description!(date_format, Date, "[year]-[month]-[day]");

#[derive(Clone, Serialize, sqlx::FromRow)]
pub struct Person {
    pub id: Uuid,
    #[serde(rename = "nome")]
    pub name: String,
    #[serde(rename = "apelido")]
    pub nick: String,
    #[serde(rename = "nascimento", with="date_format")]
    pub birth_date: Date,
    pub stack: Option<Vec<String>>,
}

macro_rules! new_string_type {
    ($type: ident, max_lengh = $max_length:expr, error = $error_message:expr) => {
        #[derive(Clone, Deserialize)]
        #[serde(try_from = "String")]
        pub struct $type(String);

        impl TryFrom<String> for $type {
            type Error = &'static str;
        
            fn try_from(value: String) -> Result<Self, Self::Error> {
                if value.len() <= $max_length {
                    Ok($type(value))
                } else {
                    Err($error_message)
                }
            }
        }
    };    
}

new_string_type!(PersonName, max_lengh = 100, error = "name is too big");
new_string_type!(Nick, max_lengh = 32, error = "nick is too big");


#[derive(Clone, Deserialize)]
#[serde(try_from = "String")]

pub struct Tech(String);

impl TryFrom<String> for Tech {
    type Error = &'static str;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() <= 32 {
            Ok(Self(value))
        } else {
            Err("tech is too big")
        }
    }
}

impl  From<Tech> for String {
    fn from(value: Tech) -> Self {
        value.0
    }
}

#[derive(Clone, Deserialize)]
pub struct NewPerson {
    #[serde(rename = "nome")]
    pub name: PersonName,
    #[serde(rename = "apelido")]
    pub nick: Nick,
    #[serde(rename = "nascimento", with="date_format")]
    pub birth_date: Date,
    pub stack: Option<Vec<Tech>>,
}

type AppState = Arc<PostgresRepository>;

#[tokio::main] //macros
async fn main() {
    
    let port = env::var("PORT")
        .ok()
        .and_then(|port| port.parse::<u16>().ok())
        .unwrap_or(9999);

    let database_url = env::var("DATABASE_URL")
        .unwrap_or(String::from("postgres://rinha:rinha@localhost:5432/rinha"));

    let repo = PostgresRepository::connect(database_url).await;
        
    let app_state = Arc::new(repo);
    
    // build our application with a single route
    let app = Router::new()
        .route("/pessoas", get(search_people))
        .route("/pessoas/:id", get(find_person))
        .route("/pessoas", post(create_person))
        .route("/contagem-pessoas", get(count_people))
        .with_state(app_state);

    // run it with hyper on localhost:3000
    axum::Server::bind(&SocketAddr::from(([0, 0, 0, 0], port)))
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[derive(Deserialize)]
struct PersonSearchQuery {
    #[serde(rename= "t")]
    query: String,
}

async fn search_people(
    State(people): State<AppState>, 
    Query(PersonSearchQuery {query}): Query<PersonSearchQuery>,
) -> impl IntoResponse {
    match people.search_people(query).await {
        Ok(person) => Ok(Json(person)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
}

async fn find_person(
    State(people): State<AppState>, 
    Path(person_id): Path<Uuid>,
) -> impl IntoResponse {

    match people.find_person(person_id).await {
        Ok(Some(person)) => Ok(Json(person)),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
}

async fn create_person(
    State(people): State<AppState>, 
    Json(new_person): Json<NewPerson>,
) -> impl IntoResponse { // ) -> Result<(StatusCode, Json<Person>), StatusCode> {

    match people.create_person(new_person).await {
        Ok(person) => Ok((StatusCode::CREATED, Json(person))),
        Err(sqlx::Error::Database(err)) if err.is_unique_violation() => {
                Err(StatusCode::UNPROCESSABLE_ENTITY)    
        }
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
}

async fn count_people(
    State(people): State<AppState>, 
) -> impl IntoResponse {
    match people.count_people().await {
        Ok(count) => Ok(Json(count)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }

}