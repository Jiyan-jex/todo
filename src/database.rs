use rusqlite::{Connection, OpenFlags, Result, Statement};
use std::env;

const DATABASE_NAME: &str = "data.db";

#[derive(Debug)]
pub struct Database {
    pub connection: Connection,
    pub path: String,
}

impl Database {
    // Initialize Database
    pub fn new() -> Result<Self> {
        let mut path = match env::var("HOME") {
            Ok(val) => val,
            Err(_) => ".".to_string(),
        };
        path += format!("/{}", DATABASE_NAME).as_str();

        let connection = Connection::open_with_flags(
            &path,
            OpenFlags::SQLITE_OPEN_READ_WRITE | OpenFlags::SQLITE_OPEN_CREATE,
        )?;

        connection.execute(
            "CREATE TABLE if NOT exists todo(
           id   INTEGER PRIMARY KEY,
           task TEXT NOT NULL,
           done BOOLEAN
       )",
            (), // empty list of parameters.
        )?;

        Ok(Self {
            connection: connection,
            path: path,
        })
    }

    pub fn add(&self, task: String) -> Result<usize> {
        self.connection.execute(
            "INSERT INTO todo (task, done) VALUES (?1, ?2)",
            (&task, false),
        )
    }

    pub fn list(&self) -> Result<Statement> {
        let stmt = self.connection.prepare("SELECT * FROM todo")?;
        Ok(stmt)
    }

    pub fn done(&self, _i: u8) -> Result<Statement> {
        let stmt = self
            .connection
            .prepare("UPDATE todo SET done=true WHERE id=:i;")?;

        Ok(stmt)
    }

    pub fn delete(&self, _i: u8) -> Result<Statement> {
        let stmt = self.connection.prepare("DELETE FROM todo WHERE id=:i;")?;
        Ok(stmt)
    }
}
