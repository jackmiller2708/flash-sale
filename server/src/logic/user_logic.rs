use sqlx::PgConnection;
use uuid::Uuid;

use crate::{domain::user::User, errors::AppError, ports::UserRepo};

pub async fn save_user<R: UserRepo + ?Sized>(
    conn: &mut PgConnection,
    repo: &R,
    user: User,
) -> Result<User, AppError> {
    repo.save(conn, user).await.map_err(Into::into)
}

pub async fn get_users<R: UserRepo + ?Sized>(
    conn: &mut PgConnection,
    repo: &R,
) -> Result<Vec<User>, AppError> {
    repo.get_all(conn).await.map_err(Into::into)
}

pub async fn get_user_by_id<R: UserRepo + ?Sized>(
    conn: &mut PgConnection,
    repo: &R,
    id: Uuid,
) -> Result<User, AppError> {
    repo.get_by_id(conn, id).await.map_err(Into::into)
}
