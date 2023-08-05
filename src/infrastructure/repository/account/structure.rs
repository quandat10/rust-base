use uuid::Uuid;

pub struct CreateAccount {
    pub name: String,
    pub email: String,
    pub password: String,
}
