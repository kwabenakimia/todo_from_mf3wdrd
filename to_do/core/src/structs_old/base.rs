//! File: to_do/core/src/structs/base.rs
use super::super::enums::TaskStatus;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Base {
    pub title: String,
    pub status: TaskStatus,
}
