use rocket::{Data, Outcome, Request, State};
use rocket::request::FromRequest;
use rocket::data::{FromData, Transform};
use std::borrow::Borrow;
use std::sync::{Arc, Mutex};

use crate::retriever::{BodyRetriever, NamedParamRetriever, Retriever, RetrieverBackend, StateRetriever};

pub struct RocketRetriever<'r> {
    request: &'r Request<'r>,
    data: Arc<Mutex<Option<Data>>>,
}

impl<'r> RocketRetriever<'r> {
    pub fn new(request: &'r Request<'r>, data: Data) -> Self {
        RocketRetriever {
            request,
            data: Arc::new(Mutex::new(Some(data))),
        }
    }
}

pub enum RocketRetrieverError {
    Mismatch,
    Error,
}

impl<'r> RetrieverBackend for RocketRetriever<'r> {}

/// 'a is the lifetime of the request, 
/// 'r is the lifetime of the borrowed object
/// the request ('a) has a lifetime containing 'r
/// since that struct will be used by the handlers. 
impl<'a, 'r, T> Retriever<'a, RocketRetriever<'r>, RocketRetrieverError>
    for StateRetriever<T>
where
    T: FromRequest<'a, 'r> + Send + Sync + 'static,
{
    type Output = &'r T;
    fn retrieve(
        &'a self,
        backend: &'a RocketRetriever<'r>,
    ) -> Result<Self::Output, RocketRetrieverError> {
        match backend.request.guard::<'a, State<'r, T>>() {
            Outcome::Success(s) => Ok( s.inner() ),
            Outcome::Forward(_) => Err(RocketRetrieverError::Mismatch),
            Outcome::Failure(_) => Err(RocketRetrieverError::Error),
        }
    }
}
 
/*
impl<'a, 'r, T> Retriever<'a, RocketRetriever<'r>, ()> for NamedParamRetriever<T> { 
    type Output = T;
    fn retrieve(&'a self, _backend: &'a RocketRetriever<'r>) -> Result<Self::Output, ()> {
        unimplemented!()
    }
}  
*/

impl<'a, 'r, T, O, B: 'r> Retriever<'a, RocketRetriever<'r>, RocketRetrieverError> for BodyRetriever<T> 
where T: FromData<'r, Owned = O, Borrowed = B>, O: Borrow<B> {
    type Output = T;
    fn retrieve(&'a self, backend: &'a RocketRetriever) -> Result<Self::Output, RocketRetrieverError> {
        let mut data = backend.data.lock().unwrap();
        if data.is_none() {
            return Err(RocketRetrieverError::Mismatch);
        }
        let data_locked = data.take().unwrap();
        let transformed = T::transform(&backend.request, data_locked).owned();
        let transformed = Transform::Owned(transformed);
        match T::from_data(&backend.request, transformed) {
            Outcome::Success(s) => Ok(s),
            Outcome::Forward(_) => Err(RocketRetrieverError::Mismatch),
            Outcome::Failure(_) => Err(RocketRetrieverError::Error),
        } 
    }
}  