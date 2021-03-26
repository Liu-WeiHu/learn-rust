use tokio::sync::oneshot;

#[tokio::main]
async fn main() {
    /*let (tx1, rx1) = oneshot::channel();
    let (tx2, rx2) = oneshot::channel();

    tokio::spawn(async {
        let _ = tx1.send("one");
    });

    tokio::spawn(async {
        let _ = tx2.send("two");
    });

    tokio::select! {
        val = rx1 => {
            println!("rx1 completed first with {:?}", val);
        }
        val = rx2 => {
            println!("rx2 completed first with {:?}", val);
        }
    }*/

    /*let (tx, rx) = oneshot::channel();
     tokio::spawn(async move {
         tx.send("hello").unwrap();
     });*/

    tokio::select! {
        socket = tokio::net::TcpStream::connect("127.0.0.1:3465") => {
            println!("Socket connected {:?}", socket);
        }
        /*msg = rx => {
            println!("received message first {:?}", msg);
        }*/
    }
}
