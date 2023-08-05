use async_trait::async_trait;
use std::error::Error;

use crate::domain::account::Account;

#[async_trait]
pub trait AccountRepository {
    async fn create(&self, account: Account) -> Result<Account, Box<dyn Error>>;
    async fn get_one(&self, code: String) -> Result<Account, Box<dyn Error>>;
    async fn update(&self, code: String) -> Result<Account, Box<dyn Error>>;
    async fn delete(&self, code: String) -> Result<Account, Box<dyn Error>>;
}
