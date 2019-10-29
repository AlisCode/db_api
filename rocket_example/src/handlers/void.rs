use db_api::endpoint::GenericEndpoint;
use db_api::mounter::rocket::RocketMounter;
use db_api::service::Service;
use db_api::Method;

/// Simple handler (Unit type)
fn handle(_unit: ()) -> String {
    "Handled!".to_owned()
}

/// Void retrievers also work
fn retrievers() {}

/// Generates the void endpoint
pub fn endpoint_void<'a, 'r: 'a>() -> impl Service<RocketMounter<'a, 'r>> {
    GenericEndpoint::new("/test".into(), Method::GET, handle, retrievers).rocket()
}

#[cfg(test)]
pub mod tests {

    use super::endpoint_void;
    use db_api::mounter::rocket::RocketMounter;
    use db_api::mounter::Mounter;
    use rocket::local::Client;
    use rocket::Rocket;

    #[test]
    fn test_endpoint_void() {
        // Mounts the endpoint
        let mut mounter = RocketMounter::new(Rocket::ignite());
        mounter.mount_service(endpoint_void());
        let rocket = mounter.finish();
        let client = Client::new(rocket).expect("The instance of Rocket should be valid");

        // Sends the GET request
        let req = client.get("/test");
        let mut res = req.dispatch();

        // Checks results
        assert_eq!(res.status(), rocket::http::Status::Ok);
        assert_eq!(res.body_string().expect("No body on response"), "Handled!");
    }
}
