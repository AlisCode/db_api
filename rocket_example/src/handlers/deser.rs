use crate::models::A;

use db_api::endpoint::GenericEndpoint;
use db_api::mounter::rocket::RocketMounter;
use db_api::retriever::DeserializeRetriever;
use db_api::service::Service;
use db_api::Method;

/// Showcases the deserialization of a struct
fn handle_a(a: A) -> String {
    format!("val is {}", a.val)
}

/// Retrievers for the deserialize endpoint
fn retrievers_a() -> DeserializeRetriever<A> {
    DeserializeRetriever::new()
}

/// Generates the deserialization endpoint
pub fn endpoint_deser<'a, 'r: 'a>() -> impl Service<RocketMounter<'a, 'r>> {
    GenericEndpoint::new("/deser".into(), Method::POST, handle_a, retrievers_a).rocket()
}

#[cfg(test)]
pub mod tests {
    use super::endpoint_deser;

    use db_api::mounter::rocket::RocketMounter;
    use db_api::mounter::Mounter;
    use rocket::local::Client;
    use rocket::Rocket;

    #[test]
    fn test_endpoint_deser() {
        // Mounts the endpoint
        let mut mounter = RocketMounter::new(Rocket::ignite());
        mounter.mount_service(endpoint_deser());
        let rocket = mounter.finish();
        let client = Client::new(rocket).expect("The instance of Rocket should be valid");

        // Sends the POST request
        let req = client.post("/deser").body("{ \"val\": 1, \"other\": 2 }");
        let mut res = req.dispatch();

        assert_eq!(res.status(), rocket::http::Status::Ok);
        assert_eq!(res.body_string().expect("No body"), "val is 1");
    }

    #[test]
    fn test_endpoint_fail_deser() {
        // Mounts the endpoint
        let mut mounter = RocketMounter::new(Rocket::ignite());
        mounter.mount_service(endpoint_deser());
        let rocket = mounter.finish();
        let client = Client::new(rocket).expect("The instance of Rocket should be valid");

        // Sends the POST request
        let req = client.post("/deser").body("{ val: 1, other: 2 }");
        let res = req.dispatch();

        assert_eq!(res.status(), rocket::http::Status::InternalServerError);

        // Sends the POST request
        let req = client.post("/deser").body("{ \"value\": 1, \"other\": 2 }");
        let res = req.dispatch();

        assert_eq!(res.status(), rocket::http::Status::InternalServerError);

        // Sends the POST request
        let req = client.post("/deser").body("blah");
        let res = req.dispatch();

        assert_eq!(res.status(), rocket::http::Status::InternalServerError);
    }
}
