use anyhow::Error;
use async_trait::async_trait;
use definitions_manager_lib::read_side_processor::{ProjectionOffsetStore, ProjectionOffsetStoreRepository};
use sqlx::PgPool;

pub struct PostgresProjectionOffsetStoreRepository {
    pool: PgPool,
}

impl PostgresProjectionOffsetStoreRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ProjectionOffsetStoreRepository for PostgresProjectionOffsetStoreRepository {
    async fn insert_record(&self, record: ProjectionOffsetStore) -> anyhow::Result<(), anyhow::Error> {
        sqlx::query!(
            r#"
            INSERT INTO pekko_projection_offset_store (projection_name, projection_key, current_offset, manifest, mergeable, last_updated)
            VALUES ($1, $2, $3, $4, $5, $6)
            "#,
            record.projection_name,
            record.projection_key,
            record.current_offset,
            record.manifest,
            record.mergeable,
            record.last_updated
        )
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    async fn read_record(&self, projection_name: &str, projection_key: &str) -> Result<Option<ProjectionOffsetStore>, Error> {
        let record = sqlx::query_as!(
            ProjectionOffsetStore,
            r#"
            SELECT projection_name, projection_key, current_offset, manifest, mergeable, last_updated
            FROM pekko_projection_offset_store
            WHERE projection_name = $1 AND projection_key = $2
            ORDER BY last_updated DESC
            LIMIT 1
            "#,
            projection_name,
            projection_key
        )
            .fetch_optional(&self.pool)
            .await?;
        Ok(record)
    }
    async fn update_record(&self, projection_name: &str, projection_key: &str, current_offset: &str) -> anyhow::Result<(), Error> {
        sqlx::query!(
            r#"
            UPDATE pekko_projection_offset_store
            SET current_offset = $3
            WHERE projection_name = $1 AND projection_key = $2
            "#,
            projection_name,
            projection_key,
            current_offset
        )
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}

