use serde_derive::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;

/// So we don't have to tackle how different database work, we'll just use
/// a simple in-memory DB, a vector synchronized by a mutex.
pub type Db = Arc<Mutex<Vec<Batch>>>;

pub fn blank_db() -> Db { Arc::new(Mutex::new(Vec::new())) }

pub static LOG_DIR: &str = "var/log/steps/";

#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
pub enum Status {
    PENDING,
    RUNNING,
    FAILED,
    SUCCEEDED,
    CANCELLED,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct BatchRuntimeInfo {
    pub status: Status,
    pub stderr: String,
    pub stdout: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Batch<'a> {
    pub id: String,
    pub exec: String,
    pub args: Vec<String>,
    pub runtime_info: Option<&'a BatchRuntimeInfo>,
}



impl Batch {
    pub fn set_batch_info(&mut self, status: Status) {
        let mut runtime_info = self.runtime_info;

        self.runtime_info = Some(BatchRuntimeInfo.)
    }
}
