//! Route abstraction layer

/// Route abstraction
pub struct Endpoint<Input, Resp, Mounter, InputRetriever> {
    /// URL of the endpoint that we're going to match
    pub url: String,
    /// HTTP Method that this endpoint is going to reply to
    pub method: http::Method,
    /// Handler function
    pub handler: fn(Input) -> Resp,
    /// Retrievers
    pub retrievers: fn() -> InputRetriever,
    /// Phantom parameter to constrain Mounter
    _mounter: std::marker::PhantomData<Mounter>,
}

impl<Input, Resp, Mounter, InputRetriever> Endpoint<Input, Resp, Mounter, InputRetriever> {
    pub fn new(
        url: String,
        method: http::Method,
        handler: fn(Input) -> Resp,
        retrievers: fn() -> InputRetriever,
    ) -> Endpoint<Input, Resp, Mounter, InputRetriever> {
        Endpoint {
            url,
            method,
            handler,
            retrievers,
            _mounter: std::marker::PhantomData,
        }
    }
}

/// Route abstraction to be transformed into an endpoint
/// that is generic over a specific mounter
pub struct GenericEndpoint<Input, Resp, InputRetriever> {
    /// URL of the endpoint that we're going to match
    pub url: String,
    /// HTTP Method that this endpoint is going to reply to
    pub method: http::Method,
    /// Handler function
    pub handler: fn(Input) -> Resp,
    /// Retrievers
    pub retrievers: fn() -> InputRetriever,
}

impl<'a, 'b, Input, Resp, InputRetriever> GenericEndpoint<Input, Resp, InputRetriever> {
    /// Creates a new instance of a Generic Endpoint
    pub fn new(
        url: String,
        method: http::Method,
        handler: fn(Input) -> Resp,
        retrievers: fn() -> InputRetriever,
    ) -> GenericEndpoint<Input, Resp, InputRetriever> {
        GenericEndpoint {
            url,
            method,
            handler,
            retrievers,
        }
    }
}

/// Trait to be implemented by Routes types.
/// Each route can take any kind of Input they want
/// Input : To be retrieved using the backend
/// Params : Parameters of the logic handler of this route
/// Resp : Response type of this route
pub trait IntoEndpoint<'a, Input, Params, Resp, InputRetriever> {
    /// Transforms this struct into a usable Endpoint
    fn into_endpoint<M: ?Sized>(self) -> Endpoint<Input, Resp, *const M, InputRetriever>;
    /// Part of the implementation that returns the InputRetriever type
    fn input_retriever() -> InputRetriever;
}
