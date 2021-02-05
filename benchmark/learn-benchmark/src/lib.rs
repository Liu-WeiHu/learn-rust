#![feature(test)]
extern crate test;

//cargo bench --help
//cargo bench  方法名
//b.iter(); 固定格式

#[cfg(test)]
mod tests {
    use test::Bencher;
    #[bench]
    fn bench_xor_1000_ints(b: &mut Bencher) {
        b.iter(|| {
            //(0..1000).fold(0, |old, new| old ^ new); -- error  闭包必须要有返回值
            (0..1000).fold(0, |old, new| old ^ new)
        });
    }

    #[bench]
    fn bench_add_1000_ints(b: &mut Bencher) {
        b.iter(|| {
            (0..100).fold(0, |a,b| a + b)
        });
    }

    #[bench]
    fn bench_add_two(b: &mut Bencher) {
        b.iter(|| {
            super::add_two(2)
        });
    }

}

pub fn add_two(a: i32) -> i32 {
    a + 2
}
