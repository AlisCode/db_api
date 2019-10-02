# Pipelines 

## Design 

Pipelines are special treatments applied to routes handlers before and after they're actually launched. 

These can apply some validation, logging, or monitoring process. They can also be chained in order to  

These are either Hooks or Validators.

A pipeline can be seen as an application of the [Chain of Responsibility](https://en.wikipedia.org/wiki/Chain-of-responsibility_pattern) pattern.

### Hooks (Middlewares)

Hooks are middlewares, applied to every routes of a given repository, they allow to apply logic before and after the route is used. 

They are given a reference to the repository they are currently acting on, so as to provide context.

Hooks are structures owned by a given repository. In the `Repository::handle()` implementation, one must ensure that these are called in the correct order, even though the type-system should provide some sort of safety.

### Validators 

Validators are structures applied on particular routes of a given repository. They allow to check for some condition, e.g. making sure that an email is properly formatted.

Validators can be chained so as to provide multi-stage verification. They do not, however, have any kind of access to the database and can only act on request's content.

## Implementation details


### Chain of Responsibility 

The Chain of Responsibility pattern can be [easily implemented](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=6a5b4a55acaa5c63034f532b6c6f31a6) in Rust. This pattern is the basis of both the Hooks and Validators. The basics is as follows: 

```rust 
trait ResponsibilityNode {
    type Input;
    type Output;
    
    fn handle(&self, input: Self::Input) -> Option<Self::Output>;
}
```

Validators should follow the above implementation.

Hooks should be implemented as a trait, as mentionned above. But because they have to provide a reference to some `Repository` for context, the trait has to take a generic on a context type that will be passed down the chain. 

### Avoid boilerplate

In the `Repository` implementation, the `handle()` function has to explicitly setup the chain for Middlewares. 

Since our goal is to avoid having to manually implement Repository, we should provide a way to set the Chain 

The `#[prehooks = ["a", "b"]]` and `#[afterhooks = ["c", "d"]]` annotations should be provided at the struct level for each repository. 