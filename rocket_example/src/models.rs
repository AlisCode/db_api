use std::sync::Mutex;

use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize)]
/// Simple Hero Queryable representation
pub struct Hero {
    pub id: i32,
    pub year: i32,
    pub name: String,
    pub power: String,
}

/// Test struct to deserialize
#[derive(Serialize, Deserialize, Clone)]
pub struct A {
    pub val: u32,
    other: u32,
}

/// Basic counter implementation using interior mutability
pub struct Counter {
    /// The counter value to be incremented
    val: Mutex<u32>,
}

impl Counter {
    /// Creates a new instance of counter
    pub fn new() -> Self {
        Counter { val: Mutex::new(0) }
    }

    /// Safely counts one by one
    pub fn count(&self) -> u32 {
        let mut val = self.val.lock().unwrap();
        *val += 1;
        *val
    }

    /// Safely adds a value to the counter
    pub fn add_val(&self, v: u32) -> u32 {
        let mut val = self.val.lock().unwrap();
        *val += v;
        *val
    }
}
