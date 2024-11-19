use async_trait::async_trait;
use crate::models::user::User;

#[async_trait]
pub trait UserService: Send + Sync {
    async fn get_all_users() -> Vec<User>;
}