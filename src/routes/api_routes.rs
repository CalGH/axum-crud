use crate::City;
use crate::{configure_deadpool_postgres_db, get_deadpool_pool};
use crate::{DeleteOneTrait, GetOneTrait, PostOneTrait, PutOneTrait};
use axum::{
    routing::{delete, get, post, put},
    Extension, Router,
};
use std::sync::Arc;
#[derive(Clone)]
pub struct Appstate {
    pub number: i32,
    pub pool: Arc<deadpool_postgres::Pool>,
}

pub async fn get_api_router() -> Router {
    //let tokio_pg_config: tokio_postgres::Config = configure_tokio_postgres_db();
    let deadpool_pg_config: deadpool_postgres::Config = configure_deadpool_postgres_db();

    //let client = tokio::spawn(get_tokio_client(tokio_pg_config)).await.unwrap();
    let pool = tokio::spawn(get_deadpool_pool(deadpool_pg_config))
        .await
        .expect("Failed to start pool");

    let apiroot: Router = Router::new()
        .route("/api/post", post(City::postroot))
        .route("/api/get/:id", get(City::getroot))
        .route("/api/put/:id", put(City::putroot))
        .route("/api/delete/:id", delete(City::deleteroot));

    let router = Router::new().nest("/", apiroot).layer(Extension(Appstate {
        number: 1,
        pool: Arc::new(pool),
    }));
    router
}

/* OR WITH STATE
let apiroot: Router = Router::new()
    .route("/api", get(homepage)).with_state(Appstate{
        number: 1,
        pool: Arc::new(pool)
    });
    // Creates box and leaks memory returning a static mut reference to it
    // .with_state(&*Box::leak(Box::new(Appstate {})))

    let router = Router::new().nest("/", apiroot);
*/
