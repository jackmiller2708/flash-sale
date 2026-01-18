use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use crate::adapters::db::user::UserRecord;
use crate::{domain::user::User, ports::UserRepo};

pub struct PostgresUserRepo {
    pool: PgPool,
}

impl PostgresUserRepo {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepo for PostgresUserRepo {
    async fn save(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
        user: User,
    ) -> anyhow::Result<User> {
        let record: UserRecord = sqlx::query_as!(
            UserRecord,
            r#"
            INSERT INTO users (id, created_at)
            VALUES ($1, $2)
            RETURNING *
            "#,
            user.id,
            user.created_at
        )
        .fetch_one(&mut **tx)
        .await?;

        Ok(User::from(record))
    }

    async fn get_all(&self) -> anyhow::Result<Vec<User>> {
        let rows = sqlx::query_as!(
            UserRecord,
            r#"
            SELECT id, created_at
            FROM users
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(rows.into_iter().map(|r| User::from(r)).collect())
    }

    async fn get_by_id(&self, id: Uuid) -> anyhow::Result<User> {
        let row = sqlx::query_as!(
            UserRecord,
            r#"
            SELECT id, created_at
            FROM users
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        match row {
            Some(r) => Ok(User::from(r)),
            None => Err(anyhow::anyhow!("User with id {} not found", id)),
        }
    }
}
