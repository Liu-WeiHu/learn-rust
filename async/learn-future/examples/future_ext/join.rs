use super::simple_future::{SimpleFuture, Poll};

/// 一个基本的 `future`，它将同时运行其他两个 `future` 直到完成.
///
/// 并发特性是通过对每个 `future` 的轮询交错调用来实现的，
/// 从而允许每个 `future` 以自己的速度前进.
pub struct Join<FutureA, FutureB> {
    // 每个字段可能包含应该运行完成的 `future`.
    // 如果 `future` 运行完成，则将该字段设置为 `None`.
    // 这可以防止我们在运行完成之后再次对 `future` 轮询，
    // 这将不符合 `future` trait 的规范.
    a: Option<FutureA>,
    b: Option<FutureB>,
}

impl <FutureA, FutureB> SimpleFuture for Join<FutureA, FutureB>
where
    FutureA: SimpleFuture<Output = ()>,
    FutureB: SimpleFuture<Output = ()>,
{
    type Output = ();

    fn poll(&mut self, wake: fn()) -> Poll<Self::Output> {
        //尝试运行完成这个 futureA
        if let Some(a) = &mut self.a {
            if let Poll::Ready(()) = a.poll(wake) {
                self.a.take();
            }
        }

        //尝试运行完成这个 futureB
        if let Some(b) = &mut self.b {
            if let Poll::Ready(()) = b.poll(wake) {
                self.b.take();
            }
        }

        if self.a.is_none() && self.b.is_none() {
            //表示两个 future 都已经运行完成， 我们可以成功回调
            Poll::Ready(())
        } else {
            //一个或者两个future 都返回了 poll::pending 说明仍需要做其他工作.
            //当有新的进度时, 他们将调用 wake().
            Poll::Pending
        }
    }
}