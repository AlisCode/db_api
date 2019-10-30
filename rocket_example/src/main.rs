#[macro_use]
extern crate diesel;

use db_api::database::ConnectionRetriever;
use db_api::mounter::rocket::RocketMounter;
use db_api::mounter::Mounter;
use db_api::retriever::DbRetriever;
use rocket::Rocket;
use rocket_contrib::database;

use std::sync::Arc;

mod handlers;
mod models;
mod schema;

use crate::models::Counter;

// ROCKET GLUE CODE

#[database("rocket_example_pgsql")]
/// Rocket bridge to access the PostgreSQL connection
struct ExampleDb(diesel::pg::PgConnection);

/// The pooled connection that we get as access to the database
type Conn =
    diesel::r2d2::PooledConnection<diesel::r2d2::ConnectionManager<diesel::pg::PgConnection>>;

/// Type alias to our database   
type MyDbRetriever = DbRetriever<diesel::pg::Pg, ExampleDb>;

impl ConnectionRetriever<diesel::pg::Pg> for ExampleDb {
    type Output = Conn;
    fn retrieve_connection(self) -> Self::Output {
        self.0
    }
}

fn main() {
    let rocket = Rocket::ignite()
        .manage(Arc::new(Counter::new()))
        .attach(ExampleDb::fairing());
    let mut mounter = RocketMounter::new(rocket);
    mounter
        .mount_service(handlers::endpoint_counter())
        .mount_service(handlers::endpoint_deser())
        .mount_service(handlers::endpoint_count_heroes())
        .mount_service(handlers::endpoint_multiple_retrievers())
        .mount_service(handlers::endpoint_url_param())
        .mount_service(handlers::endpoint_void());
    #[cfg(debug_assertions)]
    {
        mounter.mount_service(handlers::body::endpoint_body());
    }
    let rocket = mounter.finish();
    rocket.launch();
}
