//! An effort to abstract different Web Frameworks' Responder types
use serde::ser::Serialize;

pub struct StreamResponse<T> {
    pub inner: T,
}

impl<T> StreamResponse<T> {
    pub fn new(inner: T) -> Self {
        StreamResponse { inner }
    }
}

pub struct SerializeResponse<T> {
    pub inner: T,
}

impl<T: Serialize> SerializeResponse<T> {
    pub fn new(inner: T) -> Self {
        SerializeResponse { inner }
    }
}

#[cfg(feature = "rocket_integ")]
pub mod rocket;
