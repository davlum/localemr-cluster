use serde_derive::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;

/// So we don't have to tackle how different database work, we'll just use
/// a simple in-memory DB, a vector synchronized by a mutex.
pub type Db = Arc<Mutex<Vec<Batch>>>;

pub fn blank_db() -> Db { Arc::new(Mutex::new(Vec::new())) }

pub static LOG_DIR: &str = "/var/log/steps/";

#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    PENDING,
    RUNNING,
    FAILED,
    SUCCEEDED,
    CANCELLED,
}


#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct Batch {
    pub id: String,
    pub exec: String,
    pub args: Vec<String>,
    pub status: Option<Status>,
    pub log: Option<String>,
}



impl Batch {
    pub fn set_status(&mut self, status: Status) {
        self.status = Some(status);
    }
    pub fn set_log(&mut self, log: String) {
        self.log = Some(log);
    }
}
