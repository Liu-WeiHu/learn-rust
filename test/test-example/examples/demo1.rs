use std::fs::File;
use std::io::Read;
use anyhow::Context;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut s = String::new();
    File::open("aa.txt")
        .context("找不到 aa.txt 这个文件")?
        .read_to_string(&mut s)
        .expect("读取到字符串失败");
    println!("{}", s);
    Ok(())
}