use std::pin::Pin;
use std::task::{Context, Poll};
use futures::channel::mpsc;
use futures::{StreamExt, SinkExt};

fn main() {

}

trait Stream {
    //由stream 产生的值的类型
    type Item;

    ///尝试解析 stream 中的下一项
    /// 如果已经准备好, 就重新运行 poll::pending , 如果已经完成,就重新
    /// 运行 poll::ready(some(x)) 如果已经完成,就重新运行 poll::ready(none).
    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>>;
}

async fn send_recv() {
    const BUFFER_SIZE: usize = 10;
    let (mut tx, mut rx) = mpsc::channel::<i32>(BUFFER_SIZE);
    tx.send(1).await.unwrap();
    tx.send(2).await.unwrap();
    drop(tx);

    // streamext::next 类似与 iterator::next 但会返回一个实现了
    //future<output = option<T>> 的类型
    assert_eq!(Some(1), rx.next().await);
    assert_eq!(Some(2), rx.next().await);
    assert_eq!(None, rx.next().await);
}