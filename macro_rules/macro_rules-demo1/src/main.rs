/*
block：代码块（即语句和/或表达式的块，用大括号括起来）
expr： 表达式
ident：标识符，用来表示函数或变量名
item：条目，例如函数，结构，模块，impl等。
lifetime：生命周期标识（例如'foo，'static，...）
literal：文字，数字，常量（例如"Hello World!"，3.14，'🦀'，...）
meta：元项目；这里面去的东西#[...]和#![...]属性
pat： 模式，普通模式匹配（非宏本身的模式）中的模式，例如 Some(t), (3, 'a', _)
path：一个路径（例如foo，::std::mem::replace，transmute::<_, int>，...）
stmt： 单条语句，如 let a = 42;
tt：单个令牌树
ty： 类型 (例如i32, bool, ...)
vis：权限（例如pub，pub(in crate)，...）
*/

macro_rules! items {
    ($($item: item)*) => {}
}

macro_rules! blocks {
    ($($block: block)*) => {}
}

macro_rules! stmts {
    ($($stmt: stmt)*) => {}
}

macro_rules! pats {
    ($($pat: pat)*) => {}
}

macro_rules! exprs {
    ($($expr: expr)*) => {};
}

macro_rules! tys {
    ($($ty: ty)*) => {};
}

macro_rules! idents {
    ($($ident: ident)*) => {}
}

macro_rules! paths {
    ($($path: path)*) => {}
}

macro_rules! metas {
    ($($meta:meta)*) => ();
}

macro_rules! lifetimes {
    ($($lifetime:lifetime)*) => ();
}

macro_rules! viss {
    ($($vis:vis,)*) => ();
}

macro_rules! literals {
    ($($literal: literal)*) => {}
}

fn main() {
    items!{
        struct Foo;
        enum Animal {
            Dog,
            Pig,
        }
        impl Foo{}
        fn add() {}
        /*...*/
    }

    blocks!{
        {}
        {
            let a = 32;
        }
        {3}
    }

    stmts!{
        let x = 32;
        struct Foo;
        fn foo() {}
        5;
        5
        {}
        ;
        ;
        ;
        if true {} else {}
    }

    pats!{
        "literal"
        0..5
        ref mut aaa
    }

    exprs!{
        "literal"
        add()
        future.await
        break 'foo bar
    }

    tys!{
        i32
        bool
        foo::bar
        Vec[String]
    }

    idents!{
        foo
        async
        a
    }

    paths!{
        ::b::c
        std::a
        b
    }

    metas! {
        ASimplePath
        super::man
        path = "home"
        foo(bar)
    }

    lifetimes! {
        'static
        'shiv
        '_
    }

    viss!{
        ,
        pub,
        pub(crate),
        pub(in super),
        pub(in some_path),
    }

    literals! {
        -1
        0
        2.2
        "hello"
        'c'
        b'c'
        true
    }
}
