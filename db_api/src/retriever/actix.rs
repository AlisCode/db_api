use actix_web::{FromRequest, HttpRequest};
use futures::future::{Future, IntoFuture};

use crate::retriever::{Retriever, RetrieverBackend, StateRetriever};

pub struct ActixRetriever<'a> {
    request: &'a HttpRequest,
}

impl<'a> ActixRetriever<'a> {
    pub fn new(request: &'a HttpRequest) -> Self {
        ActixRetriever { request }
    }
}

pub enum ActixRetrieverError {
    Mismatch,
    Error,
}

impl<'a> RetrieverBackend for ActixRetriever<'a> {}

impl<'a, T> Retriever<ActixRetriever<'a>, ActixRetrieverError> for StateRetriever<T>
where
    T: FromRequest,
{
    type Output = T;

    fn retrieve(&self, backend: &ActixRetriever<'a>) -> Result<Self::Output, ActixRetrieverError> {
        match T::extract(backend.request).into_future().wait() {
            Ok(t) => Ok(t),
            Err(_) => Err(ActixRetrieverError::Error),
        }
    }
}
