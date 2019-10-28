# Global idea

```
       mounts  routes
User app <----------- Framework backend
| defines                      ^
| repositories                 |
v                              |
Base framework -----------+ common description of routes & repos
```

The user application:
* Defines repositories via proc macros
* Calls the framework backend's service provider so that endpoints are mountedi and usable in the app

The framework backend:
* Defines its own implementation of services - idiomatic code on the target web-framework's level
* Has to provide either a macro or a function which, given an instance of the Web Framework's struct, creates adequate routes for its services

The base framework:
* Provides the common interface (Routes & Repositories) to all backend
* Provides proc macros to derive and possibly create by hand repositories

```
+------------+            +--------+
| Repository |----------->| Routes |
+------------+            +--------+
     ^ Codegenerates          | Give to
     |                        v
+-------------+         +-----------+
| User config |         | Exporters |
| for a repo  |         +-----------+
+-------------+           |   |   | Forms for each repository
      |                 +-----------+
      |                 |  Service  |
      | defines         +-----------+
      |                       | provides
      |                 +---------------+
+-------------+   use   | Web framework |
| User code   |<------->| endpoints     |
| ( App )     | (mount) +---------------+
+-------------+
```

# Endpoints / Routes

Endpoints are structures capable of holding the logic on the web-server side. They are the combination of:

* an URL, or route
* an HTTP Method
* a list of parameters (arguments matched in the route)
* a "handler", or logic to give an HTTP Response as an answer

```rust
struct EmailValidator {
    rgx: string,
}

impl<'a> Hook<BetterHeroRepository> for EmailValidator {
    type Output = ();
    type Input = ();
    /// ...
}

pub struct BetterHeroRepository {
    requests: Arc<Mutex<u32>>,
    email_validator: EmailValidator,
}


#[method = "get"]
#[url = "/count"]
#[pre_hooks = ["email_validator", "phone_number_validator"]]
pub fn count_route(&self) -> HTTPResponse {
    let lock = self.request.lock().unwrap();
    RouteResponse {
        content: format!("This service has been called {} times", lock),
        status: 200,
    }
}
```


# HTTP Responses

Endpoints should return HTTP responses

Each framework has its own implementation of responders - types that they are able to transform and return as an HTTP Response.

The handler for the endpoint, defined in the Repository, should return an element that is then returned by the service.

# Pipelines - Hooks

Pipelines are user-defined. They specify a way to handle a request on a given endpoint. They are given a reference to the repository, and they can pass informations to one another, so as to e.g. validate inputs, or log informations accordingly.

They are defined at the route level.

# Database access

trait GetDBAccess?
logging?

# MVP

* Routes abstraction
* Repository trait
* Procedural macros to generate (basic) Repository impl
    * One template: REST
    * Possible to add custom endpoints
* Route exporters (endpoints generator) for supported backends
* Two web-framework backends supported
    * Rocket
    * actix-web
* Hooks

# Afterwards

* Async IO
    * DB
    * Pipeline

# What is generated and used when

* Repositories are generated at compile-time (codegen)
* Routes are defined at compile-time, and extracted from repositories at run-time
* Services are defined at compile-time (codegen)
* Services are mounted on the core at runtime

# Technical difficulties

* How should Repositories be implemented? Do we need a state inside a repository? How do we provide pre- and after- hooks on routes (e.g. for logging and stats/monitoring purposes)?
    * Repositories should be used as compile-time helpers to generate codes. They can hold their personal state.
    * Using the pattern of chain of responsibility
    * Services should hold a ref to a Repository so that it holds the state of the service
* How do we provide a flexible-enough integration that we and possibly other users can include services in their projects regarding the fact that they potentially have bigger architecture (e.g. we have executors, actions & commands)
    * Using callbacks?
    * Using a pattern of Chain of Responsibility?
* Security / Access check? How can we protect certain repositories from being accessed if the user should not be able to do a given action?
    * Same principle, chain of responsibility
* Concurrency / Asynchronous programming?
    * All Services, and thus Repositories should be `Send + Sync` so as to be shared between threads. Most web-frameworks will require to be thread-shareable

# Wanted API (MVP)

## Example for given Diesel model

```rust

/// Newtype containing a Hero's ID
pub struct HeroId(i32);

#[table_name = "heroes"]
#[derive(AsChangeset, Serialize, Deserialize, Queryable)]
pub struct Hero {
    pub id: Option<i32>,
    pub name: String,
    pub identity: String,
    pub hometown: String,
    pub age: i32
}

#[derive(Serialize, Deserialize, Insertable)]
pub struct InsertableHero {
    pub name: String,
    pub identity: String,
    pub hometown: String,
    pub age: i32,
}

```

## Defining a basic REST Repository

```rust
#[derive(Repository, Rest)]
#[getter = "Hero"]
#[patcher = "Hero"]
#[poster = "InsertableHero"]
#[id_field = "HeroId"]
#[url = "/heros"]
pub struct HeroRepository {}
```

## Creating and mounting a service

### Using Rocket's backend

```rust
fn main() {
    // Creates the Heroes service
    // RocketAPIServices should impl Into<Vec<Route>>
    let service_heroes = RocketAPIService<HeroRepository>::new();

    // Creates the Rocket instance and launches the web server
    Rocket::ignite()
    .mount(service_heroes)
    .launch();
}
```

### Using actix-web's backend

```rust
fn main() -> std::io::Result<()> {
    // Creates the Heroes service
    // ActixAPIServices should impl HttpServiceFactory
    let service_heroes = ActixAPIService<HeroRepository>::new();

    // Creates the Actix-Web App
    HttpServer::new(|| {
        App::new()
        .service(service_heroes)
    })
    .bind("0.0.0.0:8080")?
    .run()
}

```

## Defining a more complex Repository

Starting with the same Diesel model, we can define a service with a bit more logic inside. Let's for example generate a service that can count how many times it's been called

```rust

#[derive(Repository, Rest)]
#[getter = "Hero"]
#[patcher = "Hero"]
#[poster = "InsertableHero"]
#[id_field = "HeroId"]
#[url = "/better/heros"]
#[pipeline = ["AddOneHook"]]
pub struct BetterHeroRepository {
    requests: Arc<Mutex<u32>>,
}

impl BetterHeroRepository {
    pub fn add_one_request(&self) {
        let mut locked = self.request.lock().unwrap();
        let new = *locked + 1;
        *locked = new;
    }

    #[method = "get"]
    #[url = "/count"]
    pub fn count_route(&self) -> HTTPResponse {
        let lock = self.request.lock().unwrap();
        RouteResponse {
            content: format!("This service has been called {} times", lock),
            status: 200,
        }
    }
}

#[derive(Default)]
pub struct AddOneHook {    }

impl<'a> Hook<BetterHeroRepository> for AddOneHook {
    type Output = ();
    type Input = ();

    fn handle(&self, repository: &'a BetterHeroRepository, input: Self::Input) -> Option<Self::Output> {
        repository.add_one_request();
        Some(())
    }
}

```

