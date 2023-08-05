use std::error::Error;

use crate::{
    application::repository::{account::AccountRepository, RepositoryAbstract},
    domain::account::Account,
};

pub struct AccountUsecase {
    repository: Box<dyn AccountRepository>,
}

impl AccountUsecase {
    pub async fn create(&self) -> Result<(), Box<dyn Error>> {
        let account = Account {
            id: None,
            name: "name".to_owned(),
            phone: None,
            email: "abc@gmail.com".to_owned(),
            verified: None,
            password: "password".to_owned(),
            role: None,
            updated_at: None,
            created_at: None,
        };
        let result = self.repository.create(account).await;
        match result {
            Ok(date) => println!("Result : {:?}", date),
            Err(error) => println!("error :{}", error),
        }
        Ok(())
    }
}
