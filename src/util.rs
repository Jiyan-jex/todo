use clap::ArgMatches;
use rusqlite::Result;
use std::num::ParseIntError;

#[derive(Debug)]
pub enum Error {
    Message(String),
}

impl From<ParseIntError> for Error {
    fn from(error: ParseIntError) -> Self {
        Error::Message(format!(
            "failed, <INDEX> should be integer: {}",
            error.to_string()
        ))
    }
}

impl From<rusqlite::Error> for Error {
    fn from(error: rusqlite::Error) -> Self {
        Error::Message(format!("{}", error.to_string()))
    }
}

pub fn parse_index(arg: &ArgMatches) -> Result<u8, Error> {
    Ok(arg.value_of("INDEX").unwrap().parse::<u8>()?)
}
