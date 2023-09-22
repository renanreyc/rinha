use std::{collections::HashMap, sync::Arc};

use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Router, extract::{State, Path}, Json,
};
use serde::{Serialize, Deserialize};
use time::{Date, macros::date};
use tokio::sync::{RwLock};
use uuid::Uuid;

time::serde::format_description!(date_format, Date, "[year]-[month]-[day]");

#[derive(Clone, Serialize)]
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

#[derive(Clone, Deserialize)]
#[serde(try_from = "String")]
pub struct PersonName(String);


impl TryFrom<String> for PersonName {
    type Error = &'static str;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() <= 100 {
            Ok(PersonName(value))
        } else {
            Err("name is too big")
        }
    }
}

#[derive(Clone, Deserialize)]
#[serde(try_from = "String")]
pub struct Nick(String);

impl TryFrom<String> for Nick {
    type Error = &'static str;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() <= 32 {
            Ok(Self(value))
        } else {
            Err("nick is too big")
        }
    }
}

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

type AppState = Arc<RwLock<HashMap<Uuid, Person>>>;

#[tokio::main] //macros
async fn main() {
    
    let mut people: HashMap<Uuid, Person> = HashMap::new();

    // let id = Uuid::now_v7();

    let person = Person {
        id: Uuid::now_v7(),
        name: String::from("Renan Rey"),
        nick: String::from("rey"),
        birth_date: date!(1996 - 07 - 04),
        stack: None, 
    };

    println!("{}", person.id);

    people.insert(person.id, person);

    let app_state = Arc::new(RwLock::new(people));
    
    // build our application with a single route
    let app = Router::new()
        .route("/pessoas", get(search_people))
        .route("/pessoas/:id", get(find_person))
        .route("/pessoas", post(create_person))
        .route("/contagem-pessoas", get(count_people))
        .with_state(app_state);

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn search_people(state: State<AppState>) -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "Busca de Pessoas")
}

async fn find_person(
    State(people): State<AppState>, 
    Path(person_id): Path<Uuid>,
) -> impl IntoResponse {

    match people.read().await.get(&person_id) {
        Some(person) => Ok(Json(person.clone())),
        None => Err(StatusCode::NOT_FOUND),
    }
}

async fn create_person(
    State(people): State<AppState>, 
    Json(new_person): Json<NewPerson>,
) -> impl IntoResponse { // ) -> Result<(StatusCode, Json<Person>), StatusCode> {

    // if new_person.name.0.len() > 100 || new_person.nick.len() > 32 {
    //     return Err(StatusCode::UNPROCESSABLE_ENTITY)
    // }

    // if let Some(ref stack) = new_person.stack {
    //     if stack.iter().any(|tech| tech.len() > 32) {
    //         return Err(StatusCode::UNPROCESSABLE_ENTITY)
    //     }
    // }

    let id = Uuid::now_v7();
    let person = Person {
        id,
        name: new_person.name.0,
        birth_date: new_person.birth_date,
        nick: new_person.nick.0,
        stack: new_person
            .stack
            .map(|stack| stack.into_iter().map(String::from).collect()),
    };

    people.write().await.insert(id, person.clone());

    (StatusCode::CREATED, Json(person))

}

async fn count_people(
    State(people): State<AppState>, 
) -> impl IntoResponse {
    let count = people.read().await.len();

    (StatusCode::NOT_FOUND, Json(count))

}