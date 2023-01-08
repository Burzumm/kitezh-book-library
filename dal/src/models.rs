use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct BookItem {
    pub _id: Uuid,
    pub title: String,
    pub description: String,
    pub url: String,
}
