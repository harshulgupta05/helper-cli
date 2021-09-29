use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize)]
pub struct Event {
    pub name: String,
    pub description: String,
    pub date: DateTime,
    pub todo: Vec,
}
