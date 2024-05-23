use tokio_postgres::{Error, GenericClient, Row};

#[derive(serde::Serialize)]
pub struct Product {
    pub name: String
}

#[derive(serde::Serialize)]
pub struct Customer {
    pub id: i32,
    pub username: String
}

impl From<Row> for Customer {
    fn from(row: Row) -> Self {
        Self {
            id: row.get(0),
            username: row.get(1)
        }
    }
}

impl Customer {
    pub async fn all<C: GenericClient>(client: &C) -> Result<Vec<Customer>, Error> {
        let stmt = client.prepare("SELECT id, username, email FROM customer").await?;
        let rows = client.query(&stmt, &[]).await?;

        Ok(rows.into_iter().map(Customer::from).collect())
    }

    pub async fn register<C: GenericClient>(client: &C, username: String, password: String) -> Result<bool, &str> {
        match client.query("", &[]).await {
            Ok(_) => Ok(true),
            Err(_) => Err("failed to register user")
        }
    }
}
