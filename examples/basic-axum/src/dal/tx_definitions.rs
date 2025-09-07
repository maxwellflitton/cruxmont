use cruxmont::define_dal_transactions;
use sqlx::{Pool, Postgres};


define_dal_transactions!(
    IncreaseCount => increase_count(number: i32, pool: &Pool<Postgres>) -> (),
    DecreaseCount => decrease_count(number: i32, pool: &Pool<Postgres>) -> (),
    GetCount => get_count(number: i32, pool: &Pool<Postgres>) -> i32,
);
