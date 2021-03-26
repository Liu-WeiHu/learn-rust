//编译过程
/// rust_code   ->(分词)   TokenStream: 词条流
/// TokenStream  ->(解析)  AST: 语法树
/// AST ->(降级)  HIR: 高级中间语言     作用: 编译器对代码进行类型检查,方法查找等工作
/// HIR ->(降级)  MIR: 中级中间语言     作用: 借用检查,优化,代码生成(宏,泛型,单态化)等工作
/// MIR ->(优化)  LLVM IR 作用: 生成机器码



fn main() {
    println!("Hello, world!");
}

