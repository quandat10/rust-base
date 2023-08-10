use std::sync::Arc;

use anyhow::Ok;
use async_trait::async_trait;
use sqlx::{Pool, Postgres};

use crate::domain::{
    model::account::{Account, NewAccount},
    repository::account::AccountRepository,
};

use super::DatabaseRepositoryImpl;

#[async_trait]
impl AccountRepository for DatabaseRepositoryImpl<Account> {
    async fn get(&self, id: String) -> anyhow::Result<Option<Account>> {
        Ok(None)
    }
    async fn insert(&self, account: NewAccount) -> anyhow::Result<Account> {
        let _pool: Arc<Pool<Postgres>> = self.db.0.clone();
        let account = Account {
            id: None,
            name: "Dat".to_owned(),
            phone: None,
            email: "email@gmail.com".to_owned(),
            verified: None,
            password: "Hello world".to_owned(),
            role: None,
            created_at: None,
            updated_at: None,
        };

        Ok(account)
    }
}
