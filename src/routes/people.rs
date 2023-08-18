use axum::{
    extract::{Path, Query, State},
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    Json,
};
use chrono::NaiveDate;
use serde_json::json;
use std::sync::Arc;
use uuid::Uuid;

use crate::models::person::Person;
use crate::schema::{CreatePersonSchema, SearchTermQueryParams};
use crate::AppState;

pub async fn post(
    State(data): State<Arc<AppState>>,
    Json(body): Json<CreatePersonSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query_result = sqlx::query_as!(
        Person,
        "INSERT INTO people (id,apelido,nome,nascimento,stack) VALUES ($1, $2, $3, $4, $5) RETURNING *",
        Uuid::new_v4(),
        body.apelido,
        body.nome,
        NaiveDate::parse_from_str(&body.nascimento.unwrap(), "%Y-%m-%d").unwrap(),
        body.stack.as_deref(),
    )
    .fetch_one(&data.db)
    .await;

    match query_result {
        Ok(person) => {
            let person_response = json!({"status": "success","data": json!({
                "person": person
            })});

            let mut headers = HeaderMap::new();
            headers.insert(
                "location",
                format!("/pessoas/{}", person.id.to_string())
                    .parse()
                    .unwrap(),
            );

            return Ok((headers, (StatusCode::CREATED, Json(person_response))));
        }
        Err(e) => {
            if e.to_string().contains("database") && e.to_string().contains("error") {
                return Err((
                    StatusCode::UNPROCESSABLE_ENTITY,
                    Json(json!({"status": "error","message": format!("{:?}", e)})),
                ));
            }

            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"status": "error","message": format!("{:?}", e)})),
            ));
        }
    }
}

pub async fn get_by_uuid(
    Path(id): Path<uuid::Uuid>,
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query_result = sqlx::query_as!(Person, "SELECT * FROM people WHERE id = $1", id)
        .fetch_one(&data.db)
        .await;

    match query_result {
        Ok(note) => {
            let note_response = serde_json::json!(note);

            return Ok(Json(note_response));
        }
        Err(_) => {
            let error_response = serde_json::json!({
                "status": "fail",
                "message": format!("Person with ID: {} not found", id)
            });
            return Err((StatusCode::NOT_FOUND, Json(error_response)));
        }
    }
}

pub async fn get_by_search_term(
    Query(query_params): Query<SearchTermQueryParams>,
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query_result = sqlx::query_as!(
        Person,
        "SELECT * FROM people WHERE LOWER(apelido) LIKE $1 OR LOWER(nome) LIKE $1 OR EXISTS (
            SELECT
            FROM unnest(stack) elem
            WHERE LOWER(elem) LIKE $1
          )",
        format!("%{}%", query_params.t.to_lowercase())
    )
    .fetch_all(&data.db)
    .await;

    match query_result {
        Ok(mut people) => {
            people.truncate(50);

            let people_response = serde_json::json!(people);

            return Ok(Json(people_response));
        }
        Err(_) => {
            let error_response = serde_json::json!({
                "status": "fail",
                "message": "deu ruim :(",
            });
            return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)));
        }
    }
}

pub async fn count(State(data): State<Arc<AppState>>) -> String {
    let query_result = sqlx::query!("SELECT COUNT(*) as count FROM people")
        .fetch_one(&data.db)
        .await;

    return match query_result {
        Ok(count_result) => format!("{:?}", count_result.count.unwrap()),
        Err(error) => format!("Deu ruim :( Sadge: {}", error),
    };
}
