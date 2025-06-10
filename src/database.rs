use crate::pre_import::*;

pub async fn init_pool(db_url: &str) -> Result<PgPool, sqlx::Error> {
    PgPool::connect(db_url).await
}
