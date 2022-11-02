use clap::{arg, Command};

pub(crate) const ADD: &str = "add";
pub(crate) const LIST: &str = "list";
pub(crate) const DONE: &str = "done";
pub(crate) const DELETE: &str = "delete";

pub fn build_args() -> Command<'static> {
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
