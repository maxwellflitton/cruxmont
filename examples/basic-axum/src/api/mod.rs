pub mod decrement;
pub mod increment;
pub mod get;

// The LivePostGresPool connects to the live DB
use cruxmont::dal::connections::sqlx_postgres::{LivePostGresPool, SqlxPostGresDescriptor};
use axum::{
    Router,
    routing::get,
};


pub fn counter_factory(app: Router) -> Router {
    app.route(
        "/increment",
        get(increment::increase_and_get_count::<SqlxPostGresDescriptor, LivePostGresPool>),
    )
    .route(
        "/decrement",
        get(decrement::decrease_and_get_count::<
            SqlxPostGresDescriptor,
            LivePostGresPool,
        >),
    )
}