#[cfg(test)]
use warp::http::StatusCode;
use warp::test::request;
use std::{str, thread, time};

use super::{
    filters,
    models::{self, Batch, Status},
};

#[tokio::test]
async fn test_post_batch_succeeds() {
    let db = models::blank_db();
    let api = filters::batch(db);
    let batch_id = "sleep-1";

    let resp = request()
        .method("POST")
        .path("/batch")
        .json(&Batch {
            id: batch_id.to_string(),
            exec: "sleep".to_string(),
            args: vec!["10".to_string()],
            status: None,
            log: None
        })
        .reply(&api)
        .await;

    let batch = Batch {
        id: batch_id.to_string(),
        exec: "sleep".to_string(),
        args: vec!["10".to_string()],
        status: Some(Status::PENDING),
        log: None,
    };
    let three_secs = time::Duration::from_secs(3);
    let batch_string = str::from_utf8(&*resp.body()).unwrap();
    let mut deserialized: Batch = serde_json::from_str(batch_string).unwrap();
    assert_eq!(deserialized, batch);
    while deserialized.status == Some(Status::PENDING) {
        thread::sleep(three_secs);
        let resp = request()
            .path("/batch/sleep-1")
            .reply(&api)
            .await;
        let batch_string = str::from_utf8(&*resp.body()).unwrap();
        deserialized = serde_json::from_str(batch_string).unwrap();
    }
    assert_eq!(deserialized.status, Some(Status::RUNNING));
    while deserialized.status == Some(Status::RUNNING) {
        thread::sleep(three_secs);
        let resp = request()
            .path("/batch/sleep-1")
            .reply(&api)
            .await;
        let batch_string = str::from_utf8(&*resp.body()).unwrap();
        deserialized = serde_json::from_str(batch_string).unwrap();
    }
    assert_eq!(deserialized.status, Some(Status::SUCCEEDED));
}

#[tokio::test]
async fn test_post_batch_fails() {
    let db = models::blank_db();
    let api = filters::batch(db);
    let batch_id = "sleep-2";

    let args = vec![
        "10".to_string(),
        "&&".to_string(),
        "ls".to_string(),
        "-lthra".to_string()
    ];

    let resp = request()
        .method("POST")
        .path("/batch")
        .json(&Batch {
            id: batch_id.to_string(),
            exec: "sleep".to_string(),
            args: args.clone(),
            status: None,
            log: None
        })
        .reply(&api)
        .await;

    let batch = Batch {
        id: batch_id.to_string(),
        exec: "sleep".to_string(),
        args: args,
        status: Some(Status::PENDING),
        log: None,
    };
    let three_secs = time::Duration::from_secs(3);
    let batch_string = str::from_utf8(&*resp.body()).unwrap();
    let mut deserialized: Batch = serde_json::from_str(batch_string).unwrap();
    assert_eq!(deserialized, batch);
    while deserialized.status == Some(Status::PENDING) {
        thread::sleep(three_secs);
        let resp = request()
            .path("/batch/sleep-2")
            .reply(&api)
            .await;
        let batch_string = str::from_utf8(&*resp.body()).unwrap();
        deserialized = serde_json::from_str(batch_string).unwrap();
    }
    assert_eq!(deserialized.status, Some(Status::FAILED));
}

#[tokio::test]
async fn test_bad_batch() {
    let db = models::blank_db();
    let api = filters::batch(db);
    let batch_id = "sleep-3";

    let resp = request()
        .method("POST")
        .path("/batch/3")
        .json(&Batch {
            id: batch_id.to_string(),
            exec: "sleep".to_string(),
            args: vec!["10".to_string()],
            status: None,
            log: None
        })
        .reply(&api)
        .await;

    assert_eq!(resp.status(), StatusCode::METHOD_NOT_ALLOWED)
}

#[tokio::test]
async fn test_batch_conflicts() {
    let db = models::blank_db();
    let api = filters::batch(db);
    let batch_id = "sleep-2";

    let resp1 = request()
        .method("POST")
        .path("/batch")
        .json(&Batch {
            id: batch_id.to_string(),
            exec: "sleep".to_string(),
            args: vec!["10".to_string()],
            status: None,
            log: None
        })
        .reply(&api)
        .await;

    assert_eq!(resp1.status(), StatusCode::OK);
    let resp2 = request()
        .method("POST")
        .path("/batch")
        .json(&Batch {
            id: batch_id.to_string(),
            exec: "sleep".to_string(),
            args: vec!["10".to_string()],
            status: None,
            log: None
        })
        .reply(&api)
        .await;
    assert_eq!(resp2.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_batch_not_found() {
    let db = models::blank_db();
    let api = filters::batch(db);

    let resp = request()
        .path("/batch/1")
        .reply(&api)
        .await;

    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_health_check() {
    let db = models::blank_db();
    let api = filters::batch(db);

    let resp = request()
        .path("/health")
        .reply(&api)
        .await;

    assert_eq!(resp.status(), StatusCode::OK);
}
