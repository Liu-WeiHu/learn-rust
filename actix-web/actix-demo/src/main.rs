use actix_web::{get, post, App, Responder, HttpResponse, HttpServer, HttpRequest, web, guard};
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

#[post("/pos")]
async fn pos() -> impl Responder {
    "aaa"
}

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    HttpServer::new(||{
        App::new()
            .service(
                web::scope("/v1")
                    .service(get_name_by_id)
                    .service(pos)
            )
    })
    .bind("127.0.0.1:8082")
    .run()
    .await
}