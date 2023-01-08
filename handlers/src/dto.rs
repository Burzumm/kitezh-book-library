use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct BookItemDto {
    pub title: String,
    pub description: String,
    pub url: String,
}
