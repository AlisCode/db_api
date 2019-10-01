use crate::retriever::Retriever;

trait Handler1<'a, Backend, AR, A, AM, Rep, Error> {
    fn execute(&self, backend: &'a mut Backend, a_retriever: &'a AR) -> Result<Rep, Error>;
}

impl<
        'a,
        Backend,
        Error,
        A,
        AM,
        AR: Retriever<'a, Backend, Error, Output = A>,
        Rep,
        F: Fn(A) -> Result<Rep, Error>,
    > Handler1<'a, Backend, AR, A, AM, Rep, Error> for Box<F>
{
    fn execute(&self, backend: &'a mut Backend, a_retriever: &'a AR) -> Result<Rep, Error> {
        let a: A = a_retriever.retrieve(backend)?;
        self(a)
    }
}

trait Handler2<'a, Backend, AR, A, AM, BR, B, BM, Rep, Error> {
    fn execute(&self, backend: &'static mut Backend, a_retriever: &'a AR, b_retriever: &'a BR) -> Result<Rep, Error>;
}

impl<
        'a,
        Backend,
        Error,
        A,
        AM,
        AR: Retriever<'a, Backend, Error, Output = A>,
        B,
        BM,
        BR: Retriever<'a, Backend, Error, Output = B>,
        Rep,
        F: Fn(A, B) -> Result<Rep, Error>,
    > Handler2<'a, Backend, AR, A, AM,BR, B, BM, Rep, Error> for Box<F>
{
    fn execute(&self, backend: &'static mut Backend, a_retriever: &'a AR, b_retriever: &'a BR) -> Result<Rep, Error> {
        let a: A = a_retriever.retrieve(backend)?;
        let b: B = b_retriever.retrieve(backend)?;
        self(a, b)
    }
}