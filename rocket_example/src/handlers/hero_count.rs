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
