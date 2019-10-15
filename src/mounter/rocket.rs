use crate::retriever::rocket::{RocketRetriever, RocketRetrieverError};
use crate::retriever::{Retriever, RetrieverBackend};
use crate::mounter::Mounter;
use crate::endpoint::Endpoint;
use crate::service::Service;

use rocket::{Handler, Request, Route, Data};
use rocket::response::Responder;
use rocket::handler::Outcome;

pub struct RocketMounter<'a, 'r> { 
    _lifetime_backend: std::marker::PhantomData<&'a str>,
    _lifetime_request: std::marker::PhantomData<&'r str>,
}

impl<'a, 'r: 'a> Mounter for RocketMounter<'a, 'r> {
    type Back = RocketRetriever<'a, 'r>;

    fn mount_service<S: Service<Self>>(&mut self, service: S) {
       service.mount_service(self); 
    }
}

#[derive(Clone)]
pub struct RocketHandler<Input, Resp, InputRetriever> {
    method: rocket::http::Method,
    url: String,
    pub handler: fn(Input) -> Resp,
    pub retrievers: fn() -> InputRetriever
}

impl<'a,'r: 'a, I, R, IR> Handler for RocketHandler<I, R, IR> 
    where R: Responder<'r>, 
    I: Clone + Send + Sync + 'static,
    R: Clone + Send + Sync + 'static,
    IR: 'a + 'static + Retriever<'a, RocketRetriever<'a, 'r>, RocketRetrieverError, Output = I> + Clone  {
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

impl<'a,'r: 'a, I, P, R, M, InputRetriever: 'a> Service<RocketMounter<'a, 'r>> for Endpoint<I, P, R, *const M, InputRetriever> 
    where R: Responder<'r>, 
    InputRetriever: Retriever<'a, RocketRetriever<'a, 'r>, RocketRetrieverError, Output = I> 
    {
    fn mount_service(self, mounter: &mut RocketMounter<'a, 'r>) {
        // Transforms the endpoint into a specific Route
        // Calls implementation-specific functions to mount
        // said route into the final application 
        //let route = Route::new(rocket::http::Method::Get, self.url.clone(), self);
        unimplemented!()
    } 
}