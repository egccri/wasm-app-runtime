use micro_async_module::{run_async_block_on, AsyncRuntime, Module};
use sled::Error;
use std::sync::{Arc, OnceLock};
use thiserror::Error;

static POLL: OnceLock<Arc<sled::Db>> = OnceLock::new();

const MODULE_NAME: &str = "kv-storage";

#[derive(Debug)]
pub struct StorageSled;

impl Module for StorageSled {
    fn name(&self) -> &str {
        MODULE_NAME
    }

    fn start(&self, runtime: AsyncRuntime) {
        run_async_block_on(init(), runtime);
    }
}

async fn init() {
    tracing::info!("Init sled kv storage");
    let registry_db: sled::Db = sled::open("my_db").unwrap();
    POLL.set(Arc::new(registry_db))
        .expect("Create kv storage error!");
}

async fn initial_table() {
    tracing::info!("Test sled kv storage.");
    let db = POLL.get().unwrap();
    db.insert(b"yo!", b"v1").unwrap();
    assert_eq!(&db.get(b"yo!").unwrap().unwrap(), b"v1");
}

#[derive(Error, Debug)]
pub enum StorageError {
    #[error("storage sled error from {0:?}")]
    SledError(#[from] Error),
}
