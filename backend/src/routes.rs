use std::env::var;
use actix_web::{error::{self}, get, http::header::ContentType, post, web::{Data, Form, Json, Path}, HttpRequest, HttpResponse};
use deadpool_postgres::{Object, Pool};
use jwt_kenji::JWT;
use serde::Deserialize;
use derive_more::{Display,Error};
use crate::entities::{Customer, Product};

#[get("/user")]
pub async fn get_user(pool: Data<Pool>) -> HttpResponse {
    let client = match pool.get().await {
        Ok(client) => client,
        Err(..) => return HttpResponse::InternalServerError().body("unable to get postgres client")
    };
    match Customer::all(&**client).await {
        Ok(list) => HttpResponse::Ok().json(list),
        Err(..) => return HttpResponse::InternalServerError().body("unable to fetch users")
    }
}

#[derive(Debug, Display, Error)]
enum MyError {
    #[display(fmt = "internal error")]
    InternalError,

    #[display(fmt = "bad request")]
    BadClientData,
}

impl error::ResponseError for MyError {
    fn error_response(&self) -> HttpResponse {
        match self {
            Self::InternalError => {},
            Self::BadClientData => {}
        }
        HttpResponse::BadRequest()
            .content_type(ContentType::plaintext())
            .body(self.to_string())
    }
}

#[get("/product/{id}")]
async fn get_product(pool: Data<Pool>, id: Path<i32>) -> Result<Json<Product>, MyError> {
    let client = match pool.get().await {
        Ok(client) => client,
        Err(..) => return Err(MyError::InternalError)
    };

    match Product::get(&**client, *id).await {
        Ok(product) => Ok(Json(product)),
        Err(_err) => Err(MyError::BadClientData)
    }
}

#[get("/products")]
async fn get_products(pool: Data<Pool>, req: HttpRequest) -> HttpResponse {
    let token = match verify_token(&req) {
        Ok(token) => token,
        Err(err) => return err
    };

    if let Ok(jwt_secret) = var("JWT_SECRET") {
        if !JWT::verify(token.to_string(), jwt_secret.to_string()) {
            return HttpResponse::BadRequest().body("JWT not valid");
        }
    } else {
        return HttpResponse::InternalServerError().body("Failed");
    }

    let client = match pool.get().await {
        Ok(client) => client,
        Err(..) => return HttpResponse::InternalServerError().json("unable to get postgres client")
    };

    match Product::get_top(&**client).await {
        Ok(list) => HttpResponse::Ok().json(list),
        Err(..) => return HttpResponse::InternalServerError().body("unable to fetch products")
    }
}

fn verify_token(req: &HttpRequest) -> Result<&str, HttpResponse> {
    let auth_header: &str = match req.headers().get("Authorization") {
        Some(jwt) => jwt.to_str().unwrap(),
        None => return Err(HttpResponse::BadRequest().body("No JWT provided"))
    };
    let token = match auth_header.split(" ").last() {
        Some(token) => token,
        None => return Err(HttpResponse::BadRequest().body("No token provided"))
    };
    Ok(token)
}

#[derive(Deserialize)]
struct RegisterForm {
    username: String,
    email: String,
    password: String
}

#[post("/register")]
async fn register_user(pool: Data<Pool>, form: Form<RegisterForm>) -> HttpResponse {
    let username: String = match verify_username(&form.username) {
        Ok(username) => username,
        Err(err) => return err
    };

    let email: String = form.email.to_string();
    if email.len() >= 90 {
        return HttpResponse::BadRequest().body("Email must be shorter than 90 characters");
    }

    let password: String = match verify_password(&form.password) {
        Ok(password) => password,
        Err(err) => return err
    };

    let client = match get_client(pool).await {
        Ok(client) => client,
        Err(err) => return err
    };

    Customer::register(&**client, username, email, password).await
}

#[derive(Deserialize)]
struct LoginForm {
    username: String,
    password: String
}

#[post("/login")]
async fn login_user(pool: Data<Pool>, form: Form<LoginForm>) -> HttpResponse {
    let username: String = match verify_username(&form.username) {
        Ok(username) => username,
        Err(err) => return err
    };
    let password: String = match verify_password(&form.password) {
        Ok(password) => password,
        Err(err) => return err
    };

    let client = match get_client(pool).await {
        Ok(client) => client,
        Err(err) => return err
    };

    Customer::login(&**client, username, password).await
}

fn verify_username(username: &str) -> Result<String, HttpResponse> {
    if username.len() >= 45 {
        return Err(HttpResponse::BadRequest().json("Username must be shorter than 45 characters"));
    }
    if username.len() < 3 {
        return Err(HttpResponse::BadRequest().body("Username must be longer than 2 characters"));
    }
    Ok(username.to_string())
}

fn verify_password(password: &str) -> Result<String, HttpResponse> {
    if password.len() < 8 {
        return Err(HttpResponse::BadRequest().body("Password must be longer than 7 characters"));
    }
    Ok(password.to_string())
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
