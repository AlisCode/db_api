# Services 

## Design 

Services are to be implemented independently for each supported Web-Framework. They provide the glue-code that is necessary to bind a given Repository's routes to the actual Web Framework. 

The only kind of state they sould own is backend-specific configuration. They should be created and consumed immediately to be used at the creation of the application in order to mount routes on the app's server. 

The role of a service is to expose a repository by mouting the routes that it provides (via its `mount(m: Mounter, base: &str)` implementation).

## Implementation details

Due to Web-Frameworks handling routes in different ways, it is not possible to design a fully-agnostic solution, though it should be recommended to implement services using Macros or procedural macros, transforming Repositories into Service structures, that then gives reference to their wrapped repositories to the App and are consumed to mount routes.