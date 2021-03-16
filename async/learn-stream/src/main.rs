fn main() {
    println!("Hello, world!");
}

trait Stream {
    //由stream 产生的值的类型
    type Item;

    ///尝试解析 stream 中的下一项
    /// 如果已经准备好, 就重新运行 poll::pending , 如果已经完成,就重新
    /// 运行 poll::ready(some(x)) 如果已经完成,就重新运行 poll::ready(none).
}