use rand::prelude::*;

fn main() {
    //可以直接使用random()。它可以产生许多常见类型的值:
    let x = random::<u8>();
    println!("{}",x);

    //生成一个布尔值
    if random() {
        println!("heads!")
    }

    //如果我们想要更明确(和更有效)，我们可以
    //创建线程本地生成器的句柄:
    let mut rng = thread_rng();
    if rng.gen() {
        let x = rng.gen::<u8>();
        let y = rng.gen_range(-10.0..10.0);
        println!("x is : {}, y is : {}", x, y);
    }

    //有时候直接使用分布是很有用的:
    let distr = rand::distributions::Uniform::new_inclusive(1,100);
    let mut nums = [0i32;3];
    for x in &mut nums {
        *x = rng.sample(distr);
    }
    println!("arr is : {:?}", nums);

    //也可以与迭代器和切片器交互:
    let arrows_iter = "qwertyuiop".chars();
    let choose_char = arrows_iter.choose(&mut rng).unwrap();
    println!("choose_char is : {}", choose_char);

    //随机打乱顺序
    let mut nums = [1,2,3,4,5];
    nums.shuffle(&mut rng);
    println!("I shuffled my {:?}", nums);

    let rand_string: String = thread_rng()
        .sample_iter(&rand::distributions::Alphanumeric)
        .take(30)
        .map(char::from)
        .collect();
    println!("rand string : {}", rand_string);
}
