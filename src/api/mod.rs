mod queries;
mod startapi;

pub use queries::{get_one, post_put_or_delete_one, QueryBuilder, QueryBuilder::Single};
pub use startapi::start_axum_api;
