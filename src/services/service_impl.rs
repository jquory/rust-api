use sqlx::PgPool;
use crate::models::user::User;
use crate::repositories::repository_impl::RepositoryImpl;
use crate::repositories::user_repository::UserRepository;
use crate::services::user_service::UserService;

struct UserServiceImpl {
    pool: PgPool
}

impl UserService for UserServiceImpl {
    async fn get_all_users() -> Vec<User> {
        let result = RepositoryImpl{pool}.get_all_users().await;
        result
    }
}