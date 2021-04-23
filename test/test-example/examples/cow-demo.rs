use std::borrow::Cow;

fn abs_all(input: &mut Cow<[i32]>) {
    for i in 0..input.len() {
        let v = input[i];
        if v < 0 {
            // 克隆到一个载体，如果还没有。
            input.to_mut()[i] = -v;
        }
    }
}

/*
to_mut ：就是返回数据的可变引用，如果是借用，则进行复制；如果已拥有所有权，则无需进行复制
into_owned ：获取一个拥有所有权的对象（区别与引用），如果当前是借用，则发生复制，创建新的所有权对象，如果已拥有所有权，则转移至新对象。
 */

fn main() {
// 只读，不写，没有发生复制操作
    let slice = [0, 1, 2];
    let mut input = Cow::from(&slice[..]);
    abs_all(&mut input);

// 写时复制， 在读到-1的时候发生复制
    let slice = [-1, 0, 1];
    let mut input = Cow::from(&slice[..]);
    abs_all(&mut input);

// 没有写时复制，因为已经拥有所有权
    let mut input = Cow::from(vec![-1, 0, 1]);
    abs_all(&mut input);
}