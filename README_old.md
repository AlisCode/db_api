# API creator 

## Todo (goals): 

* Routes abstractions
* "Repositories" (CRUDRepository)
* Route exporters 
    * Rocket, 
    * Actix 
* Read/Write access
* API scope access 

## Core values 

* Simplicity (the API should be boilerplate-free allowing for quick and efficient prototyping)
* Extensibility (supporting more and more frameworks should be easy)
* Retro-compatibility (supporting older frameworks should not be a problem e.g. actix 0.7 and actix 1.0)
* Async-readyness (being generic over async & sync routes)

## Design 

### API 

The basic workflow should be as follows : 
* The user creates his model using Diesel 
* The user annotates his model structs using the procedural macros 
* The user optionally creates his own repository around a particular struct by implementing the Repository trait. 
* Using various Route exporters, the user transforms the framework's backend-agnostic routes into fully usable routes for any given backend. Support for any web framework should be feature-gated or put in a separate crate. 
* The user mounts the previously generated routes in the application, either by hand or using macros. 

### Route abstraction

#### Routes 

What is a route ? 
A Route is an endpoint on a web application. After being reached, it launches business logic to parse informations from the HTTP Request and after some implied processing, returns a Response.

Each framework has its own implementation of endpoints, but we can list common properties to provide a high-level abstraction that will be completed by the exporters.

* HTTP Method ( GET | POST | PUT | DELETE for example )
* URL ( /api/users for example )
* Params :
    * Contained in URL 
    * Contained in request's content

HTTP Method and URL are trivial since they're contained in [the `http` crate](https://docs.rs/http/0.1.18/http). 

#### Params

Params are non-trivial and dependent on the web framework's implementation, 
though they can be generalized as being retrieved either from : 
* The URL 
* The body of the Request
 
RouteParams need a type information, and a retrieve method for said type. They also need to provide a handler containing the logic of what happens when the endpoint is reached.  

We need to store types in a list. There's multiple ways we could do that : 
* We can use a Vec<RouteParams> and have RouteParams' type info stored as Box<Any>. That would probably require downcasting further down the line, so it probably isn't perfect. 
* We can spread params in the route with indices, resulting in a generic Params struct, and a Route struct that would look like this: 

```rust

pub struct Route1<A> {
    url: URI,
    http_method: HTTPMethod,
    params_0: Param<A>,
    handler: Fn(a: A) -> http::Response, 
}

pub struct Route2<A,B> {
    url: URI,
    http_method: HTTPMethod,
    params_0: Param<A>,
    params_1: Param<B>,
    handler: Fn(a: A, b: B) -> http::Response,
}

/// ... and so on
``` 

Easily implemented with a macro.  
Implies a limit on how much params we can put in a route, but then again don't Rocket and Actix do that themselves ?


#### Async routes 

Routes could possibly be asynchronous. To support this, the framework should also provide `AsyncRoute`s, which would use their handler to return a `std::future::Future<Output=Response>` 

#### Security 

A route may be protected by a set of rules : 
* API Scope (can the user's token vouch for access to this route)
* Validators based on some business logic, potentially needing the Database to be verified. Said validators shall be generic enough that they can be implemented at user's level and could eventually rely on the underlying framework's guards impl. 

### Repositories 

This is the main goal of this crate!
Repositories are routes creator. They should usually be derived and parametrized from a main struct, and can possibly be created by hand to provide more flexibility. In that case, it should be by creating a struct annotated with the "CustomRepository" derive, and implementing the Repository trait for it. 

Repositories should provide a gen_routes() function that returns an iterator of Routes.  

### Route exporters

Route exporters should usually be implemented as macros. They have to create structs which will impl some functions defined by the routes being created by the repository that the macro is handling. 

For example, say that we are using Rocket and that the CrudRepository proc macro has produced a UserCrudRepository struct. We should call the macro as `create_rocket_routes!(UserCrudRepository)` for example. 
Then we can just mount these using `mount_rocket_routes!(rocket_instance);`.

These two macros would be feature-gated behind the "rocket" feature of this crate. 

These macros are not standardized by the framework ; not forcing a particular design on this API allows for more flexibility, given that a particular web framework might change the way they handle routes from one version to another (looking at you, actix-web!)

### Read/Write access / API Scope Access

**Problem**: How should we handle the access to a route ? 

One solution : At the route level, storing the Scope as a generic resulting in routes like this :  

```rust
pub struct RouteX<Scope, A, ..> {
	url: URI,
	method: http::Method,
	params_0: A,
	handler: Fn(a: A, ..) -> http::Response
	scope: Scope,
}
```
The Scope **has to** be defined by the user (or auto-generated containing only a Scope::Global value). Each scope should be added by the derives)
