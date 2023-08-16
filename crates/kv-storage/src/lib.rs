use micro_async_module::{run_async_block_on, AsyncRuntime, Module};
use sled::Error;
use std::str::Utf8Error;
use std::sync::{Arc, OnceLock};
use thiserror::Error;
use wasmtime::component::__internal::async_trait;

wasmtime::component::bindgen!({path: "../../wit/kv/default", async: true});

#[async_trait]
impl crate::key_vlaue::Host for WasmtimeKvSled {
    async fn get(&mut self, key: String) -> wasmtime::Result<Result<String, key_vlaue::Error>> {
        todo!()
    }

    async fn set(
        &mut self,
        key: String,
        value: String,
    ) -> wasmtime::Result<Result<(), key_vlaue::Error>> {
        todo!()
    }
}

#[derive(Debug, Clone)]
pub struct WasmtimeKvSled;

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
    #[error("Storage sled error from {0:?}")]
    SledError(#[from] Error),

    #[error("Can not find db")]
    GetDBNoneError,

    #[error(transparent)]
    Utf8Error(#[from] Utf8Error),

    #[error("Not found key: {0}")]
    KeyNotFoundError(String),
}

pub fn set(key: String, value: String) -> Result<(), StorageError> {
    let db = POLL.get().ok_or_else(|| StorageError::GetDBNoneError)?;
    db.insert(key, value.as_str())
        .map_err(|err| StorageError::SledError(err))?;
    Ok(())
}

pub fn get(key: String) -> Result<String, StorageError> {
    let db = POLL.get().ok_or_else(|| StorageError::GetDBNoneError)?;
    let value = db.get(&key).map_err(|err| StorageError::SledError(err))?;
    if let Some(value) = value {
        let s = std::str::from_utf8(&value)?;
        Ok(s.to_string())
    } else {
        Err(StorageError::KeyNotFoundError(key))
    }
}
