use serde_json::json;

use crate::api::{get_one,post_put_or_delete_one};
use crate::models::City;
use crate::routes::Appstate;
use axum::{async_trait, debug_handler};
use axum::{
    extract::{rejection::JsonRejection, Extension, Json, Path, Query},
    http::StatusCode,
    response::IntoResponse,
};
use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize)]
pub struct QueryParams {
    pub id: Option<i32>,
}

#[async_trait]
pub trait GetOneTrait {
    type ExtensionType;
    type QueryParamType;
    type PathType;

    async fn getroot(
        Extension(state): Extension<Self::ExtensionType>,
        qval: Option<Query<Self::QueryParamType>>,
        Path(id): Path<Self::PathType>,
    ) -> (StatusCode, Json<Value>);
}

#[async_trait]
pub trait PutOneTrait<T> {
    type ExtensionType;
    type PathType;

    async fn putroot(
        Extension(state): Extension<Self::ExtensionType>,
        Path(id): Path<Self::PathType>,
        jsonbody: Result<Json<T>, JsonRejection>,
    ) -> (StatusCode, Json<Value>);
}

#[async_trait]
pub trait PostOneTrait<T> {
    type ExtensionType;

    async fn postroot(
        Extension(state): Extension<Self::ExtensionType>,
        jsonbody: Result<Json<T>, JsonRejection>,
    ) -> (StatusCode, Json<Value>);
}

#[async_trait]
pub trait DeleteOneTrait {
    type ExtensionType;
    type PathType;

    async fn deleteroot(
        Extension(state): Extension<Self::ExtensionType>,
        Path(id): Path<Self::PathType>,
    ) -> (StatusCode, Json<Value>);
}
/*
#[debug_handler]
pub async fn getroot(Extension(state): Extension<Appstate>, qval: Option<Query<QueryParams>>, Path(id): Path<i32>) -> (StatusCode, impl IntoResponse) {

    let qval = match qval {
        Some(qv) => qv.0,
        None => QueryParams{id : None}
    };


    let result: Option<City> = get_one(&state.pool, id, "cities").await;
    let (code, resp) = match result {


        Some(city) => (StatusCode::OK, json!({
            "query val" : qval.id,
            "state val" : state.number,
            "path val" : id,
            "city" : city
        })),

        None => (StatusCode::NOT_FOUND,json!({
            "query val" : qval.id,
            "state val" : state.number,
            "path val" : id,
        }))};

        (code, Json(resp))
}
*/

/*
#[debug_handler]
pub async fn putroot(Extension(state): Extension<Appstate> ,Path(id): Path<i32>, jsonbody: Result<Json<City>, JsonRejection>) -> (StatusCode, impl IntoResponse) {

    let body = match jsonbody {
        Ok(body) => body,
        Err(err) => return (StatusCode::BAD_REQUEST, Json(serde_json::Value::from(format!("err : {}", err.body_text()))))
    };

    let result = put_one(&state.pool, body.0, id).await;
    let (code, resp) = match result {

        Ok(_) => (StatusCode::NO_CONTENT, json!({
        })),

        Err(err) => (StatusCode::BAD_REQUEST, json!({
            "err" : format!("{}", err)
        }))
    };

    (code, Json(resp))
}

*/

/*
#[debug_handler]
pub async fn postroot(Extension(state): Extension<Appstate>, jsonbody: Result<Json<City>, JsonRejection>) -> (StatusCode, impl IntoResponse) {

    let body = match jsonbody {
        Ok(body) => body,
        Err(err) => return (StatusCode::BAD_REQUEST, Json(serde_json::Value::from(format!("err : {}", err.body_text()))))
    };

    let result = post_one(&state.pool, body.0).await;
    let (code, resp) = match result {

        Ok(name) => (StatusCode::CREATED, json!({
            "success" : format!("{} created", name)
        })),

        Err(err) => (StatusCode::BAD_REQUEST, json!({
            "err" : format!("{}", err)
        }))
    };
    (code, Json(resp))
}
*/
/*
#[debug_handler]
pub async fn deleteroot(Extension(state): Extension<Appstate> ,Path(id): Path<i32>) -> (StatusCode, impl IntoResponse) {

    let result = delete_one(&state.pool, id).await;
    let (code, resp) = match result {

        Ok(_) => (StatusCode::NO_CONTENT, json!({
        })),
        Err(err) => (StatusCode::BAD_REQUEST, json!({
            "err" : format!("{}", err)
        }))
    };
    (code, Json(resp))
}
*/
