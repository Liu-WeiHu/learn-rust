#![allow(unused)]
use std::thread::sleep;
use std::time::Duration;

#[test]
fn test_hello() {
    println!("hello");
}

#[test]
fn test_instant() {
    let now = std::time::Instant::now();
    te();
    let s = now.elapsed().as_secs();
    println!("{}", s);
}

fn te() {
    sleep(Duration::from_secs(2))
}

trait Animal {
    fn run(&self);
}

struct Cat{}

impl Animal for Cat {
    fn run(&self) {
        println!("cat run")
    }
}

struct Dog {}

impl Animal for Dog {
    fn run(&self) {
        println!("dog run");
    }
}

fn res_animal(a: impl Animal) -> impl Animal {
    a.run();
    a
}

#[test]
fn test_static() {
    let mut v: Vec<Box<dyn Animal>> = Vec::new();
    let cat = Cat{};
    v.push(Box::new(cat));
    
}

struct User { name: String, age: u8, }
enum People { User(User), Teacher, }
#[test]
fn test_if_let() {
    let u = People::User(User{name: "aa".into(), age: 22});
    if let People::User(User{name,..}) = u {
        assert_eq!(name, "aa".to_string())
    }
}

#[test]
fn test_dd() {
    let mut a = 44;
    let aa = &mut a;
    *aa = 55;
    println!("{}", a);
}

#[test]
fn test_address() {
    let arr = [0;10];
    println!("[arr] as_ptr : {:?}", arr.as_ptr());
    println!("[arr] :p : {:p}", &arr);
    println!("[arr] size-of : {}", std::mem::size_of::<&[i32;10]>());
    println!("[arr] size-of-val : {}", std::mem::size_of_val(&arr));

    let arr: &[i32] = &arr[..];
    println!("[slice] as_ptr : {:?}", arr.as_ptr());
    println!("[slice] :p : {:p}", &arr);
    println!("[slice] size-of : {}", std::mem::size_of::<&[i32]>());
    println!("[slice] size-of-val : {}", std::mem::size_of_val(arr));

    let arr: &[i32] = &arr[..5];
    println!("[slice2] as_ptr : {:?}", arr.as_ptr());
    println!("[slice2] :p : {:p}", &arr);
    println!("[slice2] size-of : {}", std::mem::size_of::<&[i32]>());
    println!("[slice2] size-of-val : {}", std::mem::size_of_val(arr));

   /* let arr: &[i32] = &[0;10];
    println!("[slice] as_ptr : {:?}", arr.as_ptr());
    println!("[slice] :p : {:p}", &arr);
    println!("[slice] size-of : {}", std::mem::size_of::<&[i32]>());
    println!("[slice] size-of-val : {}", std::mem::size_of_val(arr));

    let arr = vec![0;10];
    println!("[vec] as_ptr : {:?}", arr.as_ptr());
    println!("[vec] :p : {:p}", &arr);
    println!("[vec] size-of : {}", std::mem::size_of::<&Vec<i32>>());
    println!("[vec] size-of-val : {}", std::mem::size_of_val(&arr));*/


}

#[test]
fn test_d() {
    let s = "aa".to_string();
    let ss = &s as *const String;
    let sss;
    unsafe {sss = &*ss}
    println!("{}", sss);
}