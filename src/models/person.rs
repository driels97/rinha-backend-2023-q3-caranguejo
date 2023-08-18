use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(FromRow, Deserialize, Serialize)]
pub struct Person {
    pub id: Uuid,
    pub apelido: String,
    pub nome: String,
    pub nascimento: NaiveDate,
    pub stack: Option<Vec<String>>,
}
