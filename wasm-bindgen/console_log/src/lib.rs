use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn run() {
    bare_bones();
    using_a_macro();
    using_web_sys();
}

//首先让我们看一下绑定' console.log'，而不使用
// web_sys的帮助。这里我们正在编写' #[wasm_bindgen] '注释
//程序的正确性依赖于
//这些注释的正确性!
#[wasm_bindgen]
extern "C" {
    //这里使用' js_namespace '来绑定' console.log(..) '，而不只是
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(S: &str);

    //console.log '是相当多态的，所以我们可以将它与multiple绑定
    //签名。注意，我们需要使用' js_name '来确保始终调用
    // ' log '在JS中。
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);

    //多个参数!
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);

    fn alert(s: &str);
}

fn bare_bones() {
    log("Hello from Rust!");
    log_u32(43);
    log_many("Logging", "many values!");
}

//下面我们定义一个类似println!，只是它有用
//“console.log”。请注意,“println !实际上对wasm目标不起作用
//因为标准库目前只接收所有的输出。得到
//“println !'类似的行为在你的应用程序，你可能需要一个这样的宏。
macro_rules! console_log {
    //注意这里使用的是上面导入的' log '函数
    //“bare_bones”
    ($($t: tt)*) => (log(&format_args!($($t)*).to_string()))
}

fn using_a_macro() {
    console_log!("Hello {}!", "world");
    console_log!("Let's print some numbers...");
    console_log!("1 + 3 = {}", 1+3);
}

//最后，我们甚至不必自己定义' log '函数!的
// ' web_sys ' crate已经为我们定义了它。
fn using_web_sys() {
    use web_sys::console;
    console::log_1(&"Hello using web-sys".into());

    let js: JsValue = 4.into();
    console::log_2(&"Logging arbitrary values looks like".into(), &js);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello {}!", name));
}