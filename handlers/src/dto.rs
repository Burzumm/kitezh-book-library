use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct BookItemDto {
    pub name: String,
    pub title: String,
    pub description: String,
    pub url: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct UpdateBookItemDto {
    pub id: Uuid,
    pub name: String,
    pub title: String,
    pub description: String,
    pub url: String,
}
