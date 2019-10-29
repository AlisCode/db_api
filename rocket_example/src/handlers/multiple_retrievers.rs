use crate::models::{Counter, A};

use db_api::endpoint::GenericEndpoint;
use db_api::mounter::rocket::RocketMounter;
use db_api::retriever::{DeserializeRetriever, UniqueStateRetriever};
use db_api::service::Service;
use db_api::Method;

use std::sync::Arc;

/// Showcases multiple retrievers by incrementing the counter with the
/// given value in A
fn handle_count_deser((counter, a): (Arc<Counter>, A)) -> String {
    let new_val = counter.add_val(a.val);
    format!("This common count is {}", new_val)
}

/// Multiple retrievers
fn retrievers_count_deser() -> (UniqueStateRetriever<Counter>, DeserializeRetriever<A>) {
    (UniqueStateRetriever::new(), DeserializeRetriever::new())
}

/// Generates the "multiple retrievers" endpoint
pub fn endpoint_multiple_retrievers<'a, 'r: 'a>() -> impl Service<RocketMounter<'a, 'r>> {
    GenericEndpoint::new(
        "/count_deser".into(),
        Method::POST,
        handle_count_deser,
        retrievers_count_deser,
    )
    .rocket()
}
