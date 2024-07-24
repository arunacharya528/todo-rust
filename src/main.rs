pub mod Todo;

use dialoguer::{Input, Select};
use rusqlite::{Connection, Result};
use Todo::controller::create_todo;
use Todo::controller::delete_todo;
use Todo::controller::edit_todo;
use Todo::controller::list_todos;
use Todo::controller::toggle_completion_of_todo;
use Todo::migration::create_todo_table_if_not_exists;

fn main() -> Result<()> {
    let connection = Connection::open("./db.db3")?;

    match create_todo_table_if_not_exists(&connection) {
        Ok(_) => unimplemented!(),
        Err(err) => println!("Error occurred while migrating: {}", err),
    };

    loop {
        list_todos(&connection);

        let tier_1_selection = Select::new()
            .with_prompt("Please select an operation: ")
            .items(&["create", "select", "exit"])
            .interact()
            .unwrap();

        match tier_1_selection {
            0 => create_todo(&connection),
            1 => {
                let selected_number: String = Input::new()
                    .with_prompt("Please enter ID. of selection")
                    .interact_text()
                    .unwrap();

                let selected_id: i64 = selected_number.parse().unwrap();

                let tier_2_selection: usize = Select::new()
                    .with_prompt("Please select an operation: ")
                    .items(&["edit", "toggle completion", "delete"])
                    .interact()
                    .unwrap();

                match tier_2_selection {
                    0 => edit_todo(&connection, selected_id),
                    1 => toggle_completion_of_todo(&connection, selected_id),
                    2 => delete_todo(&connection, selected_id),
                    _ => println!("Try Again"),
                };
            }
            2 => break,
            _ => print!("Try Again"),
        };
    }

    return Ok(());
}
