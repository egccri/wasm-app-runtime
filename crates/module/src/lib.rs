use std::collections::HashMap;
use std::path::PathBuf;

/// One app contains multi egccri module, one egccri module contains multi invokers and components.
pub struct EgccriModule {
    metadata: HashMap<String, String>,
    config: HashMap<String, String>,
    invokers: Vec<Invoker>,
    components: Vec<Component>,
}

/// A String-keyed map with deterministic serialization order.
pub type LockedMap<T> = std::collections::BTreeMap<String, T>;

/// Metadata that fetch and initial invoker.
pub struct Invoker {
    /// Application-unique invoker identifier
    pub id: String,
    /// Invoker type (e.g. "grpc")
    pub invoker_type: String,
    /// Invoker config
    pub config: HashMap<String, String>,
}

pub struct Component {
    id: String,
    metadata: HashMap<String, String>,
    source: PathBuf,
    // wasi
    env: HashMap<String, String>,
    files: Vec<PathBuf>,
    config: HashMap<String, String>,
}
