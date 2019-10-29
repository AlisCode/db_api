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

#[cfg(test)]
pub mod tests {
    use super::endpoint_counter;
    use crate::models::Counter;

    use db_api::mounter::rocket::RocketMounter;
    use db_api::mounter::Mounter;
    use rocket::local::Client;
    use rocket::Rocket;

    use std::sync::Arc;

    #[test]
    fn test_endpoint_counter() {
        // Mounts the endpoint and manages the counter in the state
        let mut mounter = RocketMounter::new(Rocket::ignite().manage(Arc::new(Counter::new())));
        mounter.mount_service(endpoint_counter());
        let rocket = mounter.finish();
        let client = Client::new(rocket).expect("The instance of Rocket should be valid");

        (1..10).for_each(|i| {
            // Sends the GET request
            let req = client.get("/count");
            let mut res = req.dispatch();

            // Checks the results
            assert_eq!(res.status(), rocket::http::Status::Ok);
            assert_eq!(
                res.body_string().expect("No body"),
                format!("This page has been visited {} times", i)
            );
        });
    }

    #[test]
    fn test_endpoint_counter_unmanaged() {
        // Mounts the endpoint but doesnt manage the counter in the state
        let mut mounter = RocketMounter::new(Rocket::ignite());
        mounter.mount_service(endpoint_counter());
        let rocket = mounter.finish();
        let client = Client::new(rocket).expect("The instance of Rocket should be valid");

        // Sends the GET request
        let req = client.get("/count");
        let res = req.dispatch();

        // Checks the results
        assert_eq!(res.status(), rocket::http::Status::InternalServerError);
    }
}
