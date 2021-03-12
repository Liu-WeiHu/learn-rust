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
fn test_demo() {
    let mut a = [1,2,3];
    let x = &mut a;

    let z = &x;
    (*z)[0] = 11;
    println!("{:?}", a);
}

#[test]
fn test_dd() {
    let mut a = 44;
    let aa = &mut a;
    *aa = 55;
    println!("{}", a);
}