use actix_web::{main, App, HttpServer};
use routes::{get_product, get_user};

mod routes;
mod entities;

#[main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .service(get_user)
            .service(get_product)
    })
        .bind(("0.0.0.0", 8080))?
        .run()
    .await
}
