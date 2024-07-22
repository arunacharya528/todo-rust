use chrono::{DateTime, Utc};
use console::style;
use dialoguer::{Editor, Input, Select};

struct Todo {
    pub title: String,
    pub description: String,
    pub last_modified_at: DateTime<Utc>,
    pub is_completed: bool,
}

fn take_new_entry() -> Todo {
    let title: String = Input::new()
        .with_prompt("Please enter title of your note")
        .interact_text()
        .unwrap();

    let description: String = Input::new()
        .with_prompt("Please enter description of your note")
        .interact_text()
        .unwrap();

    return Todo {
        title: title,
        description: description,
        last_modified_at: Utc::now(),
        is_completed: false,
    };
}

fn preload_data(_todos: &mut Vec<Todo>) {
    _todos.push(Todo {
        title: String::from("Todo1"),
        description: String::from("description"),
        last_modified_at: ("2023-07-21T10:00:00+00:00")
            .parse::<DateTime<Utc>>()
            .expect("Failed to parse"),
        is_completed: false,
    });

    _todos.push(Todo {
        title: String::from("Todo2"),
        description: String::from("description"),
        last_modified_at: ("2023-07-21T10:00:00+00:00")
            .parse::<DateTime<Utc>>()
            .expect("Failed to parse"),
        is_completed: false,
    });

    _todos.push(Todo {
        title: String::from("Todo3"),
        description: String::from("description"),
        last_modified_at: ("2023-07-21T10:00:00+00:00")
            .parse::<DateTime<Utc>>()
            .expect("Failed to parse"),
        is_completed: true,
    });
}

fn list_data(todos: &Vec<Todo>) {
    for (index, todo) in todos.iter().enumerate() {
        println!("S.N. {}", (index + 1));
        println!("{}", style(&todo.title).cyan());
        println!("{}", style(&todo.description).blue());
        println!("{}", style(&todo.last_modified_at).green());
        if todo.is_completed {
            println!("{}", style("Completed").red());
        }
        println!("");
    }
}

fn edit_data(todos: &mut Vec<Todo>, index: usize) {
    if let Some(rv) = Editor::new().edit(todos[index].title.as_str()).unwrap() {
        println!("Edit title");
        println!("{}", rv);
        todos[index].title = rv;
    } else {
        println!("Abort!");
    }

    if let Some(rv) = Editor::new()
        .edit(&todos[index].description.as_str())
        .unwrap()
    {
        println!("Edit description");
        println!("{}", rv);
        todos[index].description = rv;
    } else {
        println!("Abort!");
    }

    todos[index].last_modified_at = Utc::now();
}

fn mark_completed(todos: &mut Vec<Todo>, index: usize) {
    todos[index].is_completed = true;

    println!("Successfully Marked Completed!");
}

fn delete_data(todos: &mut Vec<Todo>, index: usize) {
    let confirmation_selection_options = vec!["yes", "no"];

    let confirmation_selection = Select::new()
        .with_prompt("Are you sure you would like to do this? ")
        .items(&confirmation_selection_options)
        .interact()
        .unwrap();

    if confirmation_selection == 0 {
        todos.remove(index);

        println!("Successfully Deleted!");
    } else {
        println!("Deletion aborted");
    }
}

fn main() {
    let mut todos: Vec<Todo> = vec![];

    preload_data(&mut todos);

    loop {
        list_data(&todos);

        let tier_1_selection = Select::new()
            .with_prompt("Please select an operation: ")
            .items(&["create", "select", "exit"])
            .interact()
            .unwrap();

        match tier_1_selection {
            0 => todos.push(take_new_entry()),
            1 => {
                let selected_number: String = Input::new()
                    .with_prompt("Please enter S.N. of selection")
                    .interact_text()
                    .unwrap();

                let selected_index: usize = selected_number.parse().unwrap();

                let tier_2_selection: usize = Select::new()
                    .with_prompt("Please select an operation: ")
                    .items(&["edit", "mark completed", "delete"])
                    .interact()
                    .unwrap();

                match tier_2_selection {
                    0 => edit_data(&mut todos, selected_index - 1),
                    1 => mark_completed(&mut todos, selected_index - 1),
                    2 => delete_data(&mut todos, selected_index - 1),
                    _ => println!("Try Again"),
                };
            }
            2 => break,
            _ => print!("Try Again"),
        }
    }
}
