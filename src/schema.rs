use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct SearchTermQueryParams {
    pub t: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreatePersonSchema {
    pub apelido: Option<String>,
    pub nome: Option<String>,
    pub nascimento: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack: Option<Vec<String>>,
}
