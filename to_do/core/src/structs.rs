//! File: to_do/core/src/structs.rs
use std::fmt;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use crate::enums::TaskStatus;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ToDoItem {
    pub title: String,
    pub status: TaskStatus
}

// File: to_do/core/src/structs.rs
impl fmt::Display for ToDoItem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.status {
            TaskStatus::PENDING => write!(
                f, "Pending: {}",
                self.title
            ),
            TaskStatus::DONE => write!(
                f, "Done: {}",
                self.title
            ),
        }
    }
}

// File: to_do/core/src/structs.rs
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AllToDOItems {
    pub pending: Vec<ToDoItem>,
    pub done: Vec<ToDoItem>
}

// File: to_do/core/src/structs.rs
impl AllToDOItems {
    pub fn from_hashmap(all_items: HashMap<String, ToDoItem>)
        -> AllToDOItems {
        let mut pending = Vec::new();
        let mut done = Vec::new();
        for (_, item) in all_items {
            match item.status {
                TaskStatus::PENDING => pending.push(item),
                TaskStatus::DONE => done.push(item)
            }
        }
        AllToDOItems {
            pending,
            done
        }
    }
}