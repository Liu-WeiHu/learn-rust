use actix_web::{get, web, Error, HttpRequest, HttpResponse, Responder, HttpServer, App};
use futures::future::{ready, Ready};
use serde::{Serialize, Deserialize};
use actix_web::dev::Service;
use futures::FutureExt;

#[derive(Serialize)]
struct MyObj {
    name: String,
}

// Responder
impl Responder for MyObj {
    type Error = Error;
    type Future = Ready<Result<HttpResponse, Error>>;

    fn respond_to(self, _req: &HttpRequest) -> Self::Future {
        let body = serde_json::to_string(&self).unwrap();

        // Create response and set content type
        ready(Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(body)))
    }
}

#[get("/{name}")]
async fn index(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("aa");
    println!("接收到的信息： {}", name);
    name.to_string()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap_fn(|req, srv| {
                println!("hi from start.");
                srv.call(req).map(|res| {
                    println!("hi from response");
                    res
                })
            })
            //.service(index)
            .route("/index.html", web::get().to(||async {
                "hello  middleware!"
            }))
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}