use crate::retriever::rocket::{RocketRetriever, RocketRetrieverError};
use crate::retriever::{Retriever, RetrieverBackend};
use crate::mounter::Mounter;
use crate::endpoint::Endpoint;
use crate::service::Service;

use rocket::{Request, Data};
use rocket::response::Responder;
use rocket::handler::Outcome;

pub struct RocketMounter<'a, 'r> { 
    _lifetime_backend: std::marker::PhantomData<&'a str>,
    _lifetime_request: std::marker::PhantomData<&'r str>,
}

impl<'a, 'r> Mounter for RocketMounter<'a, 'r> {
    type Back = RocketRetriever<'r>;

    fn mount_service<S: Service<Self>>(&mut self, service: S) {
       service.mount_service(self); 
    }
}

impl<'a, 'r, I, P, R, M, InputRetriever> Service<RocketMounter<'a, 'r>> for Endpoint<I, P, R, *const M, InputRetriever> 
    where R: Responder<'r>, 
    InputRetriever: Retriever<'a, RocketRetriever<'r>, RocketRetrieverError, Output = I> 
    {
    fn mount_service(self, mounter: &mut RocketMounter<'a, 'r>) {
        // Transforms the endpoint into a specific Route
        // Calls implementation-specific functions to mount
        // said route into the final application 
        let handler = |req: &'a Request<'r>, data: Data| {
            let retrievers = (self.retrievers)();
            let backend: RocketRetriever<'r> = RocketRetriever::new(req, data); 
            let input: Result<I, RocketRetrieverError> = retrievers.retrieve(&backend);
            let res = match input {
                Ok(i) => Outcome::from(req, (self.handler)(i)),
                Err(e) => Outcome::failure(rocket::http::Status::InternalServerError),
            }; 
        };
        unimplemented!();
    } 
}