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

    use super::endpoint_body;
    use db_api::mounter::rocket::RocketMounter;
    use db_api::mounter::Mounter;
    use rocket::local::Client;
    use rocket::Rocket;

    #[test]
    fn test_endpoint_body() {
        // Mounts the endpoint
        let mut mounter = RocketMounter::new(Rocket::ignite());
        mounter.mount_service(endpoint_body());
        let rocket = mounter.finish();
        let client = Client::new(rocket).expect("The instance of Rocket should be valid");

        // Sends the GET request
        let req = client.get("/test_str").body("test!");
        let mut res = req.dispatch();

        // Checks results
        assert_eq!(res.status(), rocket::http::Status::Ok);
        assert_eq!(
            res.body_string().expect("No body on response"),
            "Handled : test!"
        );
    }
}
