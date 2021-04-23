use std::borrow::Cow;

fn remove_spaces(input: &str) -> Cow<str> {
    if input.contains(' ') {
        let mut buf = String::with_capacity(input.len());

        for c in input.chars() {
            if c != ' ' {
                buf.push(c);
            }
        }
        return Cow::Owned(buf);
    }
    return Cow::Borrowed(input);
}

fn main() {

    let a = "aaa";
    let mut res = remove_spaces(a);
    let a: &str = res.to_mut();

    //这种是 传引用入参  然后又返回原来的引用？ 等于 啥也没干 也没有其他的开销
    let b = "bbb".to_string();
    let res = remove_spaces(&b);
    let b = res.into_owned();
}
