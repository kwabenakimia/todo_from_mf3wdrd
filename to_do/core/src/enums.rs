//! File: to_do/core/src/enums.rs
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum TaskStatus {
    DONE,
    PENDING,
}

impl fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            &Self::DONE => write!(f, "Done"),
            &Self::PENDING => write!(f, "Pending"),
        }
    }
}

impl TaskStatus {
    pub fn from_string(status: &String) -> Result<TaskStatus, String> {
        match status.to_uppercase().as_str() {
            "DONE" => Ok(Self::DONE),
            "PENDING" => Ok(Self::PENDING),
            _ => Err(format!("Invalid status: {status}")),
        }
    }
}
