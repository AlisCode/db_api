use serde::Deserialize;

pub trait RetrieverBackend {}

pub trait Retriever<Backend, Error> {
    type Output;
    fn retrieve<'a>(&'a self, backend: &'a Backend) -> Result<Self::Output, Error>;
}

impl<Backend, Error> Retriever<Backend, Error> for () {
    type Output = ();
    fn retrieve<'a>(&'a self, backend: &'a Backend) -> std::result::Result<Self::Output, Error> {
        Ok(())
    }
}

/// Allows to retrieve a group of retrievers
/// with only one call on a tuple
macro_rules! impl_retriever_multiple {
    ($($vars:ident),+) => {
        impl<Backend, Error, $( $vars ),+> Retriever<Backend, Error> for ($($vars),+) where $( $vars: Retriever<Backend, Error> ),+ {
            type Output = ($( $vars::Output ),+);

            fn retrieve<'a>(&'a self, backend: &'a Backend) -> std::result::Result<Self::Output, Error> {
                #[allow(non_snake_case)]
                let ( $( $vars ),+ ) = self;
                Ok(($( $vars.retrieve(backend)? ),+))
            }
        }
    };
}

impl_retriever_multiple!(A, B);
impl_retriever_multiple!(A, B, C);
impl_retriever_multiple!(A, B, C, D);
impl_retriever_multiple!(A, B, C, D, E);
impl_retriever_multiple!(A, B, C, D, E, F);
impl_retriever_multiple!(A, B, C, D, E, F, G);
impl_retriever_multiple!(A, B, C, D, E, F, G, H);

pub struct IndexedParamRetriever<T> {
    index: usize,
    _phantom: std::marker::PhantomData<T>,
}

impl<T> IndexedParamRetriever<T> {
    pub fn new(index: usize) -> Self {
        IndexedParamRetriever {
            index,
            _phantom: std::marker::PhantomData,
        }
    }
}

#[derive(Default, Clone)]
pub struct StateRetriever<T> {
    _phantom: std::marker::PhantomData<T>,
}

impl<T> StateRetriever<T> {
    pub fn new() -> Self {
        StateRetriever {
            _phantom: std::marker::PhantomData,
        }
    }
}

#[derive(Default, Clone)]
pub struct UniqueStateRetriever<T> {
    _phantom: std::marker::PhantomData<T>,
}

impl<T> UniqueStateRetriever<T> {
    pub fn new() -> Self {
        UniqueStateRetriever {
            _phantom: std::marker::PhantomData,
        }
    }
}

#[derive(Default, Clone)]
pub struct DeserializeRetriever<T> {
    _phantom: std::marker::PhantomData<T>,
}

impl<'de, T: Deserialize<'de>> DeserializeRetriever<T> {
    pub fn new() -> Self {
        DeserializeRetriever {
            _phantom: std::marker::PhantomData,
        }
    }
}

#[derive(Default, Clone)]
pub struct BodyRetriever<T> {
    _phantom: std::marker::PhantomData<T>,
}

impl<T> BodyRetriever<T> {
    pub fn new() -> Self {
        BodyRetriever {
            _phantom: std::marker::PhantomData,
        }
    }
}

#[cfg(feature = "rocket")]
pub mod rocket;
