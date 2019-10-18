# Parameters & Retrievers

## Design

Each callback function that implements the logic of an endpoint has to work with some kind of parameters. These needs to be retrieved 
when the callback is called. There are 3 ways to retrieve a parameter : 

* From the state of the application (e.g. a DB connection),
* From the body of the request (e.g. some JSON definition),
* From the URL (e.g. an ID to search for on the DB)

to that intent, retrievers are provided (but not implemented) by the system : 
* StateRetriever
* BodyRetriever
* IndexedParamRetriever
* DeserializeRetriever
* UniqueStateRetriever

Some Retriever interface must be provided, taking generics as input : each implementation must take a "Backend", an error type, and some type to output. 
That backend will be used as the data source to retrieve other informations. 

Each backend should be able to give access to every "way" of retrieving parameters, or provide a fallback implementation if not applicable.

No mutable access should be given to the backend, so as to potentially parse all parameters in different threads. 

## Implementation details

Backends should use interior mutability if needed.
Each feature-gate hides the implementation of these retrievers, so as not to bloat the library.

## Problems

* Interior mutability on Backends : is it needed ? 