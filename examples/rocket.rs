use db_api::mounter::rocket_other::RocketHandlerOther;
use db_api::retriever::rocket::{RocketRetriever, RocketRetrieverError};
use db_api::retriever::Retriever;
use db_api::retriever::{BodyRetriever, DeserializeRetriever, UniqueStateRetriever};
use rocket::http::Method;
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
}

fn handle_count(counter: &Counter) -> String {
    let new_val = counter.count();
    format!("This page has been visited {} times", new_val)
}

fn retrievers_count() -> UniqueStateRetriever<Counter> {
    UniqueStateRetriever::new()
}

fn test_ir<'a, 'r>(
    ir: impl Retriever<RocketRetriever<'a, 'r>, RocketRetrieverError, Output = &'r Counter> + 'static,
) {
}

fn main() {
    let my_handler = RocketHandlerOther::new(Method::Get, "/test".into(), handle, retrievers);
    let my_handler_route = Route::new(Method::Get, "/test", my_handler);

    let my_handler_a =
        RocketHandlerOther::new(Method::Get, "/testa".into(), handle_a, retrievers_a);
    let my_handler_a_route = Route::new(Method::Get, "/testa", my_handler_a);

    let my_handler_str =
        RocketHandlerOther::new(Method::Get, "/teststr".into(), handle_str, retrievers_str);
    let my_handler_str_route = Route::new(Method::Get, "/teststr", my_handler_str);

    let my_handler_counter =
        RocketHandlerOther::new(Method::Get, "/count".into(), handle_count, retrievers_count);
    let my_handler_counter_route = Route::new(Method::Get, "/count", my_handler_counter);

    let rocket = Rocket::ignite()
        .manage(Counter::new())
        .mount(
            "/",
            vec![
                my_handler_route,
                my_handler_str_route,
                my_handler_a_route,
                //my_handler_counter_route,
            ],
        )
        .launch();
}
