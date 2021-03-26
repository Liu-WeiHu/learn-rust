#![allow(unused)]
use std::thread::sleep;
use std::time::Duration;
use std::ops::Deref;
use std::rc::Rc;

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
    let mut output = 10;
    compute(&7, &mut output);
    println!("{}", output);
}

fn compute(input: &u32, output: &mut u32) {
    let mut temp = *output;
    if *input > 10 {
        temp = 1;
    }
    if *input > 5 {
        temp *= 2;
    }
    *output = temp;
}

#[test]
fn test_lifetime() {
    let mut a = vec![1,2,3];
    let b = &a[0];
    a.push(4);
    //println!("{}", b);
}

struct Foo;

impl Foo {
    fn mutate_and_share(&mut self) -> &Self {&*self}
    fn share(&self) {}
}

#[test]
fn test_life() {
    let mut s = Rc::new("aa".to_string());
    let ss = s.clone();
    //s.push_str("bb");
    println!("{}", s);
}

#[test]
fn test_ab() {
    let s = Student;
    s.say_work();
    <Student as Work>::a();
}

trait Work {
    fn say_work(&self);

    fn a() {
        println!("wo shi work")
    }
}

struct Student;

impl Work for Student {
    fn say_work(&self) {
        println!("wo shi student")
    }
}

impl Student {
    fn a() {
        println!("wo shi student")
    }
}

#[test]
fn test_tuple() {
    let tuple = (1,);
    let t2 = (1,"aa","a".to_string(),false);
}

#[test]
fn test_ref() {
    let mut s = "aa".to_string();
    let mut ss = &mut s;
    ss.push_str("cc");
    let mut sss = &mut s;
    sss.push_str("bb");
    s.push_str("dd");
    println!("{}", s);
}

#[test]
fn test_bibao() {
    let mut f = counter(3);
    println!("{}", f(2));
}

fn counter(i: i32) -> impl FnMut(i32) -> i32 {
    let mut i = i;
    move |n| {i += 10; n + i}
}

#[test]
fn test_deref(){
    let u = U{name: "zhang san".to_string()};
    let u = MyBox::new(u);
    let name = u.get_name();
    U::test(&u);
    println!("{}",name );
}

struct U {
    name: String,
}

impl U {
    fn get_name(&self) -> &String {
        &self.name
    }

    fn test(u: &U) {
        println!("{}",u.name);
    }
}

struct MyBox<T>(T);
impl <T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// #[test]
// fn test_aa() {
//     let mut a = vec![1,2,3,4];
//
//     for (i,v) in a.iter_mut().enumerate() {
//         if a[i] == 2 {
//             a[i] = 22;
//         }
//     }
//
//     //println!("{:?}", a);
// }

#[test]
fn test_asd() {
    let mut s = 22;
    let ss = &mut s;
    let sss = &s;
    //*ss = 44;
    //println!("{:?}", sss);
    //sss;
}


#[test]
fn test_abv() {
    let mut a = vec![1,2];
    let aa = &a;
    //a.push(1);
    let dd = &mut a;
    //let cc = &mut a;
    dd;
    //aa;
    //aa;
}

#[test]
fn test_str() {
    let s1="hello";
    let s2 = "hello";
    println!("&s1 : {:p}", &s1);
    println!("&s2 : {:p}", &s2);

    let s1 = s1.as_ptr();
    let s2 = s2.as_ptr();
    println!("s1 ptr : {:p}", s1);
    println!("s2 ptr : {:p}", s2);
}

fn  remove_prefix<'a>(content:&'a str,prefix:&str) -> &'a str{
    if content.starts_with(prefix){
        let start:usize = prefix.len();
        let end:usize = content.len();
        let sub = content.get(start..end).unwrap();
        sub
    }else{
        content
    }
}

#[test]
fn test_life2() {
    let  s = "reload";
    let sub = remove_prefix(&s,"re");
    println!("{}",sub);
}

#[test]
fn test_fn() {
    let mut s = "aa".to_string();
}

#[test]
fn test_duotai() {
    let mut s = Studen;
    let ss = &mut s  as &mut dyn Person;
}

trait Person {}
struct Studen;
impl Person for Studen{}