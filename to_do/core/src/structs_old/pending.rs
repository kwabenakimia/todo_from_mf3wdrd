//! File: to_do/core/src/structs/pending.rs
use super::super::enums::TaskStatus;
use super::base::Base;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Pending {
    pub super_struct: Base,
}

impl Pending {
    pub fn new(input_title: &str) -> Pending {
        let base = Base {
            title: input_title.to_string(),
            status: TaskStatus::PENDING,
        };

        Pending { super_struct: base }
    }
}
