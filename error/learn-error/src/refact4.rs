use std::fs::File;
use std::io::prelude::*;
use std::{io, num};
use thiserror::Error;
use anyhow::{Result, Context};

#[derive(Error, Debug)]
pub enum MyError {
    #[error("{0}")]
    Io(#[from] io::Error),
    #[error("{0}")]
    Parse(#[from] num::ParseIntError),
}


pub fn do_something(filename: &str) -> Result<i32, anyhow::Error> {
    let mut file = File::open(filename).context(format!("unable to open {}", filename))?;
    let mut s = String::new();
    file.read_to_string(&mut s)?;
    let mut sum = 0;
    for c in s.lines() {
        let n = c.parse::<i32>()?;
        sum += n;
    }
    Ok(sum)
}