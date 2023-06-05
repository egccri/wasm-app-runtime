// store component with local file

// with registry struct and type

use std::path::PathBuf;

pub enum ComponentSource {
    Registry(String),
    Local(PathBuf),
}
