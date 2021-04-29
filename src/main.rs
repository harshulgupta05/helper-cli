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
    mark: u8,
}

#[derive(Debug, Serialize, Deserialize, Savefile)]
struct Test {
    name: String,
    date: String,
    description: String,
    mark: u8,
}

fn save_assignment(assignment: &Assignment, file: &String) {
    savefile::prelude::save_file(file, 0, assignment).unwrap();
}

fn load_assignment(file: &String) -> Assignment {
    return savefile::prelude::load_file(file, 0).unwrap();
}

fn save_test(test: &Test, file: &String) {
    savefile::prelude::save_file(file, 0, test).unwrap();
}

fn load_test(file: &String) -> Test {
    return savefile::prelude::load_file(file, 0).unwrap();
}

fn assignments(path: &String) {
    let assignments_choice : String = dialoguer::Input::new().with_prompt("What would you like to do? (create/view/edit/delete)").interact_text().unwrap();

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
        let assignment_date : String = dialoguer::Input::new().with_prompt("Enter the due date (YYYY-MM-DD)").interact().unwrap();
        let assignment_complete : String = dialoguer::Input::new().with_prompt("Is the assignment done? (y/n)").interact_text().unwrap();
        let assignment_mark : String = dialoguer::Input::new().with_prompt("Enter the assignment mark (enter 0 if unmarked)").interact().unwrap();
        let assignment_mark_int : u8 = assignment_mark.trim().parse().unwrap();
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
            Ok(dir) => { println!("reading assignments..."); }
            Err(err) => { match std::fs::create_dir(&filepath) {
                Ok(dir) => { }
                Err(err) => { println!("error creating 'assignments' folder"); }
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
            Ok(dir) => { println!("reading assignments..."); }
            Err(err) => { match std::fs::create_dir(&filepath) {
                Ok(dir) => { }
                Err(err) => { println!("error creating 'assignments' folder"); }
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
        filepath.push_str("/");
        filepath.push_str(assignment_toedit.as_str());
        filepath.push_str(".bin");

        let mut assignment : Assignment = load_assignment(&filepath);
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

        save_assignment(&assignment, &filepath);
    }
    else if assignments_choice == "delete" {
        println!("finding assignments to delete...");
        let mut filepath = (&path).to_string();

        filepath.push_str("/assignments");

        match std::fs::read_dir(&filepath) {
            Ok(dir) => { println!("reading assignments..."); }
            Err(err) => { match std::fs::create_dir(&filepath) {
                Ok(dir) => { }
                Err(err) => { println!("error creating 'assignments' folder"); }
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

fn tests(path: &String) {
    let tests_choice : String = dialoguer::Input::new().with_prompt("What would you like to do? (create, view, edit, delete)").interact_text().unwrap();

    if tests_choice == "create" {
        let mut filepath = (&path).to_string();

        filepath.push_str("/tests");

        match std::fs::read_dir(&filepath) {
            Ok(dir) => { }
            Err(err) => { match std::fs::create_dir(&filepath) {
                Ok(dir) => { }
                Err(err) => { println!("could not create 'tests' directory") }
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
            Ok(dir) => { }
            Err(err) => { match std::fs::create_dir(&filepath) {
                Ok(dir) => { }
                Err(err) => { println!("could not create 'tests' directory") }
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
            Ok(dir) => { }
            Err(err) => { match std::fs::create_dir(&filepath) {
                Ok(dir) => { }
                Err(err) => { println!("could not create 'tests' directory") }
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
        filepath.push_str("/");
        filepath.push_str(test_toedit.as_str());
        filepath.push_str(".bin");

        let mut test = load_test(&filepath);
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
    }
    else if tests_choice == "delete" {
        let mut filepath = (&path).to_string();

        filepath.push_str("/tests");

        match std::fs::read_dir(&filepath) {
            Ok(dir) => { }
            Err(err) => { match std::fs::create_dir(&filepath) {
                Ok(dir) => { }
                Err(err) => { println!("could not create 'tests' directory") }
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

    let reminders_option : String = dialoguer::Input::new().with_prompt("reminders for (assignments/tests)").interact_text().unwrap();

    if reminders_option == "assignments" {
        filepath.push_str("/assignments");

        let assignments = std::fs::read_dir(&filepath).unwrap();

        println!("finding assignments...");
        for path in assignments {
            let path_name = path.unwrap().path().display().to_string();
            let assignment = load_assignment(&path_name);

            println!("assignment {} is due on {}", assignment.name, assignment.due_date);
        }
    }
    else if reminders_option == "tests" {
        filepath.push_str("/tests");

        let tests = std::fs::read_dir(&filepath).unwrap();

        println!("finding tests...");
        for path in tests {
            let path_name = path.unwrap().path().display().to_string();
            let test = load_test(&path_name);

            println!("test {} is on {}", test.name, test.date);
        }
    }
}

#[warn(non_snake_case)]
fn option(path: &String) {
    let option : String = dialoguer::Input::new().with_prompt("What would you like to do? (assignments, tests, events, homework, reminders)").interact_text().unwrap();

    if option == "assignments" {
        assignments(path);
    }
    else if option == "tests" {
        tests(path);
    }
    else if option == "events" {
        events();
    }
    else if option == "homework" {
        homework();
    }
    else if option == "reminders" {
        reminders(path);
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
