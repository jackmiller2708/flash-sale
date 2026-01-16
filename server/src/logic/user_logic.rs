use chrono::DateTime;
use uuid::Uuid;

use crate::{domain::user::User, ports::user_repo::UserRepo};

pub async fn create_user<R: UserRepo + ?Sized>(repo: &R) -> anyhow::Result<User> {
    repo.save(User {
        id: Uuid::new_v4(),
        created_at: DateTime::default(),
    })
    .await
}

pub async fn get_users<R: UserRepo + ?Sized>(repo: &R) -> anyhow::Result<Vec<User>> {
    repo.get_all().await
}

pub async fn get_user_by_id<R: UserRepo + ?Sized>(repo: &R, id: Uuid) -> anyhow::Result<User> {
    repo.get_by_id(id).await
}
