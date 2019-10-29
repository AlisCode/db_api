use std::sync::Mutex;

use serde::{Deserialize, Serialize};

#[derive(Queryable)]
pub struct Hero {
    id: i32,
    year: i32,
    name: String,
    power: String,
}

/// Test struct to deserialize
#[derive(Serialize, Deserialize, Clone)]
pub struct A {
    pub val: u32,
    other: u32,
}

/// Basic counter implementation using interior mutability
pub struct Counter {
    val: Mutex<u32>,
}

impl Counter {
    pub fn new() -> Self {
        Counter { val: Mutex::new(0) }
    }

    pub fn count(&self) -> u32 {
        let mut val = self.val.lock().unwrap();
        *val += 1;
        *val
    }

    pub fn add_val(&self, v: u32) -> u32 {
        let mut val = self.val.lock().unwrap();
        *val += v;
        *val
    }
}
