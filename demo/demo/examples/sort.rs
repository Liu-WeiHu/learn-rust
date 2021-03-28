fn main() {
    let mut v = vec![54,3,5,12,3,55];
    v.sort();
    println!("{:?}", v);

    let mut v = vec![2.3,24.0,1.4,2.2,7.4,1.6,7.4];
    v.sort_by(|a,b| a.partial_cmp(b).unwrap() );
    println!("{:?}", v);

    let mut people = vec![
        Person::new("ag".to_string(), 22),
        Person::new("vv".to_string(), 33),
        Person::new("ad".to_string(), 23),
    ];
    people.sort();
    println!("{:?}", people);

    people.sort_by(|a, b| a.age.cmp(&b.age));
    println!("{:?}", people);
}

#[derive(Debug, PartialOrd, PartialEq, Ord, Eq)]
struct Person {
    name: String,
    age: u32,
}

impl Person {
    pub fn new(name: String, age: u32) -> Self {
        Person {
            name,
            age
        }
    }
}

