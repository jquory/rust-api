use async_trait::async_trait;
use crate::models::user::User;

#[async_trait]
pub trait UserRepository {
    async fn get_all_users(&self) -> Vec<User>;
}