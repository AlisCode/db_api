use db_api::endpoint::GenericEndpoint;
use db_api::mounter::rocket::RocketMounter;
use db_api::service::Service;
use db_api::Method;

use diesel::prelude::*;

use crate::models::Hero;
use crate::schema;
use crate::{Conn, DbRetriever, MyDbRetriever};

/// Showcases a connection to the database by counting
/// the number of registered heroes
fn handle_hero_count(conn: Conn) -> String {
    let heroes: Vec<Hero> = schema::hero::table.load(&conn).unwrap();
    format!("there are {} heroes", heroes.len())
}

/// Retrievers for database connection
fn retrievers_hero_count() -> MyDbRetriever {
    DbRetriever::new()
}

/// Generates the count_heroes endpoint
pub fn endpoint_count_heroes<'a, 'r: 'a>() -> impl Service<RocketMounter<'a, 'r>> {
    GenericEndpoint::new(
        "/heroes/count".into(),
        Method::GET,
        handle_hero_count,
        retrievers_hero_count,
    )
    .rocket()
}

/// Don't run on CI because postgres is not setup accordingly on CI for now
#[cfg(debug_assertions)]
#[cfg(test)]
pub mod tests {
    use super::endpoint_count_heroes;
    use crate::ExampleDb;

    use db_api::mounter::rocket::RocketMounter;
    use db_api::mounter::Mounter;
    use rocket::local::Client;
    use rocket::Rocket;

    #[test]
    fn test_endpoint_hero_count() {
        // Mounts the endpoint
        let mut mounter = RocketMounter::new(Rocket::ignite().attach(ExampleDb::fairing()));
        mounter.mount_service(endpoint_count_heroes());
        let rocket = mounter.finish();
        let client = Client::new(rocket).expect("The instance of Rocket should be valid");

        // Sends the GET request
        let req = client.get("/heroes/count");
        let mut res = req.dispatch();

        assert_eq!(res.status(), rocket::http::Status::Ok);
        assert_eq!(res.body_string().expect("No body"), "there are 2 heroes");
    }
}
