use crate::application::usecase::account::AccountUsecase;

use std::error::Error;

pub struct AccountController {
    service: AccountUsecase,
}

impl AccountController {
    pub async fn create(&self) -> Result<(), Box<dyn Error>> {
        self.service.create().await?;
        Ok(())
    }
}
