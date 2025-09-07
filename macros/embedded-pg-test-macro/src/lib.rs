extern crate proc_macro;
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{ItemFn, LitStr, parse_macro_input};
use uuid::Uuid;

#[proc_macro_attribute]
pub fn embedded_pg_test(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(item as ItemFn);

    // Get the function name
    let func_name = &input_fn.sig.ident;
    let stmts = &input_fn.block.stmts; // Vec<Stmt>

    let db_tag = Uuid::new_v4().simple().to_string(); // "c0a801e6be7d470691ed7d087b4e1bfd"
    let env_name = format!("DATABASE_URL_{}", db_tag); // "DATABASE_URL_c0a801e6be7d…"
    let env_lit = LitStr::new(&env_name, Span::call_site()); // `syn::LitStr` → "DATABASE_URL_c0…"
    let db_name = LitStr::new(&db_tag, Span::call_site());

    let expanded = quote! {
        #[test]
        fn #func_name() {
            use sqlx::Executor;
            use tokio::runtime::Builder;

            // create an embedded postgres DB
            let settings = postgresql_embedded::Settings {
                version: postgresql_embedded::VersionReq::parse("=16.4.0").expect("parse postgres version"),
                ..Default::default()
            };
            let mut db = postgresql_embedded::PostgreSQL::new(settings);
            let db_handle = std::sync::Arc::new(std::sync::Mutex::new(db));
            let db_handle_ref = db_handle.clone();

            let rt = Builder::new_current_thread()
                .enable_all()
                .build()
                .expect("create Tokio runtime");

            // create the DB in the core DB
            rt.block_on(async move {
                let mut db_ref = db_handle_ref.lock().expect("Lock the Mutex");
                db_ref.setup().await.expect("setup DB");
                db_ref.start().await.expect("start DB");
                db_ref.create_database(#db_name).await.expect("start DB");
                let url: String = db_ref.settings().url(#db_name);

                // set the environment variables for the DB
                unsafe {
                    std::env::set_var(#env_lit, url);
                    std::env::set_var("DB_MAX_CONNECTIONS", "1");
                }
            });

            let test_result = rt.block_on(async {
                // define the DB pool which is std::sync::LazyLock<sqlx::Pool<sqlx::Postgres>> inner = Pool<sqlx::Postgres>
                db_pool_macro::define_pg_pool!(SQLX_POSTGRES_POOL, #env_lit, "DB_MAX_CONNECTIONS");

                struct TestDbHandle;

                impl dal::connections::sqlx_postgres::YieldPostGresPool for TestDbHandle {
                    fn yield_pool() -> &'static sqlx::Pool<sqlx::Postgres> {
                        &*SQLX_POSTGRES_POOL
                    }
                }

                // execute the testing code
                let handle = tokio::spawn(async {
                    #(#stmts)*
                });
                // handle.await   // → Result<Result<(), _>, JoinError>
                let result = handle.await; // Await the handle
                SQLX_POSTGRES_POOL.close().await; // Close the pool after awaiting
                result // Return the handle's result
            });

            // rt.block_on(async move {
            //     let mut db_ref = db_handle.lock().expect("Lock the Mutex");
            //     db_ref.drop_database(#db_name).await.expect("drop DB");
            //     db_ref.stop().await
            // });
            test_result.unwrap();
        }
    };
    TokenStream::from(expanded)
}
