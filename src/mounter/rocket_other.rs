use crate::retriever::rocket::{RocketRetriever, RocketRetrieverError};
use crate::retriever::Retriever;
use rocket::data::Data;
use rocket::handler::Outcome;
use rocket::response::Responder;
use rocket::{Handler, Request};

#[derive(Clone)]
pub struct RocketHandlerOther<FH, FR> {
    pub method: rocket::http::Method,
    pub url: String,
    pub handler: FH,
    pub retrievers: FR,
}

impl<I, R, IR> Handler for RocketHandlerOther<fn(I) -> R, fn() -> IR>
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

impl<I, R, IR> RocketHandlerOther<fn(I) -> R, fn() -> IR>
where
    for<'s> R: Responder<'s>,
    for<'a, 'r> IR: 'static + Retriever<RocketRetriever<'a, 'r>, RocketRetrieverError, Output = I>,
    I: 'static,
    R: 'static,
{
    pub fn new(
        method: rocket::http::Method,
        url: String,
        handler: fn(I) -> R,
        retrievers: fn() -> IR,
    ) -> Self {
        RocketHandlerOther {
            method,
            url,
            handler,
            retrievers,
        }
    }
}
