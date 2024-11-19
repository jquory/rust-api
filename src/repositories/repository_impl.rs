use async_trait::async_trait;
use sqlx::PgPool;
use crate::models::user::User;
use crate::repositories::user_repository::UserRepository;

pub struct RepositoryImpl {
    pub pool: PgPool
}
#[async_trait]
impl UserRepository for RepositoryImpl {
    async fn get_all_users(&self) -> Vec<User> {
        let result = sqlx::query_as!(User, "SELECT * FROM users")
            .fetch_all(&self.pool)
            .await
            .unwrap_or_else(|_| Vec::new());
        result
    }
}