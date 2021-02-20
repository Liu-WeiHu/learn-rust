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

    #[bench]
    fn bench_base64_encode(b: &mut Bencher) {
        b.iter(|| {
            base64::encode(b"aaaa")
        });
    }

    #[bench]
    fn bench_base64_decode(b: &mut Bencher) {
        b.iter(|| {
            base64::decode("aaaa")
        });
    }

    #[bench]
    fn bench_parse_int(b: &mut Bencher) {
        b.iter(|| {
            "10".parse::<isize>()
        });
    }

    #[test]
    fn test_base64_decode() {
        let src = base64::decode(
            "bWFnbmV0Oj94dD11cm46YnRpaDo4MjE2YTZhM2VjNjMyMWZjMmIzYjE4MGE1Zjk0YTU2MDVjNTc5OTE4JmRuPeiQjOaxgQ=="
        ).unwrap();
        let src: &str = std::str::from_utf8(&src).unwrap();
        println!("{}", src);
    }

    #[bench]
    fn bench_sum(b: &mut Bencher) {
        //let mut arr: [usize;10000] = [0;10000]; 
        b.iter(move || {
            //let _s = (0..10000).map(|i|i+5).collect::<Vec<i32>>();
            //let _s = (0..).take(10000).map(|i|i+5).collect::<Vec<i32>>();
            /* for i in 0..10000 {
                arr[i] = i+5
            } */
        });
    }

    #[test]
    fn test_sum() {
        let s = (0..15).map(|i| i+5 ).collect::<Vec<i32>>();
        println!("{:?}", s);
    }

}

pub fn add_two(a: i32) -> i32 {
    a + 2
}
