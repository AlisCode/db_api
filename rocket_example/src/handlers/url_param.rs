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

#[cfg(test)]
pub mod tests {

    use super::endpoint_url_param;
    use db_api::mounter::rocket::RocketMounter;
    use db_api::mounter::Mounter;
    use rocket::local::Client;
    use rocket::Rocket;

    #[test]
    fn test_endpoint_body() {
        // Mounts the endpoint
        let mut mounter = RocketMounter::new(Rocket::ignite());
        mounter.mount_service(endpoint_url_param());
        let rocket = mounter.finish();
        let client = Client::new(rocket).expect("The instance of Rocket should be valid");

        // Sends the GET request
        let req = client.get("/param/1");
        let mut res = req.dispatch();

        // Checks results
        assert_eq!(res.status(), rocket::http::Status::Ok);
        assert_eq!(
            res.body_string().expect("No body on response"),
            "The value is 1"
        );

        // Sends the GET request
        let req = client.get("/param/abc");
        let res = req.dispatch();

        // Checks results
        assert_eq!(res.status(), rocket::http::Status::InternalServerError);
    }
}
