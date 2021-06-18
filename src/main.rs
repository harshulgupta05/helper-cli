mod assignments;
mod tests;

use serde::Serialize;
use serde::Deserialize;
use std::fmt::Debug;

#[macro_use]
extern crate savefile_derive;

fn events() {

}

fn homework() {

}

fn reminders(path: &String) {
    let mut filepath = (&path).to_string();

    let choices = vec!["assignments", "tests"];
    let reminders_option = dialoguer::Select::new().with_prompt("What would you like to do?").items(&choices).default(0).interact().unwrap();

    let reminders_option = choices[reminders_option].to_string();

    if reminders_option == "assignments" {
        filepath.push_str("/assignments");

        let assignments = std::fs::read_dir(&filepath).unwrap();

        println!("finding assignments...");
        for path in assignments {
            let path_name = path.unwrap().path().display().to_string();
            let assignment = assignments::load_assignment(&path_name);

            println!("");
            println!("assignment {} is due on {}", assignment.name, assignment.due_date);
            println!("");
        }
    }
    else if reminders_option == "tests" {
        filepath.push_str("/tests");

        let tests = std::fs::read_dir(&filepath).unwrap();

        println!("finding tests...");
        for path in tests {
            let path_name = path.unwrap().path().display().to_string();
            let test = tests::load_test(&path_name);

            println!("");
            println!("test {} is on {}", test.name, test.date);
            println!("");
        }
    }
}

#[warn(non_snake_case)]
fn option(path: &String) {
    let choices = vec!["assignments", "tests", "events", "homework", "reminders"];
    let option_choice = dialoguer::Select::new().with_prompt("What would you like to do?").items(&choices).default(0).interact().unwrap();

    let option_choice = choices[option_choice].to_string();

    if option_choice == "assignments" {
        assignments::assignments(path);
    }
    else if option_choice == "tests" {
        tests::tests(path);
    }
    else if option_choice == "events" {
        events();
    }
    else if option_choice == "homework" {
        homework();
    }
    else if option_choice == "reminders" {
        reminders(path);
    }

    let done : String = dialoguer::Input::new().with_prompt("Is there anything else you need? (y/n) ").interact_text().unwrap();

    if done == "y" {
        option(path);
    }
    else {
        println!("Thanks for using helper! Goodbye!");
    }
}

#[warn(non_snake_case)]
fn main() {
    println!("Welcome to Helper CLI! ");

    let mut path = std::env::current_dir().unwrap().display().to_string();

    path.push_str("/helper-files");

    match std::fs::read_dir(&path) {
        Ok(_dir) => { println!("Directory found!") }
        Err(_err) => { std::fs::create_dir(&path).expect("could not create directory"); }
    }

    option(&path);
}
