use actix_web::{get, web::Json, Responder};
use crate::entities::{Product, User};

#[get("/user")]
async fn get_user() -> impl Responder {
    let user = User {
        username: "Chris Bumstead".to_string(),
        email: "chris@bummy.dev".to_string()
    };

    Json(user)
}

#[get("/product")]
async fn get_product() -> impl Responder {
    let product = Product {
        name: "Barbell".to_string()
    };

    Json(product)
}
