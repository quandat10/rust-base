use async_trait::async_trait;

use crate::domain::model::account::{Account, NewAccount};

#[async_trait]
pub trait AccountRepository {
    async fn get(&self, id: String) -> anyhow::Result<Option<Account>>;
    async fn insert(&self, account: NewAccount) -> anyhow::Result<Account>;
}
