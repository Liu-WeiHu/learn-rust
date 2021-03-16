use crate::simple_future::{SimpleFuture, Poll};

/// 这是一个 `SimpleFuture`，依次运行直到两个 `future` 都完成.
//
// 提示: 这只是一个简单的示例, `AndThenFut` 是假设两个 `future` 在创建的时候都可用.
// 真正的 `AndThen` 允许基于第一个 `future` 输出并创建第二个 `future`, 比
// 如 `get_breakfast.and_then(|food| eat(food))`.
pub struct AndThenFut<FutureA, FutureB> {
    first: Option<FutureA>,
    second: Option<FutureB>,
}

impl<FutureA, FutureB> SimpleFuture for AndThenFut<FutureA, FutureB>
where
    FutureA: SimpleFuture<Output = ()>,
    FutureB: SimpleFuture<Output = ()>,
{
    type Output = ();

    fn poll(&mut self, wake: fn()) -> Poll<Self::Output> {
        if let Some(first) = &mut self.first {
            match first.poll(wake) {
                //我们已经完成了第一个 future
                //移除它, 开始第二个
                Poll::Ready(()) => self.first.take(),
                //我们没有完成第一个 future
                Poll::Pending => return Poll::Pending,
            };
        }
        // 现在，第一个 `future` 已经完成，
        // 那么就尝试完成第二个.
        self.second.poll(wake)
    }
}
