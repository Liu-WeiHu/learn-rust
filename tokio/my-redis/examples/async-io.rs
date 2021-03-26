use tokio::fs::File;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> io::Result<()> {
    //example1
    // let mut buffer = [0;10];
    // File::open("foo.txt").await?.read(&mut buffer[..]).await?;
    // println!("{}", std::str::from_utf8(&buffer[..n]).unwrap());

    //example2
    // let mut buffer = Vec::new();
    // File::open("foo.txt").await?.read_to_end(&mut buffer).await?;
    // println!("{}", std::str::from_utf8(&buffer).unwrap());

    //example3
    // File::create("foo2.txt").await?.write(r#"
    //     锄禾日当午，
    //     汗滴妹屁股。
    // "#.as_bytes()).await?;

    //example4
    //File::create("foo2.txt").await?.write_all(b"some bytes").await?;

    //example5
    // let mut reader: &[u8] = b"hello";
    // let mut file = File::create("foo3.txt").await?;
    // io::copy(&mut reader, &mut file).await?;

    //example6
    // let mut listener = TcpListener::bind("127.0.0.1:6142").await.unwrap();
    // loop {
    //     let (mut socket, _) = listener.accept().await?;
    //     tokio::spawn(async move {
    //         let (mut read, mut write) = socket.split();
    //
    //         if io::copy(&mut read, &mut write).await.is_err() {
    //             eprintln!("failed to copy");
    //         }
    //     })
    // }

    //example7
    // let mut listener = TcpListener::bind("127.0.0.1:6142").await.unwrap();
    // loop {
    //     let (mut socket, _) = listener.accept().await?;
    //     tokio::spawn(async move {
    //
    //         let mut buf = vec![0;1024];
    //         loop {
    //             match socket.read(&mut buf).await {
    //                 Ok(0) => return,
    //                 Ok(n) => {
    //                     if socket.write_all(&buf[..n]).await.is_err() {
    //                         return;
    //                     }
    //                 },
    //                 Err(_) => return,
    //             }
    //         }
    //     })
    // }

    Ok(())
}
