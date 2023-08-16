use crate::models::City;
use crate::GetAllFields;
use deadpool_postgres::tokio_postgres::{self, types::ToSql, Statement};
use deadpool_postgres::{Client, Pool};
use phf::phf_map;
use postgres_from_row::FromRow;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fmt::Display;
use tokio_postgres::{row::Row, error::SqlState, types::Type};

static TABLES: phf::Map<&str, &str> = phf_map! {
    "City" => "cities",
};

#[derive(Debug, Clone)]
struct NoResults;

impl fmt::Display for NoResults {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Query returned no results")
    }
}

impl std::error::Error for NoResults {}

//Enums to make things more difficult
#[derive(Debug, Serialize, Clone)]
pub enum QueryBuilderError {
    BadArgs(&'static str),
    NoResults,
    MethodUnsupported,
    Postgres(String),
    TableNotFound,
}
impl Display for QueryBuilderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use QueryBuilderError::*;
        match self {
            BadArgs(state) => write!(f, "({})", state),
            NoResults => write!(f, "No results"),
            MethodUnsupported => write!(f, "Method Not Supported"),
            Postgres(details) => write!(f, "{}", details),
            TableNotFound => write!(f, "table not found"),
        }
    }
}

impl std::error::Error for QueryBuilderError {}

impl From<NoResults> for QueryBuilderError {
    fn from(value: NoResults) -> Self {
        QueryBuilderError::NoResults
    }
}

impl From<&tokio_postgres::Error> for QueryBuilderError {
    fn from(value: &tokio_postgres::Error) -> Self {
        QueryBuilderError::Postgres(value.to_owned().to_string())
    }
}
#[derive(Debug)]
pub enum QueryBuilder<T, I>
where
    T: GetAllFields,
{
    Single(String, Option<I>, Option<T>, axum::http::Method),
}

impl<T, I> Default for QueryBuilder<T, I>
where
    T: GetAllFields,
{
    fn default() -> Self {
        QueryBuilder::Single(String::new(), None, None, axum::http::Method::GET)
    }
}

impl<T, I> QueryBuilder<T, I>
where
    T: GetAllFields,
{
    pub fn builder() -> Self {
        Self::default()
    }

    pub fn structname(self, name: String) -> Self {
        use QueryBuilder::*;
        match self {
            Single(_, val2, val3, val4) => Single(name, val2, val3, val4)
        }
    }

    pub fn pkid(self, id: I) -> Self{
        use QueryBuilder::*;
        match self {
            Single(val1, _, val3, val4) => Single(val1, Some(id), val3, val4)
        }
    }

    pub fn model(self, obj: T) -> Self{
        use QueryBuilder::*;
        match self {
            Single(val1, val2, _, val4) => Single(val1, val2, Some(obj), val4)
        }
    }

    pub fn method(self, method: axum::http::Method) -> Self {
        use QueryBuilder::*;
        match self {
            Single(val1, val2, val3, _) => Single(val1, val2, val3, method)
        }
    }

    fn get_city_query_string<'a, 'b>(
        table: &'b str,
        method: &'a axum::http::Method,
    ) -> Result<String, QueryBuilderError> {
        match *method {
                axum::http::Method::GET => Ok(format!("SELECT * FROM {} WHERE id = $1", table)),
                axum::http::Method::POST => Ok(format!("INSERT INTO {} (name, country, population, area) VALUES ($1, $2, $3, $4) RETURNING name", table)),
                axum::http::Method::PUT => Ok(format!("UPDATE {} SET (name, country, population, area) = ($1, $2, $3, $4) WHERE id = $5 RETURNING name", table)),
                axum::http::Method::DELETE => Ok(format!("DELETE FROM {} WHERE id = $1 RETURNING name", table)),
                _ => Err(QueryBuilderError::MethodUnsupported),
            }
    }

   async fn get_city_statement(
        client: &Client,
        querystring: &str,
        method: &axum::http::Method,
    ) -> Result<Statement, QueryBuilderError> {
        let result = match *method {
            axum::http::Method::GET => client
                .prepare_typed(querystring, &[Type::INT4])
                .await
                .map_or_else(|_e| Err(QueryBuilderError::BadArgs("TODO")), |v| Ok(v)),

            axum::http::Method::POST => client
                .prepare_typed(
                    querystring,
                    &[Type::VARCHAR, Type::VARCHAR, Type::INT4, Type::INT4],
                )
                .await
                .map_or_else(|_e| Err(QueryBuilderError::BadArgs("TODO")), |v| Ok(v)),

            axum::http::Method::PUT => client
                .prepare_typed(
                    querystring,
                    &[
                        Type::VARCHAR,
                        Type::VARCHAR,
                        Type::INT4,
                        Type::INT4,
                        Type::INT4,
                    ],
                )
                .await
                .map_or_else(|_e| Err(QueryBuilderError::BadArgs("TODO")), |v| Ok(v)),

            axum::http::Method::DELETE => client
                .prepare_typed(querystring, &[Type::INT4])
                .await
                .map_or_else(|_e| Err(QueryBuilderError::BadArgs("TODO")), |v| Ok(v)),
            _ => Err(QueryBuilderError::MethodUnsupported),
        };

        result
    }

    pub async fn build<'life>(self, pool: &Pool) -> Result<Vec<Row>, QueryBuilderError>
    where
        T: Serialize + Deserialize<'life> + FromRow,
        I: ToSql + Send + std::marker::Sync,
    {
        use QueryBuilder::*;
        match self {
            Single(structname, id, model, method) => {
                let table = match TABLES.get(structname.as_str()).copied() {
                    None => panic!("No table for struct"),
                    Some(tup) => tup,
                };

                let client = pool.get().await.unwrap();

                match structname.as_str() {
                    "City" => {
                        let querystring = QueryBuilder::<City, i32>::get_city_query_string(table, &method)?;
                        let query = QueryBuilder::<City, i32>::get_city_statement(&client, &querystring, &method).await.expect("All Valid");
                        match method {
                            axum::http::Method::GET => {
                                let rows = client.query_one(&query, &[&id]).await.map_err(|e| QueryBuilderError::from(&e))?;
                                Ok(vec!(rows))
                            }
                            axum::http::Method::POST => {
                                let city = model
                                    .unwrap()
                                    .get_all_fields()
                                    .map_err(|_| QueryBuilderError::BadArgs("TODO"))?;
                                let rows = client
                                    .query(
                                        &query,
                                        &[&city.get(0).unwrap(), &city.get(1).unwrap(), &city.get(2).unwrap().parse::<i32>().unwrap(), &city.get(3).unwrap().parse::<i32>().unwrap()],
                                    )
                                    .await.map_err(|e| QueryBuilderError::from(&e))?;
                                dbg!(&rows);
                                Ok(rows)
                            }
                            axum::http::Method::PUT => {
                                let city = model
                                    .unwrap()
                                    .get_all_fields()
                                    .map_err(|_| QueryBuilderError::BadArgs("TODO"))?;
                                let rows = client
                                    .query(
                                        &query,
                                        &[
                                            &city.get(0).unwrap(),
                                            &city.get(1).unwrap(),
                                            &city.get(2).unwrap().parse::<i32>().unwrap(),
                                            &city.get(3).unwrap().parse::<i32>().unwrap(),
                                            &id,
                                        ],
                                    )
                                    .await.map_err(|e| QueryBuilderError::from(&e))?;
                                Ok(rows)
                            }
                            axum::http::Method::DELETE => {
                                let rows = client.query_one(&query, &[&id]).await.map_err(|e| QueryBuilderError::from(&e))?;
                                Ok(vec!(rows))
                            },
                            _ => Err(QueryBuilderError::MethodUnsupported),
                        }
                    }
                    // More Tables Below or better yet in another file
                    _ => Err(QueryBuilderError::TableNotFound),
                }
            }
        }
    }
}

pub fn get_one<'life, T>(row: Vec<Row>) -> Result<T, tokio_postgres::Error>
where
    T: Serialize + Deserialize<'life> + FromRow,
{
    match T::try_from_row(row.get(0).expect("Rows have to be present or i messed up")) {
        Ok(obj) => Ok(obj),
        Err(err) => Err(err),
    }
}

pub fn post_put_or_delete_one(row: Vec<Row>) -> Result<String, QueryBuilderError>
where
        {
    if row.is_empty() {
        return Err(QueryBuilderError::NoResults)
    }
    Ok(row.get(0).expect("Works").get(0))
}
