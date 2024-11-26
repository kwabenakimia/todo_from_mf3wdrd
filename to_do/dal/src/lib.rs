//! File: to_do/dal/src/lib.rs

// here we're compiling the declaration of our jsoin-file module if the json-file feature
// is enabled
#[cfg(feature = "json-file")]
pub mod json_file;
