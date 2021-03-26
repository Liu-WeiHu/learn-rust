pub trait SimpleFuture {
    type Output;
    fn poll(&mut self, wake: fn()) -> Poll<Self::Output>;
}

pub enum Poll<T> {
    Ready(T),
    Pending,
}

pub struct Socket;

impl Socket {
    pub fn has_data_to_read(&self) -> bool {
        println!("有数据");
        true
    }

    pub fn set_readable_callback(&self, wake: fn()) {
        println!("数据来临,并通知唤醒线程");
        wake();
    }

    pub fn read_buf(&mut self) -> Vec<u8> {
        println!("读取数据到缓冲区");
        vec![0u8;1]
    }
}

pub struct SocketRead<'a> {
    socket: &'a mut Socket,
}

impl SimpleFuture for SocketRead<'_> {
    type Output = Vec<u8>;

    fn poll(&mut self, wake: fn()) -> Poll<Self::Output> {
        if self.socket.has_data_to_read() {
            // `socket` 有数据的时候将其读取并放置在缓冲区并返回.
            Poll::Ready(self.socket.read_buf())
        } else {
            // `socket` 还没有数据.
            //
            // 当数据来到，将调用 `wake`.
            // 这个 `future` 的调用者将知道何时调用 `poll` 并接收数据.
            self.socket.set_readable_callback(wake);
            Poll::Pending
        }
    }
}
