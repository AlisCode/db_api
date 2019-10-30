use actix_web::web::Json;
use actix_web::{FromRequest, HttpRequest};
use futures::future::{Future, IntoFuture};
use serde::de::DeserializeOwned;

use crate::retriever::{
    BodyRetriever, DeserializeRetriever, Retriever, RetrieverBackend, StateRetriever,
    UniqueStateRetriever,
};

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

impl<'a, T> Retriever<ActixRetriever<'a>, ActixRetrieverError> for BodyRetriever<T>
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

impl<'a, T> Retriever<ActixRetriever<'a>, ActixRetrieverError> for DeserializeRetriever<T>
where
    T: DeserializeOwned + 'static,
{
    type Output = T;

    fn retrieve(&self, backend: &ActixRetriever<'a>) -> Result<Self::Output, ActixRetrieverError> {
        match Json::<T>::extract(backend.request).into_future().wait() {
            Ok(t) => Ok(t.into_inner()),
            Err(_) => Err(ActixRetrieverError::Error),
        }
    }
}

impl<'a, T> Retriever<ActixRetriever<'a>, ActixRetrieverError> for UniqueStateRetriever<T>
where
    T: 'static,
{
    type Output = &'a T;
    fn retrieve(&self, backend: &ActixRetriever<'a>) -> Result<Self::Output, ActixRetrieverError> {
        match backend.request.app_data() {
            Some(t) => Ok(t),
            None => Err(ActixRetrieverError::Mismatch),
        }
    }
}
