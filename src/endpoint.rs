//! Route abstraction layer

/// Route abstraction
pub struct Endpoint<Input, Params, Resp, Mounter, InputRetriever> {
    /// URL of the endpoint that we're going to match
    pub url: String,
    /// HTTP Method that this endpoint is going to reply to
    pub method: http::Method,
    /// Handler function
    pub handler: Box<dyn Fn(Input) -> Resp>,
    pub retrievers: Box<dyn Fn() -> InputRetriever>,
    /// Phantom parameter to constrain Params
    _params: std::marker::PhantomData<Params>,
    /// Phantom parameter to constrain Mounter
    _mounter: std::marker::PhantomData<Mounter>,
}

/// Trait to be implemented by Routes types.
/// Each route can take any kind of Input they want
/// Input : To be retrieved using the backend
/// Params : Parameters of the logic handler of this route
/// Resp : Response type of this route
pub trait IntoEndpoint<'a, Input, Params, Resp, InputRetriever> {
    /// Transforms this struct into a usable Endpoint
    fn into_endpoint<M: ?Sized>(self) -> Endpoint<Input, Params, Resp, *const M, InputRetriever>;
    /// Part of the implementation that returns the InputRetriever type
    fn input_retriever() -> InputRetriever; 
}

// Mounter:
// * Input is an Endpoint
// * Create a Route (that the underlying Framework can use)
//      * Closure with input: &request and Data
//      * Create backend
//      * Launch all retrievers on backend
//      * Execute callback (handler), this callback launches the whole pipeline
//      * Wraps the result inside some sort of exporter (to make sure the answer is a Responder)
//      * Returns wrapped result
// * Mounts said created route on the back-end
