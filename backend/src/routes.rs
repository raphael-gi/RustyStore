use std::io::ErrorKind;

use actix_web::{get, post, web::{Data, Json}, HttpResponse, Responder};
use deadpool_postgres::{Object, Pool};
use crate::entities::{Product, Customer};

#[get("/user")]
pub async fn get_user(pool: Data<Pool>) -> HttpResponse {
    let client = match pool.get().await {
        Ok(client) => client,
        Err(err) => {
            println!("{}", err);
            return HttpResponse::InternalServerError().json("unable to get postgres client");
        }
    };
    match Customer::all(&**client).await {
        Ok(list) => HttpResponse::Ok().json(list),
        Err(err) => {
            println!("{}", err);
            return HttpResponse::InternalServerError().json("unable to fetch users");
        }
    }
}

#[get("/product")]
async fn get_product() -> impl Responder {
    let product = Product {
        name: "Barbell".to_string()
    };

    Json(product)
}

#[post("/register")]
async fn register_user(pool: Data<Pool>, username: String, password: String) -> HttpResponse {
    if username.len() > 45 {
        return HttpResponse::BadRequest().body("Username must be shorter than 45 characters");
    }
    if username.len() < 3 {
        return HttpResponse::BadRequest().body("Username must be longer than 3 characters");
    }
    if password.len() < 8 {
        return HttpResponse::BadRequest().body("Password must be longer than 8 characters");
    }

    let client = match get_client(pool).await {
        Ok(client) => client,
        Err(err) => return err
    };

    match Customer::register(&**client, username, password).await {
        Ok(created_user) => HttpResponse::Ok().json(created_user),
        Err(err) => 
    }
}


async fn get_client(pool: Data<Pool>) -> Result<Object, HttpResponse> {
    match pool.get().await {
        Ok(client) => Ok(client),
        Err(err) => {
            println!("{}", err);
            Err(HttpResponse::InternalServerError().json("unable to get postgres client"))
        }
    }
}
