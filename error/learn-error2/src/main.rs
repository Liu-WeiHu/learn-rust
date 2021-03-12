use thiserror::Error;

#[derive(Error, Debug)]
enum MyError {
    #[error()]
    NotNil(),
    ParseErr(),
    GtLenErr(),
}


fn main() {
    println!("Hello, world!");
}
