use db_api::endpoint::Endpoint;
use db_api::mounter::rocket::RocketMounter;
use db_api::mounter::Mounter;
use db_api::retriever::{BodyRetriever, DeserializeRetriever, UniqueStateRetriever};
use http::Method;
use rocket::response::Responder;
use rocket::{Rocket, Route};
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
    /*
    let my_handler = RocketHandlerOther::new(Method::Get, "/test".into(), handle, retrievers);
    let my_handler_route = Route::new(Method::Get, "/test", my_handler);

    let my_handler_a =
        RocketHandlerOther::new(Method::Get, "/testa".into(), handle_a, retrievers_a);
    let my_handler_a_route = Route::new(Method::Get, "/testa", my_handler_a);

    let my_handler_str =
        RocketHandlerOther::new(Method::Get, "/teststr".into(), handle_str, retrievers_str);
    let my_handler_str_route = Route::new(Method::Get, "/teststr", my_handler_str);

    let retriever: UniqueStateRetriever<Counter> = UniqueStateRetriever::new();

    let my_handler_counter =
        RocketHandlerOther::new(Method::Get, "/count".into(), handle_count, retrievers_count);
    let my_handler_counter_route = Route::new(Method::Get, "/count", my_handler_counter);

    let my_handler_counter_deser = RocketHandlerOther::new(
        Method::Post,
        "/count_deser".into(),
        handle_count_deser,
        retrievers_count_deser,
    );
    let my_handler_counter_deser_route =
        Route::new(Method::Post, "/count_deser", my_handler_counter_deser);
    */
    let endpoint_test = Endpoint::new_rocket("/test".into(), Method::GET, handle, retrievers);
    let endpoint_deser_a =
        Endpoint::new_rocket("/deser".into(), Method::POST, handle_a, retrievers_a);
    let endpoint_counter =
        Endpoint::new_rocket("/count".into(), Method::GET, handle_count, retrievers_count);
    let endpoint_counter_deser = Endpoint::new_rocket(
        "/count_deser".into(),
        Method::POST,
        handle_count_deser,
        retrievers_count_deser,
    );

    let rocket = Rocket::ignite().manage(Arc::new(Counter::new()));
    let mounter = RocketMounter::new(rocket);
    mounter.mount_service(endpoint_test);
    mounter.mount_service(endpoint_deser_a);
    mounter.mount_service(endpoint_counter);
    mounter.mount_service(endpoint_counter_deser);
}
