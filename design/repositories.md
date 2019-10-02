# Repositories 

## Design 

Repositories are Database-Accessing structures on top of the ORM layer. 

A Repository must own a state, because it is required to know what validators and hooks it's going to apply before and after the result of the logic that it's going to do. 

Repositories are hand-implemented (or code-generated). They should implement some sort of interface that would require them to mount all their routes into a mounter.

## Implementation details

### Rust implementation

Repository should be implemented as a trait. and implemented on every different user-provided Repository.
A Repository impl should provide :

* `fn mount(mounter: dyn Mounter, base: &str)` 

Mounting each of the routes (structs that implements IntoEndpoint) that the repository knows of. 

Given that certain web-frameworks (e.g. `actix-web`) could require such a struct to be shared between threads for efficiency, a repository should **always** be `Send + Sync` and kept relatively simple in order to avoid high-overhead at startup due to cloning.

### Avoiding boilerplate

The goal is to avoid implementing the Repository trait by hand since it's quite heavy and not very user-friendly.

Every design file has notes on what annotations it should provide at the struct level to make the auto-implementation of the Repository trait possible.