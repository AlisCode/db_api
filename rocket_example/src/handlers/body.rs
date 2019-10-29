//! NOTE: This module is only to be compiled on debug mode
//! since FromData is not implemented for String on release mode

use db_api::endpoint::GenericEndpoint;
use db_api::mounter::rocket::RocketMounter;
use db_api::retriever::BodyRetriever;
use db_api::service::Service;
use db_api::Method;

/// Raw content handler
fn handle_str(my_str: String) -> String {
    format!("Handled : {}", my_str)
}

/// Retriever for HTTP Request's Body
fn retrievers_str() -> BodyRetriever<String> {
    BodyRetriever::new()
}

/// Generates the body endpoint
pub fn endpoint_body<'a, 'r: 'a>() -> impl Service<RocketMounter<'a, 'r>> {
    GenericEndpoint::new("/test_str".into(), Method::GET, handle_str, retrievers_str).rocket()
}

#[cfg(test)]
#[cfg(debug_assertions)]
pub mod tests {
    #[test]
    fn it_works() {}
}
