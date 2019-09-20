trait Handler1<Backend, AR, A, Rep> { 
    fn execute(&self, backend: &Backend, a_retriever: AR) -> Rep; 
}

impl<Backend, A, AR: Retriever<Backend, A>, Rep, F: Fn(A) -> Rep> Handler1<Backend, AR, A, Rep> for Box<F> {
    fn execute(&self, backend: &Backend, a_retriever: AR) -> Rep {
        let a: A = a_retriever.retrieve(&backend);
        self(a)
    }
}

trait Handler2<Backend, AR, A, BR, B, Rep> {
    fn execute(&self, backend: &Backend, a_retriever: AR, b_retriever: BR) -> Rep; 
}

impl<Backend, A, AR: Retriever<Backend, A>, B, BR: Retriever<Backend, B>, Rep, F: Fn(A, B) -> Rep> Handler2<Backend, AR, A, BR, B, Rep> for F {
    fn execute(&self, backend: &Backend, a_retriever: AR, b_retriever: BR) -> Rep {
        let a: A = a_retriever.retrieve(&backend);
        let b: B = b_retriever.retrieve(&backend);
        self(a, b)
    }
}