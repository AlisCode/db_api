use diesel::backend::Backend;
use rocket::data::{FromData, Transform};
use rocket::request::{FromParam, FromRequest};
use rocket::{Data, Outcome, Request, State};
use rocket_contrib::json::Json;
use serde::Deserialize;
use std::borrow::Borrow;
use std::sync::{Arc, Mutex};

use crate::database::ConnectionRetriever;
use crate::retriever::{
    BodyRetriever, DbRetriever, DeserializeRetriever, IndexedParamRetriever, Retriever,
    RetrieverBackend, StateRetriever, UniqueStateRetriever,
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

impl<'a, 'r, T> Retriever<RocketRetriever<'a, 'r>, RocketRetrieverError> for StateRetriever<T>
where
    T: FromRequest<'a, 'r>,
{
    type Output = T;
    fn retrieve(
        &self,
        backend: &RocketRetriever<'a, 'r>,
    ) -> Result<Self::Output, RocketRetrieverError> {
        match backend.request.guard::<'a, T>() {
            Outcome::Success(s) => Ok(s),
            Outcome::Forward(_) => Err(RocketRetrieverError::Mismatch),
            Outcome::Failure(_) => Err(RocketRetrieverError::Error),
        }
    }
}

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
            Some(Err(_)) => Err(RocketRetrieverError::Error),
            _ => Err(RocketRetrieverError::Mismatch),
        }
    }
}

impl<'a, 'r, BA, BR> Retriever<RocketRetriever<'a, 'r>, RocketRetrieverError>
    for DbRetriever<BA, BR>
where
    BA: Backend,
    BR: FromRequest<'a, 'r> + ConnectionRetriever<BA>,
{
    type Output = BR::Output;
    fn retrieve(
        &self,
        backend: &RocketRetriever<'a, 'r>,
    ) -> Result<Self::Output, RocketRetrieverError> {
        let state_retriever: StateRetriever<BR> = StateRetriever::new();
        let bridge = state_retriever.retrieve(backend)?;
        Ok(bridge.retrieve_connection())
    }
}
