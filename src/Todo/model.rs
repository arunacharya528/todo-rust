use crate::Todo::services::get_date_time_from_string;
use chrono::{DateTime, Utc};
use rusqlite::Connection;

pub struct Todo {
    pub id: Option<i32>,
    pub title: String,
    pub description: String,
    pub last_modified_at: DateTime<Utc>,
    pub is_completed: bool,
}

pub fn fetch_all_todos(connection: &Connection) -> Result<Vec<Todo>, rusqlite::Error> {
    let mut stmt = connection
        .prepare("SELECT id, title, description, is_completed, last_modified_at FROM todos")?;

    let todo_iter = stmt.query_map([], |row| {
        Ok(Todo {
            id: row.get(0)?,
            title: row.get(1)?,
            description: row.get(2)?,
            is_completed: row.get(3)?,
            last_modified_at: get_date_time_from_string(row.get(4))?,
        })
    })?;

    let todos: Result<Vec<Todo>, rusqlite::Error> = todo_iter.collect();
    todos
}

pub fn insert_in_todos_table(connection: &Connection, todo: &Todo) -> i64 {
    let result =  connection.execute(
        "INSERT INTO todos (title, description,last_modified_at,is_completed) VALUES (?1, ?2, ?3, ?4)",
        (&todo.title, &todo.description, &todo.last_modified_at.to_rfc3339(), &todo.is_completed),
    );

    let latestId = connection.last_insert_rowid();

    match result {
        Ok(_) => println!("Inserted at ID {}", latestId),
        Err(err) => println!("SQL Error: {}", err),
    }

    connection.last_insert_rowid()
}

pub fn fetch_todo_by_id(connection: &Connection, id: i64) -> Result<Todo, rusqlite::Error> {
    let mut stmt = connection.prepare(
        "SELECT id, title, description, is_completed, last_modified_at FROM todos WHERE id = ?1",
    )?;

    stmt.query_row([id], |row| {
        Ok(Todo {
            id: row.get(0)?,
            title: row.get(1)?,
            description: row.get(2)?,
            is_completed: row.get(3)?,
            last_modified_at: get_date_time_from_string(row.get(4))?,
        })
    })
}

pub fn update_todo(connection: &Connection, todo: &Todo) -> Result<usize, rusqlite::Error> {
    connection.execute(
        "UPDATE todos SET title = ?1, description = ?2, last_modified_at = ?3, is_completed = ?4 WHERE id = ?5",
        (
            &todo.title,
            &todo.description,
            Utc::now().to_rfc3339(),
            &todo.is_completed,
            &todo.id.unwrap(),
        ),
    )
}

pub fn delete_todo_record_by_id(
    connection: &Connection,
    id: i64,
) -> Result<usize, rusqlite::Error> {
    connection.execute("DELETE from todos WHERE id = ?1", &[&id])
}
