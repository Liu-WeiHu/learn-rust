use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(default = "default_bind")]
    pub bind: String,

    #[serde(default)]
    pub services: Vec<ServiceConfig>,

    #[serde(default)]
    pub forward_headers: Vec<String>,

    pub tracing: TracingConfig,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ServiceConfig {
    pub name: String,
    pub addr: String,
    pub query_path: Option<String>,
    pub subscribe_path: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct TracingConfig {
    pub jaeger: Option<JaegerConfig>,
}

#[derive(Debug, Deserialize)]
pub struct JaegerConfig {
    pub agent_endpoint: String,

    #[serde(default = "default_jaeger_service_name")]
    pub service_name: String,
}


fn default_bind() -> String {
    "127.0.0.1:8000".to_string()
}

fn default_jaeger_service_name() -> String {
    "graphgate".to_string()
}

fn main(){
    let config: Config = toml::from_str(r#"
bind = "0.0.0.0:8000"

[tracing.jaeger]
agent_endpoint = "127.0.0.1:1234"

[[services]]
name = "accounts"
addr = "127.0.0.1:8001"

[[services]]
name = "products"
addr = "127.0.0.1:8002"

[[services]]
name = "reviews"
addr = "127.0.0.1:8003"
    "#).unwrap();
    println!("{:?}", config);
    assert_eq!(config.tracing.jaeger.as_ref().unwrap().agent_endpoint, "127.0.0.1:1234");
}