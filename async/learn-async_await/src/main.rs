use std::future::Future;

fn main() {
    futures::executor::block_on(blocks());
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
}

impl Test {
    fn new(txt: &str) -> Self {
        Test {
            a: txt.to_string(),
            b: std::ptr::null(),
        }
    }

    fn init(&mut self) {
        let self_ref: *const String = &self.a;
        self.b = self_ref;
    }

    fn get_a(&self) -> &str {
        &self.a
    }

    fn get_b(&self) -> &String {
        unsafe {&self.b}
    }
}
