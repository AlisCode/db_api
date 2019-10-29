use db_api::endpoint::GenericEndpoint;
use db_api::mounter::rocket::RocketMounter;
use db_api::retriever::UniqueStateRetriever;
use db_api::service::Service;
use db_api::Method;

use crate::models::Counter;

use std::sync::Arc;

/// Counts the number of time we've been on a particular page
fn handle_count(counter: Arc<Counter>) -> String {
    let new_val = counter.count();
    format!("This page has been visited {} times", new_val)
}

/// Retrievers for the counter endpoint
fn retrievers_count() -> UniqueStateRetriever<Counter> {
    UniqueStateRetriever::new()
}

/// Generates the counter endpoint
pub fn endpoint_counter<'a, 'r: 'a>() -> impl Service<RocketMounter<'a, 'r>> {
    GenericEndpoint::new("/count".into(), Method::GET, handle_count, retrievers_count).rocket()
}
