#[warn(non_snake_case)]
fn first_time() -> bool {
    let input : String = dialoguer::Input::new()
        .with_prompt("Have you used this app before? (y/n) ")
        .interact_text()
        .unwrap();
    
    if input == "y" {
        return false;
    }
    else {
        return true;
    }
}

use serde::Serialize;
use serde::Deserialize;
use std::fmt::Debug;

#[macro_use]
extern crate savefile_derive;

#[derive(Serialize, Deserialize, Debug, Savefile)]
struct Assignment {
    name: String,
    due_date: String,
    completed: bool,
}

fn save_assignment(assignment: &Assignment, file: &String) {
    savefile::prelude::save_file(file, 0, assignment).unwrap();
}

fn load_assignment(file: &String) -> Assignment {
    return savefile::prelude::load_file(file, 0).unwrap();
}

fn assignments(path: &String) {
    let assignments_choice : String = dialoguer::Input::new().with_prompt("What would you like to do? (create/view/delete)").interact_text().unwrap();

    if assignments_choice == "create" {
        let mut filepath : String = (&path).to_string();

        filepath.push_str("/assignments");

        match std::fs::read_dir(&filepath) {
            Ok(dir) => { println!("reading assignments..."); }
            Err(err) => { match std::fs::create_dir(&filepath) {
                Ok(dir) => { }
                Err(err) => { println!("error creating 'assignments' folder"); }
            } }
        }

        let assignment_name : String = dialoguer::Input::new().with_prompt("Enter the name of the assignment").interact().unwrap();
        let assignment_date : String = dialoguer::Input::new().with_prompt("Enter the due date (YYYY-MM-DD").interact().unwrap();
        let assignment_complete : String = dialoguer::Input::new().with_prompt("Is the assignment done? (y/n)").interact_text().unwrap();
        let assignment_complete_bool = false;

        if assignment_complete == "y" {
            let assignment_complete_bool : bool = true;
        }
        else {
            let assignment_complete_bool : bool = false;
        }

        let assignment = Assignment {
            name: assignment_name,
            due_date: assignment_date,
            completed: assignment_complete_bool,
        };

        filepath.push_str("/");
        filepath.push_str(assignment.name.as_str());
        filepath.push_str(".bin");

        std::fs::File::create(&filepath);

        save_assignment(&assignment, &filepath)
    }
    else if assignments_choice == "view" {
        let assignment_name : String = dialoguer::Input::new().with_prompt("Enter the name of the assignment you wish to view").interact().unwrap();
        let mut filepath = (&path).to_string();

        filepath.push_str("/assignments");
        filepath.push_str("/");
        filepath.push_str(assignment_name.as_str());
        filepath.push_str(".bin");

        let assignment : Assignment = load_assignment(&filepath);

        let mut assignment_completed : String = "no".to_string();

        if assignment.completed == true {
            assignment_completed = "yes".to_string();
        }
        else {
            assignment_completed = "no".to_string();
        }

        println!("Name: {}", assignment.name);
        println!("Due Date: {}", assignment.due_date);
        println!("Completed? {}", assignment_completed);
    }
    else if assignments_choice == "delete" {
        let assignment_name : String = dialoguer::Input::new().with_prompt("Enter the name of the assignment you wish to delete").interact().unwrap();
        let mut filepath = (&path).to_string();

        filepath.push_str("/assignments");
        filepath.push_str("/");
        filepath.push_str(assignment_name.as_str());
        filepath.push_str(".bin");

        std::fs::remove_file(&filepath);
    }
}

fn tests() {

}

fn events() {

}

fn homework() {

}

#[warn(non_snake_case)]
fn option(path: &String) {
    let option : String = dialoguer::Input::new().with_prompt("What would you like to do? (assignments, tests, events, homework").interact_text().unwrap();

    if option == "assignments" {
        assignments(path);
    }
    else if option == "tests" {
        tests();
    }
    else if option == "events" {
        events();
    }
    else if option == "homework" {
        homework();
    }
}

#[warn(non_snake_case)]
fn main() {
    println!("Welcome to Helper CLI! ");

    let firstTime : bool = first_time();

    if firstTime == true {
        let mut path : String = dialoguer::Input::new().with_prompt("Please enter the path to create the Helper directory").interact().unwrap();

        path.push_str("/helper-files");

        std::fs::create_dir(&path);

        let mut done : bool = false;

        while done == false {
            option(&path);

            let mut arewedoneyet : String = dialoguer::Input::new().with_prompt("Is there anything else you need? (y/n) ").interact_text().unwrap();

            if arewedoneyet == "y" {
                option(&path);
            }
            else {
                println!("Goodbye!");
                done = true;
                continue;
            }
        }
    }
    else {
        let mut path : String = dialoguer::Input::new().with_prompt("Please enter the path to find the Helper directory").interact().unwrap();

        path.push_str("/helper-files");

        match std::fs::read_dir(&path) {
            Ok(dir) => { println!("Directory found!") }
            Err(err) => { println!("The helper-files directory either cannot be found or you don't have permission to access it.") }
        }

        let mut done : bool = false;

        while done == false {
            option(&path);

            let mut arewedoneyet : String = dialoguer::Input::new().with_prompt("Is there anything else you need? (y/n) ").interact_text().unwrap();

            if arewedoneyet == "y" {
                option(&path);
            }
            else {
                println!("Goodbye!");
                done = true;
                continue;
            }
        }
    }
}
