use std::error::Error;

use crate::infrastructure::repository::account::AccountRespository;

pub struct AccountUsecase {
    repository: AccountRespository,
}

impl AccountUsecase {
    async fn get_one(&self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}
