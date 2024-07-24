use rusqlite::Connection;

pub fn create_todo_table_if_not_exists(connection: &Connection) -> Result<usize, rusqlite::Error> {
    connection.execute(
        "
        CREATE TABLE IF NOT EXISTS todos (
            id INTEGER PRIMARY KEY,
            title VARCHAR(255),
            description TEXT,
            last_modified_at DATETIME,
            is_completed BOOLEAN
        );
        ",
        (),
    )
}
