use std::marker::PhantomData;

use super::database::posgresql::DB;
pub mod account;

pub struct DatabaseRepositoryImpl<T> {
    db: DB,
    _marker: PhantomData<T>,
}

impl<T> DatabaseRepositoryImpl<T> {
    pub fn new(db: DB) -> Self {
        Self {
            db,
            _marker: PhantomData,
        }
    }
}
