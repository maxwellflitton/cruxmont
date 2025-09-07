//! Defines the macro around mapping functions to traits for transactions.

#[macro_export]
macro_rules! define_dal_transactions {
    (
        $( $trait:ident => $func_name:ident $(< $($generic:tt),* >)? ($($param:ident : $ptype:ty),*) -> $rtype:ty ),* $(,)?
    ) => {
        $(
            pub trait $trait {
                fn $func_name $(< $($generic),* >)? ($($param : $ptype),*) -> impl std::future::Future<Output = sqlx::Result<$rtype>> + Send;
            }
        )*
    };
}

// #[cfg(test)]
// mod tests {

//     use db_tx::db_transaction;
//     use std::future::Future;

//     struct TestStruct;

//     trait TestTrait {
//         fn test_fn() -> impl Future<Output = sqlx::Result<i32>> + Send;
//     }

//     #[db_transaction(TestStruct, TestTrait)]
//     async fn test_fn() -> i32 {
//         Ok(35)
//     }

//     #[tokio::test]
//     async fn test_impl_transaction() {
//         let outcome = TestStruct::test_fn().await;
//         assert_eq!(outcome.unwrap(), 35);
//     }

//     #[tokio::test]
//     async fn test_define_dal_transactions() {
//         struct NewUser;

//         define_dal_transactions!(
//             CreateUser => create(user: NewUser) -> i32,
//             DeleteUser => delete(id: i32) -> bool
//         );

//         struct PostgresHandle;

//         #[db_transaction(PostgresHandle, DeleteUser)]
//         async fn delete(_uid: i32) -> bool {
//             Ok(true)
//         }

//         #[db_transaction(PostgresHandle, CreateUser)]
//         async fn create(_user: NewUser) -> i32 {
//             Ok(1)
//         }
//         let new_user = NewUser;
//         let outcome = PostgresHandle::create(new_user).await.unwrap();
//         assert_eq!(outcome, 1);

//         let outcome = PostgresHandle::delete(1).await.unwrap();
//         assert_eq!(outcome, true);
//     }
// }
