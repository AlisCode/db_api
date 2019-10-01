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
pub trait IntoEndpoint<Backend> {
    /// The response type of this route.
    type Resp;
    /// The input of this route
    type Input;

    /// Tranforms this structure into a Route, correctly parametrized
    fn into_endpoint(self) -> Endpoint<Self::Input, Self::Resp>;
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

#[cfg(feature = "rocket_integ")]
pub mod rocket;
