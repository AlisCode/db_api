# Routes

## Design 

Routes are back-end agnostic structures, representing a common way to create HTTP routes. 

These routes will be given to a service that will hold the responsibility of mounting them on the App's router. 

A route should always contain : 

* An **HTTP Method** that it will respond to
* An **URL** to match on 
* **Handler code**, logic that will be executed. This should return a **Response**.
* **Parameters** of the routes (*e.g.* ids or strings matched in the URL, headers, cookies, content ...)

These routes will then be used by a service to create idiomatic routes at the Web-Framework level. 

## Implementation details 

### Rust implementation 

In order to represent the HTTP Method, we should use the `http` crate. 

Handler code should be written by hand in an impl block of the Repository.

Each Route must be declared as a Struct. Said struct will implement IntoEndpoint.
It has the responsibility to call the pipeline in order.  

IntoEndpoint has 3 associated types : 
* Input : the type of the input to be retrieved by the endpoint **before** going in the pipeline
* Params : the type of input for the logic handler defined by the function
* Response : the type returned by the Endpoint

Each Route implemented that way has to provide a `fn retrieve(backend: &Backend) -> Input`

### Avoiding boilerplate 

`Routes` have to provide the URL on which they are going to be matching. 

Instead of having to manually write the full URL to the API, it should be possible to set the URL for a custom Repository. To do so, we should provide an URL Mount annotation, for example : `#[mount_to = "/api/my_url"]`

All Routes of this Repository would then have to be prefixed with that value.

In order to mark each route as to be added in the app by a service, we should provide some sorte of `#[route]` annotation. A fn tagged with that annotation would then be added to the list of routes to be returned by a `Repository` implementation. 

To indicate the URL of a route, one should mark a route with `#[url = "/hero/:id"]`

To indicate the HTTP method of a route, one should mark a route with *e.g* `#[method = "get"]`

The handler code will be the core of a tagged function.

Passing parameters to a Route is done by passing parameters in the function. 