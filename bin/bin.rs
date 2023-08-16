use axum_crud_lib::start_axum_api;
use axum_crud_lib::task;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().expect(".env file must exist");

    task::block_in_place(move || {
        let _ = start_axum_api();
    });
}
