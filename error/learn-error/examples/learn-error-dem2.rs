use thiserror::Error;
use std::str::Chars;
use std::error::Error;

#[derive(Error, Debug)]
enum MyError<'a> {
    #[error("{0} 不能为空")]
    NotNil(&'a str),
    #[error("解析错误: {0}")]
    ParseErr(#[from] std::num::ParseIntError),
    #[error("长度不能大于等于 {0}")]
    GtLenErr(i32),
}

fn main() {
    let id = 0;
    let i = "f";
    let str = "张明s明1啊啊".to_string();
    match t_id(id) {
        Err(e) => println!("{}", e.to_string()),
        Ok(d) => println!("{}", d),
    }
}

fn t_id(id: i32) -> Result<i32,Box<dyn Error>> {
    if id <= 0 {
        return Err(MyError::NotNil("ID").into());
    }
    Ok(id)
}
