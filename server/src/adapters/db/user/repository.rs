use async_trait::async_trait;
use sqlx::PgConnection;
use uuid::Uuid;

use crate::{
    adapters::db::{error_mapper::map_sqlx_error, user::UserRecord},
    domain::user::User,
    errors::RepoError,
    ports::UserRepo,
};

pub struct PostgresUserRepo;

impl PostgresUserRepo {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl UserRepo for PostgresUserRepo {
    async fn save(&self, conn: &mut PgConnection, user: User) -> Result<User, RepoError> {
        let record = sqlx::query_as!(
            UserRecord,
            r#"
            INSERT INTO users (id, created_at)
            VALUES ($1, $2)
            RETURNING *
            "#,
            user.id,
            user.created_at
        )
        .fetch_one(conn)
        .await
        .map_err(|e| map_sqlx_error(e, "save_user", "user"))?;

        Ok(User::from(record))
    }

    async fn get_all(&self, conn: &mut PgConnection) -> Result<Vec<User>, RepoError> {
        let rows = sqlx::query_as!(
            UserRecord,
            r#"
            SELECT id, created_at
            FROM users
            "#
        )
        .fetch_all(conn)
        .await
        .map_err(|e| map_sqlx_error(e, "get_all_users", "user"))?;

        Ok(rows.into_iter().map(|r| User::from(r)).collect())
    }

    async fn get_by_id(&self, conn: &mut PgConnection, id: Uuid) -> Result<User, RepoError> {
        let row = sqlx::query_as!(
            UserRecord,
            r#"
            SELECT id, created_at
            FROM users
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(conn)
        .await
        .map_err(|e| map_sqlx_error(e, "get_user_by_id", "user"))?;

        match row {
            Some(r) => Ok(User::from(r)),
            None => Err(RepoError::NotFound {
                entity_type: "user",
            }),
        }
    }
}
