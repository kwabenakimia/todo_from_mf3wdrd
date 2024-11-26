//! File: to_do/core/src/structs/done.rs
use super::base::Base;
use super::super::enums::TaskStatus;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Done {
    pub super_struct: Base,
}

impl Done {
    pub fn new(input_title: &str) -> Done {
        let base = Base {
            title: input_title.to_string(),
            status: TaskStatus::DONE
        };

        Done {
            super_struct: base
        }
    }
}