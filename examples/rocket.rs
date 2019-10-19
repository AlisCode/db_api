use db_api::endpoint::GenericEndpoint;
use db_api::mounter::rocket::RocketMounter;
use db_api::mounter::Mounter;
use db_api::retriever::{BodyRetriever, DeserializeRetriever, UniqueStateRetriever};
use http::Method;
use rocket::Rocket;
use serde::{Deserialize, Serialize};

use std::sync::{Arc, Mutex};

// Simple handler (Unit retrievers)
fn handle(_unit: ()) -> String {
    "Handled!".to_owned()
}
fn retrievers() -> () {}

// Deserialize handler
#[derive(Serialize, Deserialize, Clone)]
struct A {
    pub val: u32,
    other: u32,
}

fn handle_a(a: A) -> String {
    format!("val is {}", a.val)
}

fn retrievers_a() -> DeserializeRetriever<A> {
    DeserializeRetriever::new()
}

// Raw content handler
fn handle_str(my_str: String) -> String {
    format!("Handled : {}", my_str)
}

fn retrievers_str() -> BodyRetriever<String> {
    BodyRetriever::new()
}

// State handler
#[derive(Clone)]
struct Counter {
    val: Arc<Mutex<u32>>,
}

impl Counter {
    pub fn new() -> Self {
        Counter {
            val: Arc::new(Mutex::new(0)),
        }
    }

    pub fn count(&self) -> u32 {
        let mut val = self.val.lock().unwrap();
        *val = *val + 1;
        *val
    }

    pub fn add_val(&self, v: u32) -> u32 {
        let mut val = self.val.lock().unwrap();
        *val = *val + v;
        *val
    }
}

// State retriever
fn handle_count(counter: Arc<Counter>) -> String {
    let new_val = counter.count();
    format!("This page has been visited {} times", new_val)
}

fn retrievers_count() -> UniqueStateRetriever<Counter> {
    UniqueStateRetriever::new()
}

// Multiple retrievers
fn handle_count_deser((counter, a): (Arc<Counter>, A)) -> String {
    let new_val = counter.add_val(a.val);
    format!("This common count is {}", new_val)
}

fn retrievers_count_deser() -> (UniqueStateRetriever<Counter>, DeserializeRetriever<A>) {
    (UniqueStateRetriever::new(), DeserializeRetriever::new())
}

fn main() {
    let endpoint_test = GenericEndpoint::new("/test".into(), Method::GET, handle, retrievers);
    let endpoint_str =
        GenericEndpoint::new("/test_str".into(), Method::GET, handle_str, retrievers_str);
    let endpoint_deser_a =
        GenericEndpoint::new("/deser".into(), Method::POST, handle_a, retrievers_a);
    let endpoint_counter =
        GenericEndpoint::new("/count".into(), Method::GET, handle_count, retrievers_count);
    let endpoint_counter_deser = GenericEndpoint::new(
        "/count_deser".into(),
        Method::POST,
        handle_count_deser,
        retrievers_count_deser,
    );

    let rocket = Rocket::ignite().manage(Arc::new(Counter::new()));
    let mut mounter = RocketMounter::new(rocket);
    mounter.mount_service(endpoint_test.rocket());
    mounter.mount_service(endpoint_str.rocket());
    mounter.mount_service(endpoint_deser_a.rocket());
    mounter.mount_service(endpoint_counter.rocket());
    mounter.mount_service(endpoint_counter_deser.rocket());
    let rocket = mounter.finish();
    rocket.launch();
}
