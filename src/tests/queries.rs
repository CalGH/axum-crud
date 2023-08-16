use std::error::Error;
//use postgres_from_row::FromRow;
use crate::tests::utils::common::setuptests;
use tokio::test;

#[cfg(test)]
mod tests {
    use super::*;

    /* macro_rules! blocking_await {
     *        ($e:expr) => {
     *            tokio_test::block_on($e)
        };
    } */

    #[tokio::test]
    pub async fn test_db_connections() -> Result<(), Box<dyn Error>> {
        dotenvy::dotenv().expect(".env file must exist");

        async fn test_tokio_select_all(client: tokio_postgres::Client) {
            let _rows = client.query("SELECT * FROM cities", &[]).await.unwrap();
            /* let countries: Vec<Option<Country>> = rows.into_iter().filter_map(|row|
            {
                match Country::try_from_row(&row)
                {
                    Ok(country) => Some(Some(country)),
                                                                              _ => None
                }
            }).collect();
            */
        }

        async fn test_deadpool_select_one(pool: deadpool_postgres::Pool) {
            let client = pool.get().await.unwrap();
            let stmt = client
                .prepare_cached("SELECT * FROM cities limit 1")
                .await
                .unwrap();
            let _rows: Vec<tokio_postgres::row::Row> = client.query(&stmt, &[]).await.unwrap();
            /* let countries: Vec<Country> = rows.into_iter().filter_map(|row|
            {
                if let Some(country) = Country::try_from_row(&row).ok(){
                    Some(country)
                }
                else {
                    None
                }
            }).collect();
            */
        }

        let (pool, client) = setuptests().await;
        test_tokio_select_all(client).await;
        test_deadpool_select_one(pool).await;
        Ok(())
    }
}
