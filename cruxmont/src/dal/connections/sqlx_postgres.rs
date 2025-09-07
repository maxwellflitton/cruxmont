//! Defines the connection to the PostgreSQL database and the `SqlxPostGresDescriptor` for dependency injection.
//!
//! # Overview
//! - Establishes a connection pool for a PostgreSQL database using the `sqlx` library.
//! - Provides the `SqlxPostGresDescriptor` struct to serve as a handle for database-related operations.
//! - Configures the connection pool using environment variables for flexibility and scalability.
use sqlx::{Pool, Postgres};

/// A descriptor struct used for applying database traits and dependency injection.
///
/// # Notes
/// This struct is intended to be used as a handle for implementing database-related traits
/// that define transactions or other interactions with the database.
pub struct SqlxPostGresDescriptor;

/// A descriptor struct for yielding a live PostGres DB pool
pub struct LivePostGresPool;

cruxmont_pg_pool_macro::define_pg_pool!(SQLX_POSTGRES_POOL, "DATABASE_URL", "DB_MAX_CONNECTIONS");

pub trait YieldPostGresPool {
    fn yield_pool() -> &'static Pool<Postgres>;
}

impl YieldPostGresPool for LivePostGresPool {
    fn yield_pool() -> &'static Pool<Postgres> {
        &SQLX_POSTGRES_POOL
    }
}
