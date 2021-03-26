use std::fmt::Debug;
use std::any::Any;

fn log<T: Any + Debug> (value: &T) {
    let value_any = value as &dyn Any;

    match value_any.downcast_ref::<String>() {
        Some(s) => {
            println!("string : {}, len {}", s, s.len());
        }
        None => {
            println!("{:?}", value);
        }
    }
}

fn do_work<T: Any + Debug>(value: &T) {
    log(value);
}

fn main() {
    let my_string = "hello world".to_string();
    do_work(&my_string);

    let my_i8 = 100i8;
    do_work(&my_i8);
}
