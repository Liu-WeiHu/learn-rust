
macro_rules! hashmap {
    (@unit $($x: tt)*) => (());

    (@count $($key: expr),*) => (<[()]>::len(&[$(hashmap! (@unit $key)), *]));

    ($($key: expr => $val: expr),* $(,)?) => {
        {
            let cap = hashmap!(@count $($key),*);
            let mut map = ::std::collections::HashMap::with_capacity(cap);
            $(
                map.insert($key, $val);
            )*
            map
        }
    }
}

fn main() {

    let map = hashmap!{
        "a" => 1,
        "b" => 2,
        "c" => 3,
    };

    let s = <[()]>::len(&[(),(),()]);
    println!("{:?}", map);

}
