# Responses

## Design

Each web-framework provides its own implementation of Responders to Routes.
This is usually based on the return type of a function (the handler).

## Implementation details

In order to be `mount`-able on a backend, routes' associated Response type must fulfill some sort of condition, e.g. for Rocket the must implement
the `Responder` trait. 

This should be guaranteed by services at compile-time, since the implementation of `mount`, taking some struct that implements `IntoEndpoint` is constrained. This allows every backend to put special requirements, such as `Output: Responder` in the case of Rocket.  