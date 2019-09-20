//! Route abstraction layer

/// Route abstraction
pub struct Endpoint<Params, Resp> {
    pub url: String,
    pub method: http::Method,
    pub handler: Box<dyn Fn(Params) -> Resp>,
}

/// Trait to be implemented by Routes types.
/// Each route can take any kind of Input they want, but
/// they always have to return the same kind of Response to similar inputs.  
pub trait IntoEndpoint<Params> {
    /// The response type of this route.
    type Resp;

    /// Tranforms this structure into a Route, correctly parametrized
    fn into_endpoint(self) -> Endpoint<Params, Self::Resp>;
}
