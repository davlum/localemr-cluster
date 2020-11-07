pub use crate::models::{Db,Cmd};
use crate::handlers;
use warp::Filter;

/// The 4 TODOs filters combined.
pub fn batch(
    db: Db,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    list_batch(db.clone())
        .or(create_batch(db.clone()))
        .or(get_batch(db.clone()))
        .or(delete_batch(db))
}

/// GET /batches
pub fn list_batch(
    db: Db,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("batches")
        .and(warp::get())
        .and(with_db(db))
        .and_then(handlers::list_batch_h)
}

/// GET /batch/<batch_id>
pub fn get_batch(
    db: Db,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("batch" / String)
        .and(warp::get())
        .and(with_db(db))
        .and_then(handlers::get_batch_h)
}

/// POST /batch with JSON body
pub fn create_batch(
    db: Db,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("batch" / String)
        .and(warp::post())
        .and(json_body())
        .and(with_db(db))
        .and_then(handlers::create_batch_h)
}

/// DELETE /batch/:id
pub fn delete_batch(
    db: Db,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("batch" / String)
        .and(warp::delete())
        .and(with_db(db))
        .and_then(handlers::delete_batch_h)
}

fn with_db(db: Db) -> impl Filter<Extract = (Db,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || db.clone())
}

fn json_body() -> impl Filter<Extract = (Cmd,), Error = warp::Rejection> + Clone {
    // When accepting a body, we want a JSON body
    // (and to reject huge payloads)...
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}
