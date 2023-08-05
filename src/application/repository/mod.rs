use async_trait::async_trait;
use std::error::Error;

pub mod account;

#[async_trait]
pub trait RepositoryAbstract<T, H> {
    async fn create(&self, payload: H) -> Result<T, Box<dyn Error>>;
    async fn get_one(&self, code: String) -> Result<T, Box<dyn Error>>;
    async fn update(&self, code: String) -> Result<T, Box<dyn Error>>;
    async fn delete(&self, code: String) -> Result<T, Box<dyn Error>>;
}
