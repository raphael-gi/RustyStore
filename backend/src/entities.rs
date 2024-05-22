

#[derive(serde::Serialize)]
pub struct Product {
    pub name: String
}

#[derive(serde::Serialize)]
pub struct User {
    pub username: String,
    pub email: String
}
