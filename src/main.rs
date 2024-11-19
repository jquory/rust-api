mod helper;
mod repositories;
mod models;
mod services;
mod dto;

use sqlx::{PgPool};
use std::env;
use crate::repositories::repository_impl::RepositoryImpl;
use crate::repositories::user_repository::UserRepository;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenv::dotenv().ok();
    println!("Env loaded!");

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    println!("Connected to database");
    let pool = PgPool::connect(&database_url).await?;

    let repo = RepositoryImpl{pool};

    let users = repo.get_all_users().await;

    println!("Found {:?}", users);

    Ok(())
}
