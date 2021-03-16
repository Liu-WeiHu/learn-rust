use futures::executor::block_on;
use std::thread;
use std::time::Duration;

fn main() {
    /*let song = block_on(learn_song());
    block_on(sing_song(song));
    block_on(dance());*/
    block_on(async_main());
}

#[derive(Debug)]
struct Song;

async fn hello_world() {
    println!("hello world");
}

async fn learn_song() ->  Song {
    println!("learn__song");
    Song
}

async fn sing_song(song: Song) {
    println!("sing song {:?}", song)
}

async fn dance() {
    println!("dance")
}

async fn learn_and_sing() {
    let song = learn_song().await;
    sing_song(song).await;
    thread::sleep(Duration::from_secs(5));
}

async fn async_main() {
    let f1 = learn_and_sing();
    let f2 = dance();
    futures::join!(f1, f2);
}