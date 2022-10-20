use clap::{arg, ArgMatches, Command};
use rusqlite::{Connection, OpenFlags, Result};
use std::env;
use std::num::ParseIntError;

const ADD: &str = "add";
const LIST: &str = "list";
const DONE: &str = "done";
const DELETE: &str = "delete";
const DATABASE_NAME: &str = "data.db";

enum Error {
    Message(String),
}

impl From<ParseIntError> for Error {
    fn from(error: ParseIntError) -> Self {
        Error::Message(format!("{}", error.to_string()))
    }
}

impl From<rusqlite::Error> for Error {
    fn from(error: rusqlite::Error) -> Self {
        Error::Message(format!("{}", error.to_string()))
    }
}
struct Database {
    connection: Connection,
    path: String,
}
struct Todo {
    database: Database,
}

impl Database {
    fn new() -> Result<Self> {
        
        let mut path = match env::var("HOME") {
            Ok(val) => val,
            Err(_) => ".".to_string(),
        };
        path += format!("/{}", DATABASE_NAME).as_str();

        let connection = Connection::open_with_flags(
            &path,
            OpenFlags::SQLITE_OPEN_READ_WRITE | OpenFlags::SQLITE_OPEN_CREATE,
        )?;

        connection
            .execute(
                "CREATE TABLE todo (
           id   INTEGER PRIMARY KEY,
           task TEXT NOT NULL,
           done BOOLEAN 
       )",
                (), // empty list of parameters.
            )
            .ok();

        Ok(Self {
            connection: connection,
            path: path,
        })
    }

    fn add(&self, task: String) -> Result<()> {
        self.connection.execute(
            "INSERT INTO todo (task, done) VALUES (?1, ?2)",
            (&task, false),
        )?;
        println!("create task");
        Ok(())
    }

    fn list(&self) -> Result<()> {
        let mut stmt = self.connection.prepare("SELECT * FROM todo")?;
        let todo_iter = stmt.query_map([], |row| {
            let id = row.get::<_, i32>(0)?;
            let task = row.get::<_, String>(1)?;
            let done = row.get::<_, bool>(2)?;
            Ok((id, task, done))
        })?;

        for todo in todo_iter {
            let todo = todo?;
            println!(
                "{} {}: {}",
                if todo.2 { "\u{2611}" } else { "\u{2610}" },
                todo.0,
                todo.1,
            );
        }

        Ok(())
   }

    fn done(&self, i: u8) -> Result<()> {
        let mut stmt = self
            .connection
            .prepare("UPDATE todo SET done=true WHERE id=:i;")?;
        let _done = stmt.query_row(&[(":i", i.to_string().as_str())], |row| {
            row.get::<_, i32>(0)
        })?;

        Ok(())
    }

    fn delete(&self, i: u8) -> Result<()> {
        let mut stmt = self.connection.prepare("DELETE FROM todo WHERE id=:i;")?;
        let _delete = stmt.query_row(&[(":i", i.to_string().as_str())], |row| {
            row.get::<_, i32>(0)
        })?;
        Ok(())
    }
}

impl Todo {
    fn new() -> Result<Self> {
        let database = Database::new()?;
        Ok(Self { database })
    }

    fn run(&self, commands: Command<'static>) -> Result<(), Error> {
        match commands.get_matches().subcommand().unwrap() {
            (ADD, s) => {
                let task = s.value_of("TASK").unwrap().to_string();
                self.database.add(task).ok();
            }
            (DONE, i) => {
                let id = parse_index(i)?;
                self.database.done(id).ok();
            }
            (LIST, _) => {
                self.database.list().ok();
            }
            (DELETE, i) => {
                let id = parse_index(i)?;
                self.database.delete(id).ok();
            }
            _ => unreachable!(),
        };
        Ok(())
    }
}

fn parse_index(arg: &ArgMatches) -> Result<u8, Error> {
    Ok(arg.value_of("INDEX").unwrap().parse::<u8>()?)
}

fn build_args() -> Command<'static> {
    Command::new("todo")
        .about("simple command line save in database")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new(ADD)
                .about("Add new task")
                .arg(arg!(<TASK>).required(true)),
        )
        .subcommand(
            Command::new(DONE)
                .about("done a task")
                .arg(arg!(<INDEX>).required(true)),
        )
        .subcommand(
            Command::new(DELETE)
                .about("done a task")
                .arg(arg!(<INDEX>).required(true)),
        )
        .subcommand(Command::new(LIST).about("list the tasks"))
}

fn main() -> Result<()> {
    let todo = Todo::new()?;

    let commands = build_args();

    todo.run(commands).ok();

    Ok(())
}

