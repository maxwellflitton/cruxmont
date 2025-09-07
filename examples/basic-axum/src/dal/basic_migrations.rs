use sqlx::{Pool, Postgres};


/// Basic migrations just for the simple example.
pub async fn run_migrations(pool: &Pool<Postgres>) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS counts (
            id SERIAL PRIMARY KEY,
            value INTEGER NOT NULL DEFAULT 0
        )
        "#
    )
    .execute(pool)
    .await?;

    // Insert a single row with count of zero, if it doesn't already exist
    sqlx::query(
        r#"
        INSERT INTO counts (value)
        SELECT 0
        WHERE NOT EXISTS (SELECT 1 FROM counts WHERE id = 1)
        "#
    )
    .execute(pool)
    .await?;
    Ok(())
}
