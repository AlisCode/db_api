pub trait RetrieverBackend {}

pub trait Retriever<'a, Backend: 'a, Error> {
    type Output;
    fn retrieve(&self, backend: &Backend) -> Result<Self::Output, Error>;
}

/// Allows to retrieve a group of retrievers
/// with only one call on a tuple
macro_rules! impl_retriever_multiple {
    ($($vars:ident),+) => {
        impl<'a, Backend: 'a, Error, $( $vars ),+> Retriever<'a, Backend, Error> for ($($vars),+) where $( $vars: Retriever<'a, Backend, Error> ),+ {
            type Output = ($( $vars::Output ),+);

            fn retrieve(&self, backend: &Backend) -> std::result::Result<Self::Output, Error> {
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

pub struct NamedParamRetriever<T> {
    _name: String,
    _phantom: std::marker::PhantomData<T>,
}

impl<T> NamedParamRetriever<T> {
    pub fn new(name: String) -> Self {
        NamedParamRetriever {
            _name: name,
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
