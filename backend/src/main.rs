use actix_web::{main, web::Data, App, HttpServer};
use routes::{get_product, get_user, register_user};

mod postgres;
mod routes;
mod entities;

#[main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let pg_pool = postgres::create_pool();

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(pg_pool.clone()))
            .service(get_user)
            .service(get_product)
            .service(register_user)
    })
        .bind(("0.0.0.0", 8000))?
        .run().await
}
