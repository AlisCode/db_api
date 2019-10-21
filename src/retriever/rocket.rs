use rocket::data::{FromData, Transform};
use rocket::request::FromParam;
use rocket::{Data, Outcome, Request, State};
use rocket_contrib::json::Json;
use serde::Deserialize;
use std::borrow::Borrow;
use std::sync::{Arc, Mutex};

use crate::retriever::{
    BodyRetriever, DeserializeRetriever, IndexedParamRetriever, Retriever, RetrieverBackend,
    UniqueStateRetriever,
};

pub struct RocketRetriever<'a, 'r> {
    request: &'a Request<'r>,
    data: Arc<Mutex<Option<Data>>>,
}

impl<'a, 'r> RocketRetriever<'a, 'r> {
    pub fn new(request: &'a Request<'r>, data: Data) -> Self {
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

impl<'a, 'r> RetrieverBackend for RocketRetriever<'a, 'r> {}

/*
/// 'a is the lifetime of the request,
/// 'r is the lifetime of the borrowed object
/// the request ('a) has a lifetime containing 'r
/// since that struct will be used by the handlers.
impl<'a, 'r, T> Retriever<RocketRetriever<'a, 'r>, RocketRetrieverError> for StateRetriever<T>
where
    T: FromRequest<'a, 'r> + Send + Sync + 'static,
{
    type Output = &'r T;
    fn retrieve(
        &self,
        backend: &RocketRetriever<'a, 'r>,
    ) -> Result<Self::Output, RocketRetrieverError> {
        match backend.request.guard::<'a, State<'r, T>>() {
            Outcome::Success(s) => Ok(s.inner()),
            Outcome::Forward(_) => Err(RocketRetrieverError::Mismatch),
            Outcome::Failure(_) => Err(RocketRetrieverError::Error),
        }
    }
}
*/

impl<'a, 'r, T, O, B: 'r> Retriever<RocketRetriever<'a, 'r>, RocketRetrieverError>
    for BodyRetriever<T>
where
    T: FromData<'r, Owned = O, Borrowed = B>,
    O: Borrow<B>,
{
    type Output = T;
    fn retrieve(&self, backend: &RocketRetriever) -> Result<Self::Output, RocketRetrieverError> {
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

impl<'a, 'r, T> Retriever<RocketRetriever<'a, 'r>, RocketRetrieverError> for DeserializeRetriever<T>
where
    for<'de> T: Deserialize<'de>,
{
    type Output = T;
    fn retrieve(&self, backend: &RocketRetriever) -> Result<Self::Output, RocketRetrieverError> {
        let mut data = backend.data.lock().unwrap();
        if data.is_none() {
            return Err(RocketRetrieverError::Mismatch);
        }
        let data_locked = data.take().unwrap();
        let transformed = Json::<T>::transform(backend.request, data_locked);
        let outcome = match transformed {
            Transform::Borrowed(Outcome::Success(ref v)) => {
                Transform::Borrowed(Outcome::Success(v.borrow()))
            }
            _ => unimplemented!(),
        };
        match Json::<T>::from_data(backend.request, outcome) {
            Outcome::Success(s) => Ok(s.into_inner()),
            Outcome::Forward(_) => Err(RocketRetrieverError::Mismatch),
            Outcome::Failure(_) => Err(RocketRetrieverError::Error),
        }
    }
}

impl<'a, 'r, T> Retriever<RocketRetriever<'a, 'r>, RocketRetrieverError> for UniqueStateRetriever<T>
where
    T: Send + Sync + 'static,
{
    type Output = Arc<T>;
    fn retrieve(
        &self,
        backend: &RocketRetriever<'a, 'r>,
    ) -> Result<Self::Output, RocketRetrieverError> {
        match backend.request.guard::<'a, State<'r, Arc<T>>>() {
            Outcome::Success(s) => Ok(s.inner().clone()),
            Outcome::Forward(_) => Err(RocketRetrieverError::Mismatch),
            Outcome::Failure(_) => Err(RocketRetrieverError::Error),
        }
    }
}

impl<'a, 'r, T> Retriever<RocketRetriever<'a, 'r>, RocketRetrieverError>
    for IndexedParamRetriever<T>
where
    T: FromParam<'a>,
{
    type Output = T;
    fn retrieve(
        &self,
        backend: &RocketRetriever<'a, 'r>,
    ) -> Result<Self::Output, RocketRetrieverError> {
        let res = backend.request.get_param(self.index);
        match res {
            Some(Ok(t)) => Ok(t),
            Some(Err(e)) => Err(RocketRetrieverError::Error),
            _ => Err(RocketRetrieverError::Mismatch),
        }
    }
}
