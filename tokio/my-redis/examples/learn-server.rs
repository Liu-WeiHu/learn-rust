use tokio::net::TcpListener;
use tokio::sync::oneshot;
use std::io;

#[tokio::main]
async fn main() -> io::Result<()> {
    let (tx, rx) = oneshot::channel();
    tokio::spawn(async move {
        tx.send("aa").unwrap();
    });
    let mut listener = TcpListener::bind("localhost:3465").await?;
    tokio::select! {
        _ = async {
            println!("aaa");
            let (socket, _) = listener.accept().await?;
            println!("bbb");
            Ok::<_, io::Error>(())
        } => {}
        _ = rx => {
            println!("ccc");
        }
    }
    Ok(())
}
