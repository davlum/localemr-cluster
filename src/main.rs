mod filters;
mod models;
mod handlers;
mod tests;

use std::env;
use warp::Filter;
use pretty_env_logger;
use std::fs;

/// Provides a RESTful web server managing some Batches.
///
/// API will be:
///
/// - `GET /batch`: return a JSON list of Batches.
/// - `GET /batch/:id`: Get a specific Batch.
/// - `POST /batch/:id`: create a new Batch.
/// - `DELETE /batch/:id`: delete a specific Batch.


#[tokio::main]
async fn main() {
    if env::var_os("RUST_LOG").is_none() {
        // Set `RUST_LOG=batch=debug` to see debug logs,
        // this only shows access logs.
        env::set_var("RUST_LOG", "debug");
    }
    fs::create_dir_all(models::LOG_DIR).unwrap();

    pretty_env_logger::init();
    let db = models::blank_db();

    let api = filters::batch(db);


    // View access logs by setting `RUST_LOG=batch`.
    let routes = api.with(warp::log("batch"));
    // Start up the server...
    warp::serve(routes).run(([127, 0, 0, 1], 8998)).await;
}
