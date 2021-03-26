use proc_macro_lib::hello;

fn main() {
    hello!(echo);
    fn_echo("hello, proc-macro");
}
