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

impl Account {
    pub fn new(
        id: Option<Uuid>,
        name: String,
        phone: Option<String>,
        email: String,
        verified: Option<bool>,
        password: String,
        role: Option<String>,
        updated_at: Option<chrono::DateTime<chrono::Utc>>,
        created_at: Option<chrono::DateTime<chrono::Utc>>,
    ) -> Self {
        Account {
            id,
            name,
            phone,
            email,
            verified,
            password,
            role,
            updated_at,
            created_at,
        }
    }
}
