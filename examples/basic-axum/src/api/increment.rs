//! Core logic for increasing the count and retrieving the updated count.
//!
//! # Overview
//! This file contains the core functionality for increasing the count in the database
//! and returning the updated count value. It uses the data access layer (DAL) traits
//! to perform database transactions.
//!
//! # Features
//! - Increases the count for id = 1 by 1.
//! - Retrieves the updated count value.
//! - Uses generic traits for database operations to enable testing with mocks.
//!
//! # Notes
//! - The `increase_and_get_count` function is generic, allowing flexibility with different database implementations.
//! - Errors are propagated as `NanoServiceError` for consistency with the application's error handling.
use crate::dal::tx_definitions::{IncreaseCount, GetCount};
use cruxmont::errors::CruxmontError;
use cruxmont::dal::connections::sqlx_postgres::YieldPostGresPool;
use axum::{extract::Json, http::StatusCode, response::IntoResponse};

/// Increases the count for id = 1 by 1 and returns the updated count.
///
/// # Arguments
/// - `pool`: The PostgreSQL connection pool.
///
/// # Returns
/// - `Ok(i32)`: The updated count value.
/// - `Err(NanoServiceError)`: If the operation fails (e.g., row not found or database error).
pub async fn increase_and_get_count<X, Y>() -> Result<impl IntoResponse, CruxmontError>
where
    X: IncreaseCount + GetCount,
    Y: YieldPostGresPool
{
    let pool = Y::yield_pool();
    // Increase the count by 1 for id = 1
    X::increase_count(1, pool).await.map_err(|e| {
        CruxmontError::unknown(format!("Failed to increase count: {}", e))
    })?;

    // Retrieve the updated count
    let count = X::get_count(1, pool).await.map_err(|e| {
        CruxmontError::unknown(format!("Failed to retrieve count: {}", e))
    })?;

    Ok((StatusCode::OK, Json(count)))
}

#[cfg(test)]
mod tests {
    
    use super::*;
    use cruxmont::dal::connections::sqlx_postgres::SqlxPostGresDescriptor;
    use sqlx::{Pool, Postgres};
    use cruxmont::embedded_pg_test::embedded_pg_test;
    use crate::dal::basic_migrations::run_migrations;
    use cruxmont::postgresql_embedded;
    use axum::http::StatusCode;
    use http_body_util::BodyExt;

    #[embedded_pg_test]
    async fn test_increase_and_get_count() {
        // The SQLX_POSTGRES_POOL is provided by the test macro
        let pool: &Pool<Postgres> = &*SQLX_POSTGRES_TEST_POOL;
        run_migrations(pool).await.expect("run migrations");

        // Initial count should be 0 (from migration)
        let initial_count = SqlxPostGresDescriptor::get_count(1, pool)
            .await
            .expect("Failed to get initial count");
        assert_eq!(initial_count, 0);

        // Call the function to increase and get count
        let result = increase_and_get_count::<SqlxPostGresDescriptor, TestDbHandle>()
            .await
            .expect("Failed to increase and get count");

        // Verify response
        let mut response = result.into_response();
        assert_eq!(response.status(), StatusCode::OK);
        let body = response
            .body_mut()
            .collect()
            .await
            .expect("extract bytes from response body")
            .to_bytes();
        let count: i32 =
            serde_json::from_slice(&body).expect("Failed to deserialize department");
        assert_eq!(count, 1);

        // Verify the count in the database
        let updated_count = SqlxPostGresDescriptor::get_count(1, pool)
            .await
            .expect("Failed to get updated count");
        assert_eq!(updated_count, 1);

        // Call again to ensure increment works multiple times
        let result = increase_and_get_count::<SqlxPostGresDescriptor, TestDbHandle>()
            .await
            .expect("Failed to increase and get count again");
        
        // Verify response
        let mut response = result.into_response();
        assert_eq!(response.status(), StatusCode::OK);
        let body = response
            .body_mut()
            .collect()
            .await
            .expect("extract bytes from response body")
            .to_bytes();
        let count: i32 =
            serde_json::from_slice(&body).expect("Failed to deserialize department");
        assert_eq!(count, 2);
    }
}