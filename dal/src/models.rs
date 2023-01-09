use std::str::FromStr;

use serde::ser::{Serialize, SerializeStruct};
use serde::{Deserialize, Deserializer};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
pub struct BookItem {
    #[serde(rename = "_id", deserialize_with = "uuid_from_str")]
    pub id: Uuid,
    pub name: String,
    pub title: String,
    pub description: String,
    pub url: String,
}

impl Serialize for BookItem {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("BookItem", 5)?;
        state.serialize_field("_id", &self.id.to_string())?;
        state.serialize_field("name", &self.name)?;
        state.serialize_field("title", &self.title)?;
        state.serialize_field("description", &self.description)?;
        state.serialize_field("url", &self.url)?;
        state.end()
    }
}

fn uuid_from_str<'de, D>(deserializer: D) -> Result<Uuid, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(deserializer)?;
    return Ok(uuid::Uuid::from_str(s).unwrap());
}
