use db_api::endpoint::GenericEndpoint;
use db_api::mounter::rocket::RocketMounter;
use db_api::service::Service;
use db_api::Method;

/// Simple handler (Unit retrievers)
fn handle(_unit: ()) -> String {
    "Handled!".to_owned()
}

fn retrievers() {}

pub fn endpoint_void<'a, 'r: 'a>() -> impl Service<RocketMounter<'a, 'r>> {
    GenericEndpoint::new("/test".into(), Method::GET, handle, retrievers).rocket()
}
