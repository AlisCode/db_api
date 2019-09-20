# Repositories 

## Design 

Repositories are Database-Accessing structures on top of the ORM layer. 
A Repository must own a state, because it is required to know what validators and hooks it's going to apply before and after the result of the Database queries that it's going to launch.

Because this framework is both Database- and Web-Framework- agnostic, a different implementation has to be provided for each Database that the application wants to support.

The connection to the database is handled by a `DatabaseAccessor`, that each Repository has to own in order to be able to launch database queries at some point (be it in a validator, hook, or for a handler itself). 

## Implementation details

### Rust implementation

Repository should be implemented as a trait. and implemented on every different user-provided Repository.
A Repository impl should provide :

* `fn routes(&self) -> impl Iterator<Item=Route>` 

Giving acccess to the list of routes (access-commands on the Database) that the repository provides. 

Given that certain web-frameworks (e.g. `actix-web`) could require such a struct to be shared between threads for efficiency, a repository should **always** be `Send + Sync` and kept relatively simple in order to avoid high-overhead at startup due to cloning.

### Avoiding boilerplate

The goal is to avoid implementing the Repository trait by hand since it's quite heavy and not very user-friendly.

Every design file has notes on what annotations it should provide at the struct level to make the auto-implementation of the Repository trait possible.