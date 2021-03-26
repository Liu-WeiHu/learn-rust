use std::pin::Pin;
use futures::task::{Context, Poll};

pub trait Future {
    type Output;
    fn poll(
        //注意这个 &mut self 到 pin<&mut Self> 的变化;
        self: Pin<&mut Self>,
        //以及葱 wake: fn() 到 cx: &mut Context<'_> 的变化;
        cx: &mut Context<'_>,
    ) -> Poll<Self::Output>;
}