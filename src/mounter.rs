use crate::endpoint::Endpoint;

/// A Mounter is a struct capable of mouting `Route`s on an instance
/// of a Web App Framework, e.g. Rocket's `Rocket` or actix-web's `App`
pub trait Mounter {
    /// Mounts the given Endpoint on the given app Instance
    fn mount<P, R>(self, endpoint: Endpoint<P, R>, base: &str) -> Self;
}
