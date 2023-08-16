mod routes;
pub use routes::{get_api_router, Appstate};

mod check_env;
pub use check_env::get_key_cert;

mod models;
pub use models::{City, GetAllFields};

mod connect_db;
pub use connect_db::{
    configure_deadpool_postgres_db, configure_tokio_postgres_db, get_deadpool_pool,
    get_tokio_client,
};

pub mod api;
pub use api::start_axum_api;

mod handlers;
pub use handlers::*;

pub use tokio::task;

mod tests;
