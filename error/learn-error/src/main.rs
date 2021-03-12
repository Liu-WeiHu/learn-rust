//mod origin;
//mod refact1;
//mod refact2;
//mod refact3;
//mod refact4;

use std::fs::File;
use std::io::Read;
use std::{io, num};
use thiserror::Error;

#[derive(Error, Debug)]
enum MyError {
    #[error("IO 异常拉 {0}")]
    Io(#[from] io::Error),
    #[error("解析错误啦")]
    Parse(#[from] num::ParseIntError),
}

fn main(){
    /*let s = refact4::do_something("./sum.txt");
    match s {
        Ok(n) => {println!("{}", n)},
        Err(e) => {println!("{:?}", e)},
    }*/

    let s = aa();
    match s {
        Ok(n) => {println!("{}", n)},
        Err(e) => {println!("{}", e.to_string())},
    }
}

fn aa() -> Result<i32, MyError>  {
    let mut f = File::open("su1m.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    let mut sum = 0;
    for c in s.lines() {
        sum += c.parse::<i32>()?;
    }
    println!("{}", sum);

    Ok(sum)
}

/*mod tests {
    use thiserror::Error;
    use std::{io, num};
    use std::fs::File;
    use std::io::Read;

    #[derive(Error, Debug)]
    enum MyError {
        #[error("IO 异常拉 {0}")]
        Io(#[from] io::Error),
        #[error("类型解析失败 {0}")]
        Parse(#[from] num::ParseIntError)
    }

    #[test]
    fn test_1() {
        let mut f = File::open("sum.txt")?;
        let mut s = String::new();
        f.read_to_string(&mut s)?;
        let mut sum = 0;
        for c in s.lines() {
            sum += c.parse::<i32>()?;
        }
        println!("{}", sum);
    }
}*/
