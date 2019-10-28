use diesel::backend::Backend;
use diesel::Connection;

pub trait ConnectionRetriever<B: Backend> {
    type Output: Connection<Backend = B>;
    fn retrieve_connection(&self) -> Self::Output;
}
