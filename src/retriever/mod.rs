pub trait Retriever<'a, Backend, Error> {
    type Output;
    fn retrieve(&'a self, backend: &'a Backend) -> Result<Self::Output, Error>;
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

#[cfg(feature = "rocket")]
pub mod rocket;
