use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct Account {
    pub id: Option<Uuid>,
    pub name: String,
    pub phone: Option<String>,
    pub email: String,
    pub verified: Option<bool>,
    pub password: String,
    pub role: Option<String>,
    #[serde(rename = "createdAt")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

pub struct NewAccount {
    pub name: String,
    pub email: String,
    pub password: String,
}

impl NewAccount {
    pub fn new(name: String, email: String, password: String) -> Self {
        NewAccount {
            name,
            email,
            password,
        }
    }
}
