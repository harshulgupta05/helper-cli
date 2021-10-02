use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Savefile)]
pub struct Event {
    pub name: String,
    pub description: String,
    pub date: String,
    pub todo: Vec,
}

pub fn save_event(event: &Event, file: &String) {
    savefile::prelude::save_file(file, 0, event).unwrap();
}

pub fn load_event(file: &String) -> Event {
    return savefile::prelude::load_file(file, 0);
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

    match choice {
        "create" => createEvent(path),
        "edit" => editEvent(),
        "view" => viewEvent(path),
        "delete" => deleteEvent()
    }
}

pub fn createEvent(path: &String) {
    let mut filepath = (&path).to_string();

    filepath.push_str("/events");

    match std::fs::read_dir(&filepath) {
        Ok(_dir) => { println!("reading events..."); },
        Err(_err) => ( match std::fs::create_dir(&filepath); ) {
            Ok(_dir) => { },
            Err(_dir) => { println!("error creating events folder. please ensure that you have access to the active directory."); }
        }}
    }

    let event_todo = new Vec;
    let event_name : String = dialoguer::Input::new().with_prompt("Enter the name of the event: ").interact().unwrap();
    let event_date : String = dialoguer::Input::new().with_prompt("Enter the date of the event:").interact().unwrap();
    let event_desc : String = dialoguer::Input::new().with_prompt("Enter the event description (don't hit 'Enter' until done):").interact().unwrap();
    while (dialoguer::Confirm.with_prompt("Do you have something to add to the todo?").interact()) {
        event_todo.insert(dialoguer::Input::new().with_prompt("Enter the next line in the todo list:").interact().unwrap());
    }

    let event = Event {
        name: event_name,
        description: event_desc,
        date: event_date,
        todo: event_todo
    }

    filepath.push_str("/" + event_name.as_str() + ".bin");

    println!("saving event...");
    std::fs::File::create(&filepath).expect("event could not be saved.");

    println!();
    println!("The following has been saved:");
    println!("Event Name: {}", event.name);
    println!("Event Date: {}", event.date);
    println!("Event Description {}", event.description);
    for todo in event.todo {
        println!("Event TODO: {}", todo);
    }

    save_event(&event, &filepath);
}

pub fn viewEvent(path: &String) {
    let mut filepath = (&path).to_string();

    filepath.push_str("/events");

    println!("finding events to view...");

    match std::fs::read_dir(&filepath) {
        Ok(_dir) => { println!("reading events..."); },
        Err(_err) => {
            println!("events directory either does not exist or is not in the active directory. there are no events to view.");
            return;
        }
    }

    let events_choices = std::fs::read_dir(&filepath).unwrap();

    let mut options = new Vec;

    for path in events_choices {
        let option = load_event(path.unwrap().path().display().to_string());
        options.insert(option);
    }
} 