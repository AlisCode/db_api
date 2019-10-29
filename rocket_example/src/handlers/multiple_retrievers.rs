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

#[cfg(test)]
pub mod tests {
    use super::endpoint_multiple_retrievers;
    use crate::Counter;

    use db_api::mounter::rocket::RocketMounter;
    use db_api::mounter::Mounter;
    use rocket::local::Client;
    use rocket::Rocket;

    use std::sync::Arc;

    #[test]
    fn test_endpoint_multiple_retrievers() {
        // Mounts the endpoint
        let mut mounter = RocketMounter::new(Rocket::ignite().manage(Arc::new(Counter::new())));
        mounter.mount_service(endpoint_multiple_retrievers());
        let rocket = mounter.finish();
        let client = Client::new(rocket).expect("The instance of Rocket should be valid");

        // Sends the GET request
        let req = client
            .post("/count_deser")
            .body("{ \"val\": 1, \"other\": 2}");
        let mut res = req.dispatch();

        assert_eq!(res.status(), rocket::http::Status::Ok);
        assert_eq!(
            res.body_string().expect("No body"),
            "This common count is 1"
        );

        // Sends the GET request
        let req = client
            .post("/count_deser")
            .body("{ \"val\": 2, \"other\": 5}");
        let mut res = req.dispatch();

        assert_eq!(res.status(), rocket::http::Status::Ok);
        assert_eq!(
            res.body_string().expect("No body"),
            "This common count is 3"
        );
    }
}
