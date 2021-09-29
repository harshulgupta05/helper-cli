use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize)]
pub struct Event {
    pub name: String,
    pub description: String,
    pub date: DateTime,
    pub todo: Vec,
}

pub fn events(path: &String) {
    let choices = vec!["create", "edit", "view", "delete"];
    let choice = dialoguer::Select::new()
        .with_prompt("What would you like to do?")
        .items(choices)
        .default(0)
        .interact()
        .unwrap();

    let choice = choices[choice].to_string();
}
