# Database Accessors

## Design 

Database Accessors are structures that can be used by Repositories to provide access to some sort of connection to a Database-backend that the ORM layer is capable of using.

Database Accessors should be implemented either in the glue-code for each backend (Web-Framework), or in the application itself if need be (sometimes you need a special Database Access to some particular schemas, as we do in the Impero application for example). 

## Implementation details

Database Accessors should be implemented as traits. They should have a generic type on what Database Connection struct they provide. 

A Database Accessor implementor should provide a `fn provide(&self) -> Connection`.

The DatabaseAccessor trait is: 

* Defined in the base framework
* Implemented most likely on newtypes(wrappers for Database access), because of the Rust limitation that we cannot implement foreign `Trait`s on structures that are not defined in the crate.