use db_api::endpoint::GenericEndpoint;
use db_api::mounter::rocket::RocketMounter;
use db_api::retriever::IndexedParamRetriever;
use db_api::service::Service;
use db_api::Method;

/// Showcases URL parsing
fn handle_url_param(a: u32) -> String {
    format!("The value is {}", a)
}

/// Retrievers for the URL Param endpoint
fn retrievers_url_param() -> IndexedParamRetriever<u32> {
    IndexedParamRetriever::new(1)
}

/// Generates the URL Param endpoint
pub fn endpoint_url_param<'a, 'r: 'a>() -> impl Service<RocketMounter<'a, 'r>> {
    GenericEndpoint::new(
        "/param/<id>".into(),
        Method::GET,
        handle_url_param,
        retrievers_url_param,
    )
    .rocket()
}
