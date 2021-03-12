use std::fs::File;
use std::io::prelude::*;
use std::fmt;
use std::error::Error;
use std::num::ParseIntError;

#[derive(Debug)]
pub enum MyError {
    Io(std::io::Error),
    Parse(std::num::ParseIntError),
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MyError::Io(e) => write!(f, "IO ERROR: {}", e),
            MyError::Parse(e) => write!(f, "PARSEINT ERROR: {}", e),
        }
    }
}

impl Error for MyError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            MyError::Io(e) => Some(e),
            MyError::Parse(e) => Some(e),
        }
    }
}

impl From<std::io::Error> for MyError {
    fn from(err: std::io::Error) -> Self {
        MyError::Io(err)
    }
}

impl From<ParseIntError> for MyError {
    fn from(err: ParseIntError) -> Self {
        MyError::Parse(err)
    }
}

pub fn do_something(filename: &str) -> Result<i32, MyError> {
    let mut file = File::open(filename)?;
    let mut s = String::new();
    file.read_to_string(&mut s)?;
    let mut sum= 0;
    for c in s.lines() {
        let n = c.parse::<i32>()?;
        sum += n;
    }
    Ok(sum)
}