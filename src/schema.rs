use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct SearchTermQueryParams {
    pub t: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreatePersonSchema {
    pub apelido: String,
    pub nome: String,
    pub nascimento: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack: Option<Vec<String>>,
}
