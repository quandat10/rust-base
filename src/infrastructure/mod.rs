use crate::domain::{model::account::Account, repository::account::AccountRepository};

use self::{database::posgresql::DB, repository::DatabaseRepositoryImpl};

pub mod config;
pub mod database;
pub mod repository;

pub struct RepositoriesModule {
    account_repository: DatabaseRepositoryImpl<Account>,
}

pub trait RepositoriesModuleExt {
    type AccountRepo: AccountRepository;

    fn account_repository(&self) -> &Self::AccountRepo;
}

impl RepositoriesModuleExt for RepositoriesModule {
    type AccountRepo = DatabaseRepositoryImpl<Account>;

    fn account_repository(&self) -> &Self::AccountRepo {
        &self.account_repository
    }
}

impl RepositoriesModule {
    pub fn new(db: DB) -> Self {
        let account_repository = DatabaseRepositoryImpl::new(db);

        Self { account_repository }
    }
}
