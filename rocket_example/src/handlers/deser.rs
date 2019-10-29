use crate::models::A;

use db_api::endpoint::GenericEndpoint;
use db_api::mounter::rocket::RocketMounter;
use db_api::retriever::DeserializeRetriever;
use db_api::service::Service;
use db_api::Method;

fn handle_a(a: A) -> String {
    format!("val is {}", a.val)
}

fn retrievers_a() -> DeserializeRetriever<A> {
    DeserializeRetriever::new()
}

pub fn endpoint_deser<'a, 'r: 'a>() -> impl Service<RocketMounter<'a, 'r>> {
    GenericEndpoint::new("/deser".into(), Method::POST, handle_a, retrievers_a).rocket()
}
