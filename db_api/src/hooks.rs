/// A Hook is an element of the pipeline that is going to be applied
/// to a route in a repository.
pub trait Hook<Repository> {
    type Input;
    type Output;
    type Error;

    fn handle(
        &self,
        input: Self::Input,
        repository: &Repository,
    ) -> Result<Self::Output, Self::Error>;
}

/// Tuple implementations allows multiple hooks to compose a pipeline
impl<A, B, In, OA, OB, Repository, Error> Hook<Repository> for (A, B)
where
    A: Hook<Repository, Input = In, Output = OA, Error = Error>,
    B: Hook<Repository, Input = OA, Output = OB, Error = Error>,
{
    type Input = In;
    type Output = OB;
    type Error = Error;

    fn handle(
        &self,
        input: Self::Input,
        repository: &Repository,
    ) -> Result<Self::Output, Self::Error> {
        #[allow(non_snake_case)]
        let (A, B) = self;
        A.handle(input, repository)
            .and_then(|res| B.handle(res, repository))
    }
}
