use serde::Serialize;
use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Savefile)]
pub struct Assignment {
    pub name: String,
    pub due_date: String,
    pub completed: bool,
    pub mark: u8,
}

pub fn save_assignment(assignment: &Assignment, file: &String) {
    savefile::prelude::save_file(file, 0, assignment).unwrap();
}

pub fn load_assignment(file: &String) -> Assignment {
    return savefile::prelude::load_file(file, 0).unwrap();
}

pub fn assignments(path: &String) {
    let choices = vec!["create", "view", "edit", "delete"];
    let assignments_choice = dialoguer::Select::new().with_prompt("What would you like to do?").items(&choices).default(0).interact().unwrap();

    let assignments_choice = choices[assignments_choice].to_string();

    if assignments_choice == "create" {
        let mut filepath : String = (&path).to_string();

        filepath.push_str("/assignments");

        match std::fs::read_dir(&filepath) {
            Ok(_dir) => { println!("reading assignments..."); }
            Err(_err) => { match std::fs::create_dir(&filepath) {
                Ok(_dir) => { }
                Err(_err) => { println!("error creating 'assignments' folder"); }
            } }
        }

        let assignment_name : String = dialoguer::Input::new().with_prompt("Enter the name of the assignment").interact().unwrap();
        let assignment_date : String = dialoguer::Input::new().with_prompt("Enter the due date (YYYY-MM-DD)").interact().unwrap();
        let assignment_complete : String = dialoguer::Input::new().with_prompt("Is the assignment done? (y/n)").interact_text().unwrap();
        let assignment_mark : String = dialoguer::Input::new().with_prompt("Enter the assignment mark (enter 0 if unmarked)").interact().unwrap();
        let assignment_mark_int : u8 = assignment_mark.trim().parse().unwrap();
        let mut assignment_complete_bool : bool = false;

        if assignment_complete == "y" {
            assignment_complete_bool = true;
        }
        else {
            assignment_complete_bool = false;
        }

        let assignment = Assignment {
            name: assignment_name,
            due_date: assignment_date,
            completed: assignment_complete_bool,
            mark: assignment_mark_int,
        };

        filepath.push_str("/");
        filepath.push_str(assignment.name.as_str());
        filepath.push_str(".bin");

        println!("saving assignment...");
        std::fs::File::create(&filepath).expect("assignment could not be created.");

        println!("");
        println!("The following will be saved:");
        println!("Name: {}", assignment.name);
        println!("Due Date: {}", assignment.due_date);
        println!("Completed?: {}", assignment.completed);
        println!("Mark: {}", assignment.mark);
        println!("");

        save_assignment(&assignment, &filepath)
    }
    else if assignments_choice == "view" {
        println!("finding assignments to view...");
        let mut filepath = (&path).to_string();

        filepath.push_str("/assignments");

        match std::fs::read_dir(&filepath) {
            Ok(_dir) => { println!("reading assignments..."); }
            Err(_err) => { match std::fs::create_dir(&filepath) {
                Ok(_dir) => { }
                Err(_err) => { println!("error creating 'assignments' folder"); }
            } }
        }

        let assignments = std::fs::read_dir(&filepath).unwrap();

        println!("These are the assignments you have saved:");
        for path in assignments {
            let path_name = path.unwrap().path().display().to_string();
            let assignment_option = load_assignment(&path_name);

            println!("{}", assignment_option.name);
        }
        
        let assignment_name : String = dialoguer::Input::new().with_prompt("Enter the name of the assignment you wish to view").interact().unwrap();

        println!("finding assignment...");
        filepath.push_str("/");
        filepath.push_str(assignment_name.as_str());
        filepath.push_str(".bin");

        let assignment : Assignment = load_assignment(&filepath);
        println!("assignment found!");

        let mut assignment_completed : String = "no".to_string();

        if assignment.completed == true {
            assignment_completed = "yes".to_string();
        }
        else {
            assignment_completed = "no".to_string();
        }

        println!("");
        println!("Name: {}", assignment.name);
        println!("Due Date: {}", assignment.due_date);
        println!("Completed? {}", assignment_completed);
        println!("Mark: {}", assignment.mark);
        println!("");
    }
    else if assignments_choice == "edit" {
        println!("finding assignments to edit...");
        let mut filepath = (&path).to_string();

        filepath.push_str("/assignments");

        match std::fs::read_dir(&filepath) {
            Ok(_dir) => { println!("reading assignments..."); }
            Err(_err) => { match std::fs::create_dir(&filepath) {
                Ok(_dir) => { }
                Err(_err) => { println!("error creating 'assignments' folder"); }
            } }
        }

        let assignments = std::fs::read_dir(&filepath).unwrap();

        println!("These are the assignments you have saved:");
        for path in assignments {
            let path_name = path.unwrap().path().display().to_string();
            let assignment_option = load_assignment(&path_name);

            println!("{}", assignment_option.name);
        }

        let assignment_toedit : String = dialoguer::Input::new().with_prompt("Enter the name of the assignment you wish to edit").interact().unwrap();

        println!("finding assignment...");

        let mut filepath_toedit = filepath.clone();

        filepath_toedit.push_str("/");
        filepath_toedit.push_str(assignment_toedit.as_str());
        filepath_toedit.push_str(".bin");

        let mut assignment : Assignment = load_assignment(&filepath_toedit);
        println!("assignment found!");

        let assignment_mark_toedit : String = assignment.mark.to_string();

        let assignment_name : String = dialoguer::Input::new().with_prompt("Name").with_initial_text(assignment.name.as_str()).interact().unwrap();
        let assignment_duedate : String = dialoguer::Input::new().with_prompt("Due Date").with_initial_text(assignment.due_date.as_str()).interact().unwrap();
        if assignment.completed == true {
            let assignment_complete : String = dialoguer::Input::new().with_prompt("Completed? (y/n)").with_initial_text("y").interact().unwrap();

            if assignment_complete == "y" {
                assignment.completed = true;
            }
            else {
                assignment.completed = false;
            }
        }
        else {
            let assignment_complete : String = dialoguer::Input::new().with_prompt("Completed? (y/n)").with_initial_text("n").interact().unwrap();

            if assignment_complete == "y" {
                assignment.completed = true;
            }
            else {
                assignment.completed = false;
            }
        }
        let assignment_mark_edited : String = dialoguer::Input::new().with_prompt("Mark").with_initial_text(assignment_mark_toedit.as_str()).interact().unwrap();

        assignment.name = assignment_name;
        assignment.due_date = assignment_duedate;
        assignment.mark = assignment_mark_edited.parse().unwrap();

        println!("");
        println!("The following will be saved:");
        println!("Name: {}", assignment.name);
        println!("Due Date: {}", assignment.due_date);
        println!("Completed?: {}", assignment.completed);
        println!("Mark: {}", assignment.mark);
        println!("");

        let mut filepath_edited = filepath.clone();

        filepath_edited.push_str("/");
        filepath_edited.push_str(assignment.name.as_str());
        filepath_edited.push_str(".bin");

        std::fs::remove_file(&filepath_toedit).expect("could not remove assignment");
        save_assignment(&assignment, &filepath_edited);
        println!("assignment edited!");
    }
    else if assignments_choice == "delete" {
        println!("finding assignments to delete...");
        let mut filepath = (&path).to_string();

        filepath.push_str("/assignments");

        match std::fs::read_dir(&filepath) {
            Ok(_dir) => { println!("reading assignments..."); }
            Err(_err) => { match std::fs::create_dir(&filepath) {
                Ok(_dir) => { }
                Err(_err) => { println!("error creating 'assignments' folder"); }
            } }
        }

        let assignments = std::fs::read_dir(&filepath).unwrap();

        println!("These are the assignments you have saved:");
        for path in assignments {
            let path_name = path.unwrap().path().display().to_string();
            let assignment_option = load_assignment(&path_name);

            println!("{}", assignment_option.name);
        }
        
        let assignment_name : String = dialoguer::Input::new().with_prompt("Enter the name of the assignment you wish to delete").interact().unwrap();

        filepath.push_str("/");
        filepath.push_str(assignment_name.as_str());
        filepath.push_str(".bin");

        std::fs::remove_file(&filepath).expect("assignment could not be found.");
    }
}