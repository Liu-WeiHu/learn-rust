use actix_files::{Files};
use actix_web::{App, HttpResponse, HttpServer, web};
use handlebars::Handlebars;

async fn index(hb: web::Data<Handlebars<'_>>) -> HttpResponse {
    let data = serde_json::json!({
        "project_name": "Catdex",
        "cats": [
            {
                "name": "aaa",
                "image_path":
                    "/static/image/11.jpg"
            },
            {
                "name": "bbb",
                "image_path":
                    "/static/image/22.jpg"
            },
            {
                "name": "ccc",
                "image_path":
                    "/static/image/33.jpg"
            }
        ]
    });
    let body = hb.render("index", &data).unwrap();
    HttpResponse::Ok().body(body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut handlebars = Handlebars::new();
    handlebars
    .register_templates_directory(".html", "./static/")
    .unwrap();
    let handlebars_ref = web::Data::new(handlebars);
    println!("localhost:8080");
    HttpServer::new(move || {
        App::new()
            .app_data(handlebars_ref.clone())
            .service(
                Files::new("/static", "static")
                            .show_files_listing(),
            )
            .route("/", web::get().to(index))
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
