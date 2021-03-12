use std::fs::File;
use std::io::prelude::*;
use std::error::Error;

pub fn do_something(filename: &str) -> Result<i32,Box<dyn Error>> {
    File::open(filename)
        .map_err(|e| e.into())
        .and_then(|mut f| {
            let mut s = String::new();
            f.read_to_string(&mut s)
                .map_err(|e| e.into())
                .map(|_| s)
        })
        .and_then(|s| {
            let mut sum = 0;
            for c in s.lines() {
                match c.parse::<i32>() {
                    Ok(n) => {sum += n},
                    Err(e) => {return Err(e.into())},
                }
            }
            Ok(sum)
        })
}