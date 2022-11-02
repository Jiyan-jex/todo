use clap::Command;
use rusqlite::Result;
use std::process;

mod cli;
mod database;
mod util;

#[derive(Debug)]
struct Todo {
    database: database::Database,
}

impl Todo {
    // Initialize Todo
    fn new() -> Self {
        let database = match database::Database::new() {
            Ok(database) => database,
            Err(e) => panic!("{}", e),
        };
        Todo { database: database }
    }

    fn run(&self, commands: Command<'static>) -> Result<(), util::Error> {
        match commands.get_matches().subcommand().unwrap() {
            (cli::ADD, s) => {
                let task = s.value_of("TASK").unwrap().to_string();
                //conn.add

                match self.database.add(task) {
                    Ok(_) => println!("added successfully"),
                    Err(e) => eprintln!("failed to add a task: {}", e),
                }
            }
            (cli::DONE, i) => {
                let i = util::parse_index(i)?;
                let mut stmt = self.database.done(i)?;

                let _done = stmt.query_row(&[(":i", i.to_string().as_str())], |row| {
                    Ok(row.get::<_, i32>(0))
                });
            }

            (cli::DELETE, i) => {
                let i = util::parse_index(i)?;
                let mut stmt = self.database.delete(i)?;

                let _delete = stmt.query_row(&[(":i", i.to_string().as_str())], |row| {
                    Ok(row.get::<_, i32>(0))
                });
            }

            (cli::LIST, _) => {
                let mut stmt = self.database.list().unwrap_or_else(|e| {
                    eprint!("failed to show todo list: {}", e);
                    process::exit(1);
                });
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
            }
            _ => unreachable!(),
        };
        Ok(())
    }
}

fn main() {
    let commands = cli::build_args();

    let todo = Todo::new();
    match todo.run(commands) {
        Ok(()) => (),
        Err(e) => panic!("{:?}", e),
    };
}
