#[macro_use]
extern crate diesel;

use crate::models::Counter;
use db_api::database::ConnectionRetriever;
use db_api::endpoint::GenericEndpoint;
use db_api::mounter::rocket::RocketMounter;
use db_api::mounter::Mounter;
use db_api::retriever::{
    BodyRetriever, DbRetriever, DeserializeRetriever, IndexedParamRetriever, UniqueStateRetriever,
};
use db_api::Method;
use diesel::prelude::*;
use rocket::Rocket;
use rocket_contrib::database;

use std::sync::{Arc, Mutex};

mod handlers;
mod models;
mod schema;

use crate::models::Hero;

#[database("rocket_example_pgsql")]
struct ExampleDb(diesel::pg::PgConnection);

type Conn =
    diesel::r2d2::PooledConnection<diesel::r2d2::ConnectionManager<diesel::pg::PgConnection>>;
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
    mounter.mount_service(handlers::endpoint_body());
    mounter.mount_service(handlers::endpoint_counter());
    mounter.mount_service(handlers::endpoint_deser());
    mounter.mount_service(handlers::endpoint_count_heroes());
    mounter.mount_service(handlers::endpoint_multiple_retrievers());
    mounter.mount_service(handlers::endpoint_url_param());
    mounter.mount_service(handlers::endpoint_void());
    let rocket = mounter.finish();
    rocket.launch();
}
