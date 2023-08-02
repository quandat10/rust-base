#[derive(Debug, Clone)]
pub struct Account {
    pub code: String,
    pub phone: String,
    pub email: String,
    pub updated_at: String,
    pub created_at: String,
}

impl Account {
    pub fn new(
        code: String,
        phone: String,
        email: String,
        updated_at: String,
        created_at: String,
    ) -> Self {
        Account {
            code,
            phone,
            email,
            updated_at,
            created_at,
        }
    }
}
