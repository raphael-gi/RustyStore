use actix_web::{get, main, web::Json, App, HttpServer, Responder};

#[get("/")]
async fn greet() -> impl Responder {
    Json(["hello there"])
}

#[main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .service(greet)
    })
        .bind(("0.0.0.0", 8080))?
        .run()
    .await
}
