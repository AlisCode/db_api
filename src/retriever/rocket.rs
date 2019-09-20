use rocket::{Data, Outcome, Request, State};
use rocket::request::FromRequest;

use crate::retriever::{BorrowRetriever, StateRetriever, BorrowedParam};

pub struct RocketRetriever<'r> {
    request: Request<'r>,
    data: Data,
}

pub enum RocketRetrieverError {
    Mismatch,
    Error,
}

/// 'a is the lifetime of the request, 
/// 'r is the lifetime of the borrowed object
/// the request ('a) has a lifetime containing 'r
/// since that struct will be used by the handlers. 
impl<'a, 'r, T: FromRequest<'a, 'r> + Send + Sync + 'r> BorrowRetriever<'a, 'r, RocketRetriever<'r>, T, RocketRetrieverError> for StateRetriever<T> {
    fn retrieve(&self, backend: &RocketRetriever) -> Result<BorrowedParam<'r, T>, RocketRetrieverError> {
        match backend.request.guard::<State<'r, T>>() {
            Outcome::Success(s) => Ok(BorrowedParam { inner: s.inner() }),
            Outcome::Forward(_) => Err(RocketRetrieverError::Mismatch),
            Outcome::Failure(_) => Err(RocketRetrieverError::Error)
        } 
    }
} 
