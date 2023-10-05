use axum::{Json, response::IntoResponse, http::StatusCode, extract::{State, Path, Query}};
use crate::AppState;
use super::models::my_models::{PersonSearchQuery, NewPerson};
use uuid::Uuid;

pub async fn search_people(
    State(people): State<AppState>, 
    Query(query_params): Query<PersonSearchQuery>,
) -> impl IntoResponse {
    let query = query_params.query().to_string();
    match people.search_people(query).await {
        Ok(person) => Ok(Json(person)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
}

pub async fn find_person(
    State(people): State<AppState>, 
    Path(person_id): Path<Uuid>,
) -> impl IntoResponse {

    match people.find_person(person_id).await {
        Ok(Some(person)) => Ok(Json(person)),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
}

pub async fn create_person(
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

pub async fn count_people(
    State(people): State<AppState>, 
) -> impl IntoResponse {
    
    match people.count_people().await {
        Ok(count) => Ok(Json(count)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}