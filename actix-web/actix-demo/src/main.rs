use actix_web::{get, App, Responder, HttpResponse, HttpServer, HttpRequest};
use serde::Serialize;

#[derive(Serialize)]
struct Res {
    result: String,
}

#[get("/{name}")]
async fn get_name_by_id(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("张三");
    HttpResponse::Ok()
        .json(Res{result: format!("{} 你好！", name)})
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("127.0.0.1:8082/李四");
    HttpServer::new(||{
        App::new()
            .service(get_name_by_id)
    })
    .bind("127.0.0.1:8082")?
    .run()
    .await
}