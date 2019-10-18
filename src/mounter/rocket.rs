use rocket::data::Data;
use rocket::handler::Outcome;
use rocket::response::Responder;
use rocket::{Handler, Request, Rocket, Route};

use crate::endpoint::Endpoint;
use crate::mounter::Mounter;
use crate::retriever::rocket::{RocketRetriever, RocketRetrieverError};
use crate::retriever::{Retriever, RetrieverBackend};
use crate::service::Service;

#[derive(Clone)]
pub struct RocketHandler<FH, FR> {
    pub handler: FH,
    pub retrievers: FR,
}

impl<I, R, IR> Handler for RocketHandler<fn(I) -> R, fn() -> IR>
where
    for<'s> R: Responder<'s>,
    for<'a, 'r> IR: 'static + Retriever<RocketRetriever<'a, 'r>, RocketRetrieverError, Output = I>,
    I: 'static,
    R: 'static,
{
    fn handle<'s>(&self, req: &'s Request, data: Data) -> Outcome<'s> {
        let retrievers = (self.retrievers)();
        let backend: RocketRetriever = RocketRetriever::new(req, data);
        let input: Result<I, RocketRetrieverError> = retrievers.retrieve(&backend);
        match input {
            Ok(i) => Outcome::from(req, (self.handler)(i)),
            Err(e) => Outcome::failure(rocket::http::Status::InternalServerError),
        }
    }
}

impl<I, R, IR> RocketHandler<fn(I) -> R, fn() -> IR>
where
    for<'s> R: Responder<'s>,
    for<'a, 'r> IR: 'static + Retriever<RocketRetriever<'a, 'r>, RocketRetrieverError, Output = I>,
    I: 'static,
    R: 'static,
{
    pub fn new(handler: fn(I) -> R, retrievers: fn() -> IR) -> Self {
        RocketHandler {
            handler,
            retrievers,
        }
    }
}

pub struct RocketMounter<'a, 'r> {
    rocket: Option<Rocket>,
    _lifetime_a: std::marker::PhantomData<&'a str>,
    _lifetime_r: std::marker::PhantomData<&'r str>,
}

impl<'a, 'r> RocketMounter<'a, 'r> {
    pub fn add_route(&mut self, url: &str, route: Route) {
        let rocket = self.rocket.take().unwrap();
        let rocket = rocket.mount(url, vec![route]);
        self.rocket = Some(rocket);
    }

    pub fn new(rocket: Rocket) -> Self {
        RocketMounter {
            rocket: Some(rocket),
            _lifetime_a: std::marker::PhantomData,
            _lifetime_r: std::marker::PhantomData,
        }
    }

    pub fn finish(mut self) -> Rocket {
        self.rocket.take().unwrap()
    }
}

impl<'a, 'r: 'a> Mounter for RocketMounter<'a, 'r> {
    type Back = RocketRetriever<'a, 'r>;

    fn mount_service<S: Service<Self>>(&mut self, service: S) {
        service.mount_service(self);
    }
}

impl<'a, 'r: 'a, I, R, M, InputRetriever> Service<RocketMounter<'a, 'r>>
    for Endpoint<I, R, *const M, InputRetriever>
where
    for<'s> R: Responder<'s>,
    for<'aa, 'rr> InputRetriever:
        Retriever<RocketRetriever<'aa, 'rr>, RocketRetrieverError, Output = I> + 'static,
    I: 'static,
    R: 'static,
{
    fn mount_service(self, mounter: &mut RocketMounter) {
        let method = self.method;
        let url = self.url;
        let handler = self.handler;
        let retrievers = self.retrievers;
        let rocket_handler = RocketHandler::new(handler, retrievers);
        let route = Route::new(http_into_rocket_method(method), &url, rocket_handler);
        mounter.add_route("/", route);
    }
}

fn http_into_rocket_method(m: http::Method) -> rocket::http::Method {
    match m {
        http::Method::GET => rocket::http::Method::Get,
        http::Method::POST => rocket::http::Method::Post,
        http::Method::PATCH => rocket::http::Method::Patch,
        http::Method::PUT => rocket::http::Method::Put,
        http::Method::DELETE => rocket::http::Method::Delete,
        http::Method::OPTIONS => rocket::http::Method::Options,
        http::Method::TRACE => rocket::http::Method::Trace,
        _ => unimplemented!(),
    }
}
