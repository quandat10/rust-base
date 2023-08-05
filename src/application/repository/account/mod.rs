use async_trait::async_trait;
use sqlx::{Pool, Postgres};
use std::error::Error;

pub mod structure;

use crate::{application::repository::RepositoryAbstract, domain::account::Account};

use self::structure::CreateAccount;

pub struct AccountRespository {
    db: Pool<Postgres>,
}

impl AccountRespository {
    pub async fn get_by_code(code: String) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}

#[async_trait]
impl RepositoryAbstract<Account, CreateAccount> for AccountRespository {
    async fn create(&self, account: CreateAccount) -> Result<Account, Box<dyn Error>> {
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
