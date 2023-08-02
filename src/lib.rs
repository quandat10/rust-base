pub mod application;
pub mod domain;
pub mod infrastructure;

pub async fn run() {
    infrastructure::server().await
}
