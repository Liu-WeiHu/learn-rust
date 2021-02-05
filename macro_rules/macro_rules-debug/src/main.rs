#![feature(trace_macros)]

macro_rules! each_tt {
    () => {};
    ($_tt:tt $($rest: tt)*) => {each_tt!($($rest)*);}
}


fn main() {
    each_tt!(foo bar baz quux);
    trace_macros!(true);
    each_tt!(spim wak plee whum);
    trace_macros!(false);
    each_tt!(trom qlip winp xod);
}
