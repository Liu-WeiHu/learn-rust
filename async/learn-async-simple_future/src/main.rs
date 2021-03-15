fn main() {
    println!("Hello, world!");
}

trait SimpleFuture {
    type Output;
    fn poll(&mut self, wake: fn()) -> Poll<Self::Output>;
}

enum Poll<T> {
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
    socket: &'a Socket,
}

impl SimpleFuture for SocketRead<'_> {
    type Output = Vec<u8>;

    fn poll(&mut self, wake: fn()) -> Poll<Self::Output> {
        if self.socket.has_data_to_read() {
            Poll::Ready(self.socket.read_buf())
        } else {
            self.socket.set_readable_callback(wake);
            Poll::Pending
        }
    }
}