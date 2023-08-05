use async_trait::async_trait;
use sqlx::{Pool, Postgres};
use std::error::Error;

pub mod structure;

use crate::{
    application::repository::{account::AccountRepository, RepositoryAbstract},
    domain::account::Account,
};

pub struct AccountRespositoryInDB {
    db: Pool<Postgres>,
}

#[async_trait]
impl AccountRepository for AccountRespositoryInDB {
    async fn create(&self, account: Account) -> Result<Account, Box<dyn Error>> {
        let email = account.email;
        let name: String = account.name;
        let password = account.password;

        let query_result = sqlx::query_as!(
            Account,
            "INSERT INTO accounts (name, email, password) VALUES ($1, $2, $3) RETURNING *",
            name,
            email,
            password
        )
        .fetch_one(&self.db)
        .await?;
        Ok(query_result)
    }

    async fn get_one(&self, code: String) -> Result<Account, Box<dyn Error>> {
        todo!()
    }

    async fn update(&self, code: String) -> Result<Account, Box<dyn Error>> {
        todo!()
    }

    async fn delete(&self, code: String) -> Result<Account, Box<dyn Error>> {
        todo!()
    }
}
