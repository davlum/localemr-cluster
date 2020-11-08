use crate::models::{Db,Batch,Status,LOG_DIR};
use log;
use std::fs;
use std::convert::Infallible;
use warp::http::StatusCode;
use std::thread;
use std::process::Command;
use futures::executor::block_on;
use warp::Reply;

/// These are our API handlers, the ends of each filter chain.
/// Notice how thanks to using `Filter::and`, we can define a function
/// with the exact arguments we'd expect from each filter in the chain.
/// No tuples are needed, it's auto flattened for the functions.
pub async fn list_batch_h(db: Db) -> Result<impl warp::Reply, Infallible> {
    // Just return a JSON array of todos, applying the limit and offset.
    let batches = db.lock().await;
    let batches: Vec<Batch> = batches.clone();
    Ok(warp::reply::json(&batches))
}

pub async fn get_batch_h(id: String, db: Db) -> Result<impl warp::Reply, warp::Rejection> {
    log::debug!("create_batch: {:?}", id);

    let vec = db.lock().await;

    for batch in vec.iter() {
        if batch.id == id {
            return Ok(warp::reply::json(&batch).into_response());
        }
    }

    return Ok(warp::reply::with_status("Batch not found.", StatusCode::NOT_FOUND)
        .into_response());
}


async fn add_batch_to_db(batch: Batch, db: &Db) -> Option<Batch> {
    let mut vec = db.lock().await;
    for b in vec.iter() {
        if batch.id == b.id {
            log::debug!("ID already exists: {}", batch.id);
            return None
        }
    }
    let batch = Batch {
        id: batch.id,
        exec: batch.exec,
        args: batch.args,
        status: Some(Status::PENDING),
        log: None,
    };

    // No existing Batch with id, so insert and return it.
    vec.push(batch.clone());
    Some(batch)
}

pub async fn create_batch_h(batch: Batch, db: Db) -> Result<impl warp::Reply, warp::Rejection> {
    log::debug!("create_batch: {:?}", batch);
    return match add_batch_to_db(batch, &db).await {
        None => Ok(warp::reply::with_status("ID already used.", StatusCode::BAD_REQUEST)
            .into_response()),
        Some(batch) => {
            let bc = batch.clone();
            thread::spawn(move || {
                log::debug!("Inside thread.");
                block_on(run_batch(db, batch));
            });
            Ok(warp::reply::json(&bc).into_response())
        }
    };
}

async fn set_batch_state(db: &Db, batch: &Batch, state: Status) {
    let mut vec = db.lock().await;

    for b in vec.iter_mut() {
        if b.id == batch.id {
            b.set_status(state);
        }
    }
}

async fn set_batch_log(db: &Db, batch: &Batch, log: String) {
    let mut vec = db.lock().await;

    for b in vec.iter_mut() {
        if b.id == batch.id {
            b.set_log(log.clone());
        }
    }
}

async fn run_batch(db: Db, batch: Batch) {
    let db = &db;
    let batch = &batch;
    set_batch_state(db, batch, Status::RUNNING).await;

    log::debug!("run_batch: id={}", batch.id);
    let output = Command::new(&batch.exec)
        .args(&batch.args[..])
        .output()
        .expect("failed to execute process");

    let log_dir = [LOG_DIR, batch.id.as_str()].concat();
    let stderr = &[log_dir.clone(), "/stderr.log".to_string()].concat();
    let stdout = &[log_dir.clone(), "/stdout.log".to_string()].concat();

    fs::create_dir_all(log_dir.clone()).unwrap();
    fs::write(stderr, &output.stdout).unwrap();
    fs::write(stdout, &output.stderr).unwrap();
    if output.status.success() {
        set_batch_state(db, batch, Status::SUCCEEDED).await;
        let log = fs::read_to_string(stdout).unwrap();
        set_batch_log(db, batch, log).await;
        log::debug!("Batch {} Succeeded", batch.id);
    } else {
        set_batch_state(db, batch, Status::FAILED).await;
        let log = fs::read_to_string(stderr).unwrap();
        set_batch_log(db, batch, log).await;
        log::debug!("Batch {} Failed", batch.id);
    }
}


pub async fn delete_batch_h(id: String, db: Db) -> Result<impl warp::Reply, Infallible> {
    log::debug!("delete_batch: id={}", id);

    let mut vec = db.lock().await;

    let len = vec.len();
    vec.retain(|batch| {
        // Retain all batches that aren't this id...
        // In other words, remove all that *are* this id...
        batch.id != id
    });

    // If the vec is smaller, we found and deleted a Batch!
    let deleted = vec.len() != len;

    if deleted {
        // respond with a `204 No Content`, which means successful,
        // yet no body expected...
        Ok(StatusCode::NO_CONTENT)
    } else {
        log::debug!("{} -> batch id not found!", id);
        Ok(StatusCode::NOT_FOUND)
    }
}
