use crate::{
    configure_deadpool_postgres_db, configure_tokio_postgres_db, get_deadpool_pool,
    get_tokio_client,
};

use tokio_postgres::Client;

pub(crate) async fn setuptests() -> (deadpool_postgres::Pool, Client) {
    let tokio_pg_config: tokio_postgres::Config = configure_tokio_postgres_db();
    let deadpool_pg_config: deadpool_postgres::Config = configure_deadpool_postgres_db();

    let client = tokio::spawn(get_tokio_client(tokio_pg_config))
        .await
        .unwrap();
    let pool = tokio::spawn(get_deadpool_pool(deadpool_pg_config))
        .await
        .unwrap();

    (pool, client)
}
