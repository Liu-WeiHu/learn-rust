use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Config {
    global_string: Option<String>,
    global_integer: Option<u64>,
    server: Option<ServerConfig>,
    peers: Option<Vec<PeerConfig>>,
}

#[derive(Debug, Deserialize)]
struct ServerConfig{
    ip: Option<String>,
    port: Option<u64>
}

#[derive(Debug, Deserialize)]
struct PeerConfig {
    ip: Option<String>,
    port: Option<u64>,
}

fn main() {
    let toml_str = r#"
        global_string = "test"
        global_integer = 5

        [test]
        ip = "127.0.0.1"
        port = 80

        [[peers]]
        ip = "127.0.0.1"
        port = 8080

        [[peers]]
        ip = "127.0.0.1"
    "#;

    let decoded: Config = toml::from_str(toml_str).unwrap();
    println!("{:#?}", decoded);
}

mod tests {
    use serde::Deserialize;
    use std::path::Path;
    use std::io::Read;

    #[derive(Debug, Deserialize)]
    enum MyEnum {
        Plain,
        Tuple(i64, bool),
        NewType(String),
        Struct{value: i64},
    }

    #[derive(Debug, Deserialize)]
    struct Config {
        plain: MyEnum,
        plain_table: MyEnum,
        tuple: MyEnum,
        #[serde(rename="struct")]
        structv: MyEnum,
        newtype: MyEnum,
        my_enum: Vec<MyEnum>,
    }

    #[test]
    fn test_deser() {
        let toml_str = r#"
            plain = "Plain"
            plain_table = {Plain = {}}
            tuple = {Tuple = { 0 = 123, 1 = true }}
            struct = { Struct = { value = 123 }}
            newtype = { NewType = "value" }
            my_enum = [
                { Plain = {} },
                { Tuple = { 0 = 123, 1 = true }},
                { NewType = "value" },
                { Struct = { value = 123 }},
            ]"#;

        let decoded: Config = toml::from_str(toml_str).unwrap();
        println!("{:#?}", decoded);
    }

    #[test]
    fn test_toml2json() {
        use std::fs::File;
        use std::io;
        use std::io::prelude::*;
        let mut input = String::new();
        File::open(Path::new("Cargo.toml"))
            .and_then(|mut f| f.read_to_string(&mut input))
            .unwrap();
        println!("{}", input);

        match input.parse() {
            Ok(toml) => {
                let json = convert(toml);
                println!("{}",serde_json::to_string_pretty(&json).unwrap());
            }
            Err(error) => {
                println!("failed to parse Toml : {}", error)
            }
        }
    }

    fn convert(v: toml::Value) -> serde_json::Value {
        match v {
            toml::Value::String(s) => {println!("111");serde_json::Value::String(s)},
            toml::Value::Integer(i) => {println!("222");serde_json::Value::Number(i.into())},
            toml::Value::Float(f) => {
                println!("333");
                let n = serde_json::Number::from_f64(f).expect("float infinite and nan not allowed");
                serde_json::Value::Number(n)
            }
            toml::Value::Boolean(b) => {println!("444");serde_json::Value::Bool(b)},
            toml::Value::Array(arr) => {println!("555");serde_json::Value::Array(arr.into_iter().map(convert).collect())},
            toml::Value::Table(table) => {
                println!("666");
                serde_json::Value::Object(table.into_iter().map(|(k,v)| (k, convert(v))).collect())
            }
            toml::Value::Datetime(dt) => {println!("777");serde_json::Value::String(dt.to_string())},
        }
    }
}