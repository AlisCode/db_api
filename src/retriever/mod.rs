pub trait Retriever<Backend, T, Error> {
    fn retrieve(&self, backend: &Backend) -> Result<T, Error>;
}

pub trait BorrowRetriever<'a, 'r, Backend, T, Error> {
    fn retrieve(&self, backend: &Backend) -> Result<BorrowedParam<'r, T>, Error>;
}

pub struct NamedParamRetriever<T> {
    name: String,
    _phantom: std::marker::PhantomData<T>,
}

impl<T> NamedParamRetriever<T> {
    pub fn new(name: String) -> Self {
        NamedParamRetriever {
            name,
            _phantom: std::marker::PhantomData,
        }
    }
}

#[derive(Default)]
pub struct StateRetriever<T> {
    _phantom: std::marker::PhantomData<T>,
}

#[derive(Default)]
pub struct BodyRetriever<T> {
    _phantom: std::marker::PhantomData<T>,
}

pub struct BorrowedParam<'r, T> {
    pub inner: &'r T,
}

#[cfg(feature = "rocket")]
pub mod rocket;
