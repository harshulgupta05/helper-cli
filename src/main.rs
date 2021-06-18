mod assignments;

use serde::Serialize;
use serde::Deserialize;
use std::fmt::Debug;

#[macro_use]
extern crate savefile_derive;

#[derive(Debug, Serialize, Deserialize, Savefile)]
struct Test {
    name: String,
    date: String,
    description: String,
    mark: u8,
}

fn save_test(test: &Test, file: &String) {
    savefile::prelude::save_file(file, 0, test).unwrap();
}

fn load_test(file: &String) -> Test {
    return savefile::prelude::load_file(file, 0).unwrap();
}

fn tests(path: &String) {
    let choices = vec!["create", "view", "edit", "delete"];
    let tests_choice = dialoguer::Select::new().with_prompt("What would you like to do?").items(&choices).default(0).interact().unwrap();

    let tests_choice = choices[tests_choice].to_string();

    if tests_choice == "create" {
        let mut filepath = (&path).to_string();

        filepath.push_str("/tests");

        match std::fs::read_dir(&filepath) {
            Ok(_dir) => { }
            Err(_err) => { match std::fs::create_dir(&filepath) {
                Ok(_dir) => { }
                Err(_err) => { println!("could not create 'tests' directory") }
            } }
        }

        let test_name : String = dialoguer::Input::new().with_prompt("Enter the name of the test").interact().unwrap();
        let test_date : String = dialoguer::Input::new().with_prompt("Enter the date of the test (YYYY-MM-DD)").interact().unwrap();
        let test_description : String = dialoguer::Input::new().with_prompt("Enter test description/lessons").interact().unwrap();
        let test_mark : String = dialoguer::Input::new().with_prompt("Enter the test mark (enter 0 if unmarked)").interact().unwrap();

        let test_mark : u8 = test_mark.trim().parse().unwrap();

        let test = Test {
            name: test_name,
            date: test_date,
            description: test_description,
            mark: test_mark,
        };

        filepath.push_str("/");
        filepath.push_str(test.name.as_str());
        filepath.push_str(".bin");

        println!("creating test....");
        std::fs::File::create(&filepath).expect("could not create file");

        println!("");
        println!("the following info will be saved:");
        println!("name: {}", test.name);
        println!("date: {}", test.date);
        println!("description: {}", test.description);
        println!("mark: {}", test.mark);
        println!("");

        save_test(&test, &filepath);
        println!("test created!");
    }
    else if tests_choice == "view" {
        let mut filepath = (&path).to_string();

        filepath.push_str("/tests");

        match std::fs::read_dir(&filepath) {
            Ok(_dir) => { }
            Err(_err) => { match std::fs::create_dir(&filepath) {
                Ok(_dir) => { }
                Err(_err) => { println!("could not create 'tests' directory") }
            } }
        }

        let tests = std::fs::read_dir(&filepath).unwrap();

        println!("finding tests...");
        for path in tests {
            let path_name = path.unwrap().path().display().to_string();
            let test_option = load_test(&path_name);

            println!("{}", test_option.name);
        }

        let test_name : String = dialoguer::Input::new().with_prompt("enter the name of the test you wish to view").interact().unwrap();

        filepath.push_str("/");
        filepath.push_str(test_name.as_str());
        filepath.push_str(".bin");

        println!("loading test...");
        let test = load_test(&filepath);

        println!("");
        println!("name: {}", test.name);
        println!("date: {}", test.date);
        println!("description: {}", test.description);
        if test.mark == 0 {
            println!("mark: unmarked");
        }
        else {
            println!("mark: {}", test.mark);
        }
        println!("");
    }
    else if tests_choice == "edit" {
        let mut filepath = (&path).to_string();

        filepath.push_str("/tests");

        match std::fs::read_dir(&filepath) {
            Ok(_dir) => { }
            Err(_err) => { match std::fs::create_dir(&filepath) {
                Ok(_dir) => { }
                Err(_err) => { println!("could not create 'tests' directory") }
            } }
        }

        let tests = std::fs::read_dir(&filepath).unwrap();

        println!("finding tests...");
        for path in tests {
            let path_name = path.unwrap().path().display().to_string();
            let test_option = load_test(&path_name);

            println!("{}", test_option.name);
        }

        let test_toedit : String = dialoguer::Input::new().with_prompt("enter the name of the test you wish to edit").interact().unwrap();

        println!("finding test...");

        let mut filepath_toedit = filepath.clone();

        filepath_toedit.push_str("/");
        filepath_toedit.push_str(test_toedit.as_str());
        filepath_toedit.push_str(".bin");

        let mut test = load_test(&filepath_toedit);
        println!("test found!");

        let test_mark_toedit : String = test.mark.to_string();

        let test_name : String = dialoguer::Input::new().with_prompt("Name").with_initial_text(test.name.as_str()).interact().unwrap();
        let test_date : String = dialoguer::Input::new().with_prompt("Date").with_initial_text(test.date.as_str()).interact().unwrap();
        let test_description : String = dialoguer::Input::new().with_prompt("Description").with_initial_text(test.description.as_str()).interact().unwrap();
        let test_mark_edited : String = dialoguer::Input::new().with_prompt("Mark").with_initial_text(test_mark_toedit.as_str()).interact().unwrap();

        test.name = test_name;
        test.date = test_date;
        test.description = test_description;
        test.mark = test_mark_edited.parse().unwrap();

        println!("");
        println!("The following will be saved:");
        println!("Name: {}", test.name);
        println!("Date: {}", test.date);
        println!("Description: {}", test.description);
        println!("Mark: {}", test.mark);
        println!("");

        let mut filepath_edited = filepath.clone();
        
        filepath_edited.push_str("/");
        filepath_edited.push_str(test.name.as_str());
        filepath_edited.push_str(".bin");

        std::fs::remove_file(&filepath_toedit).expect("could not delete test");
        save_test(&test, &filepath_edited);
        println!("test edited!");
    }
    else if tests_choice == "delete" {
        let mut filepath = (&path).to_string();

        filepath.push_str("/tests");

        match std::fs::read_dir(&filepath) {
            Ok(_dir) => { }
            Err(_err) => { match std::fs::create_dir(&filepath) {
                Ok(_dir) => { }
                Err(_err) => { println!("could not create 'tests' directory") }
            } }
        }

        let tests = std::fs::read_dir(&filepath).unwrap();

        println!("finding tests...");
        for path in tests {
            let path_name = path.unwrap().path().display().to_string();
            let test_option = load_test(&path_name);

            println!("{}", test_option.name);
        }

        let test_delete : String = dialoguer::Input::new().with_prompt("enter the name of the test you wish to delete").interact().unwrap();

        filepath.push_str("/");
        filepath.push_str(test_delete.as_str());
        filepath.push_str(".bin");

        println!("deleting test...");

        std::fs::remove_file(&filepath).expect("test could not be removed.");
        println!("test deleted!");
    }
}

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
            let test = load_test(&path_name);

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
        tests(path);
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
