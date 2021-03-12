#![allow(unused)]
#![feature(try_blocks)]

use std::num::ParseIntError;

fn main() {

    let result: Result<i32, ParseIntError> = try {
        "1".parse::<i32>()?
        + "2".parse::<i32>()?
        + "3".parse::<i32>()?
    };

    assert_eq!(result, Ok(6));

    let result: Result<i32, Box<dyn std::error::Error>> = try {
        "1".parse::<i32>()?
        + "f".parse::<i32>()?
        + "3".parse::<i32>()?
    };

    match result {
        Err(e) => {println!("err: {:?}", e)}
        _ => ()
    }
}
