use std::future::Future;
use std::marker::PhantomPinned;
use std::pin::Pin;

fn main() {
    //futures::executor::block_on(blocks());
    let mut test1 = Test::new("test1");;
    let mut test2 = Test::new("test2");

    println!("a: {},b: {}", test1.as_ref().get_a(), test2.as_ref().get_b());
    println!("a: {},b: {}", test2.as_ref().get_a(), test2.as_ref().get_b());
}

async fn blocks() {
    let s = "foo".to_string();

    let future_one = async {
        println!("{}", s);
    };

    let future_two = async {
        println!("{}", s);
    };

    let ((),()) = futures::join!(future_one, future_two);
}

#[derive(Debug)]
struct Test {
    a: String,
    b: *const String,
    _marker: PhantomPinned,
}

impl Test {
    fn new(txt: &str) -> Pin<Box<Self>> {
        let t = Test {
            a: txt.to_string(),
            b: std::ptr::null(),
            _marker: PhantomPinned,
        };
        let mut boxed = Box::pin(t);
        let self_ptr: *const String = &boxed.as_ref().a;
        unsafe{boxed.as_mut().get_unchecked_mut().b = self_ptr};
        boxed
    }

    fn get_a<'a>(self: Pin<&'a Self>) -> &'a str {
        &self.get_ref().a
    }

    fn get_b<'a>(self: Pin<&'a Self>) -> &'a String {
        unsafe {&*self.b}
    }
}
