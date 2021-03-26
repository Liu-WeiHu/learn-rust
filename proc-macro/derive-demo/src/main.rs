
use into_map_derive::{IntoMap};

#[derive(Debug, IntoMap)]
struct User {
    name: String,
    age: u8,
    id: u8,
    active: bool,
}

fn main() {
    let my_bar = User{name: "zhaoyun".to_string(), age: 22, id: 11, active: true};
    let map = my_bar.into_map();
    println!("{:?}", map); //{"name": "zhaoyun", "id": "11", "age": "22", "active": "true"}
}
