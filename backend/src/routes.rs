use actix_web::{get, post, web::{Data, Form}, HttpResponse};
use deadpool_postgres::{Object, Pool};
use serde::Deserialize;
use crate::entities::{Customer, Product};

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

#[get("/products")]
async fn get_products(pool: Data<Pool>) -> HttpResponse {
    let client = match pool.get().await {
        Ok(client) => client,
        Err(err) => {
            println!("{}", err);
            return HttpResponse::InternalServerError().json("unable to get postgres client");
        }
    };

    match Product::get_top(&**client).await {
        Ok(list) => HttpResponse::Ok().json(list),
        Err(err) => {
            println!("{}", err);
            return HttpResponse::InternalServerError().body("unable to fetch products");
        }
    }
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
