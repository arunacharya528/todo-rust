use chrono::Utc;
use console::style;
use dialoguer::Select;
use dialoguer::{Editor, Input};
use rusqlite::Connection;

use super::model::{
    delete_todo_record_by_id, fetch_all_todos, fetch_todo_by_id, insert_in_todos_table,
    update_todo, Todo,
};

pub fn list_todos(connection: &Connection) {
    match fetch_all_todos(&connection) {
        Ok(todos) => {
            for (index, todo) in todos.iter().enumerate() {
                show_todo(todo);
            }
        }
        Err(err) => println!("SQL ERROR: {}", err),
    };
}

pub fn create_todo(connection: &Connection) {
    let title: String = Input::new()
        .with_prompt("Please enter title of your note")
        .interact_text()
        .unwrap();

    let description: String = Input::new()
        .with_prompt("Please enter description of your note")
        .interact_text()
        .unwrap();

    let newTodo = Todo {
        id: None,
        title: title,
        description: description,
        last_modified_at: Utc::now(),
        is_completed: false,
    };

    insert_in_todos_table(&connection, &newTodo);
}

fn show_todo(todo: &Todo) {
    println!("ID: {}", style(&todo.id.unwrap()).yellow());
    println!("{}", style(&todo.title).cyan());
    println!("{}", style(&todo.description).blue());
    println!("{}", style(&todo.last_modified_at).green());
    if todo.is_completed {
        println!("{}", style("Completed").red());
    }
    println!("");
}

pub fn edit_todo(connection: &Connection, id: i64) {
    match fetch_todo_by_id(&connection, id) {
        Ok(mut todo) => {
            if let Some(rv) = Editor::new().edit(todo.title.as_str()).unwrap() {
                println!("Edit title");
                println!("{}", rv);
                todo.title = rv;
            } else {
                println!("Abort!");
            }

            if let Some(rv) = Editor::new().edit(&todo.description.as_str()).unwrap() {
                println!("Edit description");
                println!("{}", rv);
                todo.description = rv;
            } else {
                println!("Abort!");
            }

            todo.last_modified_at = Utc::now();

            match update_todo(connection, &todo) {
                Ok(_) => println!("Successfully updated record"),
                Err(err) => println!("Error occured: {}", err),
            }
        }

        Err(err) => println!("Error occurred: {}", err),
    };
}

pub fn delete_todo(connection: &Connection, id: i64) {
    let confirmation: usize = Select::new()
        .with_prompt("Are you sure you want to do this?")
        .items(&["yes", "no"])
        .interact()
        .unwrap();

    match confirmation {
        0 => match delete_todo_record_by_id(connection, id) {
            Ok(_) => print!("Successfully deleted"),
            Err(_) => print!("Error occurred while deleting"),
        },
        1 => print!("Deletion aborted"),
        _ => println!("Try Again"),
    }
}

pub fn toggle_completion_of_todo(connection: &Connection, id: i64) {
    match fetch_todo_by_id(connection, id) {
        Ok(todo) => {
            match update_todo(
                connection,
                &Todo {
                    is_completed: !todo.is_completed,
                    ..todo
                },
            ) {
                Ok(_) => println!("Successfully toggled completion status"),
                Err(err) => println!("Error occured: {}", err),
            }
        }
        Err(err) => println!("Error occured {}", err),
    };
}
