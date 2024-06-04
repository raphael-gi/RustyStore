use std::env::var;
use actix_web::HttpResponse;
use bcrypt::{hash, verify};
use jwt_kenji::JWT;
use serde::Serialize;
use tokio_postgres::{Error, GenericClient, Row};
use rust_decimal::Decimal;

#[derive(serde::Serialize)]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub price: Decimal,
    pub description: String
}

impl Product {
    pub async fn get_top<C: GenericClient>(client: &C) -> Result<Vec<Product>, Error> {
        let stmt = client.prepare("SELECT id, name, price, description FROM product LIMIT 20").await?;
        let rows = client.query(&stmt, &[]).await?;
        Ok(rows.into_iter().map(Product::from).collect::<Vec<Product>>())
    }

    fn from(row: Row) -> Self {
        Product {
            id: row.get("id"),
            name: row.get("name"),
            price: row.get("price"),
            description: row.get("description")
        }
    }
}

#[derive(serde::Serialize)]
pub struct Customer {
    pub id: i32,
    pub username: String,
    pub email: String
}

#[derive(Serialize)]
struct DatabaseError {
    message: String,
    hint: String
}

impl Customer {
    fn from(row: Row) -> Self {
        Self {
            id: row.get("id"),
            username: row.get("username"),
            email: row.get("email")
        }
    }

    pub async fn all<C: GenericClient>(client: &C) -> Result<Vec<Customer>, Error> {
        let stmt = client.prepare("SELECT id, username, email FROM customer").await?;
        let rows = client.query(&stmt, &[]).await?;
        Ok(rows.into_iter().map(Customer::from).collect())
    }

    pub async fn register<C: GenericClient>(client: &C, username: String, email: String, password: String) -> HttpResponse {
        let password_hash = match hash(password.clone(), 11) {
            Ok(hash) => hash,
            Err(..) => return HttpResponse::InternalServerError().json("Failed to hash password")
        };
        match client.query("CALL register($1,$2,$3)", &[&username, &email, &password_hash]).await {
            Ok(..) => HttpResponse::Ok().finish(),
            Err(err) => {
                match err.as_db_error() {
                    Some(err) => HttpResponse::BadRequest().json(DatabaseError {
                        message: err.message().to_string(),
                        hint: err.hint().unwrap_or("").to_string()
                    }),
                    None => HttpResponse::InternalServerError().into()
                }
            }
        }
    }

    pub async fn login<C: GenericClient>(client: &C, username: String, password: String) -> HttpResponse {
        let stmt = match client.prepare("SELECT id, email, password FROM customer WHERE username = $1").await {
            Ok(stmt) => stmt,
            Err(..) => return HttpResponse::InternalServerError().into()
        };
        let rows = match client.query(&stmt, &[&username]).await {
            Ok(rows) => rows,
            Err(..) => return HttpResponse::InternalServerError().into()
        };

        let found_row = match rows.into_iter().next() {
            Some(found_row) => found_row,
            None => return HttpResponse::BadRequest().body("User not found")
        };

        let found_password: String = found_row.get("password");

        let is_correct = match verify(password, &found_password) {
            Ok(result) => result,
            Err(..) => return HttpResponse::InternalServerError().body("Couldn't verify password")
        };

        if !is_correct {
            return HttpResponse::BadRequest().json("Password incorrect");
        }

        if let Ok(jwt_secret) = var("JWT_SECRET") {
            let id: i32 = found_row.get("id");
            let jwt = JWT::new(jwt_secret.to_string())
                .add_header("alg", "kenji_hash")
                .add_header("typ", "JWT")
                .add_payload("id", &id.to_string())
                .add_payload("username", &username)
                .add_payload("email", found_row.get("email"))
                .build();
            return HttpResponse::Ok().insert_header(("Authorization", format!("Bearer {}", jwt))).finish();
        }

        HttpResponse::InternalServerError().finish()
    }
}
