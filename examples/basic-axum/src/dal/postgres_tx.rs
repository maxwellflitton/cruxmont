use cruxmont::dal::connections::sqlx_postgres::SqlxPostGresDescriptor;
use crate::dal::tx_definitions::{
    IncreaseCount, DecreaseCount, GetCount,
};
use cruxmont::db_tx::db_transaction;
use sqlx::{Pool, Postgres};

/// Implements the `IncreaseCount` trait for the `SqlxPostGresDescriptor`.
///
/// Increases the count value for the row with id = 1 by the specified number.
///
/// # Arguments
/// - `number`: The amount to increase the count by.
/// - `pool`: The PostgreSQL connection pool.
///
/// # Returns
/// - `Ok(())`: If the update operation is successful.
/// - `Err`: If the row with id = 1 is not found or the update fails.
#[db_transaction(SqlxPostGresDescriptor, IncreaseCount)]
async fn increase_count(number: i32, pool: &Pool<Postgres>) -> () {
    let query = r#"
        UPDATE counts
        SET value = value + $1
        WHERE id = 1
        RETURNING id
    "#;

    let result = sqlx::query(query)
        .bind(number)
        .fetch_optional(pool)
        .await?;

    if result.is_none() {
        return Err(sqlx::Error::RowNotFound);
    }

    Ok(())
}

/// Implements the `DecreaseCount` trait for the `SqlxPostGresDescriptor`.
///
/// Decreases the count value for the row with id = 1 by the specified number.
///
/// # Arguments
/// - `number`: The amount to decrease the count by.
/// - `pool`: The PostgreSQL connection pool.
///
/// # Returns
/// - `Ok(())`: If the update operation is successful.
/// - `Err`: If the row with id = 1 is not found or the update fails.
#[db_transaction(SqlxPostGresDescriptor, DecreaseCount)]
async fn decrease_count(number: i32, pool: &Pool<Postgres>) -> () {
    let query = r#"
        UPDATE counts
        SET value = value - $1
        WHERE id = 1
        RETURNING id
    "#;

    let result = sqlx::query(query)
        .bind(number)
        .fetch_optional(pool)
        .await?;

    if result.is_none() {
        return Err(sqlx::Error::RowNotFound);
    }

    Ok(())
}

/// Implements the `GetCount` trait for the `SqlxPostGresDescriptor`.
///
/// Retrieves the current count value for the row with id = 1.
///
/// # Arguments
/// - `number`: The id of the count (fixed to 1 in this case).
/// - `pool`: The PostgreSQL connection pool.
///
/// # Returns
/// - `Ok(i32)`: The current count value.
/// - `Err`: If the row with id = 1 is not found.
#[db_transaction(SqlxPostGresDescriptor, GetCount)]
async fn get_count(number: i32, pool: &Pool<Postgres>) -> i32 {
    let query = r#"
        SELECT value
        FROM counts
        WHERE id = $1
    "#;

    let row: (i32,) = sqlx::query_as(query)
        .bind(number)
        .fetch_one(pool)
        .await?;

    Ok(row.0)
}